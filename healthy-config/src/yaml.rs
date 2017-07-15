//! **Author** - Arthur Asatryan<br/>
//! **Email** - biacoder@gmail.com

use super::std;
use super::{Configuration, ConfigurationParser, app_dirs, serde_yaml};
use app_dirs::*;

const CONFIGURATION_FILE: &'static str = "healthy.yaml";
const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo { name: "healthy", author: "Biacode" };

/// Email `yaml` configuration.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct EmailYamlConfiguration {
    from: String,
    to: String,
    content: String
}

impl EmailYamlConfiguration {
    /// Creates default configuration.
    ///
    /// Chain with builder methods to construct configuration.
    /// # Example
    /// ```rust
    ///YamlEmailConfiguration::new()
    ///    .to("foo@bar.com")
    ///    .from("biacoder@gmail.com")
    ///    .content("<p>Hello, World!</p>");
    /// ```
    fn new() -> Self {
        EmailYamlConfiguration {
            from: "".to_owned(),
            to: "".to_owned(),
            content: "".to_owned()
        }
    }

    /// From email.
    fn from(mut self, from: String) -> Self {
        self.from = from;
        self
    }

    /// To email.
    fn to(mut self, to: String) -> Self {
        self.to = to;
        self
    }

    /// Either html or plain text.
    fn content(mut self, content: String) -> Self {
        self.content = content;
        self
    }
}

impl Configuration for EmailYamlConfiguration {}

/// Email `yaml` configuration parser
struct EmailYamlConfigurationParser {}

impl EmailYamlConfigurationParser {
    fn new() -> Self {
        EmailYamlConfigurationParser {}
    }
}

/// Possible error which can occur while parsing `yaml`
#[derive(Debug)]
enum HealthyYamlParserError {
    CantReadPath,
    CantReadFile,
    CantDeserializeYaml
}

impl ConfigurationParser<EmailYamlConfiguration> for EmailYamlConfigurationParser {
    type HealthyParserError = HealthyYamlParserError;

    fn parse(self) -> Result<EmailYamlConfiguration, HealthyYamlParserError> {
        let config_path = match get_app_dir(AppDataType::UserConfig, &APP_INFO, CONFIGURATION_FILE) {
            Ok(path) => path,
            Err(_) => return Err(HealthyYamlParserError::CantReadPath)
        };
        let yaml_string = match read_file(config_path) {
            Ok(parsed_string) => parsed_string,
            Err(_) => return Err(HealthyYamlParserError::CantReadFile)
        };
        match serde_yaml::from_str(&yaml_string) {
            Ok(deser_config) => return Ok(deser_config),
            Err(_) => return Err(HealthyYamlParserError::CantDeserializeYaml)
        }
    }
}

/// Reads file for the given path.
fn read_file(conf_path: std::path::PathBuf) -> Result<String, std::io::Error> {
    use std::io::Read;
    let file = std::fs::File::open(conf_path)?;
    let mut buf_reader = std::io::BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn it_works() {}

    #[test]
    fn has_yaml_email_configuration_builder() {
        let to = "foo@bar.com".to_owned();
        let from = "biacoder@gmail.com".to_owned();
        let content = "<p>Hello, World!</p>".to_owned();
        let builder = EmailYamlConfiguration::new()
            .to(to.clone())
            .from(from.clone())
            .content(content.clone());
        assert_eq!(to, builder.to);
        assert_eq!(from, builder.from);
        assert_eq!(content, builder.content);
    }

    #[test]
    fn can_parse_email_configuration_from_file() {
        let configuration = EmailYamlConfigurationParser::new().parse();
        println!("configuration = {:?}", configuration.unwrap());
    }
}