use std::fs::{self, File};
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;

pub fn create_directory(dir_name: &str) -> Result<(), io::Error> {
    if std::path::Path::new(dir_name).exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Directory already exists."));
    }
    fs::create_dir(dir_name)?;
    std::env::set_current_dir(dir_name)?;
    Ok(())
}

pub fn agree_to_eula() {
    let eula_path = format!("eula.txt");
    let mut file = File::create(&eula_path).expect("Failed to create eula.txt");
    file.write_all(b"eula=true").expect("Failed to write to eula.txt");
}

pub fn create_start_script(server_jar: &str, version: &str) {
    let start_script_path = format!("run.sh");
    let java_path = if version < "1.17" {
        "/usr/lib/jvm/java-8-openjdk-amd64/bin/java"
    } else {
        "/usr/lib/jvm/java-17-openjdk-amd64/bin/java"
    };
    let start_script_content = format!(
        "#!/bin/bash\n{} -Xmx1024M -Xms1024M -jar {} nogui",
        java_path, server_jar
    );

    let mut file = File::create(&start_script_path).expect("Failed to create run.sh");
    file.write_all(start_script_content.as_bytes()).expect("Failed to write to run.sh");
    let _ = fs::set_permissions(&start_script_path, fs::Permissions::from_mode(0o755));
}