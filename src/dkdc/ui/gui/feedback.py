# imports
from shiny import ui, module, reactive

from dkdc_lake import Lake
from dkdc_util import uuid

# global state
lake = Lake()


@module.ui
def feedback_page():
    return ui.page_fluid(
        ui.br(),
        ui.card(
            ui.input_action_button(
                "to_home", "home", class_="btn-secondary", width="100%"
            )
        ),
        ui.card(
            ui.card_header("feedback"),
            ui.input_text_area("feedback_text", "feedback"),
            ui.input_action_button(
                "submit", "submit", class_="btn-primary", width="100%"
            ),
        ),
    )


@module.server
def feedback_server(input, output, session, _to_home):
    @reactive.Effect
    @reactive.event(input.submit)
    def submit():
        lake.append_file(
            user_id="feedback",
            path="dkdc",
            filename=f"{uuid()}.txt",
            filetype="txt",
            data=input.feedback_text().encode(),
            labels=["feedback"],
        )
        ui.notification_show("feedback submitted!", type="default")
        _to_home()

    @reactive.Effect
    @reactive.event(input.to_home, ignore_init=True)
    def to_home():
        _to_home()
