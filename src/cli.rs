use glob::glob;
use std::collections::HashMap;
use std::env;
use std::fs::create_dir;
use std::path::{Path, PathBuf};

use crate::errors::SAError;
use crate::shortcut::Shortcut;

// Discover command files.
pub fn discover_commands(mut folder: String) -> Result<HashMap<String, Shortcut>, SAError> {
    let mut shortcuts: HashMap<String, Shortcut> = HashMap::new();

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
        let shortcut_file = Shortcut::new(&path)?;
        shortcuts.insert(shortcut_file.name.to_owned(), shortcut_file);
    }

    Ok(shortcuts)
}

pub fn discover_config_dir() -> String {
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

                if !folder_path.exists() {
                    create_dir(folder_path).unwrap();
                };
                directory = folder
            }
            None => directory = String::from("./.shortcut"),
        }
    };

    directory
}

pub fn build_cli(shortcuts: Vec<&Shortcut>) -> clap::Command {
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
                .help("Set to 'off' to turn terminal colour off."),
        )
        .arg(
            clap::Arg::new("header")
                .action(clap::ArgAction::Set)
                .long("header")
                .short('e')
                .default_value("on")
                .required(false)
                .help("Set to 'off' to not show command header."),
        )
        .arg(
            clap::Arg::new("body")
                .action(clap::ArgAction::Set)
                .long("body")
                .short('b')
                .default_value("on")
                .required(false)
                .help("Set to 'off' to not show command output."),
        )
        .arg(
            clap::Arg::new("footer")
                .action(clap::ArgAction::Set)
                .long("footer")
                .short('f')
                .default_value("on")
                .required(false)
                .help("Set to 'off' to not show the command footer."),
        )
        .arg(
            clap::Arg::new("silent")
                .action(clap::ArgAction::SetTrue)
                .long("silent")
                .short('s')
                .required(false)
                .help("Set to suppress all output."),
        );

    for config in shortcuts {
        cli = cli.subcommand(config.command());
    }

    cli
}
