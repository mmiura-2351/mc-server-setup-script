use std::fs::{self, File};
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;

pub fn create_directory(dir_name: &str) -> Result<(), io::Error> {
    if std::path::Path::new(dir_name).exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "ディレクトリは既に存在します"));
    }
    fs::create_dir(dir_name)?;
    std::env::set_current_dir(dir_name)?;
    Ok(())
}

pub fn agree_to_eula() {
    let eula_path = format!("eula.txt");
    let mut file = File::create(&eula_path).expect("eula.txtの作成に失敗しました");
    file.write_all(b"eula=true").expect("eula.txtへの書き込みに失敗しました");
}

pub fn create_start_script(server_jar: &str, version: &str) {
    let start_script_path = format!("run.sh");
    let java_path = if version < "1.17" {
        "/usr/lib/jvm/java-8-openjdk-amd64/bin/java"
    } else {
        "java"
    };
    let start_script_content = format!(
        "#!/bin/bash\n{} -Xmx1024M -Xms1024M -jar {} nogui",
        java_path, server_jar
    );

    let mut file = File::create(&start_script_path).expect("run.shの作成に失敗しました");
    file.write_all(start_script_content.as_bytes()).expect("run.shへの書き込みに失敗しました");
    let _ = fs::set_permissions(&start_script_path, fs::Permissions::from_mode(0o755));
}