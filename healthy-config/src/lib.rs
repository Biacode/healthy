//! Author - Arthur Asatryan<br/>
//! Email - biacoder@gmail.com

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

#[macro_use]
extern crate log;

extern crate app_dirs;

extern crate healthy_core;

pub mod parser;
pub mod util;
pub mod email;

/// Application info constant
pub const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo { name: "healthy", author: "Biacode" };

pub use email::EmailConfiguration;
pub use parser::{YamlConfigurationParser, ConfigurationParseError};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}