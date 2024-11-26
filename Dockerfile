# dkdc Dockerfile
FROM ghcr.io/astral-sh/uv:latest AS uv
FROM python:3.12-slim
# set the environment variables
ENV PATH=/root/.local/bin:$PATH
# set the working directory
WORKDIR /app
# copy the files
COPY readme.md /app/readme.md
COPY pyproject.toml /app/pyproject.toml
COPY src /app/src
# install the Python packages
RUN --mount=type=cache,target=/root/.cache/uv \
    --mount=from=uv,source=/uv,target=./uv \
    ./uv pip install '.' --system --upgrade
