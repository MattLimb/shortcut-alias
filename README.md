# shortcut-alias

This is a personal project for configurable multiple command aliases. 

Disclaimer - This is a personal project, intended to learn and progress my programming knowlege. If you do find this project useful, and want to contribute, please feel free to open PRs or issues, and I'll do my best to process them as quickly as I can.

## Install Using Rust

Firstly, this repo will require that Rust is installed, along with Cargo.

To install Rust, please refer to the Rust docs: https://rustup.rs/

1. Clone the Git repo.

```sh
$ git clone https://github.com/MattLimb/shortcut-alias.git
```

2. Install Shortcut Alias using Cargo

```sh
$ cargo install --path .
```

## Install with Python

It is reccommmended to install this project using `pipx` for easier dependancy management.

```sh
$ pipx run shortcut-alias
```

Alternatively, if you'd prefer to use `pip`:

```sh
$ pip install shortcut-alias
```


## First Run 

On the first run, `shortcut-alias` will generate the needed file structure on first run, or on a new config directory. 

By default the folder structure will be the following:

On Windows:

| Name            | Filepath                                     |
| --------------- | -------------------------------------------- |
| shortcut folder | `C:\Users\<username>\.shortcut`              |

On Linux:

| Name            | Filepath                   |
| --------------- | -------------------------- |
| shortcut folder | `~\.shortcut`               |

To change this, set the environment variable "SHORTCUT_CONFIG".

Windows:

```powershell
> $Env:SA_CONFIG=<filepath>
```

Linux:

```sh
export SA_COFNIG=<filepath>
```

## shortcut files

Please view `docs/shortcut_files.md` for this.

## CLI

Please view `docs/cli.md` for this.

## Templating

Please view `docs/templating.md` for this.

# Future Features

__Please note: This section is for me to outline some future plans I have for this project. I cannot guarantee these features will materialize.__

- Unit Tests
- Simple Command Conditionals
    - Allows for commands to run if conditions are met. Such as a flag is specified or a command fails.
- Templated Variables
- Default values for environment variables
- Built-in shortcut file linter
