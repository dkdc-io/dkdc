# imports
import plotly.express as px

from shiny import ui, render, reactive, module

# global state

# plotly dark mode
px.defaults.template = "plotly_dark"


# individual chat card
@module.ui
def chat_card(header, body):
    return ui.card(
        ui.card_header(header),
        ui.markdown(body),
        ui.chat_ui("chat"),
        ui.layout_columns(
            ui.input_action_button("done", "done", class_="btn-primary"),
            ui.input_action_button("delete", "delete", class_="btn-danger"),
        ),
    )


@module.server
def chat_card_server(input, output, session):
    chat = ui.Chat(
        id="chat",
        messages=[
            {
                "role": "assistant",
                "content": "hello!",
            }
        ],
    )

    def _get_id(session):
        # TODO: this isn't ideal (?)
        return str(session.ns).split("-")[-1]

    @reactive.Effect
    @reactive.event(input.done, ignore_init=True)
    def _done():
        id = _get_id(session)
        print(f"done: {id}")

    @reactive.Effect
    @reactive.event(input.delete, ignore_init=True)
    def _delete():
        id = _get_id(session)
        print(f"delete: {id}")

    @chat.on_user_submit
    async def _():
        user = chat.user_input()
        await chat.append_message(f"you: {user}")


# page of chat
@module.ui
def chat_page():
    return (ui.br(), ui.output_ui("chat_list"))


@module.server
def chat_server(input, output, session, username):
    # reactive values
    _chat_modified = reactive.Value(0)

    # servers
    chat_card_server("chat_card")

    # effects
    @render.ui
    def stats():
        return ui.markdown("stats")

    @render.ui
    def chat_list():
        return chat_card("chat_card", header="chat", body="hi")
