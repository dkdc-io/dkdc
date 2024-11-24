# justfile

# load environment variables
set dotenv-load

# aliases
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

# docs-build
docs-build:
    @quarto render website

# docs-preview
docs-preview:
    @quarto preview website

# open
open:
    @open https://dkdc.dev
