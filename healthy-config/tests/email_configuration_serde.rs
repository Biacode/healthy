extern crate healthy_config;

mod helper;

use helper::prepare_configuration_file;
use healthy_config::*;

#[test]
fn can_parse_email_configuration_from_file() {
    let file_name = "healthy.yaml".to_owned();
    let configuration = prepare_configuration_file(&file_name);
    let result = EmailConfigurationParser::new(file_name).parse();
    assert_eq!(configuration, result.unwrap());
}