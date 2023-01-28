use core::convert::From;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::commands::CommandOutput;
use crate::errors::SAError;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Variables {
    pub args: HashMap<String, String>,
    pub variables: HashMap<String, String>,
    pub env: HashMap<String, String>,
    pub commands: HashMap<String, CommandOutput>,
}

impl Variables {
    pub fn new(shortcut: &Shortcut, cli_matches: &clap::ArgMatches) -> Variables {
        let mut vars = Variables {
            args: HashMap::new(),
            variables: if let Some(variables) = shortcut.variables.clone() {
                variables
            } else {
                HashMap::new()
            },
            env: HashMap::new(),
            commands: HashMap::new(),
        };

        if let Some(arguments) = shortcut.args.clone() {
            for arg in arguments {
                if let Some(value) = cli_matches.get_one::<String>(&arg.name) {
                    vars.args.insert(arg.name.clone(), value.clone());
                }
            }
        }

        if let Some(env_vars) = shortcut.env.clone() {
            for env_key in env_vars.iter() {
                if let Ok(value) = env::var(env_key) {
                    vars.env.insert(String::from(env_key), value);
                };
            }
        }

        vars
    }

    pub fn add_command(&mut self, command_name: String, command: CommandOutput) {
        let cmd_name: String = command_name.replace(' ', "_").to_lowercase();
        self.commands.insert(cmd_name, command);
    }

    pub fn render_command(&self, command: &str) -> String {
        minijinja::render!(
            command,
            args => self.args,
            variables => self.variables,
            env => self.env,
            commands => self.commands
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ArgumentType {
    #[serde(alias = "flag")]
    Flag,
    #[serde(alias = "data")]
    Data,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Argument {
    pub arg_type: ArgumentType,
    pub name: String,
    pub default: Option<String>,
    pub help: Option<String>,
}

impl Argument {
    pub fn argument(&self) -> clap::Arg {
        let mut arg: clap::Arg = clap::Arg::new(&self.name).long(&self.name);
        let mut action = clap::ArgAction::Set;

        // Set the flag variables.
        if self.arg_type == ArgumentType::Flag {
            // Default is false for a flag.
            arg = arg.default_value("false");
            action = clap::ArgAction::SetTrue;
        } else {
            // Set the default value if the argtype is data.
            if let Some(default) = &self.default {
                arg = arg.default_value(default);
            };
        };

        // Set help message.
        if let Some(help) = &self.help {
            arg = arg.help(help);
        };

        // Set the action required.
        arg = arg.action(action);

        arg
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub description: Option<String>,
    pub command: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    pub name: String,
    pub description: Option<String>,
    pub args: Option<Vec<Argument>>,
    pub env: Option<Vec<String>>,
    pub variables: Option<HashMap<String, String>>,
    pub commands: Vec<Command>,
}

impl Shortcut {
    pub fn new(filepath: &PathBuf) -> Result<Shortcut, SAError> {
        let content = match fs::read_to_string(filepath) {
            Ok(file_content) => file_content,
            Err(e) => return Err(SAError::ShortcutFileRead(e)),
        };

        match serde_yaml::from_str(&content) {
            Ok(short) => Ok(short),
            Err(e) => Err(SAError::ShortcutFileParse(e)),
        }
    }

    pub fn command(&self) -> clap::Command {
        let mut command = clap::Command::new(&self.name);

        // Set help message
        if let Some(help) = &self.description {
            command = command.about(help);
        };

        // Set arguments
        if let Some(arguments) = &self.args {
            for arg in arguments.iter() {
                command = command.arg(arg.argument());
            }
        };

        command
    }
}
