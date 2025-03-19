fn main() {
    if let Err(err) = list_program_files() {
        eprintln!("failed to list program files: {}", err);
    }
    if let Err(err) = list_node_js_files() {
        eprintln!("failed to list nodejs files: {}", err);
    }
    // print path
    let path = std::env::var("PATH").expect("PATH environment variable is not set");
    eprintln!("PATH: {:?}", path);

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

fn list_node_js_files() -> Result<(), String> {
    let path = "C:\\Program Files\\nodejs";
    let node_js_files =
        std::fs::read_dir(path).map_err(|err| format!("failed to read {}: {}", path, err))?;
    for entry in node_js_files {
        let entry = entry.map_err(|err| format!("failed to read entry: {}", err))?;
        let path = entry.path();
        eprintln!("nodejs file: {}", path.display());
    }
    Ok(())
}
