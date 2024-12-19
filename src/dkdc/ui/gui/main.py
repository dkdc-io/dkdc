# imports
from enum import Enum

from shiny import App, ui, reactive, render
from shinyswatch import theme
from urllib.parse import urlparse

from dkdc.ui.gui.home import home_page, home_server
from dkdc.ui.gui.todo import todo_page, todo_server
from dkdc.ui.gui.chat import chat_page, chat_server
from dkdc.ui.gui.lake import lake_page, lake_server
from dkdc.ui.gui.auth import login_signup_page, login_signup_server
from dkdc.ui.gui.email import email_page
from dkdc.ui.gui.calendar import calendar_page
from dkdc.ui.gui.feedback import feedback_page, feedback_server
from dkdc.ui.gui.settings import settings_page, settings_server
from dkdc.ui.gui.analytics import analytics_page, analytics_server


# enums
class Mainpage(Enum):
    DEFAULT = None
    SIGNUP_LOGIN = 0
    SETTINGS = 1
    FEEDBACK = 2
    ANALYTICS = 3


# gui
gui = ui.page_fluid(ui.output_ui("header"), ui.output_ui("main"), theme=theme.pulse)


# server
def gui_server(input, output, session):
    # setup reactive values
    username = reactive.Value(None)
    mainpage = reactive.Value(Mainpage.SIGNUP_LOGIN)

    # functions
    # TODO: we'll probably want this at some point
    def _get_query_params():
        search = session.input[".clientdata_url_search"]()
        parsed = urlparse(search)

        query_params = (
            {x.split("=")[0]: x.split("=")[1] for x in parsed.query.split("&")}
            if parsed.query
            else {}
        )

        return query_params

    def main_main_page():
        elements = [
            ui.nav_panel("home", home_page("home")),
            ui.nav_panel("todo", todo_page("todo")),
            ui.nav_panel("chat", chat_page("chat")),
            ui.nav_panel("email", email_page("email")),
            ui.nav_panel("calendar", calendar_page("calendar")),
            ui.nav_panel("lake", lake_page("lake")),
        ]

        gui = ui.navset_pill(*elements, id="navpill")

        return gui

    # servers
    home_server("home", username=username)
    todo_server("todo", username=username)
    chat_server("chat", username=username)
    lake_server("lake", username=username)
    feedback_server(
        "feedback", _to_home=lambda: mainpage.set(Mainpage.DEFAULT), username=username
    )
    analytics_server(
        "analytics", _to_home=lambda: mainpage.set(Mainpage.DEFAULT), username=username
    )
    settings_server(
        "settings", _to_home=lambda: mainpage.set(Mainpage.DEFAULT), username=username
    )
    login_signup_server(
        "login_signup",
        _to_home=lambda: mainpage.set(Mainpage.DEFAULT),
        _set_username=lambda u: username.set(u),
    )

    # global effects
    @reactive.Effect
    @reactive.event(input.to_home, ignore_init=True)
    def _to_home():
        mainpage.set(Mainpage.DEFAULT)

    @reactive.Effect
    @reactive.event(input.to_settings, ignore_init=True)
    def _to_settings():
        mainpage.set(Mainpage.SETTINGS)

    @reactive.Effect
    @reactive.event(input.feedback, ignore_init=True)
    def _to_feedback():
        mainpage.set(Mainpage.FEEDBACK)

    @reactive.Effect
    @reactive.event(input.to_analytics, ignore_init=True)
    def _to_analytics():
        mainpage.set(Mainpage.ANALYTICS)

    @reactive.Effect
    @reactive.event(input.logout, ignore_init=True)
    def _logout():
        username.set(None)
        mainpage.set(Mainpage.SIGNUP_LOGIN)

    @render.ui
    def header():
        s = mainpage.get()
        u = username.get()

        # get fancy on em
        match s:
            case Mainpage.DEFAULT:
                nav_text = "software for me"
            case Mainpage.SIGNUP_LOGIN:
                nav_text = "login / signup"
            case Mainpage.FEEDBACK:
                nav_text = "feedback"
            case Mainpage.SETTINGS:
                nav_text = "settings"
            case Mainpage.ANALYTICS:
                nav_text = "analytics"
            case _:
                nav_text = "how'd you get here?"

        elements = [
            ui.nav_control(nav_text),
            ui.nav_spacer(),
        ]

        if u:
            elements += [
                ui.nav_menu(
                    username.get(),
                    ui.nav_control(
                        ui.input_action_button(
                            "to_home",
                            "home",
                            class_="btn-secondary",
                            width="100%",
                        ),
                    ),
                    ui.nav_control(
                        ui.input_action_button(
                            "to_settings",
                            "settings",
                            class_="btn-primary",
                            width="100%",
                        ),
                    ),
                    ui.nav_control(
                        ui.input_action_button(
                            "to_analytics",
                            "analytics",
                            class_="btn-dark",
                            width="100%",
                        ),
                    ),
                    ui.nav_control(
                        ui.input_action_button(
                            "feedback",
                            "feedback",
                            class_="btn-info",
                            width="100%",
                        ),
                    ),
                    ui.nav_control(
                        ui.input_action_button(
                            "logout",
                            "logout",
                            class_="btn-danger",
                            width="100%",
                        ),
                    ),
                ),
            ]
        elements += [
            ui.nav_control(ui.input_dark_mode()),
        ]

        gui = (
            ui.tags.head(
                ui.tags.style("""
                .navbar .dropdown-menu {
                    right: 0 !important;
                    left: auto !important;
                }
            """)
            ),
            ui.navset_bar(
                *elements,
                title=ui.a("dkdc.io", href="/", class_="navbar-brand"),
                id="navbar",
            ),
        )

        return gui

    @render.ui
    def main():
        # get mainpage
        s = mainpage.get()

        # it's fancy!
        match s:
            case Mainpage.DEFAULT:
                return main_main_page()
            case Mainpage.SETTINGS:
                return settings_page("settings")
            case Mainpage.FEEDBACK:
                return feedback_page("feedback")
            case Mainpage.SIGNUP_LOGIN:
                return login_signup_page("login_signup")
            case Mainpage.ANALYTICS:
                return analytics_page("analytics")
            case _:
                return ui.markdown("how'd you get here?")


# app
app = App(gui, gui_server)
