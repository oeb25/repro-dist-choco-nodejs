use std::path::PathBuf;

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

    let npm = get_npm();

    eprintln!("using npm: {:?}", npm);

    // get npm version
    let npm_version = std::process::Command::new(npm)
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

fn get_npm() -> PathBuf {
    // first, add C:\Program Files\nodejs to the path if we are on windows
    if cfg!(windows) {
        let nodejs_path = PathBuf::from("C:\\Program Files\\nodejs");
        let path = std::env::var("PATH").expect("PATH environment variable is not set");
        let mut paths = std::env::split_paths(&path).collect::<Vec<_>>();
        paths.push(nodejs_path);
        let new_path = std::env::join_paths(paths).expect("failed to join paths");
        unsafe {
            std::env::set_var("PATH", &new_path);
        }
    }

    // check if npm is in path
    let npm_in_path = std::process::Command::new("npm")
        .arg("--version")
        .output()
        .is_ok();
    if npm_in_path {
        return PathBuf::from("npm");
    }

    // check if npm is in C:\Program Files\nodejs\npm
    let npm_path = PathBuf::from("C:\\Program Files\\nodejs\\npm");
    if npm_path.exists() {
        return npm_path;
    }

    panic!("npm not found");
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
