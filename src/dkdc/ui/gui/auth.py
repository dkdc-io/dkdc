# imports
from shiny import ui, module, reactive

from dkdc_env import Env
from dkdc_vault import Vault

# global states
env = Env()
vault = Vault()


@module.ui
def login_signup_page():
    return (
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
    # TODO: actual login logic
    u = env("username")
    p = env("passphrase")

    def _logged_in(u):
        ui.notification_show(f"Welcome, {u}!", type="default")
        _set_username(u)
        _to_home()

    if u and p:
        _logged_in(u)

    @reactive.Effect
    @reactive.event(input.login_submit)
    def submit():
        u = input.username() or "dkdc"
        p = input.passphrase() or "dkdc"  # noqa
        _logged_in(u)
