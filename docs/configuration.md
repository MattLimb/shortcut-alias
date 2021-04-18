# `settings.yaml`

`settings.yaml` is a configuration file that is created on launch of `shortcut-alias`. Its main purpose is to change the way that `shortcut-alias` outputs data to the screen.

# Contents
- [`settings.yaml`](#settingsyaml)
- [Contents](#contents)
- [Config Options](#config-options)

# Config Options

The following table shows the different configuration options, and will describe their impact. All the following options take a boolean value of either `true` or `false` to enable, or disable them.

| Option               | Description                                                                                                                                                                  |
| :------------------: | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `show_command`       | This will show the command name that is being run in the command header.                                                                                                     |
| `show_reason`        | This will show the reason that the command is being run.                                                                                                                     |
| `show_output`        | This will show the command output in the terminal.                                                                                                                           |
| `show_output_header` | This will show the command output header in the terminal. This is so that the output header can be turned off, whilst allowing the command output to be displayed on screen. |
| `show_skip`          | This will show the skip header when command verification succeeds.                                                                                                           |
| `colour`             | Show the command header in green text. This helps with clarity.                                                                                                              |