use super::{APP_INFO, Configuration, Parser, FileConfigurationParser, FileConfigurationParseError, std, app_dirs};

/// Reads file as `String` from the given path and file name.
///
/// # Examples
///
/// ```rust
/// healthy_config::util::read_file(&std::path::PathBuf::from("/tmp/"), &"foo.txt".to_owned());
/// ```
///
/// # Errors
///
/// Returns `Err(std::io::Error)` if failed to read file.
pub fn read_file(path: &std::path::PathBuf, file_name: &String) -> Result<String, std::io::Error> {
    use std::io::Read;
    debug!("Reading file with path - {:?} and file name - {}", path, file_name);
    let mut clone_config_path = path.clone();
    clone_config_path.push(&file_name);
    let file = std::fs::File::open(clone_config_path)?;
    let mut buf_reader = std::io::BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    debug!("Successfully read content - {}", &content);
    Ok(content)
}

/// Writes file for the given file content, path and file name.
///
/// # Examples
///
/// ```rust
/// healthy_config::util::write_file(&"Hello, World!".to_owned(), &std::path::PathBuf::from("/tmp"), &"foo.txt".to_owned());
/// ```
///
/// # Errors
///
/// Returns `Err(std::io::Error)` if failed to write file
pub fn write_file(file_content: &String, path: &std::path::PathBuf, file_name: &String) -> Result<(), std::io::Error> {
    use std::io::Write;
    debug!("Writing file with content - {}, to path - {:?} with file name - {}", file_content, path, file_name);
    let mut clone_config_path = path.clone();
    clone_config_path.push(&file_name);
    let file = std::fs::File::create(clone_config_path)?;
    let mut file = std::io::BufWriter::new(file);
    file.write_all(file_content.as_bytes())?;
    debug!("File has been successfully written.");
    Ok(())
}

/// Reads given `file_parser`'s file as `String`
///
/// # Examples
///
/// ```rust
/// extern crate healthy_config;
/// healthy_config::util::read_parser_file(&healthy_config::parser::YamlFileConfigurationParser::new("foo.txt".to_owned()));
/// ```
///
/// # Errors
///
/// Returns `Err(healthy_config::parser::FileConfigurationParseError)` if failed to read parser's file.
pub fn read_parser_file<C: Configuration, P: Parser<C> + FileConfigurationParser>(file_parser: &P) -> Result<String, FileConfigurationParseError> {
    let config_path = match app_dirs::get_app_root(app_dirs::AppDataType::UserConfig, &APP_INFO) {
        Ok(path) => path,
        Err(e) => {
            error!("An error - {} occurs while trying to read configuration path", e);
            return Err(FileConfigurationParseError::CantReadPath);
        }
    };
    debug!("Successfully read configuration path - {:?}", &config_path);
    let parsed_string = match super::util::read_file(&config_path, file_parser.get_file_name()) {
        Ok(s) => s,
        Err(e) => return {
            error!("An error - {} occurs while trying to read file - {}", e, file_parser.get_file_name());
            Err(FileConfigurationParseError::CantReadFile)
        }
    };
    debug!("Successfully read configuration file as string - {}", &parsed_string);
    return Ok(parsed_string);
}