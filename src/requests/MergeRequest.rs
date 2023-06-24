use std::{thread, time};
use loading::{Loading, Spinner};
use reqwest::header::{CONTENT_TYPE, ACCEPT};
use serde::{Serialize, Deserialize};
use json::{JsonValue};


#[derive(Serialize, Deserialize, Debug)]
struct Response {
    number: i16
}

async fn _create_merge_request(api_token: &str, params: JsonValue, project_id: &String) -> Result<i16, String> {
    let mut url = String::new();

    // url.push_str("https://multiplier.jetbrains.space/api/http/projects/id:");
    // url.push_str(&project_id); 
    // url.push_str("/code-reviews/merge-requests?$fields=number"); 

    url.push_str("http://localhost:3000");

    let client: reqwest::Client = reqwest::Client::new();

    let response: reqwest::Response = client.post(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", "Bearer ".to_owned() +  api_token)
        .body(json::stringify(params))
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            return match response.json::<Response>().await {
                Ok(parsed) => Ok(parsed.number),
                Err(_) => Err("Erro ao tentar criar MR".to_string()),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            return Err("Token expirado".to_string());
        }
        other => {
            return Err(format!("Algo de errado aconteceu: Http Status Code: {other:?}"));
        }
     };
}

pub async fn create_merge_request(api_token: &str, params: JsonValue, project_id: &String) -> Result<i16, String> {
    let retry_attempts = 3;

    let mut attempts = 0;
    let mut merge_request_number = 0;

    while attempts < retry_attempts {

        // loading style
        let loading = Loading::new(Spinner::new(vec!["...", "●..", ".●.", "..●"]));
        loading.text("Criando Merge Request...");

        // realiza criação da MR
        let result = _create_merge_request(api_token, params.clone(), project_id).await;

        match result {
            Ok(number) => {
                loading.success("Merge Request criado com sucesso");
                merge_request_number = number
            },
            Err(error_message) => {
                loading.fail(error_message);
            } 
        }

        loading.end();

        if merge_request_number != 0 {
            break;
        } else if attempts < 2 {
            println!("Tentando novamente em alguns segundos...");
        
            // pausa por 3 segundos antes de tentar novamente
            let three_seconds = time::Duration::from_secs(3);
            thread::sleep(three_seconds);
        }

        attempts += 1;
    }

    if merge_request_number == 0 {
        return Err("Não foi possível criar um MR, verifique sua conexão".to_string());
    }

    return Ok(merge_request_number)
}