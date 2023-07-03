use std::{thread, time};
use futures::{stream, StreamExt}; // 0.3.27

use json::object;
use loading::{Loading, Spinner};
use reqwest::{
    header::{
        CONTENT_TYPE, 
        ACCEPT
    }, 
    Client
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IssuesResponse {
    pub data: Option<Vec<IssuesResponseData>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssuesResponseData {
    pub projectRef: ProjectRef,
    pub number: i32,
    pub title: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectRef {
    pub key: ProjectRefKey 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectRefKey {
    pub key: String
}

#[derive(PartialEq, PartialOrd)]
enum IssuesRequestErrorsKind {
    FatalError,
    Error
}
#[derive(PartialEq, PartialOrd)]
struct IssuesRequestErrors {
    kind: IssuesRequestErrorsKind,
    message: String
}

async fn _get_in_progress_issues(api_token: &String, project_id: &String) -> Result<IssuesResponse, IssuesRequestErrors> {

    let mut url = String::new();

    url.push_str("https://multiplier.jetbrains.space/api/http/projects/id:");
    url.push_str(&project_id);
    url.push_str("/planning/issues?assigneeId=me&statuses=43Wrzo4NkB7E&sorting=UPDATED&descending=true&$fields=data(number,projectRef(key),title)");


    let client: reqwest::Client = reqwest::Client::new();

    let response: Result<reqwest::Response, reqwest::Error> = client.get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", api_token)
        .send()
        .await;

    match response {
        Ok(response) => match response.status() {
            reqwest::StatusCode::OK => {
                match response.json::<IssuesResponse>().await {
                    Ok(parsed) => return Ok(parsed),
                    Err(err) =>  return Err(IssuesRequestErrors{ 
                        kind: IssuesRequestErrorsKind::Error, 
                        message: "Erro ao parsear resposta".to_string()
                    })
                }
            },
            reqwest::StatusCode::UNAUTHORIZED => {
                return Err(IssuesRequestErrors{
                    kind: IssuesRequestErrorsKind::FatalError,
                    message: "Token expirado".to_string()
                });
            }
            other => {
                return Err(IssuesRequestErrors{
                    kind: IssuesRequestErrorsKind::Error,
                    message: format!("Algo de errado aconteceu: {:?}", other)
                });
            }
        },
        Err(error) => {
            if error.is_connect() {
                return Err(IssuesRequestErrors{
                    kind: IssuesRequestErrorsKind::Error,
                    message: "Erro de conexão com a internet".to_string()
                })
            }

            if error.is_timeout() {
                return Err(IssuesRequestErrors{
                    kind: IssuesRequestErrorsKind::Error,
                    message: "Timeout".to_string()
                })
            }

            return Err(IssuesRequestErrors{ 
                kind: IssuesRequestErrorsKind::Error,
                message: "Erro desconnhecido".to_string()
            })
        }
    }
}


pub async fn get_in_progress_issues(api_token: &String, project_id: &String) -> Result<Vec<IssuesResponseData>, String> {
    let retry_attempts = 3;

    let mut attempts = 0;
    let mut issues: IssuesResponse = IssuesResponse{ data: None };
    let mut fatal_error: bool = false;

    while attempts < retry_attempts {
       
        // loading style
        let loading = Loading::new(Spinner::new(vec!["...", "●..", ".●.", "..●"]));
        loading.text("Obtendo issues");

        // realiza criação da MR
        let result = _get_in_progress_issues(api_token, project_id).await;

        match result {
            Ok(res) => {
                match &res.data {
                    Some(issues) => {
                        if issues.is_empty() {
                            let empty_message: String = format!("\x1b[93m{}\x1b[0m", "Não há issues em progressos atrelados à você");
                            loading.warn(empty_message)
                        } else {
                            loading.success("Issues obtidas com sucesso");
                        }

                    },
                    None => {}                    
                }

                issues = res               
            },
            Err(error) => {

                if error.kind == IssuesRequestErrorsKind::FatalError {
                    fatal_error = true
                }

                loading.fail(error.message)
            }
        }

        loading.end();

        if issues.data.is_some() || fatal_error {
            break;
        } 
        
        if attempts < 2 {
            println!("Tentando novamente em alguns segundos...");
        
            // pausa por 3 segundos antes de tentar novamente
            let three_seconds = time::Duration::from_secs(3);
            thread::sleep(three_seconds);
        }

        attempts += 1;
    };

    match issues.data {
        Some(data) => return Ok(data),
        None => return Err(String::new())
    }
}

const CONCURRENT_REQUESTS: usize = 2;

pub async fn update_status(api_token: String, project_id: String, issues: Vec<String>) {

    let urls: Vec<String> = issues
        .iter()
        .map( | issue | {
            let mut url = String::new();

            url.push_str("https://multiplier.jetbrains.space/api/http/projects/id:");
            url.push_str(&project_id);
            url.push_str("/planning/issues/key:");
            url.push_str(issue);

            return url 
        })
        .collect();

    let client: Client = Client::new();

    // id do status abaixo representa "em revisão de código"
    let body = object!{ "status": "fNG0L1lSYbc" };

    stream::iter(&urls)
        .map(|url| {
            let client = &client;
            let body = &body;
            let api_token = &api_token;

            let loading = Loading::new(Spinner::new(vec!["...", "●..", ".●.", "..●"]));
            loading.text("Atualizado status das issues");
         
            async move {
                let response: Result<reqwest::Response, reqwest::Error> = client.patch(&**url)
                    .header(CONTENT_TYPE, "application/json")
                    .header(ACCEPT, "application/json")
                    .header("Authorization", api_token)
                    .body(json::stringify(body.clone()))
                    .send()
                    .await;

                match response {
                    Ok(response) => match response.status() {
                        reqwest::StatusCode::OK => {
                            let url_splitted = url.split(":").collect::<Vec<&str>>();
                            let issue = url_splitted[url_splitted.len() - 1];
                            loading.success(format!("{} foi atualizada com sucesso", issue))
                        },
                        reqwest::StatusCode::UNAUTHORIZED => {
                            loading.fail("Erro ao atualizar status da issue:: Token expirado")
                        },
                        other => {
                            println!("{other:?}");
                            loading.fail("Ocorreu um error ao atualizar status da issue")
                        }
                        
                    },
                    Err(error) => {
                        if error.is_connect() {
                            return loading.fail("Erro ao atualizar status da issue: Problema com sua conexão")
                        }

                        if error.is_timeout() {
                            return loading.fail("Erro ao atualizar status da issue: Timeout")
                        }

                        println!("{error:?}");

                        loading.fail("Ocorreu um error ao atualizar status da issue")
                    }
                }
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS)
        .collect::<Vec<_>>()
        .await;
}
