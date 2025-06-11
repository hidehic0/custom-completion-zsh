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

# Dependencies

- [github.com/BurntSushi/toml](https://github.com/BurntSushi/toml) - toml library
- [github.com/briandowns/spinner](https://github.com/briandowns/spinner) - spinner library
- [github.com/fatih/color](https://github.com/fatih/color) - color string library
- [github.com/spf13/cobra](https://github.com/spf13/cobra) - cli framework

For detailed dependencies, see go.mod.
