use std::process::Command;

fn main() {
    let foo = Command::new("git")
        .arg("log")
        .arg("--format='{\"commitHash\": \"%H\", \"authorName\": \"%an\", \"authorDate\": \"%ad\", \"subject\": \"%s\"}'")
        .output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    if foo.status.success() {
        let s = String::from_utf8_lossy(&foo.stdout);
        println!("{}", s);
    }
}