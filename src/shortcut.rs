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

#[cfg(test)]
mod tests {
    use crate::cli::build_cli;
    use crate::commands::CommandOutput;
    use crate::shortcut::{Argument, ArgumentType, Command, Shortcut, Variables};
    use clap::ArgAction;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;
    use tempdir;
    use tmp_env;

    struct TempDir {
        directory: tempdir::TempDir,
    }

    impl TempDir {
        fn new(path: String) -> Self {
            let temp_dir = tempdir::TempDir::new(&path)
                .expect(&format!("Couldn't create temporary directory: {path}"));

            Self {
                directory: temp_dir,
            }
        }

        fn create_file(&self, filename: &str, content: String) -> File {
            let mut file = File::create(self.directory.path().join(filename))
                .expect("Couldn't create temporary file.");
            writeln!(file, "{}", content).expect("Couldn't write to temporary file.");
            file
        }

        fn close(self) {
            self.directory
                .close()
                .expect("Couldn't close temporary directory.");
        }
    }

    fn simple_shortcut() -> String {
        "---
name: version
description: Get the Cargo and Rust Versions.

commands:
  - name: Cargo Version
    description: Get the cargo version
    command: cargo -V
  - name: Rust Version
    description: get the rustc version
    command: rustc -V
"
        .to_string()
    }

    fn adv_shortcut() -> Shortcut {
        Shortcut {
            name: "version".to_string(),
            description: None,
            args: Some(vec![simple_argument()]),
            env: Some(vec!["ENV_VARIABLE".to_string()]),
            variables: Some(HashMap::from([(
                "VARIABLE".to_string(),
                "world".to_string(),
            )])),
            commands: vec![Command {
                name: "Python Version".to_string(),
                description: None,
                command: "{{ args.bin }} -V".to_string(),
            }],
        }
    }

    fn simple_argument() -> Argument {
        Argument {
            arg_type: ArgumentType::Data,
            name: "bin".to_string(),
            default: Some("python".to_string()),
            help: Some("The Python binary to use.".to_string()),
        }
    }

    fn simple_argument_flag() -> Argument {
        Argument {
            arg_type: ArgumentType::Flag,
            name: "color".to_string(),
            default: None,
            help: None,
        }
    }

    #[test]
    fn test_shortcut_from_file() {
        let tmp_dir = TempDir::new("shortcut_alias".to_string());
        let shortcut_file = &tmp_dir.create_file("version.yaml", simple_shortcut());

        let path = tmp_dir.directory.path().join("version.yaml");
        let shortcut = Shortcut::new(&path).expect("Couldn't crete Shortcut from file.");

        assert_eq!(shortcut.name, "version".to_string());
        assert_eq!(
            shortcut.description,
            Some("Get the Cargo and Rust Versions.".to_string())
        );
        assert_eq!(shortcut.commands.len(), 2);

        drop(shortcut_file);
        tmp_dir.close();
    }

    #[test]
    fn test_shortcut_get_command() {
        let tmp_dir = TempDir::new("shortcut_alias".to_string());
        let shortcut_file = &tmp_dir.create_file("version.yaml", simple_shortcut());

        let path = tmp_dir.directory.path().join("version.yaml");
        let shortcut = Shortcut::new(&path).expect("Couldn't crete Shortcut from file.");

        let command = shortcut.command();

        assert_eq!(command.get_name(), "version");
        assert_eq!(command.get_arguments().into_iter().next(), None);

        drop(shortcut_file);
        tmp_dir.close();
    }

    #[test]
    fn test_argument_get_argument() {
        let arg = simple_argument();
        let clap_arg = arg.argument();

        assert_eq!(clap_arg.get_id(), "bin");
        assert!(match clap_arg.get_action() {
            ArgAction::Set => true,
            _ => false,
        });
        assert_eq!(clap_arg.get_default_values(), &["python"]);

        let arg_flag = simple_argument_flag();
        let clap_arg = arg_flag.argument();

        assert_eq!(clap_arg.get_id(), "color");
        assert!(match clap_arg.get_action() {
            ArgAction::SetTrue => true,
            _ => false,
        });
    }

    #[test]
    fn test_new_variables() {
        let _tmp_env = tmp_env::set_var("ENV_VARIABLE", "some_variable");

        let shortcut = adv_shortcut();
        let cli = build_cli(vec![&shortcut]);
        let matches = cli.get_matches_from(["shortcut-alias", "version", "--bin", "python"]);

        let (_, sub_matches) = matches.subcommand()
            .expect("No subcommand specified.");

        let variables = Variables::new(&shortcut, &sub_matches);

        assert!(variables.args.contains_key("bin"));
        assert_eq!(variables.args.get("bin"), Some(&"python".to_string()));

        assert!(variables.variables.contains_key("VARIABLE"));
        assert_eq!(
            variables.variables.get("VARIABLE"),
            Some(&"world".to_string())
        );

        assert!(variables.env.contains_key("ENV_VARIABLE"));
        assert_eq!(
            variables.env.get("ENV_VARIABLE"),
            Some(&"some_variable".to_string())
        );
    }

    #[test]
    fn test_variables_add_command() {
        let _tmp_env = tmp_env::set_var("ENV_VARIABLE", "some_variable");

        let shortcut = adv_shortcut();
        let cli = build_cli(vec![&shortcut]);
        let matches = cli.get_matches_from(["shortcut-alias", "version", "--bin", "python"]);

        let (_, sub_matches) = matches.subcommand()
            .expect("No subcommand specified.");

        let mut variables = Variables::new(&shortcut, &sub_matches);

        assert_eq!(variables.commands.len(), 0);

        variables.add_command(
            "Python_Version".to_string(),
            CommandOutput {
                output: "python 3.10.0".to_string(),
                status: 0,
            },
        );

        assert_eq!(variables.commands.len(), 1);
        assert!(variables.commands.contains_key("python_version"));
    }

    #[test]
    fn test_variables_render_command() {
        let _tmp_env = tmp_env::set_var("ENV_VARIABLE", "some_variable");

        let shortcut = adv_shortcut();
        let cli = build_cli(vec![&shortcut]);
        let matches = cli.get_matches_from(["shortcut-alias", "version", "--bin", "python"]);
    
        let (_, sub_matches) = matches.subcommand()
            .expect("No subcommand specified.");

        let variables = Variables::new(&shortcut, &sub_matches);

        let rendered = variables.render_command(&shortcut.commands[0].command);
        assert_eq!(rendered, "python -V".to_string());
    }
}
