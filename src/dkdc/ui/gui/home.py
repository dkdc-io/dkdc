# imports
from shiny import ui, module, render, reactive

from dkdc_util import now


@module.ui
def home_page():
    return (
        ui.br(),
        ui.layout_columns(
            ui.card(ui.card_header("welcome"), ui.output_text("welcome_text")),
            ui.card(ui.card_header("time"), ui.output_text("current_time")),
        ),
    )


@module.server
def home_server(input, output, session, username):
    @render.text
    def welcome_text():
        u = username.get() or "{username}"
        return f"Welcome, {u}!"

    @render.text
    def current_time():
        current_time = now()
        reactive.invalidate_later(1)
        return f"{current_time:%H:%M:%S}"
