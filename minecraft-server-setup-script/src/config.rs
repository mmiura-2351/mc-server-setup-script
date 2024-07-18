pub struct Config {
    pub version: String,
    pub server_type: String,
    pub dir_name: String,
}

impl Config {
    pub fn new(version: &str, server_type: &str, dir_name: &str) -> Config {
        Config {
            version: version.to_string(),
            server_type: server_type.to_string(),
            dir_name: dir_name.to_string(),
        }
    }
}
