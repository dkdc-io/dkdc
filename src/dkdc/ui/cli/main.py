# imports
import typer

from dkdc.ui.console import print
from dkdc.ui.cli.todo import todo_app
from dkdc.ui.cli.common import default_kwargs

# typer config
## main app
app = typer.Typer(help="dkdc", **default_kwargs)

## add sub-apps
app.add_typer(todo_app, name="todo")
app.add_typer(todo_app, name="t", hidden=True)


# commands
# functions
@app.command()
@app.command("c", hidden=True)
def config(
    vim: bool = typer.Option(False, "--vim", "-v", help="open with (n)vim"),
    env: bool = typer.Option(False, "--env", "-e", help="open .env file"),
):
    """
    open config file
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
    import subprocess

    from dkdc_util import get_config_toml

    def open_it(thing: str) -> None:
        """
        open a thing
        """
        config = get_config_toml()

        if thing in config["open"]["aliases"]:
            thing = config["open"]["things"][config["open"]["aliases"][thing]]
        elif thing in config["open"]["things"]:
            thing = config["open"]["things"][thing]

        print(f"opening {thing}...")
        subprocess.call(["open", thing])

    def list_things() -> None:
        """
        list things
        """
        config = get_config_toml()

        aliases = []
        things = []

        for alias, thing in config["open"]["aliases"].items():
            aliases.append((alias, thing))

        for thing in config["open"]["things"]:
            things.append((thing, config["open"]["things"][thing]))

        aliases.sort(key=lambda x: (len(x[0]), x[0]))
        things.sort(key=lambda x: (len(x[0]), x[0]))

        alias_max = max([len(alias) for alias, _ in aliases])
        thing_max = max([len(thing) for thing, _ in things])

        to_print = "aliases:\n"
        for alias, thing in aliases:
            to_print += f"  - {alias.ljust(alias_max)} | {thing}\n"

        to_print += "\n\nthings:\n"
        for thing, path in things:
            to_print += f"  - {thing.ljust(thing_max)} | {path}\n"

        print(to_print)

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
