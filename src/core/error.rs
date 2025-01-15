use std::fmt;

#[derive(Debug)]
pub enum Error {
    Plugin(String),
    Event(String),
    Gui(String),
    Action(String),
    Pipe(String),
    Config(String),
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Plugin(msg) => write!(f, "Plugin error: {}", msg),
            Error::Event(msg) => write!(f, "Event error: {}", msg),
            Error::Gui(msg) => write!(f, "GUI error: {}", msg),
            Error::Action(msg) => write!(f, "Action error: {}", msg),
            Error::Pipe(msg) => write!(f, "Pipe error: {}", msg),
            Error::Config(msg) => write!(f, "Config error: {}", msg),
            Error::Io(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
} 