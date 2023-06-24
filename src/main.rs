mod libs;
use std::process::ExitCode;

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

use json::{JsonValue, object};
use loading::{Loading, Spinner};


#[tokio::main]
async fn main() -> ExitCode {
    let mut authorization: Authorization = Authorization::new();
    let configurations: Configurations = Configurations::new();

    // if authorization.api_token.is_empty() {
    //     let initial_config = QuestionnaireInitial::Questionnaire::start_questionnaire().await;
    //     authorization = Authorization::save_values(&initial_config.api_token, &initial_config.profile_id)
    // }

    // if configurations.scopes.is_empty() {
    //     panic!("Escopo do projeto não estar vazio, vá até o Config.toml e preecha o campo scopes")
    // }

    // let answers = QuestionnaireMain::Questionnaire::start_questionnaire(
    //     &authorization.api_token, 
    //     &configurations.project_id, 
    //     configurations.scopes, 
    //     configurations.prefix, 
    //     configurations.default_target_branch,
    // ).await;

    // let merge_request_body:JsonValue = Utils::build_merge_request_body(
    //     &answers, 
    //     authorization.profile_id, 
    //     configurations.reviewers,
    //     configurations.repository
    // );

    let merge_request_body = object!{
        "repository": "teste" ,
        "sourceBranch": "teste" ,
        "targetBranch": "teste" ,
        "title": "teste" ,
        "description": "teste" ,
        "reviewers": "teste" 
    };


    let merge_request_number: Result<i16, String> = MergeRequest::create_merge_request(&authorization.api_token, merge_request_body, &configurations.project_id).await;

    match merge_request_number {
        Ok(number) => {
            println!("Link para MR: https://multiplier.jetbrains.space/p/srp/reviews/{}/timeline", number);
        },
        Err(error_message) => {
            println!("{:?}", error_message);
            return ExitCode::FAILURE
        } 
    }
    
    // let concatenated_number_issues: String = answers.issues.clone();
    // let number_issues: Vec<&str> = concatenated_number_issues
    //     .split(" ")
    //     .filter(| issue | !issue.is_empty())
    //     .collect::<Vec<&str>>();

    
    // if !number_issues.is_empty(){
    //     for number_issue in number_issues {

    //         let loading_update_status_issue = Loading::new(Spinner::new(vec!["...", "●..", ".●.", "..●"]));
        
    //         loading_update_status_issue.text(format!("Alterando status da issue: {number_issue:?}"));

    //         let result: Result<String, String> = Issues::update_status(&authorization.api_token, &configurations.project_id, &number_issue).await;

    //         match result {
    //             Ok(_) => {
    //                 loading_update_status_issue.text(format!("Status da Issue: {number_issue:?} alterado com sucesso"));
    //                 loading_update_status_issue.end()
    //             },
    //             Err(error_message) => {
    //                 loading_update_status_issue.fail(error_message);
    //                 loading_update_status_issue.end();
    //                 return ExitCode::FAILURE;
    //             },
    //         }
    //     }
    // }

    ExitCode::SUCCESS
}

