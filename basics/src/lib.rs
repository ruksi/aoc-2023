use std::env;
use std::fs;
use std::path::PathBuf;

/// Read input contents from the file specified in the command line arguments.
pub fn read_input() -> Result<String, String> {
    let input_path = get_input_path()?;
    let input = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    // remove trailing newline(s) if present to get consistent input content
    if input.ends_with('\n') {
        return Ok(input.trim_end_matches('\n').to_string());
    }
    Ok(input)
}

/// Get and validate the input path from the command line arguments.
pub fn get_input_path() -> Result<PathBuf, String> {
    // iterators must be mutable to be iterated over
    let mut args = env::args();
    let _binary_path_we_dont_need = args.next();
    let relative = args.next().ok_or("Please provide input file path")?;
    let working_directory = env::current_dir().map_err(|e| e.to_string())?;
    let absolute = working_directory.join(relative);
    if !absolute.exists() {
        return Err(format!("File does not exist: {}", absolute.display()));
    }
    if !absolute.is_file() {
        return Err(format!("Path is not a file: {}", absolute.display()));
    }
    fs::File::options()
        .read(true)
        .open(absolute.clone())
        .map_err(|_e| format!("File is not readable: {}", absolute.display()))?; // probably 🤷
    Ok(absolute)
}
