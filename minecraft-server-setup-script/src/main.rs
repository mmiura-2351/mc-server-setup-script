mod cli;
mod config;
mod download;
mod setup;

use crate::config::Config;
use crate::download::{download_forge_installer, download_vanilla_server};
use crate::setup::{agree_to_eula, create_directory, create_start_script};
use cli::parse_cli;
use std::process;

#[tokio::main]
async fn main() {
    let cli = parse_cli();

    let version = cli.version.unwrap();
    let server_type = cli.server_type.unwrap();
    let dir_name = cli.dir_name.unwrap();

    let config = Config::new(&version, &server_type, &dir_name);

    if create_directory(&config.dir_name).is_err() {
        eprintln!("Directory {} already exists. Please specify another name", config.dir_name);
        process::exit(1);
    }

    let server_jar = match config.server_type.as_str() {
        "vanilla" => download_vanilla_server(&config.version).await,
        "forge" => {
            let jar_result = download_forge_installer(&config.version).await;
            jar_result
        }
        _ => {
            eprintln!("Invalid server type. Please specify 'vanilla' or 'forge'.");
            process::exit(1);
        }
    };

    if let Err(e) = server_jar {
        eprintln!("{}", e);
        process::exit(1);
    }

    let server_jar = server_jar.unwrap();

    // server_typeがforgeかつversionが1.17以降の場合のみ処理をスキップする
    if config.server_type == "forge" && config.version.as_str() >= "1.17" {
        println!("Skipping start script creation for Forge server version 1.17 or later.");
    } else {
        create_start_script(&server_jar, &config.version);
    }

    agree_to_eula();

    println!("Minecraft {} server setup is complete.", config.server_type);
    println!("To start the server, run '. /run.sh' to start the server.");
}
