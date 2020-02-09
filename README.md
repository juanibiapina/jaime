# Jaime

A command line launcher inspired by Alfred.

## Install

Clone this repository and install locally with:

```
cargo install --path .
```

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
