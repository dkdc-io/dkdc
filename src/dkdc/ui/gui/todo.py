# imports
import plotly.express as px

from shiny import ui, render, reactive, module
from shinywidgets import output_widget, render_widget

from dkdc_todo import Todo
from dkdc_util import uuid_parts
from dkdc_state import ibis

# global state
todo = Todo()

# plotly dark mode
px.defaults.template = "plotly_dark"


# individual todo card
@module.ui
def todo_card(header, body, priority):
    return ui.card(
        ui.card_header(header),
        ui.layout_columns(
            ui.markdown(body),
            ui.input_numeric("priority", "priority", value=priority, min=0, max=100),
        ),
        ui.layout_columns(
            ui.input_action_button("done", "done", class_="btn-primary"),
            # TODO: implement edit flow (modal?) (perhaps clear all flow w/ confirmation while we're here)
            ui.input_action_button("edit", "edit", class_="btn-info"),
            ui.input_action_button("delete", "delete", class_="btn-danger"),
        ),
    )


@module.server
def todo_card_server(input, output, session, todos_modified):
    def _get_id(session):
        # TODO: this isn't ideal (?)
        return str(session.ns).split("-")[-1]

    @reactive.Effect
    @reactive.event(input.priority, ignore_init=True)
    def _update_priority():
        id = _get_id(session)
        t = todo.get_todo(id=id)
        todo.update_todo(
            id=id,
            user_id=t["user_id"],
            subject=t["subject"],
            body=t["body"],
            priority=input.priority(),
            status=t["status"],
            description=t["description"],
            labels=t["labels"],
        )
        todos_modified()

    @reactive.Effect
    @reactive.event(input.done, ignore_init=True)
    def _done():
        id = _get_id(session)
        t = todo.get_todo(id=id)
        todo.update_todo(
            id=id,
            user_id=t["user_id"],
            subject=t["subject"],
            body=t["body"],
            priority=t["priority"],
            status="done",
            description=t["description"],
            labels=t["labels"],
        )
        todos_modified()

    @reactive.Effect
    @reactive.event(input.edit, ignore_init=True)
    def _edit():
        id = _get_id(session)
        t = todo.get_todo(id=id)
        ui.modal_show(
            ui.modal(
                ui.input_text_area("todo_edit", "todo", value=t["body"]),
                ui.input_action_button(
                    "todo_edit_submit", "submit", class_="btn-primary"
                ),
                title=f"editing {id}",
                easy_close=True,
            )
        )

    @reactive.Effect
    @reactive.event(input.todo_edit_submit, ignore_init=True)
    def _todo_edit_submit():
        id = _get_id(session)
        t = todo.get_todo(id=id)
        todo.update_todo(
            id=id,
            user_id=t["user_id"],
            subject=t["subject"],
            body=input.todo_edit(),
            priority=t["priority"],
            status=t["status"],
            description=t["description"],
            labels=t["labels"],
        )
        todos_modified()
        ui.modal_remove()

    @reactive.Effect
    @reactive.event(input.delete, ignore_init=True)
    def _delete():
        id = _get_id(session)
        t = todo.get_todo(id=id)
        todo.update_todo(
            id=id,
            user_id=t["user_id"],
            subject=t["subject"],
            body=t["body"],
            priority=t["priority"],
            status="deleted",
            description=t["description"],
            labels=t["labels"],
        )
        todos_modified()


# page of todos
@module.ui
def todo_page():
    return ui.page_fluid(
        ui.br(),
        ui.layout_columns(
            ui.card(
                ui.card_header("add todo"),
                ui.layout_columns(
                    ui.input_text_area("todo", "todo"),
                    ui.input_slider("priority", "priority", value=100, min=0, max=100),
                ),
                ui.layout_columns(
                    ui.input_action_button("add", "add", class_="btn-primary"),
                    ui.input_action_button("clear", "clear all", class_="btn-danger"),
                ),
            ),
            ui.card(
                ui.card_header("todos stats"),
                ui.output_ui("stats"),
            ),
        ),
        ui.card(
            ui.card_header("todos"),
            ui.output_ui("todos_list"),
        ),
    )


@module.server
def todo_server(input, output, session):
    # reactive values
    todo_modified = reactive.Value(0)

    # servers
    [
        todo_card_server(
            t["id"], todos_modified=lambda: todo_modified.set(todo_modified.get() + 1)
        )
        for t in todo.get_todos()
    ]

    # effects
    @render.ui
    def stats():
        _ = todo_modified.get()
        return ui.markdown(
            f"total todos: {todo.t().filter(ibis._["status"].isnull()).count().to_pyarrow().as_py()}"
        ), output_widget("status_plot")

    @render_widget
    def status_plot():
        _ = todo_modified.get()
        t = todo.t()
        c = px.pie(
            t.fill_null({"status": "todo"})
            .group_by("status")
            .agg(count=ibis._.count())
            .order_by("status"),
            names="status",
            values="count",
            color="status",
            # TODO: match colors to buttons
            color_discrete_map={
                "todo": "turquoise",
                "done": "purple",
                "deleted": "red",
            },
        )
        return c

    @reactive.Effect
    @reactive.event(input.add, ignore_init=True)
    def _add():
        _, id = uuid_parts()
        todo_text = input.todo()
        todo_priority = input.priority()
        todo.append_todo(
            id=id,
            user_id=None,
            subject=None,
            body=todo_text,
            priority=todo_priority,
        )
        todo_card_server(
            id, todos_modified=lambda: todo_modified.set(todo_modified.get() + 1)
        )
        ui.update_text("todo", value="")
        ui.update_slider("priority", value=100)
        todo_modified.set(todo_modified.get() + 1)

    @reactive.Effect
    @reactive.event(input.clear, ignore_init=True)
    def _clear():
        t = todo.t().filter(ibis._["status"].isnull())
        ui.modal_show(
            ui.modal(
                ui.markdown("are you sure?"),
                ui.markdown(f"this will delete {t.count().to_pyarrow().as_py()} todos"),
                ui.input_action_button("clear_confirmed", "yes", class_="btn-danger"),
                title="clear all todos",
                easy_close=True,
            )
        )

    @reactive.Effect
    @reactive.event(input.clear_confirmed, ignore_init=True)
    def _clear_confirmed():
        for t in todo.t().filter(ibis._["status"].isnull()).to_pyarrow().to_pylist():
            todo.update_todo(
                id=t["id"],
                user_id=t["user_id"],
                subject=t["subject"],
                body=t["body"],
                priority=t["priority"],
                status="deleted",
                description=t["description"],
                labels=t["labels"],
            )
        todo_modified.set(todo_modified.get() + 1)
        ui.modal_remove()

    @render.ui
    def todos_list():
        _ = todo_modified.get()
        return (
            ui.layout_columns(
                *[
                    todo_card(t["id"], t["id"], t["body"], t["priority"])
                    for t in todo.t()
                    .filter(ibis._["status"].isnull())
                    .to_pyarrow()
                    .to_pylist()
                ],
                col_widths=(4, 4, 4),
            ),
        )
