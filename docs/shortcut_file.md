# Shortcut Files

Shortcut files contains one or more Command Sets. A Command Set a list of instructions for `shortcut-alias`. This document explains the various features present in Command Sets. 

# Contents
- [Shortcut Files](#shortcut-files)
- [Contents](#contents)
- [The Files](#the-files)
- [The Shortcut File Contents](#the-shortcut-file-contents)
  - [Config](#config)
  - [Variables](#variables)
  - [Environments](#environments)
  - [Options](#options)
  - [Commands](#commands)
    - [Basic Setup](#basic-setup)
    - [`cmd`](#cmd)
    - [`if`](#if)

# The Files

Shortcut files are YAML formatted files which are placed in the `shortcut.d` directory. These files are automatically picked up when `shortcut-alias` is invoked. These files can be called anything you like, as the filenames are not relevant to `shortcut-alias`. The command set that the file contains, are named within the file itself. It is reccommended that filenames are fairly short, and descriptive.

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
| `config`    | No       | Configuration options to override the defaults and those found in `settings.yaml`. |
| `variables` | No       | Static variables to use throughout the command set.                                |
| `env`       | No       | Variables to query the environment for, to use as static variables.                |
| `options`   | No       | Add command line options to provide dynamic configuration options.                 |
| `commands`  | No       | The commands to be run in order.                                                   |

## Config

The config key allows for overriding all settings in `settings.yaml` for the given configuration set, which allows for more dynamic commands.

To use these settings for the command set, just create the `config` key and use any combination of the settings you need. Not all settings need to be specified.

```yaml
config:
  show_output: false
  colour: true
```

## Variables

The `variables` key is a YAML associative array of static variables. The key is the name of the variable, and the value is the statc value of the variable.

These `variables` are merged into the global `variables` key, so will be availiable from the gloabl `variables` key. 

To use this key, just specify `variables` and enter as many variables as you need. Values can also be Jinja2 tempates. 

```yaml
variables:
  python_maj: 3
  python_min: 9
  python_patch: 0
  python_version: "{{ variables.python_maj }}.{{ variables.python_min }}.{{ variables.python_patch }}"
```

## Environments

Sometimes extracting keys from the environment is necessary. The `env` key is designed for this very purpose. This key works similarly to the way the `variables` key works. 

The `env` key is a YAML associative array. The keys are the names of the variables to store the value from the ENV in, and the values are the variable names from the environment. 

`env` variables are not Jinja2 templates. They will not be processed as such.

```yaml
env:
  example: SOME_VARIABLE
```

## Options

Options are a dynamic method user input. They are inputted on the command line, after the command.

```sh
shortcut-alias <cmd> <user_options>
```

There are two types of options:

- `flag` 

Flags create a `true` or `false` value. They are usually used to enable or diable commands. The method to do this wlll be exlained futher down in this document.

- `data`

Data will take in a value from the command line. This allows for variable data to be passed into the commands, depending on the needs of the commands.

A option block may look like this:

```yaml
options:
  <option_name>:
    ...
```

`<option_name>` can be any name. To reference this value in a Jinja2 template, use the following syntax, replacing the `<option_name>` with the name of the option:

```jinja2
{{ variables.options.<option_name> }}
```

The following table outlines the configuration options that can be used to configure an option.

| Configuration Option | Valid Option Types | Required | Description                                                                                                                                                                                                                                     |
| :------------------: | :----------------: | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `arg_type`           | `flag` & `data`    | No       | Dictates what type of option this is. Either `flag` or `data`. DEFAULT `flag`.                                                                                                                                                                  |
| `description`        | `flag` & `data`    | No       | Describes the purpose of the option. This has no functional benefit apart from documenting the feature for readers of the configuration files.                                                                                                  |
| `short`              | `flag` & `data`    | No       | A short value for the option. Typically one letter, prefixed with a `-`. Used to specify this flag on the command line.                                                                                                                         |
| `long`               | `flag` & `data`    | No       | A longer value for the option. Prefixed with `--`. Used to specify this flag on the command line.                                                                                                                                               |
| `default`            | `flag` & `data`    | No       | A default value for the option if it is not given on the command line. DEFAULT `None`. __If the option is `flag`, setting a default value as `true` the option turns into a negative flag, and when specified, will set the value to `false`.__ |
| `type`               | `data`             | No       | Convert data taken in via the command line to the chosen type. One of `float`, `integer` or `string`. DEFAULT `string`.                                                                                                                         |
| `required`           | `flag` & `data`    | No       | If `true` this option will be required on the command line.                                                                                                                                                                                     |

There can be as many options as necessary for your command set.

## Commands

Commands are the actual commands that will be run by `shortcut-alias`. 

A command block may look like this, where `<command_name>` is a friendly name given to the command, and is the name used when referencing the command in Jinja2:

```yaml
commands:
  <command_name>:
    ...
```

Commands are special in Jinja2 templates. They are actually a dictionary, with the name that is specified. They provide the following two options:

- `returns`

The return code of the command as an integer.

- `output`

The actual output of a command. 

__NOTE: Commands that are sent to the background do not exist in variables. This is because it `shortcut-alias` triggers the command in the background, then forgets about it. It will not track its completion or STDOUT output.__

### Basic Setup

This section outlines the setup keys that help define a command.

| Key           | Required | Description                                                                                                                                  |
| :-----------: | :------: | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `description` | No       | A brief description of the purpose of the command. Used for the reader, and has no other meaning.                                            |
| `background`  | No       | `true` or `false` - run this command in the background. Away from the main `shortcut-alias` thread.                                          |
| `config`      | No       | Override default, `settings.yaml` and command set `config` settings for this one command only.                                               |
| `cmd`         | Yes      | The actual commands being run. This will be explained futher in an upcoming section.                                                         |
| `if`          | No       | Conditionals that will decide if the command is run or not.                                                                                  |
| `mode`        | No       | How to run the command. Either `shell` or `direct`. `shell` runs the command via a shell environment. `direct` runs the command without one. |

### `cmd`

This key is a YAML list of the command being run. Each word of the command should be its own item in the list. 

For example, the following command:

```sh
echo "Hello World!"
```

would be represented like so:

```yaml
  cmd:
    - echo
    - Hello
    - World!
```

### `if`

This key creates conditionals that can be used to allow `shortcut-alias` to run the commands.

The `if` key should contain a YAML associative array. The contents of the associative arrays are other associative arrays, whose keys act as values to compare to. See the example below:

```yaml
if:
  <value>: # This is a Jinja2 template that creates the value to compare against
    ... # This is where the comparisions will happen.
```

The table below shows the valid comparision operators:

| Comparision Key | Allowed Types                              | Description                                                                                         |
| :-------------: | :----------------------------------------: | --------------------------------------------------------------------------------------------------- |
| `eq`            | `integer`, `string`, `boolean` and `float` | Equal. Tests that the `<value>` is equal to the value of this key.                                  |
| `neq`           | `integer`, `string`, `boolean` and `float` | Not equal. Tests that the `<value>` is not equal to the value of this key.                          |
| `gt`            | `intger` and `float`                       | Greater Than. Tests that the `<value>` is greater than the value of this key.                       |
| `ge`            | `intger` and `float`                       | Greater Than, or Equal To. Tests that `<value>` is greater than, or equal to the value of this key. |
| `lt`            | `intger` and `float`                       | Less Than. Tests that the `<value>` is less than the value of this key.                             |
| `le`            | `intger` and `float`                       | Less Than, or Equal To. Tests that `<value>` is less than, or equal to the value of this key.       |

The following document gives examples of each of these:

```yaml
if:
  # The following equates to "a_test_string" == "a_test_strnig", and would evaluate to True.
  a_test_string:
    eq: a_test_string
  
  # The following equates to "a_test_string" != "some_test_string" and would evaluate to True
  a_test_string:
    neq: some_test_string

  # The following equates to 5 > 4 and would evaluate to True
  5:
    gt: 4
  
  # The following equates to 5>=5 and would evaluate to True
  5:
    ge: 5
  
  # The following equates to 6 < 9 and would evaluate to True
  6:
    lt: 9

  # The following equates to 6 <= 6 and would evaluate to True
  6: 
    le: 6
```

There are two more logical operators which can be placed at any level below `if`. These are `and` and `or`. 

- `and`

`and` takes whats below it, and returns True if ALL of the conditionals below it return True. 

- `or`

`or` takes whats below it and returns True if ANY of the conditionals below it return True.


The following shows how they can be used. `and` and `or` work in the same way.

```yaml
if:
  and:
    1: 
      eq: 1

  1:
    and:
      gt: 0
      lt: 10
```