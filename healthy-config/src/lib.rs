//! Author - Arthur Asatryan<br/>
//! Email - biacoder@gmail.com

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

#[macro_use]
extern crate log;

extern crate app_dirs;

pub mod parser;
pub mod util;
pub mod email;

/// Application info constant
pub const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo { name: "healthy", author: "Biacode" };

///re-exports
pub use email::EmailConfiguration;
pub use parser::{YamlConfigurationParser, ConfigurationParseError};

/// Implement this trait for custom configuration.
pub trait Configuration {}

/// Implement this trait for custom parser error.
pub trait ParserError {}

/// Implement this trait for custom configuration parser.
pub trait Parser<T: Configuration> {
    /// Parsing error type
    type ParserError: ParserError;

    /// Parses configuration and returns result of parse.
    fn parse(self) -> Result<T, Self::ParserError>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}