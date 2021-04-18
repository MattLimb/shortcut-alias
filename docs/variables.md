# Variables

There are two categories of variables. `variables` and `constants`. All variables are accessiable though the `variables` super key.

# Contents 
- [Variables](#variables)
- [Contents](#contents)
- [Major Keys](#major-keys)
  - [Variables](#variables-1)

# Major Keys

There are three major keys to retrieve a variable:

- `variables`

Variables that can change between versions or even runnings of `shortcut-alias`.

- `commands`

The output and return code of all commands.

- `constants`

Constant variables with static values. These will not change between instances or runnings of `shortcut-alias`.

## Variables

| Variable Key                       | Type      | Description                                                         |
| :--------------------------------: | :-------: | :-----------------------------------------------------------------: |
| `variables.datetime.date`          | `string`  | The current date.                                                   |
| `variables.datetime.date_utc`      | `string`  | The current date in the UTC timezone.                               |
| `variables.datetime.time`          | `string`  | The current time.                                                   |
| `variables.datetime.time_utc`      | `string`  | The current time in the UTC timezone.                               |
| `variables.datetime.weekday`       | `string`  | The current day of the week.                                        |
| `variables.datetime.weekday_short` | `string`  | The current day of the week, as its short version.                  |
| `variables.datetime.timezone`      | `string`  | The current timezone.                                               |
| `variables.platform.name`          | `string`  | The current platform. `Windows` `Linux`, `Darwin` or `Java`         |
| `variables.platform.release`       | `string`  | The current platform release.                                       |
| `variables.platform.version`       | `string`  | The current platform version.                                       |
| `variables.device.name`            | `string`  | The current machine name.                                           |
| `variables.device.arch`            | `string`  | The current cpu architecture.                                       |
| `variables.device.processor`       | `string`  | The current processor type.                                         |
| `variables.python.compiler`        | `string`  | The current Python Compiler version.                                |
| `variables.python.implementaton`   | `string`  | The current Python Implementation.                                  |
| `variables.python.revision`        | `string`  | The current Python revision.                                        |
| `variables.python.version`         | `string`  | The current Python version.                                         |
| `variables.python.version_major`   | `integer` | The current Python major version.                                   |
| `variables.python.version_minor`   | `integer` | The current Python minor version.                                   |
| `variables.python.version_patch`   | `integer` | The current Python patch version.                                   |
| `variables.python.c_api`           | `string`  | The current Python C API version.                                   |
| `variables.options.<option_name>`  |           | The value of the command options for the commandset being run.      |
| `variables.env.<variable_name>`    |           | The Value of the environment variables for the current command set. |