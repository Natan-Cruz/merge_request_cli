use std::{thread, time, process::ExitCode, future::Future};
use loading::{Loading, Spinner};
use reqwest::{header::{CONTENT_TYPE, ACCEPT}};
use serde::{Serialize, Deserialize};
use json::{JsonValue};


#[derive(Serialize, Deserialize, Debug)]
struct Response {
    number: Option<i16>
}

#[derive(PartialEq, PartialOrd)]
enum MergeRequestErrorsKind {
    FatalError,
    Error
}
#[derive(PartialEq, PartialOrd)]
struct MergeRequestErrors {
    kind: MergeRequestErrorsKind,
    message: String
}


async fn _create_merge_request(api_token: &str, params: JsonValue, project_id: &String) -> Result<Response, MergeRequestErrors> {
    let mut url = String::new();

    url.push_str("https://multiplier.jetbrains.space/api/http/projects/id:");
    url.push_str(&project_id); 
    url.push_str("/code-reviews/merge-requests?$fields=number"); 

    let client: reqwest::Client = reqwest::Client::new();

    let response: Result<reqwest::Response, reqwest::Error> = client.post(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", api_token)
        .body(json::stringify(params))
        .send()
        .await;

   match response {
       Ok(response) =>  match response.status() {
            reqwest::StatusCode::OK => {
                // on success, parse our JSON to an APIResponse
                return match response.json::<Response>().await {
                    Ok(parsed) => Ok(parsed),
                    Err(_) => {
                        return Err(MergeRequestErrors{ 
                            kind: MergeRequestErrorsKind::FatalError, 
                            message: "MR criado, mas ocorreu um erro ao parsear resposta".to_string()
                        })
                    }
                };
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                return Err(MergeRequestErrors{ 
                    kind: MergeRequestErrorsKind::FatalError, 
                    message: "Token expirado".to_string()
                })
            }
            reqwest::StatusCode::CONFLICT => {
                return Err(MergeRequestErrors{ 
                    kind: MergeRequestErrorsKind::FatalError, 
                    message: "A branch atual já possuí um MR".to_string()
                })
            }
            reqwest::StatusCode::NOT_FOUND => {
                return Err(MergeRequestErrors{ 
                    kind: MergeRequestErrorsKind::FatalError, 
                    message: "A branch atual não está sincronizado com origem, faça um PUSH".to_string()
                })
            }
            other => {
                return Err(MergeRequestErrors{ 
                    kind: MergeRequestErrorsKind::Error, 
                    message: format!("Algo de errado aconteceu: Http Status Code: {other:?}")
                })
            }
        },
        Err(error) => {
            if error.is_connect() {
                return Err(MergeRequestErrors{ 
                    kind: MergeRequestErrorsKind::Error, 
                    message: "Erro de conexão".to_string()
                })
            }

            if error.is_timeout() {
                return Err(MergeRequestErrors{ 
                    kind: MergeRequestErrorsKind::Error, 
                    message: "Timeout".to_string()
                })
            }

            return Err(MergeRequestErrors{ 
                kind: MergeRequestErrorsKind::Error, 
                message: "Erro desconnhecido".to_string()
            })
        }
   }
}

pub async fn create_merge_request(api_token: &str, project_id: &String, params: JsonValue,) -> Result<i16, String> {
    let retry_attempts = 3;

    let mut attempts = 0;
    let mut merge_request_response: Response = Response { number: None };
    let mut fatal_error: bool = false;

    while attempts < retry_attempts {

        // loading style
        let loading = Loading::new(Spinner::new(vec!["...", "●..", ".●.", "..●"]));
        loading.text("Criando Merge Request...");

        // realiza criação da MR
        let result = _create_merge_request(api_token, params.clone(), project_id).await;

        match result {
            Ok(result) => {
                loading.success("Merge Request criado com sucesso");
                merge_request_response = result
            },
            Err(error) => {
                if  error.kind == MergeRequestErrorsKind::FatalError {
                    fatal_error = true
                }

                loading.fail(error.message);
            } 
        }

        loading.end();

        if merge_request_response.number.is_some() || fatal_error {
            break;
        } 
        
        if attempts < 2 {
            println!("Tentando novamente em alguns segundos...");
        
            // pausa por 3 segundos antes de tentar novamente
            let three_seconds = time::Duration::from_secs(3);
            thread::sleep(three_seconds);
        }

        attempts += 1;
    }


    match merge_request_response.number {
        Some(merge_request_number) => Ok(merge_request_number),
        None => Err(String::new())
    }
}
