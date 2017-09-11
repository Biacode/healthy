extern crate healthy_config;

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
fn when_email_configuration_file_does_not_exists() {
    let file_name = "healthy.yaml".to_owned();
    helper::remove_configuration_file(&file_name);
    match YamlFileConfigurationParser::new(file_name).parse() {
        Ok(_) => { panic!("The result should not be Ok"); }
        Err(err) => {
            assert_eq!(healthy_config::parser::FileConfigurationParseError::CantReadFile, err);
        }
    }
}