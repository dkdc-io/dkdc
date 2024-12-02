# imports
from shiny import ui, module, reactive


@module.ui
def analytics_page():
    elements = [
        ui.nav_panel("analytics", main_page("analytics")),
    ]

    return (
        ui.br(),
        ui.card(
            ui.input_action_button(
                "to_home", "home", class_="btn-secondary", width="100%"
            )
        ),
        ui.navset_pill(*elements, id="navpill"),
    )


@module.ui
def main_page():
    return (
        ui.br(),
        ui.layout_columns(
            ui.card(
                ui.card_header("sessions"),
                ui.markdown("so many!"),
            ),
            ui.card(
                ui.card_header("logins"),
                ui.markdown("wow!"),
            ),
        ),
    )


@module.server
def analytics_server(input, output, session, _to_home, username):
    @reactive.Effect
    @reactive.event(input.to_home, ignore_init=True)
    def to_home():
        _to_home()
