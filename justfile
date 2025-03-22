# justfile

# load environment variables
set dotenv-load

# aliases
alias fmt:=format

# list justfile recipes
default:
    just --list

# rust things
format:
    @cargo fmt

install:
    @cargo install --path .

# publish-test
release-test:
    @cargo publish --dry-run

# publish
release:
    @cargo publish

# docs-preview
docs:
    @cargo doc --open
