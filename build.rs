fn main() {
    // get npm version
    let npm_version = std::process::Command::new("npm")
        .arg("--version")
        .output()
        .expect("failed to execute `npm --version`")
        .stdout;
    let npm_version = String::from_utf8(npm_version).unwrap().trim().to_string();

    eprintln!("npm version: {:?}", npm_version);
}
