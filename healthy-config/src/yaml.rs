//! **Author** - Arthur Asatryan<br/>
//! **Email** - biacoder@gmail.com

use super::std;
use super::{Configuration, ConfigurationParser, app_dirs, serde_yaml};

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

/// Email yaml configuration implementation of `Configuration`
impl Configuration for EmailYamlConfiguration {}

/// Email yaml configuration parser
struct EmailYamlConfigurationParser {}

/// Email yaml configuration parser concrete implementations
impl EmailYamlConfigurationParser {
    fn new() -> Self {
        EmailYamlConfigurationParser {}
    }
}

/// Email yaml configuration parser implementation of `ConfigurationParser`
/// with `EmailYamlConfiguration` configuration
impl ConfigurationParser<EmailYamlConfiguration> for EmailYamlConfigurationParser {
    fn parse(self) -> EmailYamlConfiguration {
        let config_path = app_dirs::get_app_dir(
            app_dirs::AppDataType::UserConfig,
            &APP_INFO,
            CONFIGURATION_FILE
        ).unwrap();
        let yaml_string = read_file(config_path).unwrap();
        let deser_config: EmailYamlConfiguration = serde_yaml::from_str(&yaml_string).unwrap();
        deser_config
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
        println!("configuration = {:?}", configuration);
    }
}