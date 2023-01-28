use colored::Colorize;
use std::collections::HashMap;

mod cli;
mod commands;
mod errors;
mod settings;
mod shortcut;
use cli::{build_cli, discover_commands, discover_config_dir};
use commands::{run_command, CommandOutput};
use errors::SAError;
use settings::Settings;
use shortcut::{Shortcut, Variables};

fn shortcut_alias() -> Result<(), SAError> {
    let config_dir: String = discover_config_dir();
    let shortcuts: HashMap<String, Shortcut> = discover_commands(config_dir)?;
    let mut cli = build_cli(shortcuts.values().into_iter().collect());
    let cli_matches = &cli.clone().get_matches();

    let settings = Settings::new_from_matches(cli_matches);
    settings.set_terminal_color();

    if let Some((cmd_name, arg_matches)) = cli_matches.subcommand() {
        let shortcut = &shortcuts[cmd_name];

        let mut vars = Variables::new(shortcut, arg_matches);

        let mut first: bool = true;

        for cmd in shortcut.commands.iter() {
            if settings.show_header {
                let mut header = format!("{:=<80}", format!("[SA] Running '{}' ", &cmd.name));

                if let Some(desc) = &cmd.description {
                    header = format!("{header}\n{desc}\n{:=<80}", String::new());
                };

                if first {
                    println!("{}", header.green());
                    first = false;
                } else {
                    println!("\n{}", header.green());
                };
            };

            let command: String = vars.render_command(&cmd.command);
            let result: CommandOutput = run_command(&command);

            if settings.show_body {
                print!("{}", &result.output);
            };

            if settings.show_footer {
                let footer: String = format!(
                    "{:=<80}",
                    format!("[SA] Exit Code: {} ", result.status.clone())
                );
                println!("{}", footer.green());
            };

            if result.status != 0 {
                return Err(SAError::CommandFailed(format!(
                    "Command '{}' failed.",
                    cmd.name
                )));
            }

            vars.add_command(cmd.name.to_owned(), result)
        }
    } else {
        cli.print_help().unwrap();
    };

    Ok(())
}

fn main() {
    let run_program = shortcut_alias();

    if let Err(error) = run_program {
        match error {
            SAError::ShortcutFileRead(err) => {
                println!("{}", format!("[SA] Failed to read file: {}", err).red());
            },
            SAError::ShortcutFileParse(err) => {
                println!(
                    "{}",
                    format!("[SA] Failed to parse YAML file: {}", err).red()
                );
            },
            SAError::CommandFailed(err) => {
                println!("{}", format!("[SA] Failed to run command: {}", err).red());
            }
        }
    };
}
