# Shortcut Files

Shortcut files contains one or more Command Sets. A Command Set a list of instructions for `shortcut-alias`. This document explains the various features present in Command Sets. 

# Contents
- [Shortcut Files](#shortcut-files)
- [Contents](#contents)
- [The Files](#the-files)
- [The Shortcut File Contents](#the-shortcut-file-contents)
  - [Variables](#variables)
  - [Environments](#environments)
  - [Args](#args)
  - [Commands](#commands)

# The Files

Shortcut files are YAML formatted files which are placed in the `.shortcut` directory in your home directory. On *nix systems it should be under: `~/.shortcut`. On Windows it will be under `C:\Users\<username>\.shortcut`. 

This folder can be modified by using the Environment Variable: `SA_CONFIG` or `SHORTCUT_ALIAS_CONFIG.


These files are automatically picked up when `shortcut-alias` is invoked. These files can be called anything you like, as the filenames are not relevant to `shortcut-alias`. The command set that the file contains, are named within the file itself. It is reccommended that filenames are fairly short, and descriptive.

It is worth noting, that `shortcut-alias` supports YAML Multiple Documents in a single file, as defined in YAML specification, using `---` to seperate each file. (There needs to be one at the start of the start of the document if this feature is in use.)

__NOTE: Typically YAML files have the `.yaml` extension. `shortcut-alias` prefers this extension is used, however as `.yml` is also a common file extension for YAML files, `shortcut-alias` will also pick up these files.__

# The Shortcut File Contents

From ths section on, this document will be explaining the keys and values that can go into a shortcut file. This will explain how to use each key, and will provide relevant examples where appropriate.

To start a shortcut file the following keys must be present at the document root:

| Key           | Type     | Required | Description                                                                                               |
| :-----------: | :------: | :------: | --------------------------------------------------------------------------------------------------------- |
| `name`        | `string` | Yes      | A human readable name for the command set                                                                 |
| `description` | `string` | No       | A short description of the command. Used to tell viewers of the commandset the purpose it was created for |
| `cmd`         | `string` | Yes      | The command used to trigger the command set                                                               |

These keys must be present at the document root, which means that they must be present with no indentation.

For example:

```yaml
name: Python Version
description: Get the Python Version
cmd: pyver
```

This serves as the absolute minimum needed to create a command set. The command would not do anything, however it will parse through `shortcut-alias`.

The following table shows the other optional root keys, and broadly explains what they are used for. The following sections then futher explain what to incldue in those keys.

| Key         | Required | Description                                                                        |
| :---------: | :------: | ---------------------------------------------------------------------------------- |
| `name`   | Yes      | The name of the command. This will be used to invoke it from the command line. |
| `description` | No   | A brief description of the commnd. |
| `args`   | No       | Add command line options to provide dynamic configuration options.                 |
| `variables` | No       | Key value pairs of static variables for use in multiple commands. |
| `env`       | No       | Variables to be found in the environment. These are retrieved when Shortcut-Alias starts. |
| `commands`  | Yes      | The commands to be run. They will be run top to bottom. |

## Variables

The `variables` key is a YAML associative array of static variables. The key is the name of the variable, and the value is the statc value of the variable.

These `variables` are merged into the global `variables` key, so will be availiable from the gloabl `variables` key. 


```yaml
variables:
  python_maj: 3
  python_min: 9
  python_patch: 0
```

## Environments

Sometimes extracting keys from the environment is necessary. The `env` key is designed for this very purpose. This key works similarly to the way the `variables` key works. 

The `env` key is a YAML associative array. The keys are the names of the variables to store the value from the ENV in, and the values are the variable names from the environment. 

`env` variables are not Jinja2 templates. They will not be processed as such.

```yaml
env:
  example: SOME_VARIABLE
```

## Args

Args are a dynamic method user input. They are inputted on the command line, after the command.

```sh
shortcut-alias [OPTIONS] [COMMAND] [COMMAND_OPTIONS]
```

There are two types of options:

- `flag` 

Flags create a `true` or `false` value. They are usually used to enable or diable commands. The method to do this wlll be exlained futher down in this document.

- `data`

Data will take in a value from the command line. This allows for variable data to be passed into the commands, depending on the needs of the commands.

A option block may look like this:

```yaml
args:
  - arg_type: flag
    name: debug
    help: Whether debug logs should be shown.
```

The `name` key can be any name you would like. However, spaces ` ` are converted to dashes `-`.

The following table outlines the configuration options that can be used to configure an option.

| Configuration Option | Required | Description   |
| :------------------: | :------: | :------------ |
| `arg_type`           | Yes      | The type of argument to create. Valid options are `flag` or `data`. |
| `name`               | Yes      | The name of the argument. This will represent the argument name. It will be automatically preceeded by `--` and spaces will be turned into dashes `-`. |
| `default`            | No       | A default value in case the option is not specified. If this is key is not included, the argument will be required on the command line. |
| `help`               | No       | A string to provide help text for the option if the `shortcut-alias <cmd> --help` is specified. |

There can be as many options as necessary for your command set.

## Commands

Commands are the actual commands that will be run by `shortcut-alias`. 

```yaml
commands:
  - name: Python Version
    description: Get the current Python Version
    command: "python -V"
```

| Key | Required | Description |
| :-: | :------: | :---------- |
| `name` | Yes | A name for the commad. This name is shown in the header during execution. |
| `description` | No | A brief explanation of what the command does. This is shown in the header if availiable. |
| `command` | Yes | The command to run. This is a Jinja2 string. Please see the `docs/templating.md` for instructions on Jinja2 strings. | 
