# imports
from shiny import ui, module
from datetime import datetime

from dkdc_util import now


def _get_next_year_month(year: int, month: int) -> (int, int):
    month += 1
    if month == 13:
        month = 1
        year += 1
    return year, month


def get_cal_str(year: int = None, month: int = None) -> str:
    assert not (year is None) ^ (
        month is None
    ), "year and month must be both None or both not None"

    if year is None and month is None:
        today = now()
        month = today.month
        year = today.year

    cal_str = f"    {datetime(year, month, 1):%B %Y}    "
    cal_str += "\nSu Mo Tu We Th Fr Sa \n"
    first_day_of_month = datetime(year, month, 1).weekday()
    cal_str += "   " * (first_day_of_month + 1) if first_day_of_month != 6 else ""

    ny, nm = _get_next_year_month(year, month)
    days_in_month = (datetime(ny, nm, 1) - datetime(year, month, 1)).days

    for day in range(1, days_in_month + 1):
        if (first_day_of_month + day) % 7 == 0 and day != 1:
            cal_str += "\n"
        if day == now().day and year == now().year and month == now().month:
            cal_str += f"{day:2}⬅️"
        else:
            cal_str += f"{day:2} "

    cal_str += "   " * (7 - 1 - (first_day_of_month + days_in_month) % 7)

    return cal_str


def double_cal_str() -> str:
    c1 = get_cal_str()
    ny, nm = _get_next_year_month(now().year, now().month)
    c2 = get_cal_str(ny, nm)

    c1_lines = c1.split("\n")
    c2_lines = c2.split("\n")

    if len(c1_lines) < len(c2_lines):
        c1_lines += [
            " " * len(c1_lines[0]) for _ in range(len(c2_lines) - len(c1_lines))
        ]
    elif len(c2_lines) < len(c1_lines):
        c2_lines += [
            " " * len(c2_lines[0]) for _ in range(len(c1_lines) - len(c2_lines))
        ]

    s = "\n".join(
        [f"{c1_line}{' ' * 5}{c2_line}" for c1_line, c2_line in zip(c1_lines, c2_lines)]
    )
    s = s.replace("⬅️ ", "⬅️")

    return s


@module.ui
def calendar_page():
    cal_str = double_cal_str()
    return (
        ui.br(),
        ui.layout_columns(
            ui.card(
                ui.card_header("today's date"),
                ui.markdown(f"{now():%B %d, %Y} ({now():%Y-%m-%d})"),
            ),
            ui.card(
                ui.card_header("calendar"),
                ui.markdown(f"````\n{cal_str}\n````"),
            ),
        ),
    )
