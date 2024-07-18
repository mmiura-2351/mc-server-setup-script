mod config;
mod download;
mod setup;
mod utils;

use crate::config::Config;
use crate::download::{download_vanilla_server, download_forge_installer};
use crate::setup::{create_directory, create_start_script, agree_to_eula};
use std::env;
use std::process;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <version> <type> <dir_name>", args[0]);
        process::exit(1);
    }

    let config = Config::new(&args[1], &args[2], &args[3]);
    
    if create_directory(&config.dir_name).is_err() {
        eprintln!("ディレクトリ {} は既に存在します。別の名前を指定してください。", config.dir_name);
        process::exit(1);
    }

    let server_jar = match config.server_type.as_str() {
        "vanilla" => download_vanilla_server(&config.version).await,
        "forge" => download_forge_installer(&config.version).await,
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
    
    agree_to_eula(&config.dir_name);
    create_start_script(&config.dir_name, &server_jar, &config.version);
    
    println!("Minecraft {} server setup is complete.", config.server_type);
    println!("サーバーを起動するには './start.sh' を実行してください。");
}
