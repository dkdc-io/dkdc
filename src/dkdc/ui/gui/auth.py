# imports
from shiny import ui, module, reactive


@module.ui
def login_signup_page():
    return ui.page_fluid(
        ui.br(),
        ui.card(
            ui.card_header("login"),
            ui.input_text("username", "username"),
            ui.input_password("passphrase", "passphrase"),
            ui.input_action_button("login_submit", "login", class_="btn-primary"),
        ),
        ui.tags.script("""
            document.addEventListener('keypress', function(e) {
                if (e.key === 'Enter') {
                    e.preventDefault();
                    const button = document.querySelector('button[id$="-login_submit"]');
                    if (button) {
                        setTimeout(() => {
                            button.click();
                        }, 500);
                    }
                }
            });
        """),
    )


@module.server
def login_signup_server(input, output, session, _to_home, _set_username):
    @reactive.Effect
    @reactive.event(input.login_submit)
    def submit():
        u = input.username() or "dkdc"
        _p = input.passphrase() or "password"
        _set_username(u)
        ui.notification_show(f"Welcome, {u}!", type="default")
        _to_home()
