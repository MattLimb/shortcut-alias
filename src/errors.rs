#[derive(Debug)]
pub enum SAError {
    ShortcutFileRead(std::io::Error),
    ShortcutFileParse(serde_yaml::Error),
    CommandFailed(String),
    GlobFailure(glob::PatternError),
}
