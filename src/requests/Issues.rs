use json::object;
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
    pub data: Vec<IssuesResponseData>
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

pub async fn get_in_progress_issues(api_token: &String, project_id: &String) -> Result<IssuesResponse, String> {

    let mut url = String::new();

    url.push_str("https://multiplier.jetbrains.space/api/http/projects/id:");
    url.push_str(&project_id);
    url.push_str("/planning/issues?assigneeId=me&statuses=43Wrzo4NkB7E&sorting=UPDATED&descending=true&$fields=data(number,projectRef(key),title)");

    let client: reqwest::Client = reqwest::Client::new();

    let response: reqwest::Response = client.get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", "Bearer ".to_string() + api_token)
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<IssuesResponse>().await {
                Ok(parsed) => return Ok(parsed),
                Err(err) => return Err(format!("Ocorreu um erro ao listar issues {err:?}")),
            }
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            return Err("Token expirado".to_string());
        }
        other => {
            return Err(format!("Algo de errado aconteceu: {:?}", other));
        }
    }
}

pub async fn update_status(api_token: &str, project_id: &str, issue: &str) -> Result<String, String> {

    let mut url = String::new();

    url.push_str("https://multiplier.jetbrains.space/api/http/projects/id:");
    url.push_str(project_id);
    url.push_str("planning/issues/key:");
    url.push_str(issue);

    let client: Client = Client::new();

    // id do status abaixo representa "em revisão de código"
    let body = object!{ "status": "fNG0L1lSYbc" };

    let response = client.patch(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", "Bearer ".to_string() +  api_token)
        .body(json::stringify(body))
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK =>  { 
            return Ok(issue.to_string())
        },
        other => {
            return Err(format!("Algo de errado aconteceu: Http Status Code: {other:?}"));
        }
    }
}