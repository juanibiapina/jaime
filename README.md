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
`jaime-`. The second part of the name is displayed as a command. For instance,
for a plug-in called `plugin`:

Jaime first invokes the plug-in without arguments (`jaime-plugin`). The output
is then used to populate the list of available options. When an option is
selected, jaime invokes the plug-in's run command with the selected option
(`jaime-plugin run selected-option). If no output was returned (no arguments
are available for that command), it invokes the plug-in passing just `run` as
an argument (`jaime-plugin run`).

In order to generate a preview, jaime invokes `jaime-plugin preview` for top
level commands, or `jaime-plugin preview current-option` for arguments.
