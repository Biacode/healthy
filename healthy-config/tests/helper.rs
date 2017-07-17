extern crate serde;
extern crate serde_yaml;

extern crate healthy_config;
extern crate app_dirs;

use healthy_config::*;

/// Prepares configuration file for test scenario.
pub fn create_email_configuration_file(file_name: &String) -> EmailConfiguration {
    let configuration = EmailConfiguration::new()
        .to("foo@bar.com".to_owned())
        .from("biacoder@gmail.com".to_owned())
        .content("Hello, World!".to_owned());
    util::write_file(
        &serialize_to_yaml(&configuration),
        &app_dirs::get_app_root(app_dirs::AppDataType::UserConfig, &APP_INFO).unwrap(),
        file_name
    ).unwrap();
    configuration
}

/// Serializes given serializable to string
pub fn serialize_to_yaml<T: serde::Serialize>(serializable: &T) -> String {
    serde_yaml::to_string(&serializable).unwrap()
}