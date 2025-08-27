# custom-completion-zsh

A tool for zsh that automatically sets completion commands set by the user
Linux or MacOS only

# example config

```toml
[[tool]]
name = "custom-completion"
exec = "custom-completion-zsh completion zsh"
```

The configuration file should be placed in `$XDG_CONFIG_HOME/custom-completion-zsh/config.toml`

To use the completion files, run `custom-completion-zsh build` and add `$XDG_DATA_HOME/zsh/custom-completion-zsh` to your fpath.
