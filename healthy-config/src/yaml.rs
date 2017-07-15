//! Author - Arthur Asatryan<br/>
//! Email - biacoder@gmail.com

use super::{ConfigurationParser, app_dirs, serde_yaml, APP_INFO};
use super::email::EmailConfiguration;

/// Email configuration parser
pub struct EmailConfigurationParser {
    file_name: String
}

impl EmailConfigurationParser {
    /// Creates a new email yaml configuration parser for the given `file_name`.
    pub fn new(file_name: String) -> Self {
        debug!("Creating a new email configuration parser for file {}", file_name);
        EmailConfigurationParser { file_name: file_name }
    }
}

/// Possible error which can occur while parsing `yaml` file.
#[derive(Debug)]
pub enum EmailConfigurationParserError {
    CantReadPath,
    CantReadFile,
    CantDeserializeFile
}

impl ConfigurationParser<EmailConfiguration> for EmailConfigurationParser {
    type HealthyParserError = EmailConfigurationParserError;

    fn parse(self) -> Result<EmailConfiguration, EmailConfigurationParserError> {
        let config_path = match app_dirs::get_app_root(app_dirs::AppDataType::UserConfig, &APP_INFO) {
            Ok(path) => path,
            Err(e) => {
                error!("An error - {} occurs while trying to read configuration path", e);
                return Err(EmailConfigurationParserError::CantReadPath);
            }
        };
        debug!("Successfully read configuration path - {:?}", &config_path);
        let parsed_string = match super::path::read_file(&config_path, &self.file_name) {
            Ok(s) => s,
            Err(e) => return {
                error!("An error - {} occurs while trying to read file - {}", e, &self.file_name);
                Err(EmailConfigurationParserError::CantReadFile)
            }
        };
        debug!("Successfully read configuration file as string - {}", &parsed_string);
        match serde_yaml::from_str(&parsed_string) {
            Ok(config) => return {
                debug!("Successfully deserialize - {:?}", &config);
                Ok(config)
            },
            Err(e) => {
                error!("An error - {} occurs while trying to parse string - {}", e, &parsed_string);
                return Err(EmailConfigurationParserError::CantDeserializeFile);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}

    #[test]
    fn has_email_yaml_configuration_parser() {
        let file_name = "healthy.yaml".to_owned();
        let parser = super::EmailConfigurationParser::new(file_name.clone());
        assert_eq!(file_name, parser.file_name);
    }
}