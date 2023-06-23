mod libs;
use crate::libs::{
    Config::{
        Authorization,
        Configurations
    },
    Utils
};

mod requests;
use crate::requests::{
    MergeRequest,
    Issues,
};

mod questionnaire;
use crate::questionnaire::{
    QuestionnaireInitial, 
    QuestionnaireMain
};

use json::JsonValue;
use loading::{Loading, Spinner};

#[tokio::main]
async fn main() {
    let mut authorization: Authorization = Authorization::new();
    let configurations: Configurations = Configurations::new();

    if authorization.api_token.is_empty() {
        let initial_config = QuestionnaireInitial::Questionnaire::start_questionnaire().await;
        authorization = Authorization::save_values(&initial_config.api_token, &initial_config.profile_id)
    }

    if configurations.scopes.is_empty() {
        panic!("Escopo do projeto não estar vazio, vá até o Config.toml e preecha o campo scopes")
    }

    let answers = QuestionnaireMain::Questionnaire::start_questionnaire(&authorization.api_token, configurations.scopes, configurations.prefix, configurations.default_target_branch).await;

    let merge_request_body:JsonValue = Utils::build_merge_request_body(
        &answers, 
        authorization.profile_id, 
        configurations.reviewers,
        configurations.repository
    );

    let loading_merge_request = Loading::new(Spinner::new(vec!["...", "●..", ".●.", "..●"]));
    loading_merge_request.text("Criando Merge Request");

    MergeRequest::create_merge_request(&authorization.api_token, merge_request_body, &configurations.project_id).await;

    loading_merge_request.end();

    let issues_cloned = answers.issues.clone();
    let issues_cloned = issues_cloned
        .split(" ")
        .filter(| issue | !issue.is_empty())
        .collect::<Vec<&str>>();


    let loading_update_status_issues = Loading::new(Spinner::new(vec!["...", "●..", ".●.", "..●"]));
    loading_update_status_issues.text("Alterando status das issues");

    if !issues_cloned.is_empty(){
        for f in issues_cloned {
            Issues::update_status(&authorization.api_token, f).await;
        }
    }

    loading_update_status_issues.end()
}

