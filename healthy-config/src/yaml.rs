//! Author - Arthur Asatryan<br/>
//! Email - biacoder@gmail.com

use super::{ConfigurationParser, app_dirs, serde_yaml, APP_INFO};
use super::email::EmailConfiguration;

/// Email `yaml` configuration parser
pub struct EmailYamlConfigurationParser {
    file_name: String
}

impl EmailYamlConfigurationParser {
    /// Creates a new email yaml configuration parser for the given `file_name`.
    pub fn new(file_name: String) -> Self {
        debug!("Creating a new email yaml configuration parser for file {}", file_name);
        EmailYamlConfigurationParser { file_name: file_name }
    }
}

/// Possible error which can occur while parsing `yaml` file.
#[derive(Debug)]
pub enum HealthyYamlParserError {
    CantReadPath,
    CantReadFile,
    CantDeserializeYaml
}

impl ConfigurationParser<EmailConfiguration> for EmailYamlConfigurationParser {
    type HealthyParserError = HealthyYamlParserError;

    fn parse(self) -> Result<EmailConfiguration, HealthyYamlParserError> {
        let config_path = match app_dirs::get_app_root(app_dirs::AppDataType::UserConfig, &APP_INFO) {
            Ok(path) => path,
            Err(e) => {
                error!("An error - {} occurs while trying to read configuration path", e);
                return Err(HealthyYamlParserError::CantReadPath);
            }
        };
        debug!("Successfully read configuration path - {:?}", &config_path);
        let yaml_string = match super::path::read_file(&config_path, &self.file_name) {
            Ok(parsed_string) => parsed_string,
            Err(e) => return {
                error!("An error - {} occurs while trying to read file - {}", e, &self.file_name);
                Err(HealthyYamlParserError::CantReadFile)
            }
        };
        debug!("Successfully read configuration file as string - {}", &yaml_string);
        match serde_yaml::from_str(&yaml_string) {
            Ok(deser_config) => return {
                debug!("Successfully deserialize - {:?}", &deser_config);
                Ok(deser_config)
            },
            Err(e) => {
                error!("An error - {} occurs while trying to parse yaml string - {}", e, &yaml_string);
                return Err(HealthyYamlParserError::CantDeserializeYaml);
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
        let parser = super::EmailYamlConfigurationParser::new(file_name.clone());
        assert_eq!(file_name, parser.file_name);
    }
}