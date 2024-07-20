use reqwest::{header::USER_AGENT, Client};
use serde_json::Value;
use std::fs::File;
use std::io::copy;

pub async fn download_vanilla_server(version: &str) -> Result<String, String> {
    let client = Client::new();
    let manifest_url = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
    // let response = reqwest::get(manifest_url).await.map_err(|_| "Failed to fetch version manifest")?;
    let response = client
        .get(manifest_url)
        .header(USER_AGENT, "MyMinecraftServerDownloader/1.0")
        .send()
        .await
        .map_err(|_| "Failed to fetch version manifest")?;
    let manifest: Value = response.json().await.map_err(|_| "Failed to parse version manifest")?;

    let version_url = manifest["versions"]
        .as_array()
        .ok_or("Invalid version manifest format")?
        .iter()
        .find(|v| v["id"] == version)
        .ok_or("Specified version not found")?["url"]
        .as_str()
        .ok_or("Invalid version manifest format")?;

    let response = reqwest::get(version_url).await.map_err(|_| "Failed to fetch version details")?;
    let version_details: Value = response.json().await.map_err(|_| "Failed to parse version details")?;
    let download_url = version_details["downloads"]["server"]["url"].as_str().ok_or("Server download URL not found")?;

    download_file(download_url, "server.jar").await
}

pub async fn download_forge_installer(version: &str) -> Result<String, String> {
    let client = Client::new();
    let promotions_url = "https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json";
    // let response = reqwest::get(promotions_url).await.map_err(|_| "Failed to fetch Forge promotions")?;
    let response = client
        .get(promotions_url)
        .header(USER_AGENT, "MyMinecraftServerDownloader/1.0")
        .send()
        .await
        .map_err(|_| "Failed to fetch Forge promotions")?;
    let promotions: Value = response.json().await.map_err(|_| "Failed to parse Forge promotions")?;

    let forge_version = promotions["promos"]
        .as_object()
        .ok_or("Invalid promotions format")?
        .iter()
        .find(|(k, _)| k.starts_with(version) && k.ends_with("recommended"))
        .ok_or("Specified Forge version not found")?
        .1
        .as_str()
        .ok_or("Invalid promotions format")?;

    let forge_url = format!(
        "https://maven.minecraftforge.net/net/minecraftforge/forge/{version}-{forge_version}/forge-{version}-{forge_version}-installer.jar",
        version = version,
        forge_version = forge_version
    );

    download_file(&forge_url, "forge-installer.jar").await?;

    // Run the Forge installer
    let status = std::process::Command::new("java")
        .arg("-jar")
        .arg("forge-installer.jar")
        .arg("--installServer")
        .status()
        .expect("Failed to run Forge installer");

    if !status.success() {
        return Err("Failed to install Forge server".to_string());
    }

    std::fs::remove_file("forge-installer.jar").map_err(|_| "Failed to remove Forge installer")?;

    // Forge1.17以降のバージョンではディレクトリ構造が変わるため別の処理を行う
    let server_jar = if version < "1.17" {
        let server_jar_str = std::fs::read_dir(".")
            .map_err(|_| "Failed to read directory")?
            .filter_map(Result::ok)
            .find(|entry| entry.path().is_file() && entry.file_name().to_str().map_or(false, |f| f.starts_with("forge-") && f.ends_with(".jar")))
            .ok_or("Forge server jar not found")?;
        server_jar_str.file_name().to_string_lossy().to_string()
    } else {
        let server_jar_str = "libraries/net/minecraftforge/forge/".to_string()
            + version
            + "-"
            + forge_version
            + "/forge-"
            + version
            + "-"
            + forge_version
            + "-server.jar";
        server_jar_str
    };

    Ok(server_jar)
}

async fn download_file(url: &str, filename: &str) -> Result<String, String> {
    let response = reqwest::get(url).await.map_err(|_| "Failed to download file")?;
    let mut file = File::create(filename).map_err(|_| "Failed to create file")?;
    let content = response.bytes().await.map_err(|_| "Failed to read response bytes")?;
    copy(&mut content.as_ref(), &mut file).map_err(|_| "Failed to copy content to file")?;

    Ok(filename.to_string())
}
