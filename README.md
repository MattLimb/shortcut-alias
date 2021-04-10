# shortcut-alias

This is a personal project for configurable aliases. 

As a personal project - there is no formal progression or documentation. I shall do my best to update this project, as I find issues with it, and add new features as I find needs. 

## Install

```sh
> pip install shortcut-alias
```

## Usage

Shortcut-alias installs the following commands onto your system:

```sh
> shortcut-alias <command> <command_options>
> shortcut <command> <command_options>
> sa <command> <command_options>

```

## First Run 

On the first run, `shortcut-alias` will generate the needed file structure on first run, or on a new config directory. 

By default the folder structure will be the following:

On Windows:

| Name            | Filepath                                     |
| --------------- | -------------------------------------------- |
| root folder     | `C:\Users\<username>\shortcut`               |
| settings        | `C:\Users\<username>\shortcut\settings.yaml` |
| commands folder | `C:\Users\<username>\shortcut\shortcut.d`    |

On Linux:

| Name            | Filepath                   |
| --------------- | -------------------------- |
| root folder     | `~\shortcut`               |
| settings        | `~\shortcut\settings.yaml` |
| commands folder | `~\shortcut\shortcut.d`    |

To change this, set the environment variable "SHORTCUT_CONFIG".

Windows:

```powershell
> $Env:SHORTCUT_CONFIG=<filepath>
```

Linux:

```sh
export SHORTCUT_COFNIG=<filepath>
```

## settings.yaml

The following options can be specified in the settings file. 

| Setting        | Description                                                                                 | Default |
| -------------- | ------------------------------------------------------------------------------------------- | ------- |
| `show_command` | Show the command being run or a skip command run                                            | True    |
| `show_reason`  | Show the reason for the command being run, or skipped.                                      | True    |
| `show_output`  | Show the output of the command                                                              | True    |
| `colour`       | Display the output of `show_command`, `show_reason` and `show_output` admin tasks in green. | True    |

## shortcut.d files

This is the folder for commands. These can be called anything, however they must end in the `.yaml` extensive.

The contents of the file must follow this template:

```yaml
---
# General Purpose Administrative Commands

name: # The Name of the Command Set - REQUIRED
description: # A brief description of the command's intended purpose - REQUIRED
cmd: # The word used to run the command - REQUIRED

# Opional inforamtion to be added as options.
options:
  # Use YAML arrays to identify each options.
  <option_name>: # String - This is the name of the option. You can use this: `option:<option_name>` to access this futher down.
    # Option Access
    short: # String - OPTIONAL :- the short version of the option, if wanted. Please use a single dash prefix for this option.
    long: # String - OPTIONAL :- the long version of the option if wanted. Please use double dash prefix to represent this.
    # If neither short or long is specified, the <option_name> is used as the long option. 

    # Option Type
    arg_type: # <flag|data> REQUIRED Default Flag
      # Flag provides a boolean result, with a default of False.
      # Data requires at least one arg to be passed to it.
    
    # Data only attributes
    default: # String, Integer, Float - Provide a default value if none passed in. Defaults to None.
    type: # <string|integer|float> Will attempt to convert the input to a given type. Only supports str, int and float.

    # Generic attributes
    required: # Boolean - Is the option required to run the command.
    description: # String - Help text for the option - OPTIONAL

# The command list. The instructions to follow when called.
commands:
  <command_name>: # String - A simple command name.
    # Basic command settings
    description: # String - A brief description of the command
    cmd: # String or list of strings- The command to run. (Can include `option:<option_name>` in the cmd string. This will substitute this for name of value of the option.)
    if: # String or list of strings - Conditional statements in the form of `option:<option_name>` or `command:<command_name>`.
      option:<option_name>: 
        # If data option, support these
        # For Strings|Integers|Floats AND FLAGS
        eq: # Bool, String, Integer, Float - The value to be equal to.
        not_eq: # Bool, String, Integer, Float - The value to be not equal to.
        
        # Integers and Floats ONLY
        gt: # Greater Than
        lt: # Less Than
        le: # Greater Than or Equal To
        ge: # Less Than or Equal To

      # `option:<option_name>` - Substitute for the value of `<option_name>`
      # `command:<option_name`> - Only run this if the the command exits successfully. (Code 0)
      command:<command_name>:
        status: # Status Code integer
        includes: # A line of output to 
```
