use std::{fs, io::Error as IoError};
use serde::{Serialize, Deserialize};
use toml;


#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    profile_id: Option<String>,
    api_token: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub profile_id: String,
    pub api_token: String,
}

impl Config {
    pub fn new() -> Self {
        let config_path: String = "./Config.toml".to_owned();

        let result: Result<String, IoError> = fs::read_to_string(config_path);

        let content: String = match result {
            Ok(fileConfig) => fileConfig,
            Err(error) => "".to_owned()
        };


        let config_toml = toml::from_str(&content).unwrap_or_else(| _ | {
            println!("Erro ao criar ConfigToml Object");
            ConfigToml{
                profile_id: None,
                api_token: None,
            }
        });

        let profile_id: String = match config_toml.profile_id {
            Some(profile_id) => profile_id,
            None => {
                println!("Error: profile_id não encontrado no Config.toml");
                "unknown".to_owned()
            }
        };

        let api_token: String = match config_toml.api_token {
            Some(api_token) => api_token,
            None => {
                println!("Error: api_token não encontrado no Config.toml");
                "unknown".to_owned()
            }
        };


        Config {
            profile_id,
            api_token,
        }
    }
}