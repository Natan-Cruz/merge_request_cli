use std::{fs, io::Error as IoError};
use serde::{Serialize, Deserialize};
use toml;

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    authorization: AuthorizationToml,
    project_config: ProjectConfigToml
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthorizationToml {
    profile_id: Option<String>,
    api_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectConfigToml {
    project_id: Option<String>,
    repository: Option<String>,
    reviewers: Option<Vec<String>>,
    scopes: Option<Vec<String>>,
    prefix: Option<Vec<String>>,
    default_target_branch: Option<String>
}

#[derive(Debug)]
pub struct Authorization {
    pub profile_id: String,
    pub api_token: String,
}

pub struct Configurations {
    pub project_id: String,
    pub repository: String,
    pub reviewers: Vec<String>,
    pub scopes: Vec<String>,
    pub prefix: Vec<String>,
    pub default_target_branch: String
}

impl Authorization {
    pub fn new() -> Self {
        let result: Result<String, IoError> = fs::read_to_string("./Config.toml");

        let content: String = match result {
            Ok(file_config) => file_config,
            Err(_) => String::new()
        };


        let config_toml = toml::from_str(&content).unwrap_or_else(| _ | {
            println!("Erro ao criar AuthorizationToml Object");
            ConfigToml{
                authorization: AuthorizationToml { profile_id: None, api_token: None },
                project_config: ProjectConfigToml { 
                    project_id: None, 
                    repository: None, 
                    reviewers: None, 
                    scopes: None, 
                    prefix: None,
                    default_target_branch: None 
                }
            }
        });

        let profile_id: String = match config_toml.authorization.profile_id {
            Some(profile_id) => profile_id,
            None => String::new()
        };

        let api_token: String = match config_toml.authorization.api_token {
            Some(api_token) => api_token,
            None =>  String::new()
        };

        Authorization {
            profile_id,
            api_token,
        }
    }

    pub fn save_values(api_token: &str, profile_id: &str) -> Self {

        let result: Result<String, IoError> = fs::read_to_string("Config.toml");

        let content: String = match result {
            Ok(file_config) => file_config,
            Err(_) => String::new()
        };

        let mut config_toml = toml::from_str(&content).unwrap_or_else(| _ | {
            println!("Erro ao criar AuthorizationToml Object");
            
            ConfigToml{
                authorization: AuthorizationToml { profile_id: None, api_token: None },
                project_config: ProjectConfigToml { 
                    project_id: None, 
                    repository: None, 
                    reviewers: None, 
                    scopes: None, 
                    prefix: None,
                    default_target_branch: None 
                }
            }
        });

        config_toml.authorization.api_token = Some(api_token.to_string());
        config_toml.authorization.profile_id = Some(profile_id.to_string());

        let content_string: Result<String, toml::ser::Error> = toml::to_string(&config_toml);

        let content: String = match  content_string{
            Ok(response) => response,
            Err(err) => panic!("Erro ao transformar toml em string, {err:?}")
        };


        fs::write("./Config.toml", content).unwrap();

        Authorization { profile_id: api_token.to_string(), api_token: api_token.to_string() }
    }
}

impl Configurations {
    pub fn new() -> Self {
        let result: Result<String, IoError> = fs::read_to_string("./Config.toml");

        let content: String = match result {
            Ok(file_config) => file_config,
            Err(_) => String::new()
        };


        let config_toml = toml::from_str(&content).unwrap_or_else(| _ | {
            println!("Erro ao criar AuthorizationToml Object");
            ConfigToml{
                authorization: AuthorizationToml { profile_id: None, api_token: None },
                project_config: ProjectConfigToml { 
                    project_id: None, 
                    repository: None, 
                    reviewers: None, 
                    scopes: None, 
                    prefix: None,
                    default_target_branch: None 
                }
            }
        });

        let project_id: String = match config_toml.project_config.project_id {
            Some(result) => result,
            None => String::new()
        };


        let repository: String = match config_toml.project_config.repository {
            Some(result) => result,
            None => String::new()
        };

        let reviewers: Vec<String> = match config_toml.project_config.reviewers {
            Some(result) => result,
            None => Vec::new()
        };

        let scopes: Vec<String> = match config_toml.project_config.scopes {
            Some(result) => result,
            None => Vec::new()
        };

        let prefix: Vec<String> = match config_toml.project_config.prefix {
            Some(result) => result,
            None => Vec::new()
        };

        let default_target_branch: String = match config_toml.project_config.default_target_branch {
            Some(result) => result,
            None => String::new()
        };

        Configurations {
            project_id,
            repository,
            reviewers,
            scopes,
            prefix,
            default_target_branch
        }
    }
}