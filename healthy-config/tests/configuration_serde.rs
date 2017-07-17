extern crate healthy_config;
extern crate healthy_core;

mod helper;

use helper::create_email_configuration_file;
use healthy_config::*;

#[test]
fn can_parse_email_configuration_from_file() {
    let file_name = "healthy.yaml".to_owned();
    let configuration = create_email_configuration_file(&file_name);
    let result = YamlFileConfigurationParser::new(file_name).parse();
    assert_eq!(configuration, result.unwrap());
}

#[test]
fn when_email_configuration_file_does_not_exists() {}