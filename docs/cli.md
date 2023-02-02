# Shortcut Alias - CLI

Shortcut Alias is a dynamic command-line utility, which can be configured to run a series of commands from a single option. 

# Contents
- [Basic Usage](#basic-usage)
    - [Command](#command)
    - [Options](#options)


## Basic Usage

`shortcut-alias [OPTIONS] [COMMAND] [...]`

### Command

This is the name of a command a user has created within the config directory. (`~/.shortcut`). For documentation on how to create these, please use the `docs/shortcut_file.md` 

### Options

Shortcut Alias can be completely configured using the command line. The following table outlines all availiable options.


| Option | Type | Default | Description |
| :----- | :--: | :-----: | :---------- |
| `--color` / `-c` | Data Flag | `on` | Whether Shortcut Alias should use color in its output. Use "off" to turn the color off. |
| `--header` / `-e` | Data Flag | `on` | Whether or not to show the header during command execution. Use "off" to turn the color off. |
| `--body` / `-b` | Data Flag | `on` | Whether or not to show the command output during command execution. Use "off" to turn the color off. |
| `--footer` / `-f` | Data Flag | `on` | Whether or not to show the footer output during command execution. Use `off` to turn the color off. |
| `--silent` / `-s` | Flag | inactive | Quick flag to turn supress all output during command execution. |

