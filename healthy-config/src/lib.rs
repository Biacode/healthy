#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;

extern crate app_dirs;

pub mod yaml;
pub mod path;

/// Implement this trait for custom configuration.
///
/// # Example
/// ```rust
///struct YamlEmailConfiguration {
///    to: String,
///    from: String
///}
///
///impl YamlEmailConfiguration {
///    fn new(to: String, from: String) -> Self {
///        YamlEmailConfiguration {
///            to: to,
///            from: from
///        }
///    }
///}
///
///impl Configuration for YamlEmailConfiguration {}
/// ```
pub trait Configuration {}

/// Implement this trait for custom configuration parser.
///
/// # Example
/// ```rust
///struct YamlEmailConfigurationParser {
///    configuration: YamlEmailConfiguration
///}
///
///impl ConfigurationParser<YamlEmailConfiguration> for YamlEmailConfigurationParser {
///    fn parse(self) -> YamlEmailConfiguration {
///        self.configuration
///    }
///}
/// ```
pub trait ConfigurationParser<T: Configuration> {
    fn parse(self) -> T;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}