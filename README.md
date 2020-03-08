# Jaime

A command line launcher inspired by Alfred.

![Usage](assets/usage.gif)

## Install

Download a release from Github or clone this repository and install locally
with:

```
cargo install --path .
```

## Configuration

Configuration file that I use:
https://github.com/juanibiapina/dotfiles/blob/master/jaime/config.yml

Jaime looks for a config file in the XDG Config directory (usually
~/.config/jaime/config.yml). The configuration specifies which actions will be
available. For instance, two simple actions for `screensaver` and `brew`:

```
---
options:
  screensaver:
    type: Command
    command: open -a ScreenSaverEngine
  brew:
    type: Select
    options:
      install:
        type: Command
        widgets:
          - type: FromCommand
            command: brew search
            preview: brew info {}
        command: brew install {0}
```

### Actions

Actions can be of two types:

#### Select

Presents a static list of options in a fuzzy finder. Each option is another
action:

```
options:
  cmd:
    type: Select
    options:
      build:
        type: Command
        command: make build
      install:
        type: Command
        command: make install
```

Attributes:

- `type`: `Select`
- `options`: A map of action names to actions

#### Command

Runs a command using the shell:

```
options:
  brew-install:
    type: Command
    command: brew install {0}
    widgets:
      - type: FromCommand
        command: brew search
        preview: brew info {}
```

Attributes:

- `type`: `Command`
- `command`: The command to run
- `widgets`: A list of widgets

The `command` string can contain placeholder values like `{0}`, `{1}` etc.
These values are replaced with the result of running the corresponding widget
in the `widgets` key.

### Widgets

Widgets are used to get input from the user. There are currently two types:

#### FromCommand

Presents a list of options in a fuzzy finder. The list of options comes from
running an external command:

```
options:
  asdf-install:
    type: Command
    command: asdf install {0} {1}
    widgets:
      - type: FromCommand
        command: asdf plugin list
      - type: FromCommand
        command: asdf list-all {0}
```

Attributes:

- `type`: `FromCommand`
- `command`: command to run to get the options
- `preview` (optional): command to run to generate a preview window

In this example the second widget refers to the result of the first widget
using the placeholder `{0}`.

#### FreeText

Takes free text input from the user.

```
options:
  duck:
    type: Command
    command: open "https://duckduckgo.com/?q={0}"
    widgets:
      - type: FreeText
```

Attributes:

- `type`: `FreeText`

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
