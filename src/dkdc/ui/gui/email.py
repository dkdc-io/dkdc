# imports
from shiny import ui, module


@module.ui
def email_page():
    return (ui.br(), ui.markdown("email"))
