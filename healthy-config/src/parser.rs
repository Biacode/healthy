use super::{util, serde_yaml};
use super::email::EmailConfiguration;
use healthy_core::{Parser, ParserError};

/// File configuration parser.
pub trait FileConfigurationParser {
    fn get_file_name(&self) -> &String;
}

/// yaml file configuration parser.
pub struct YamlFileConfigurationParser {
    file_name: String
}

impl FileConfigurationParser for YamlFileConfigurationParser {
    fn get_file_name(&self) -> &String {
        &self.file_name
    }
}

impl YamlFileConfigurationParser {
    /// Creates a new yaml file configuration parser for the given `file_name`.
    pub fn new(file_name: String) -> Self {
        debug!("Creating a new configuration parser for file with name {}", file_name);
        YamlFileConfigurationParser { file_name }
    }
}

/// Possible error which can occur while parsing file.
#[derive(Debug, Eq, PartialEq)]
pub enum FileConfigurationParseError {
    CantReadPath,
    CantReadFile,
    CantDeserializeFile
}

impl ParserError for FileConfigurationParseError {}

impl Parser<EmailConfiguration> for YamlFileConfigurationParser {
    type ParserError = FileConfigurationParseError;

    /// Parses yaml email configuration file in to `EmailConfiguration` struct
    ///
    /// # Examples
    ///
    /// ```rust
    /// use healthy_config::Parser;
    /// healthy_config::parser::YamlFileConfigurationParser::new("foo.txt".to_owned()).parse();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Err(healthy_config::parser::FileConfigurationParseError)` if failed to parse configuration file
    fn parse(self) -> Result<EmailConfiguration, FileConfigurationParseError> {
        let parsed_string = util::read_parser_file(&self)?;
        match serde_yaml::from_str(&parsed_string) {
            Ok(config) => return {
                debug!("Successfully deserialize - {:?}", &config);
                Ok(config)
            },
            Err(e) => {
                error!("An error - {} occurs while trying to parse string - {}", e, &parsed_string);
                return Err(FileConfigurationParseError::CantDeserializeFile);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}

    #[test]
    fn has_email_configuration_parser() {
        let file_name = "healthy.yaml".to_owned();
        let parser = super::YamlFileConfigurationParser::new(file_name.clone());
        assert_eq!(file_name, parser.file_name);
    }
}