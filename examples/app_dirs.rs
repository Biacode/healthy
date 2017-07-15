extern crate app_dirs;

use app_dirs::*;

const APP_INFO: AppInfo = AppInfo { name: "healthy", author: "Biacode" };

fn main() {
    println!("{:?}", app_dirs::get_app_dir(app_dirs::AppDataType::UserConfig, &APP_INFO, "healthy.yaml"));
}