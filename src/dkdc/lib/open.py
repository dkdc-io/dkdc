import subprocess

from dkdc_util import get_config_toml
from dkdc.ui.console import print


def open_it(thing: str) -> None:
    """
    open a thing
    """
    config = get_config_toml()

    if thing in config["open"]["aliases"]:
        thing = config["open"]["things"][config["open"]["aliases"][thing]]
    elif thing in config["open"]["things"]:
        thing = config["open"]["things"][thing]
    else:
        print(f'thing "{thing}" not found')
        return

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
