# Pdoro

pomodoro daemon server

# completions

make sure you have custom completions availible

```zsh
mkdir -p ~/.zsh/completions
```

somewhere in your config, where you configure completions, it has to look like this

```zsh
# completion
fpath=(~/.zsh/completions $fpath)

autoload -Uz compinit
compinit

autoload -U bashcompinit
bashcompinit
```

WIP

tail -f /private/tmp/pdoro.out
