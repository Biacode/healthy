use std::env;

fn main() {
    let u = env::home_dir().map(|dir| dir.join(std::path::PathBuf::from("foo"))).unwrap();
    println!("u = {:?}", u);
}