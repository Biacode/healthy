//! Author - Arthur Asatryan<br/>
//! Email - biacoder@gmail.com

use super::{APP_INFO, app_dirs, serde_yaml};
use super::email::EmailConfiguration;
use healthy_core::{Parser, ParserError};

/// Email configuration parser
pub struct YamlConfigurationParser {
    file_name: String
}

impl YamlConfigurationParser {
    /// Creates a new email yaml configuration parser for the given `file_name`.
    pub fn new(file_name: String) -> Self {
        debug!("Creating a new configuration parser for file with name {}", file_name);
        YamlConfigurationParser { file_name: file_name }
    }
}

/// Possible error which can occur while parsing file.
#[derive(Debug)]
pub enum ConfigurationParseError {
    CantReadPath,
    CantReadFile,
    CantDeserializeFile
}

impl ParserError for ConfigurationParseError {}

impl Parser<EmailConfiguration> for YamlConfigurationParser {
    type ParserError = ConfigurationParseError;

    fn parse(self) -> Result<EmailConfiguration, ConfigurationParseError> {
        let parsed_string = read_configuration_file_as_string(&self)?;
        match serde_yaml::from_str(&read_configuration_file_as_string(&self)?) {
            Ok(config) => return {
                debug!("Successfully deserialize - {:?}", &config);
                Ok(config)
            },
            Err(e) => {
                error!("An error - {} occurs while trying to parse string - {}", e, &parsed_string);
                return Err(ConfigurationParseError::CantDeserializeFile);
            }
        }
    }
}

/// Reads the given `configuration_parser`'s file as string.
fn read_configuration_file_as_string(configuration_parser: &YamlConfigurationParser) -> Result<String, ConfigurationParseError> {
    let config_path = match app_dirs::get_app_root(app_dirs::AppDataType::UserConfig, &APP_INFO) {
        Ok(path) => path,
        Err(e) => {
            error!("An error - {} occurs while trying to read configuration path", e);
            return Err(ConfigurationParseError::CantReadPath);
        }
    };
    debug!("Successfully read configuration path - {:?}", &config_path);
    let parsed_string = match super::util::read_file(&config_path, &configuration_parser.file_name) {
        Ok(s) => s,
        Err(e) => return {
            error!("An error - {} occurs while trying to read file - {}", e, &configuration_parser.file_name);
            Err(ConfigurationParseError::CantReadFile)
        }
    };
    debug!("Successfully read configuration file as string - {}", &parsed_string);
    return Ok(parsed_string);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}

    #[test]
    fn has_email_configuration_parser() {
        let file_name = "healthy.yaml".to_owned();
        let parser = super::YamlConfigurationParser::new(file_name.clone());
        assert_eq!(file_name, parser.file_name);
    }
}

