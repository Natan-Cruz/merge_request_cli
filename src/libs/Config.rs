use std::{fs, io::Error as IoError, path::{Path}};
use serde::{Serialize, Deserialize};
use toml;

use std::env::current_dir;
use relative_path::RelativePath;

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
        let root = current_dir();

        let root = match root {
            Ok(root) => root,
            Err(_) => panic!("Erro ao obter caminho do arquivo de Configuração")
        };

        // to_path unconditionally concatenates a relative path with its base:
        let relative_path = RelativePath::new("Config.toml");
        let full_path = relative_path.to_path(&root);

        let result: Result<String, IoError> = fs::read_to_string(full_path);

        let content: String = match result {
            Ok(file_config) => file_config,
            Err(_) => "".to_owned()
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
                panic!("Error: profile_id não encontrado no Config.toml");
                
            }
        };

        let api_token: String = match config_toml.api_token {
            Some(api_token) => api_token,
            None => {
                panic!("Error: api_token não encontrado no Config.toml");
            }
        };


        Config {
            profile_id,
            api_token,
        }
    }
}