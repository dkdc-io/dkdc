# imports
from shiny import App, ui, reactive, render
from shinyswatch import theme

from dkdc.ui.gui.todos import todo_page, todo_server

# gui
gui = ui.page_fluid(
    ui.output_ui("main"),
    theme=theme.pulse,
)


# server
def gui_server(input, output, session):
    # setup reactive values
    _val = reactive.Value(None)

    # WARNING: hack for testing only (so you don't have to login/use a test user)
    # current_username.set("test")

    # callbacks
    def _whatever(u):
        _val.set(u)

    # servers
    todo_server("todo")

    # global effects
    @render.ui
    def main():
        elements = [ui.nav_panel("todo", todo_page("todo"))]
        elements += [
            ui.nav_spacer(),
            ui.nav_control(
                ui.input_dark_mode(),
            ),
        ]

        return ui.navset_bar(
            *elements,
            title=ui.a("dkdc-io", href="https://dkdc.io", class_="navbar-brand"),
            id="navbar",
            selected="todo",
        )


# app
app = App(gui, gui_server)
