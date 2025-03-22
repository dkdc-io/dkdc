# justfile

# load environment variables
set dotenv-load

# aliases
alias c:=config
alias fmt:=format

# list justfile recipes
default:
    just --list

# config
config:
    @$EDITOR Cargo.toml

# format
format:
    @cargo fmt

# build
build *args:
    @cargo build {{args}}

# test
test *args:
    @cargo test {{args}}

# install
install:
    @cargo install --path .

# release-test
release-test *args:
    @cargo publish --dry-run {{args}}

# release
release *args:
    @cargo publish {{args}}

# docs
docs:
    @cargo doc --open

# clean
clean:
    @cargo clean
