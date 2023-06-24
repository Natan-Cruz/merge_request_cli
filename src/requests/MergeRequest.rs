use reqwest::header::{CONTENT_TYPE, ACCEPT};
use serde::{Serialize, Deserialize};
use json::{JsonValue};


#[derive(Serialize, Deserialize, Debug)]
struct Response {
    number: u16
}

pub async fn create_merge_request(api_token: &str, params: JsonValue, project_id: &String) -> Result<String, String> {
    let mut url = String::new();

    url.push_str("https://multiplier.jetbrains.space/api/http/projects/id:");
    url.push_str(&project_id); 
    url.push_str("/code-reviews/merge-requests?$fields=number"); 

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
                Ok(parsed) => Ok(parsed.number.to_string()),
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
