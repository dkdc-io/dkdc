# imports
import typer

from dkdc.ui.console import print
from dkdc.ui.cli.common import default_kwargs

# todo app
todo_app = typer.Typer(help="todo", **default_kwargs)


# commands
@todo_app.command("add")
@todo_app.command("a", hidden=True)
def todo_add(
    text: str = typer.Argument(..., help="text"),
    priority: int = typer.Option(100, help="priority", show_default=True),
):
    """
    add todo
    """
    from dkdc_todo import Todo
    from dkdc_util import uuid_parts

    _, id = uuid_parts()

    print(f"adding todo: {text}...")

    todo = Todo()

    todo.append_todo(
        id=id,
        user_id=None,
        subject=None,
        body=text,
        priority=priority,
        labels=["cli"],
    )
