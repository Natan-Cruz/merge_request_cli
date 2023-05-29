use reqwest::header::{CONTENT_TYPE, ACCEPT};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct IssuesResponse {
    data: Vec<IssuesResponseData>
}

#[derive(Serialize, Deserialize, Debug)]
struct IssuesResponseData {
    projectRef: ProjectRef,
    number: i32,
    title: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectRef {
    key: ProjectRefKey 
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectRefKey {
    key: String
}

pub async fn get_in_progress_issues(api_token: String) -> IssuesResponse {
    let client: reqwest::Client = reqwest::Client::new();

    let url = "https://multiplier.jetbrains.space/api/http/projects/id:2ZsKnR42KI6t/planning/issues?assigneeId=username:nata.cruz&sorting=UPDATED&descending=true&$fields=data(number,projectRef(key),title)";

    let response: reqwest::Response = client.get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", api_token)
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
