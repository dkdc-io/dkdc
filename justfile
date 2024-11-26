# justfile

# load environment variables
set dotenv-load

# variables
container := "lostmydockeraccount/dkdc"

# aliases
alias up:=serve
alias fmt:=format
alias render:=docs-build
alias preview:=docs-preview

# list justfile recipes
default:
    just --list

# python things
setup:
    @uv venv --python=3.12 --allow-existing
    just install

install:
    @uv pip install -e '.[dev,test]'

lock:
    @echo "this is kinda messed up..."
    @uv lock

sync:
    @echo "this is kinda messed up..."
    @uv sync

build-python:
    @rm -r dist || true
    @uv build

format:
    @ruff format .

# publish-test
release-test:
    just build-python
    @uv publish --publish-url https://test.pypi.org/legacy/ --token ${PYPI_TEST_TOKEN}

# publish
release:
    just build-python
    @uv publish --token ${PYPI_TOKEN}

# docker stuff
build:
    @docker build -t {{container}} .

run:
    @docker run -it --rm --name dkdc-dev --entrypoint bash {{container}}

run-gui:
    @docker run -d --rm -p 8010:8010 --name dkdc-gui {{container}} 'dkdc gui'

serve *args:
    @docker compose up -d --build --remove-orphans {{args}}

down *args:
    @docker compose down {{args}}

# docs-build
docs-build:
    @quarto render website

# docs-preview
docs-preview:
    @quarto preview website

# open
open:
    @open https://dkdc.io
