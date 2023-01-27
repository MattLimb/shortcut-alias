
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
fn discover_commands(mut folder: String) -> Vec<Shortcut>{
    let mut shortcuts: Vec<Shortcut> = vec![];

    if folder.ends_with('/') {
        folder = String::from(folder.strip_suffix('/').unwrap());
    } else if folder.ends_with('\\') {
        folder = String::from(folder.strip_suffix('\\').unwrap());
    };

    let glob_pattern: String = format!("{}/**/*.[yaml]*", folder);

    let files = glob(&glob_pattern).unwrap();
    let files: Vec<PathBuf> = files.into_iter().filter_map(|path| path.ok()).collect();

    for path in files {
        println!("{}", &path.display());
    
        let shortcut_file = Shortcut::new(&path);

        if let Ok(shortcut) = shortcut_file {
            shortcuts.push(shortcut);
        } else {
            panic!("Shortcut file is not Valid.");
        };
    }

    shortcuts
}


fn discover_config_dir() -> String {
    let directory: String;


    if let Ok(dir) = env::var("SHORTCUT_ALIAS_CONFIG") {
        directory = dir;
    } else if let Ok(dir) = env::var("SA_CONFIG") {
        directory = dir;
    } else {
        match home::home_dir() {
            Some(dir) => {
                let folder: String = format!("{}/.shortcut", dir.display());
                let folder_path: &Path = Path::new(&folder);

                if ! folder_path.exists() {
                    create_dir(folder_path).unwrap();
                };
                directory = folder
            },
            None => directory = String::from("./.shortcut")
        }
    };

    directory
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

    for shortcut in discover_commands(config_dir).iter() {
        cli = cli.subcommand(&shortcut.command());
        shortcuts.insert(shortcut.name.clone(), shortcut.clone());
    }

    let matches = cli.clone().get_matches();

    let settings = Settings::new_from_matches(&matches);
    settings.set_terminal_color();

    if let Some((cmd_name, arg_matches)) = matches.subcommand() {
        let shortcut = &shortcuts[cmd_name];

        let mut vars = Variables::new();

        if let Some(arguments) = &shortcut.args {
            vars.add_args(arguments.clone(), arg_matches);
        };

        if let Some(variables) = &shortcut.variables {
            vars.add_variables(variables.clone());
        };

        if let Some(envs) = &shortcut.env {
            vars.add_envs(envs.clone());
        };

        shortcut.print_header(settings.show_header);

        let exit: Result<(), String> = shortcut.run(&mut vars, &settings);

        if exit.is_err() {
            println!("{:=<80}", "Command Failed! ");
        };
    } else {
        cli.print_long_help().unwrap();
    };

}
