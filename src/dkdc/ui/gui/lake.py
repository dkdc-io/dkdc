# imports
import io
import matplotlib.pyplot as plt

from shiny import ui, module, reactive, render

from dkdc_lake import Lake
from dkdc_state import ibis

# global state
lake = Lake()


@module.ui
def lake_page():
    return (
        ui.br(),
        ui.layout_columns(
            ui.card(ui.card_header("edit files"), ui.output_ui("files_chat")),
            ui.card(ui.card_header("files!"), ui.output_ui("files_ui")),
        ),
        ui.card(
            ui.card_header("show file"),
            ui.output_ui("files_show"),
            ui.output_ui("file_content_router"),
            min_height="70vh",
        ),
    )


@module.server
def lake_server(input, output, session, username: reactive.Value):
    chat = ui.Chat(
        "chat", messages=[{"role": "assistant", "content": "edit files via chat"}]
    )
    file_added = reactive.Value(0)

    @render.ui
    def files_ui():
        _ = file_added.get()
        files = (
            lake.t(user_id=username.get())
            .filter(ibis._["status"].isnull())
            .to_pyarrow()
            .to_pylist()
        )
        return (
            ui.input_file("file", "upload file"),
            ui.span(),
            ui.input_checkbox_group(
                "files_boxes", "", {t["filename"]: t["filename"] for t in files}
            ),
        )

    @render.ui
    def files_chat():
        return (ui.chat_ui("chat", placeholder="edit files via chat"),)

    @render.ui
    def files_show():
        _ = file_added.get()
        return (
            ui.input_selectize(
                "file_select",
                "select file",
                {
                    t["filename"]: t["filename"]
                    for t in lake.t(user_id=username.get())
                    .filter(ibis._["status"].isnull())
                    .to_pyarrow()
                    .to_pylist()
                },
            ),
        )

    @render.plot
    def file_content():
        filename = input.file_select()
        print(f"filename={filename}")

        file = lake.get_file(filename=filename, user_id=username.get())

        data = file["data"]

        img = plt.imread(io.BytesIO(data))
        fig = plt.imshow(img)

        return fig

    @render.ui
    def file_content_router():
        filename = input.file_select()
        if filename is None or filename == "":
            return None

        print(f"filename={filename}")

        file = lake.get_file(filename=filename, user_id=username.get())
        filename = file["filename"]
        print(f"filename={filename}")
        ext = filename.split(".")[-1]
        print(f"ext={ext}")

        if ext in ["png", "jpg", "jpeg"]:
            print("plotting image!")
            return ui.output_plot("file_content")
        elif ext in ["txt", "md", "qmd"]:
            print("markdowning text!")
            return ui.markdown(file["data"].decode())
        elif ext in ["py"]:
            print("pythoning text!")
            return ui.markdown(f"""```python\n{file["data"].decode()}\n```""")
        elif ext in ["sh"]:
            print("bash scripting text!")
            return ui.markdown(f"""```bash\n{file["data"].decode()}\n```""")
        elif ext in ["json", "jsonl"]:
            print("jsoning text!")
            return ui.markdown(f"""```json\n{file["data"].decode()}\n```""")
        elif ext in ["yaml", "yml"]:
            print("yamling text!")
            return ui.markdown(f"""```yaml\n{file["data"].decode()}\n```""")
        elif ext in ["toml"]:
            print("tomling text!")
            return ui.markdown(f"""```toml\n{file["data"].decode()}\n```""")
        elif ext in ["csv"]:
            print("csving text!")
            return ui.markdown(f"""```csv\n{file["data"].decode()}\n```""")
        else:
            try:
                return ui.markdown(file["data"].decode())
            except Exception as e:
                return ui.markdown(f"error: {e}")

    @chat.on_user_submit
    async def _chat_submitted():
        print("chat_submitted")
        content = chat.user_input()
        print(f"content={content}")
        u = username.get()
        print(f"u={u}")

        await chat.append_message({"role": "assistant", "content": f"{content}"})

    @reactive.Effect
    @reactive.event(input.file)
    def upload_file():
        print("upload_file")
        print(input.file())

        files = input.file()

        for file in files:
            datapath = file["datapath"]
            with open(datapath, "rb") as f:
                data = f.read()

            print(f"type(data)={type(data)}")

            try:
                lake.append_file(
                    user_id=username.get(),
                    filename=file["name"],
                    filetype=file["type"],
                    data=data,
                )
                file_added.set(file_added.get() + 1)
            except Exception as e:
                print(f"error uploading file: {e}")
                ui.notification_show(f"{e}", type="error")

    @reactive.Effect
    @reactive.event(input.files_boxes)
    def _file_boxes():
        print("files_boxes")
        print(input.files_boxes())
        u = username.get()
        for filename in input.files_boxes():
            file = lake.get_file(filename=filename, user_id=u)
            lake.update_file(
                user_id=u,
                path=file["path"],
                filename=filename,
                filetype=file["filetype"],
                data=file["data"],
                version=file["version"],
                status="archived",
                description=file["description"],
                labels=file["labels"],
            )
        file_added.set(file_added.get() + 1)
