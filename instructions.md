# AI instructions

I am working on converting my CLI `dkdc` from Python to Rust. I'm sorta new to Rust -- I know the basics, but I'm rusty.

`dkdc` is used to open things -- like bookmarks but as a CLI commands. It just calls `open` in a shell. I typically use it to open URLs, but also applications.

`dkdc configure` opens the `~/.dkdc/config.toml` file with `[things]` and `[aliases]`. An example might be:

```toml
[things]
"search" = "https://duckduckgo.com"
"mail" = "https://mail.google.com"
"calendar" = "https://calendar.google.com"
"edge" = "/Applications/Microsoft Edge.app"

[aliases]
"s" = "search"
"browser" = "edge"
```

Use is `dkdc open <thing> | <alias>`.
