use glob::glob;
use std::collections::HashMap;
use std::env;
use std::fs::create_dir;
use std::path::{Path, PathBuf};

use crate::errors::SAError;
use crate::shortcut::Shortcut;

pub fn discover_commands(mut folder: String) -> Result<HashMap<String, Shortcut>, SAError> {
    let mut shortcuts: HashMap<String, Shortcut> = HashMap::new();
    let suffix_char = match folder.ends_with("/") {
        true => '/',
        false => '\\',
    };

    folder = match folder.strip_suffix(suffix_char) {
        Some(s) => s.to_string(),
        _ => folder,
    };

    let glob_pattern: String = format!("{}/**/*.y*ml", folder);

    let files = match glob(&glob_pattern) {
        Ok(f) => f,
        Err(e) => return Err(SAError::GlobFailure(e)),
    };

    let files: Vec<PathBuf> = files.into_iter().filter_map(|path| path.ok()).collect();

    for path in files {
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

#[cfg(test)]
mod tests {
    use crate::cli::{build_cli, discover_commands, discover_config_dir};
    use crate::shortcut::{Argument, ArgumentType, Command, Shortcut};
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;
    use tempdir::TempDir;
    use tmp_env::set_var;

    #[test]
    fn test_build_cli_no_shortcuts() {
        let cli = build_cli(vec![]);
        assert_eq!(cli.has_subcommands(), false);
        assert_eq!(cli.get_name(), "shortcut-alias");
        assert_eq!(cli.get_version(), Some(env!("CARGO_PKG_VERSION")));

        let mut options_iter = cli.get_arguments().into_iter();

        let item = options_iter.next().expect("color Arg not configured.");
        assert_eq!(item.get_id(), "color");
        assert_eq!(item.get_long(), Some("color"));
        assert_eq!(item.get_short(), Some('c'));

        let item = options_iter.next().expect("header Arg not configured.");
        assert_eq!(item.get_id(), "header");
        assert_eq!(item.get_long(), Some("header"));
        assert_eq!(item.get_short(), Some('e'));

        let item = options_iter.next().expect("body Arg not configured.");
        assert_eq!(item.get_id(), "body");
        assert_eq!(item.get_long(), Some("body"));
        assert_eq!(item.get_short(), Some('b'));

        let item = options_iter.next().expect("footer Arg not configured.");
        assert_eq!(item.get_id(), "footer");
        assert_eq!(item.get_long(), Some("footer"));
        assert_eq!(item.get_short(), Some('f'));

        let item = options_iter.next().expect("silent Arg not configured.");
        assert_eq!(item.get_id(), "silent");
        assert_eq!(item.get_long(), Some("silent"));
        assert_eq!(item.get_short(), Some('s'));

        assert!(options_iter.next().is_none());
    }

    #[test]
    fn test_build_cli_with_shortcuts() {
        let mut shortcuts: HashMap<String, Shortcut> = HashMap::new();
        shortcuts.insert(
            "example".to_string(),
            Shortcut {
                name: "ExampleCommand".to_string(),
                description: None,
                args: Some(vec![Argument {
                    arg_type: ArgumentType::Data,
                    name: "example".to_string(),
                    default: Some("no".to_string()),
                    help: Some("Some Help Text".to_string()),
                }]),
                env: None,
                variables: None,
                commands: vec![Command {
                    name: "ExampleCommand".to_string(),
                    description: None,
                    command: "echo 'HelloWorld!'".to_string(),
                }],
            },
        );

        let cli = build_cli(shortcuts.values().collect());
        assert_eq!(cli.has_subcommands(), true);
        assert_eq!(cli.get_name(), "shortcut-alias");
        assert_eq!(cli.get_version(), Some(env!("CARGO_PKG_VERSION")));

        let mut subcommand_iter = cli.get_subcommands().into_iter();

        let item = subcommand_iter
            .next()
            .expect("ExampleCommand Subcommand not configured.");
        assert_eq!(item.get_name(), "ExampleCommand");
        assert_eq!(item.has_subcommands(), false);

        let mut arg_iter = item.get_arguments().into_iter();

        let arg_item = arg_iter.next().expect("example Arg not configured.");
        assert_eq!(arg_item.get_id(), "example");
        assert_eq!(arg_item.get_long(), Some("example"));

        assert!(arg_iter.next().is_none());
    }

    #[test]
    fn test_discover_config_dir() {
        let config_dir = discover_config_dir();
        assert!(config_dir.contains("/.shortcut"));

        // Scope to temporarily set the SHORTCUT_ALIAS_CONFIG_DIR env variable.
        {
            let _tmp_env = set_var("SHORTCUT_ALIAS_CONFIG", "/some/config/dir");

            let config_dir = discover_config_dir();
            assert_eq!(config_dir, "/some/config/dir".to_string());
        }

        // Scope to temporarily set the SA_CONFIG_DIR env variable.
        {
            let _tmp_env = set_var("SA_CONFIG", "/some/other/config/dir");

            let config_dir = discover_config_dir();
            assert_eq!(config_dir, "/some/other/config/dir".to_string());
        }
    }

    #[test]
    fn test_discover_commands() {
        // Setup tempdir
        let tmp_dir = TempDir::new("shortcut_alias").expect("Couldn't setup tempdir.");
        let filepath = tmp_dir.path().join("version.yaml");

        println!("{}", filepath.display());
        let mut shortcut_file = File::create(&filepath).expect("Couldn't create temporary file.");
        writeln!(
            shortcut_file,
            "name: version\ncommands:\n    - name: Cargo Version\n      command: cargo -V"
        )
        .expect("Couldn't write to temporary file.");

        let _tmp_env = set_var("SHORTCUT_ALIAS_CONFIG", tmp_dir.path().as_os_str());

        // Perform Test
        let shortcuts = discover_commands(discover_config_dir());
        println!("{:?}", shortcuts);
        assert!(shortcuts.is_ok());

        let shortcuts = shortcuts.unwrap();

        let example_shortcut = shortcuts
            .get("version")
            .expect("Couldn't find configured shortcut.");

        assert_eq!(example_shortcut.name, "version".to_string());
        assert_eq!(example_shortcut.description, None);
        assert_eq!(example_shortcut.args, None);
        assert_eq!(example_shortcut.variables, None);
        assert_eq!(example_shortcut.env, None);
        assert_eq!(
            example_shortcut.commands,
            vec![Command {
                name: "Cargo Version".to_string(),
                description: None,
                command: "cargo -V".to_string()
            }]
        );

        // Cleanup
        drop(shortcut_file);
        tmp_dir.close().expect("Couldn't close temp_dir.");
    }
}
