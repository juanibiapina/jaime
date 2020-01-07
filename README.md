# Jaime

A command line launcher inspired by Alfred.

## Install

Clone this repository and add the `bin` directory to your `PATH`.

## Usage

### From zsh

Source `shell/key-bindings.zsh` in order to bind `ctrl+space` to the Jaime widget.

### From tmux

Add this to your tmux.conf to make <prefix>Space open the Jaime launcher in a split window.

```
# Run Jaime
unbind Space
bind Space split-window -v "jaime"
```
