extern crate healthy_core;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
#[macro_use]
extern crate log;
extern crate app_dirs;

///! This module holds parsing functionality.
pub mod parser;
///! This module holds utility functions.
pub mod util;
///! This module holds email specific structures.
pub mod email;

/// Application info constant.
pub const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo { name: "healthy", author: "Biacode" };

pub use email::EmailConfiguration;
pub use parser::{FileConfigurationParser, YamlFileConfigurationParser, FileConfigurationParseError};
pub use healthy_core::{Configuration, Parser};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}