# imports
from shiny import ui, module


@module.ui
def email_page():
    return (ui.br(), ui.card(ui.card_header("emails"), ui.markdown("email")))
