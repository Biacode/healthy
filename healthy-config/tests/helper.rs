extern crate serde;
extern crate serde_yaml;

extern crate healthy_config;
extern crate app_dirs;

use healthy_config::*;
use std::path::PathBuf;

/// Prepares configuration file for test scenario.
pub fn create_email_configuration_file(file_name: &String) -> EmailConfiguration {
    let configuration = EmailConfiguration::new()
        .to("foo@bar.com".to_owned())
        .from("biacoder@gmail.com".to_owned())
        .content("Hello, World!".to_owned());
    util::write_file(
        &serialize_to_yaml(&configuration),
        &get_user_config_app_root(),
        file_name
    ).unwrap();
    configuration
}

/// Serializes given serializable to string
pub fn serialize_to_yaml<T: serde::Serialize>(serializable: &T) -> String {
    serde_yaml::to_string(&serializable).unwrap()
}

#[allow(unused)]
/// Removes configuration file
pub fn remove_configuration_file(file_name: &String) {
    use std::fs::remove_file;
    let mut path = get_user_config_app_root();
    path.push(file_name.clone());
    remove_file(path);
}

/// Gets user's configuration application root
pub fn get_user_config_app_root() -> PathBuf {
    app_dirs::get_app_root(app_dirs::AppDataType::UserConfig, &APP_INFO).unwrap()
}