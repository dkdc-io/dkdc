# imports
from rich.panel import Panel
from rich.console import Console
from rich.markdown import Markdown

# console
console = Console()

# style map
style_map = {
    "user": "bold cyan",
    "dkdc.io": "bold violet",
    "dkdc.ai": "bold white",
}


# functions
def print(
    text: str, as_markdown: bool = True, as_panel: bool = True, header: str = "dkdc.io"
) -> None:
    """
    print text
    """
    if as_markdown:
        text = Markdown(text)

    if as_panel:
        text = Panel(text, title=header, border_style=style_map[header])

    console.print(text)
