use reqwest::{
    Client, 
    StatusCode,
    header::{
        ACCEPT, 
        CONTENT_TYPE
    }, 
};
use serde::{
    Serialize, 
    Deserialize
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub id: String,
    pub name: ProfileName
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileName {
    pub firstName: String,
    pub lastName: String
}

pub async fn get_profile(api_token: &str) -> Profile {
    let client: Client = Client::new();

    let url: &str = "https://multiplier.jetbrains.space/api/http/team-directory/profiles/me?$fields=id,name";

    let response = client.get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", "Bearer ".to_owned() + &api_token)
        .send()
        .await
        .unwrap();


    match response.status() {
        StatusCode::OK => {
            match response.json::<Profile>().await {
                Ok(parsed) => parsed,
                Err(err) => panic!("Ocorreu um erro ao obter seus dados, {:?}", err)
            }
        },
        StatusCode::UNAUTHORIZED => {
            panic!("Token expirado");
        }
        other => {
            panic!("Algo de errado aconteceu: {:?}", other);
        }
    }
}