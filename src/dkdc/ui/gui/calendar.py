# imports
from datetime import datetime
from shiny import ui, module

from dkdc_util import now


# let's write a better function relying on datetime to build a calendar string
# that looks like the `cal` command, for the current month w/ today highlighted
def get_cal_str() -> str:
    today = now()
    month = today.month
    year = today.year
    cal_str = f"    {today:%B %Y}"
    cal_str += "\nSu Mo Tu We Th Fr Sa\n"
    first_day_of_month = datetime(year, month, 1).weekday()
    cal_str += "   " * (first_day_of_month + 1)
    days_in_month = (datetime(year, month + 1, 1) - datetime(year, month, 1)).days
    for day in range(1, days_in_month + 1):
        if (first_day_of_month + day) % 7 == 0:
            cal_str += "\n"
        if day == today.day:
            cal_str += f"{day:2}⬅️"
        else:
            cal_str += f"{day:2} "
    return cal_str


@module.ui
def calendar_page():
    # cal_str = subprocess.run(["cal"], capture_output=True, text=True).stdout
    # adjust to capture the highlighted day of the `cal` command
    # cal_str = subprocess.run(["cal"], capture_output=True, text=True).stdout
    cal_str = get_cal_str()
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
