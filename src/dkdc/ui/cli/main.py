# imports
import typer

from dkdc.ui.console import print
from dkdc.ui.cli.common import default_kwargs

# typer config
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
    open config file
    """
    import os
    import subprocess

    from dkdc_util import get_dkdc_dir

    program = "vim" if vim else "nvim"
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
    from dkdc.lib.open import open_it, list_things

    if thing is None:
        list_things()
    else:
        open_it(thing)


# if __name__ == "__main__":
if __name__ == "__main__":
    app()
