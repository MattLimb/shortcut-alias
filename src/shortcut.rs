use core::convert::From;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::env;

use colored::Colorize;
use serde::{Serialize, Deserialize};

use crate::commands::run_command;
use crate::settings::Settings;


#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CommandOutput {
    pub output: String,
    pub status: i32
}


#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Variables {
    pub args: HashMap<String, String>,
    pub variables: HashMap<String, String>,
    pub env: HashMap<String, String>,
    pub commands: HashMap<String, CommandOutput>
}


impl Variables {
    pub fn new() -> Variables {
        Variables { 
            args: HashMap::new(),
            variables: HashMap::new(),
            env: HashMap::new(),
            commands: HashMap::new()
        }
    }

    pub fn add_args(&mut self, args: Vec<Argument>, arg_matches: &clap::ArgMatches) {
        for arg in args {
            if let Some(value) = arg_matches.get_one::<String>(&arg.name) {
                self.args.insert(arg.name.clone(), value.clone());
            }
        }
    }

    pub fn add_variables(&mut self, vars: HashMap<String, String>) {
        for (key, value) in vars.into_iter() {
            self.variables.insert(key, value);
        }
    }

    pub fn add_command(&mut self, command_name: String, command: CommandOutput) {
        let cmd_name: String = command_name.replace(' ', "_").to_lowercase();

        self.commands.insert(cmd_name, command);
    }

    pub fn add_envs(&mut self, env_vars: Vec<String>) {
        for env_key in env_vars.iter() {
            if let Ok(value) = env::var(env_key) {
                self.env.insert(String::from(env_key), value);
            };
        }
    }

}


#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ArgumentType {
    #[serde(alias="flag")]
    Flag,
    #[serde(alias="data")]
    Data
}


#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Argument {
    pub arg_type: ArgumentType,
    pub name: String,
    pub default: Option<String>,
    pub help: Option<String>
}

impl Argument {
    pub fn argument(&self) -> clap::Arg{
        let mut arg: clap::Arg = clap::Arg::new(&self.name)
            .long(&self.name);
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
    pub command: String
}

impl Command {
    pub fn run(&self, vars: &mut Variables, settings: &Settings) -> i32 {
        
        if settings.show_header {
            println!("\n{}", format!("{:=<80}", format!("{} ", &self.name)).green());
        };

        let mut jinja = minijinja::Environment::new();
        jinja.add_template("template", &self.command).unwrap();

        let template = jinja.get_template("template").unwrap();
        let command = &template.render(
            minijinja::context!(
                args => &vars.args,
                variables => &vars.variables,
                env => &vars.env,
                commands => &vars.commands
            )
        ).unwrap_or_else(|_| String::from(&self.command)); 

        let (output, status) = run_command(command);

        if settings.show_body {
            print!("{}", &output);
        };

        if settings.show_footer {
            let line: String = format!("{:=<80}", format!("Exit Code: {} ", status.clone()));
            println!("{}", line.green());
        };

        vars.add_command(
            String::from(&self.name),
            CommandOutput { 
                output,
                status
            }
        );

        status
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    pub name: String,
    pub description: Option<String>,
    pub args: Option<Vec<Argument>>,
    pub env: Option<Vec<String>>,
    pub variables: Option<HashMap<String, String>>,
    pub commands: Vec<Command>
}

impl Shortcut {
    pub fn new(filepath: &PathBuf) -> Result<Shortcut, String> {
        let file_display = &filepath.display();
        let content = fs::read_to_string(filepath);

        if content.is_err() {
            return Err(format!("Could not read shortcut file: {}", file_display));
        };

        let parsed = serde_yaml::from_str(&content.unwrap());

        if parsed.is_err() {
            return Err(format!("Could parse shortcut YAML: {}", file_display));
        };

        Ok(parsed.unwrap())
    }

    pub fn command(&self) -> clap::Command {
        let mut command = clap::Command::new(&self.name.clone());

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

    pub fn print_header(&self, show: bool) {
        if show {
            println!("{:=<80}", format!("{} - {} ", "Shortcut Alias", &self.name).green());

            if let Some(desc) = &self.description {
                println!("{}", format!("{} ", desc).green());
            };
        };
    }

    pub fn run(&self, vars: &mut Variables, settings: &Settings) -> Result<(), String> {
        for command in &self.commands {
            let command_status: i32 = command.run(vars, settings);

            if command_status != 0 {
                return Err(String::from("Command failed."));
            };
        }

        Ok(())
    }

}