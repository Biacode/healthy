//! Author - Arthur Asatryan<br/>
//! Email - biacoder@gmail.com

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

#[macro_use]
extern crate log;

extern crate app_dirs;

pub mod yaml;
pub mod path;
pub mod email;

/// Application info constant
pub const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo { name: "healthy", author: "Biacode" };

///reexports
pub use email::EmailConfiguration;
pub use yaml::{EmailConfigurationParser, EmailConfigurationParserError};

/// Implement this trait for custom configuration.
pub trait Configuration {}

/// Implement this trait for custom configuration parser.
pub trait ConfigurationParser<T: Configuration> {
    /// Parsing error type
    type HealthyParserError;

    /// Parses configuration and returns result of parsing.
    ///
    /// `Ok(T)` if parsing is succeed. <br/>
    /// `Err(HealthyParserError)` if failed.
    fn parse(self) -> Result<T, Self::HealthyParserError>;
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}