# Jaime

A command line launcher inspired by Alfred.

## Dependencies

- [fzf](https://github.com/junegunn/fzf)

## Install

Clone this repository and add the `bin` directory to your `PATH`.

## Usage

Run `jaime` to launch a fuzzy search window and select commands.

## Shortcuts

### Zsh

Source `shell/key-bindings.zsh` in order to bind `ctrl+space` to the Jaime widget.

### Tmux

Add this to your tmux.conf to make `<prefix>-Space` open the Jaime launcher in a split window.

```
# Run Jaime
unbind Space
bind Space split-window -v "jaime"
```

## Plug-ins

A plug-in is simply an executable file anywhere in the path that starts with
`jaime-`. The second part of the name is displayed as a command.

Jaime first invokes the plug-in without arguments (`jaime-plugin`). If no
output is returned (no arguments are available), it invokes the plug-in again
passing `run` as the first argument (`jaime-plugin run`). If any output is
returned, it is used to populate the list of possible arguments. When an
argument is selected, Jaime invokes the plug-in passing the argument
(`jaime-plugin selected-argument`).

In order to generate a preview, jaime invokes `jaime-plugin --preview` for top
level commands, or `jaime-plugin --preview current-argument` for arguments.
