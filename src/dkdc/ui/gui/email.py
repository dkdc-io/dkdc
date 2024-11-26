# imports
from shiny import ui, module


@module.ui
def email_page():
    return ui.page_fluid(ui.br(), ui.markdown("email"))
