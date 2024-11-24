# imports
import typer

from dkdc.ui.console import print

# typer config
## default kwargs
default_kwargs = {
    "no_args_is_help": True,
    "add_completion": False,
    "context_settings": {"help_option_names": ["-h", "--help"]},
}

## main app
app = typer.Typer(help="dkdc", **default_kwargs)


# commands
# functions
@app.command()
@app.command("c", hidden=True)
def config(
    vim: bool = typer.Option(False, "--vim", "-v", help="open with (n)vim"),
    env: bool = typer.Option(False, "--env", "-e", help="open .env file"),
):
    """
    open config file(s)
    """
    import os
    import subprocess

    from dkdc_util import get_dkdc_dir

    program = "nvim" if vim else "code"
    filename = ".env" if env else "config.toml"

    filename = os.path.join(get_dkdc_dir(), filename)

    print(f"opening {filename} with {program}...")
    subprocess.call([program, f"{filename}"])


@app.command()
@app.command("o", hidden=True)
def open(
    thing: str = typer.Argument(None, help="thing to open"),
):
    """
    open thing
    """
    from dkdc.open import open_it, list_things

    if thing is None:
        list_things()
    else:
        open_it(thing)


## servers
@app.command()
@app.command("g", hidden=True)
def gui(
    port: int = typer.Option(1913, help="port", show_default=True),
    prod: bool = typer.Option(False, help="prod?", show_default=True),
):
    """
    gui
    """
    from shiny import run_app as run_gui_app
    from dkdc.ui.gui import app  # noqa

    if prod:
        run_gui_app(
            app=app,
            host="0.0.0.0",
            port=port,
        )
    else:
        run_gui_app(
            app="dkdc.ui.gui:app",  # goofy! but needed to reload
            host="0.0.0.0",
            port=port,
            reload=True,
            launch_browser=True,
        )


# main
if __name__ == "__main__":
    typer.run(app)
