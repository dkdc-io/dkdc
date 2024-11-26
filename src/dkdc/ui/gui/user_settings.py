# imports
from shiny import ui, module, reactive


@module.ui
def user_settings_page():
    elements = [
        ui.nav_panel("user info", user_page("user_settings")),
        ui.nav_panel("authentication", auth_page("user_settings")),
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
def user_page():
    return (
        ui.br(),
        ui.card(
            ui.card_header("username"),
            ui.markdown("dkdc"),
        ),
    )


@module.ui
def auth_page():
    return (
        ui.br(),
        ui.layout_columns(
            ui.card(
                ui.card_header("emails"),
                ui.markdown("emails"),
            ),
            ui.card(
                ui.card_header("OpenAI API keys"),
                ui.markdown("OpenAI API key"),
            ),
        ),
    )


@module.server
def user_settings_server(input, output, session, _to_home):
    @reactive.Effect
    @reactive.event(input.to_home, ignore_init=True)
    def to_home():
        _to_home()
