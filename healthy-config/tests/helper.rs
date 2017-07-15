extern crate serde;
extern crate serde_yaml;

extern crate healthy_config;
extern crate app_dirs;

use healthy_config::*;

/// prepares configuration file for test scenario.
pub fn prepare_configuration_file(file_name: &String) -> EmailConfiguration {
    let configuration = EmailConfiguration::new()
        .to("foo@bar.com".to_owned())
        .from("biacoder@gmail.com".to_owned())
        .content("Hello, World!".to_owned());
    path::write_file(
        &serialize_configuration(&configuration),
        &app_dirs::get_app_root(app_dirs::AppDataType::UserConfig, &APP_INFO).unwrap(),
        file_name
    ).unwrap();
    configuration
}

pub fn serialize_configuration<T: serde::Serialize>(configuration: &T) -> String {
    serde_yaml::to_string(&configuration).unwrap()
}