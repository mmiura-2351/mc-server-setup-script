use clap::Parser;
use std::io::{self, Write};

#[derive(Parser, Debug)]
pub struct Cli {
    /// サーバーのバージョン（任意）
    #[clap(short, long)]
    pub version: Option<String>,

    /// サーバーの種類（vanillaまたはforge、任意）
    #[clap(short, long)]
    pub server_type: Option<String>,

    /// ディレクトリ名（任意）
    #[clap(short, long)]
    pub dir_name: Option<String>,
}

pub fn parse_cli() -> Cli {
    let cli = Cli::parse();

    let version = if let Some(v) = &cli.version {
        v.clone()
    } else {
        print!("Server version: ");
        io::stdout().flush().expect("Flush Error");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input Error. Please try again.");
        input.trim().to_string()
    };

    let server_type = if let Some(sv) = &cli.server_type {
        sv.clone()
    } else {
        print!("Server type(vanilla|forge): ");
        io::stdout().flush().expect("Flush Error");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input Error. Please try again.");
        input.trim().to_string()
    };

    let dir_name = if let Some(dn) = &cli.dir_name {
        dn.clone()
    } else {
        print!("Directory name: ");
        io::stdout().flush().expect("Flush Error");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input Error. Please try again.");
        input.trim().to_string()
    };

    Cli {
        version: Some(version),
        server_type: Some(server_type),
        dir_name: Some(dir_name),
    }
}
