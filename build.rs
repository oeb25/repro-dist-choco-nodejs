fn main() {
    match list_program_files() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("failed to list program files: {}", err);
        }
    }

    // get npm version
    let npm_version = std::process::Command::new("npm")
        .arg("--version")
        .output()
        .expect("failed to execute npm --version")
        .stdout;
    let npm_version = String::from_utf8(npm_version)
        .expect("npm --version output is not valid utf8")
        .trim()
        .to_string();

    eprintln!("npm version: {:?}", npm_version);
}

fn list_program_files() -> Result<(), String> {
    let program_files = std::fs::read_dir("C:\\Program Files")
        .map_err(|err| format!("failed to read C:\\Program Files: {}", err))?;
    for entry in program_files {
        let entry = entry.map_err(|err| format!("failed to read entry: {}", err))?;
        let path = entry.path();
        eprintln!("program file: {}", path.display());
    }
    Ok(())
}
