use std::fmt;

use reqwest::header::{CONTENT_TYPE, ACCEPT};
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

pub async fn get_in_progress_issues(api_token: String) -> IssuesResponse {
    let client: reqwest::Client = reqwest::Client::new();

    let url = "https://multiplier.jetbrains.space/api/http/projects/id:2ZsKnR42KI6t/planning/issues?assigneeId=me&statuses=43Wrzo4NkB7E&sorting=UPDATED&descending=true&$fields=data(number,projectRef(key),title)";

    let response: reqwest::Response = client.get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", "Bearer ".to_owned() + &api_token)
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<IssuesResponse>().await {
                Ok(parsed) => parsed,
                Err(err) => panic!("Ocorreu um erro ao listar issues {:?}", err),

            }
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Token expirado");
        }
        other => {
            panic!("Algo de errado aconteceu: {:?}", other);
        }
    }
}
