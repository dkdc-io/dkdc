# justfile

# load environment variables
set dotenv-load

# aliases
alias fmt:=format

# list justfile recipes
default:
    just --list

# format
format:
    @cargo fmt

# install
install:
    @cargo install --path .

# release-test
release-test:
    @cargo publish --dry-run

# release
release:
    @cargo publish

# docs
docs:
    @cargo doc --open
