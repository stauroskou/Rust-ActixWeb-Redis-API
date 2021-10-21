use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub server_bind: String,
    pub redis: String,
    pub cache_timeout: u64,
    pub sentry_key: String,
}

impl Config {
    pub fn get_config() -> std::result::Result<Config, std::io::Error> {
        let json_file_path = Path::new("src/conf.json");
        let file = File::open(json_file_path);
        match file {
            Ok(f) => {
                let conf: Config = serde_json::from_reader(f).unwrap();
                Ok(conf)
            }
            Err(err) => Err(err),
        }
    }
}
