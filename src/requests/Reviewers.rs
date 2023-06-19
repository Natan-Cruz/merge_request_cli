use reqwest::{
    Client, 
    header::{
        ACCEPT, 
        CONTENT_TYPE
    }, StatusCode
};
use serde::{ 
    Serialize, 
    Deserialize
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Profiles {
    pub data: Vec<ProfileData>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileData {
    pub id: String, 
    pub name: ProfileName
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileName {
    pub firstName: String,
    pub lastName: String
}


pub async fn get_profiles(api_token: &String) -> Profiles {
    let client: Client = Client::new();

    let url: &str = "https://multiplier.jetbrains.space/api/http/team-directory/profiles?$fields=data(id,name)";

    let response = client.get(url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header("Authorization", "Bearer ".to_owned() + &api_token)
        .send()
        .await
        .unwrap();

    match response.status() {
        StatusCode::OK => match response.json::<Profiles>().await {
            Ok(parsed) => parsed,
            Err(err) => panic!("Erro ao buscar reviewers {:?}", err),
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Token expirado");
        }
        other => {
            panic!("Algo de errado aconteceu: {:?}", other);
        }
    }
}