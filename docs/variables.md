# Variables

There are two categories of variables. `variables` and `constants`. All variables are accessiable though the `variables` super key.

# Contents 
- [Variables](#variables)
- [Contents](#contents)
- [Major Keys](#major-keys)
  - [Variables](#variables-1)
    - [Linux Specific Variables](#linux-specific-variables)
    - [Windows Specific Variables](#windows-specific-variables)
    - [MacOS Specific Variables](#macos-specific-variables)
    - [Java Specific Variables](#java-specific-variables)
  - [Constants](#constants)
    - [Windows Specific Constants](#windows-specific-constants)

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

### Linux Specific Variables

| Variable Key                   | Type     | Description                     |
| :----------------------------: | :------: | :-----------------------------: |
| `variables.linux.libc`         | `string` | The version of Libc availiable. |
| `variables.linux.libc_version` | `string` | The version number of Libc.     |

### Windows Specific Variables

| Variable Key                           | Type     | Description                         |
| :------------------------------------: | :------: | :---------------------------------: |
| `variables.windows.win32_release`      | `string` | The relase of windows being run.    |
| `variables.windows.win32_ver`          | `string` | The version number of Windows.      |
| `variables.windows.win32_service_pack` | `string` | The Service Pack of Windows.        |
| `variables.windows.win32_os_type`      | `string` | The Operating System Type.          |
| `variables.windows.win32_edition`      | `string` | The edition of Windows being run.   |
| `variables.windows.win32.iot_edition`  | `bool`   | If this Windows is the IOT version. |

### MacOS Specific Variables

__NOTE: Unfortunately I cannot test this, as I do not have a MacOS capable machine.__

| Variable Key                            | Type     | Description |
| :-------------------------------------: | :------: | :---------: |
| `variables.mac.mac_release`             | `string` |             |
| `variables.mac.mac_version`             | `string` |             |
| `variables.mac.mac_dev_stage`           | `string` |             |
| `variables.mac.mac_non_release_version` | `string` |             |
| `variables.mac.mac_machine`             | `string` |             |

### Java Specific Variables

__NOTE: Unfortunately I have not tested this with a Java capable machine.__

| Variable Key                     | Type     | Description |
| :------------------------------: | :------: | :---------: |
| `variables.java.java_version`    | `string` |             |
| `variables.java.java_vendor`     | `string` |             |
| `variables.java.java_vm_name`    | `string` |             |
| `variables.java.java_vm_release` | `string` |             |
| `variables.java.java_vm_vendor`  | `string` |             |
| `variables.java.os_name`         | `string` |             |
| `variables.java.os_version`      | `string` |             |
| `variables.java.os_version`      | `string` |             |
| `variables.java.os_arch`         | `string` |             |


## Constants

| Variable Key                              | Value            |
| :---------------------------------------: | :--------------: |
| `constants.string.empty`                  | `""`             |
| `constants.platform.windows`              | `Windows`        |
| `constants.platform.linux`                | `Linux`          |
| `constants.platform.mac`                  | `Darwin`         |
| `constants.platform.java`                 | `Java`           |
| `constants.windows_edition.enterprise`    | `Enterprise`     |
| `constants.windows_edition.iotuap`        | `IoTUAP`         |
| `constants.windows_edition.server`        | `ServerStandard` |
| `constants.windows_edition.nanoserver`    | `nanoserver`     |
| `constants.shell.pipe`                    | `|`              |
| `constants.shell.pipe_err`                | `|&`             |
| `constants.shell.background`              | `&`              |
| `constants.shell.and`                     | `&&`             |
| `constants.shell.or`                      | `||`             |
| `constants.shell.not`                     | `!`              |
| `constants.shell.tilda`                   | `~`              |
| `constants.shell.file_descriptors.stdin`  | `0`              |
| `constants.shell.file_descriptors.stdout` | `1`              |
| `constants.shell.file_descriptors.stderr` | `2`              | 
| `constants.shell.redirect.input`          | `<`              |
| `constants.shell.redirect.output.out`     | `>`              |
| `constants.shell.redirect.output.append`  | `>>`             |
| `constants.shell.redirect.here.document`  | `<<`             |
| `constants.shell.redirect.here.word`      | `<<<`            |
| `constants.shell.redirect.merge.output`   | `>&`             |
| `constants.shell.redirect.merge.input`    | `<&`             |
| `constants.shell.redirect.merge.outerr`   | `2>&1`           |
| `constants.shell.eof`                     | `EOF`            |
| `constants.shell.dev_null`                | `/dev/null`      |


### Windows Specific Constants

| Variable Key               | Value   |
| :------------------------: | :-----: |
| `constants.shell.ps_null`  | `$null` |
| `constants.shell.cmd_null` | `NUL`   |
| `constants.shell.dev_null` | `$null` |
