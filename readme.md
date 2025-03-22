# dkdc

bookmarks for opening things from the terminal.

## installation

```bash
cargo install dkdc
```

## usage

Open the config file:

```bash
dkdc c
```

Copy/paste:

```toml
[open.aliases]
s = "search"

[open.things]
search = "https://duckduckgo.com"
```

And save the file. Now you can open the search page by running:

```bash
dkdc search
```

Or:

```bash
dkdc s
```

