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

Please view `docs/configuration.md` for this. 
## shortcut.d files

Please view `docs/shortcut_files.md` for this.

## Variables

Please view `docs/variables.md` for this.

## Templating

Please view `docs/templating.md` for this.
