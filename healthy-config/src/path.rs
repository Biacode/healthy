//! Author - Arthur Asatryan<br/>
//! Email - biacoder@gmail.com

use super::std;

/// Reads file for the given path and file name.
pub fn read_file(path: &std::path::PathBuf, file_name: &String) -> Result<String, std::io::Error> {
    use std::io::Read;
    let mut clone_config_path = path.clone();
    clone_config_path.push(&file_name);
    let file = std::fs::File::open(clone_config_path)?;
    let mut buf_reader = std::io::BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Writes file for the given file content, path and file name.
pub fn write_file(file_content: &String, path: &std::path::PathBuf, file_name: &String) -> Result<(), std::io::Error> {
    use std::io::Write;
    let mut clone_config_path = path.clone();
    clone_config_path.push(&file_name);
    let file = std::fs::File::create(clone_config_path)?;
    let mut file = std::io::BufWriter::new(file);
    file.write_all(file_content.as_bytes())?;
    Ok(())
}