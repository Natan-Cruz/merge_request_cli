
use inquire::Text;
use crate::requests::Profile::get_profile;

#[derive(Debug)]
pub struct Questionnaire {
    pub api_token: String,
    pub profile_id: String,
}

impl Questionnaire {
    pub async fn start_questionnaire() -> Self {
        let api_token: String = show_api_token();

        let profile = get_profile(&api_token).await;

        println!("Bem-vindo {} {} ao CLI", profile.name.firstName, profile.name.lastName);

        Questionnaire {
            api_token,
            profile_id: profile.id,
        }
    }
}

fn show_api_token() -> String {
    let message: &str = "Qual seu api token?";
    
    let reviewers = Text::new(message).prompt();

    match reviewers {
        Ok(res) => return res,
        Err(err) => panic!("Algo deu errado: {:?}", err)
    }
}