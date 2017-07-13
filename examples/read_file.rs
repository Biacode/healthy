#[macro_use]
extern crate log;
extern crate env_logger;
extern crate app_dirs;

use app_dirs::*;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const APP_INFO: AppInfo = AppInfo { name: "healthy", author: "Biacode" };

fn main() {
    env_logger::init();
    info!("starting up");
    let res = read_file();
    println!("res = {:?}", res);
}

fn read_file() -> Result<String, std::io::Error> {
    let file = File::open(app_dirs::get_app_dir(AppDataType::UserConfig, &APP_INFO, "foo.txt").unwrap())?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hello, World from foo.txt\n");
    Ok(contents)
}