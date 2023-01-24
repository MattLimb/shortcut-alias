
use clap;
use home;
use glob::glob;
use std::fs::create_dir;
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::env;

mod commands;
mod shortcut;
mod settings;
use shortcut::{Shortcut, Variables};
use settings::Settings;


// Discover command files.
fn discover_commands(mut folder: String) -> Vec<PathBuf>{
    let mut shortcuts: Vec<PathBuf> = vec![];

    if folder.ends_with("/") {
        folder = String::from(folder.strip_suffix("/").unwrap());
    }

    // Find .yaml files.
    let files = glob(&format!("{}/**/*.yaml", folder)).unwrap();
    for path in files.into_iter() {
        if let Ok(item) = path {
            shortcuts.push(item)
        }
    }

    // Find .yml files.
    let files = glob(&format!("{}/**/*.yml", folder)).unwrap();
    for path in files.into_iter() {
        if let Ok(item) = path {
            shortcuts.push(item)
        }
    }

    shortcuts
}


fn discover_config_dir() -> String {
    if let Ok(dir) = env::var("SHORTCUT_ALIAS_CONFIG") {
        return dir;
    } else if let Ok(dir) = env::var("SA_CONFIG") {
        return dir;
    } else {
        match home::home_dir() {
            Some(dir) => {
                let folder: String = format!("{}/.shortcut", dir.display());
                let folder_path: &Path = Path::new(&folder);

                if ! folder_path.exists() {
                    create_dir(folder_path).unwrap();
                }
                return folder
            },
            None => return String::from("./.shortcut")
        }
    }
}


fn main() {
    let mut cli = clap::Command::new("shortcut-alias")
        .about("A powerful alias tool.")
        .author("Matt Limb <matt.limb17@gmail.com>")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            clap::Arg::new("color")
                .action(clap::ArgAction::Set)
                .long("color")
                .short('c')
                .default_value("on")
                .required(false)
                .help("Set to 'off' to turn terminal colour off.")
        )
        .arg(
            clap::Arg::new("header")
                .action(clap::ArgAction::Set)
                .long("header")
                .short('e')
                .default_value("on")
                .required(false)
                .help("Set to 'off' to not show command header.")
        )
        .arg(
            clap::Arg::new("body")
                .action(clap::ArgAction::Set)
                .long("body")
                .short('b')
                .default_value("on")
                .required(false)
                .help("Set to 'off' to not show command output.")
        )
        .arg(
            clap::Arg::new("footer")
                .action(clap::ArgAction::Set)
                .long("footer")
                .short('f')
                .default_value("on")
                .required(false)
                .help("Set to 'off' to not show the command footer.")
        )
        .arg(
            clap::Arg::new("silent")
                .action(clap::ArgAction::SetTrue)
                .long("silent")
                .short('s')
                .required(false)
                .help("Set to suppress all output.")
        );

    let mut shortcuts: HashMap<String, Shortcut> = HashMap::new();
    let config_dir: String = discover_config_dir();
    println!("{}", &config_dir);

    for file_path in discover_commands(config_dir).iter() {
        if let Ok(cut) = Shortcut::new(file_path) {
            cli = cli.subcommand(&cut.command());
            shortcuts.insert(cut.name.clone(), cut);
        }
    }

    let matches = cli.clone().get_matches();

    let settings = Settings::new_from_matches(&matches);
    settings.set_terminal_color();

    if let Some((cmd_name, arg_matches)) = matches.subcommand() {
        let shortcut = &shortcuts[cmd_name];

        let mut vars = Variables::new();

        if let Some(arguments) = &shortcut.args {
            vars.add_args(arguments.clone(), arg_matches);
        }

        if let Some(variables) = &shortcut.variables {
            vars.add_variables(variables.clone());
        }

        if let Some(envs) = &shortcut.env {
            vars.add_envs(envs.clone());
        }

        shortcut.print_header(settings.show_header);

        let exit: Result<(), String> = shortcut.run(&mut vars, &settings);

        if let Err(_) = exit {
            println!("{:=<80}", "Command Failed! ");
        }
    } else {
        cli.print_long_help().unwrap();
    }

}
