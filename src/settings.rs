use colored::control::SHOULD_COLORIZE;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Settings {
    pub show_color: bool,
    pub show_header: bool,
    pub show_body: bool,
    pub show_footer: bool,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            show_color: true,
            show_header: true,
            show_body: true,
            show_footer: true,
        }
    }

    pub fn new_from_matches(args: &clap::ArgMatches) -> Settings {
        let mut settings = Settings::new();

        if let Some(value) = args.get_one::<String>("color") {
            if value == &String::from("on") {
                settings.show_color = true;
            } else {
                settings.show_color = false;
            };
        };

        if let Some(value) = args.get_one::<String>("header") {
            if value == &String::from("on") {
                settings.show_header = true;
            } else {
                settings.show_header = false;
            };
        };

        if let Some(value) = args.get_one::<String>("body") {
            if value == &String::from("on") {
                settings.show_body = true;
            } else {
                settings.show_body = false;
            };
        };

        if let Some(value) = args.get_one::<String>("footer") {
            if value == &String::from("on") {
                settings.show_footer = true;
            } else {
                settings.show_footer = false;
            };
        };

        if let Some(value) = args.get_one::<bool>("silent") {
            if *value {
                settings.show_color = false;
                settings.show_header = false;
                settings.show_body = false;
                settings.show_footer = false;
            };
        };

        settings
    }

    // Use the value of show_color to set the terminal color override.
    pub fn set_terminal_color(&self) {
        SHOULD_COLORIZE.set_override(self.show_color);
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::build_cli;
    use crate::settings::Settings;
    use colored::control::SHOULD_COLORIZE;

    #[test]
    fn test_new_settings() {
        let settings = Settings::new();

        assert_eq!(settings.show_color, true);
        assert_eq!(settings.show_header, true);
        assert_eq!(settings.show_body, true);
        assert_eq!(settings.show_footer, true);
    }

    #[test]
    fn test_should_colorize() {
        let mut settings = Settings::new();
        settings.set_terminal_color();
        assert_eq!(SHOULD_COLORIZE.should_colorize(), true);

        settings.show_color = false;
        settings.set_terminal_color();
        assert_eq!(SHOULD_COLORIZE.should_colorize(), false);
    }

    #[test]
    fn test_settings_from_matches() {
        let cli = build_cli(vec![]);
        let args = vec!["shortcut-alias", "--color", "off", "--footer", "off"];

        let matches = cli.get_matches_from(args);
        let settings = Settings::new_from_matches(&matches);

        assert_eq!(settings.show_color, false);
        assert_eq!(settings.show_header, true);
        assert_eq!(settings.show_body, true);
        assert_eq!(settings.show_footer, false);
    }
}
