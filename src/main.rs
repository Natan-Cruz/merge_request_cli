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
use requests::Issues::get_in_progress_issues;



#[tokio::main]
async fn main() -> ExitCode {
    println!("{:*<40}", "");
    println!("Bem-vindo ao criador de Merge Request");
    println!("{:*<40}", "");
    
    let authorization: Authorization = Authorization::new();
    let configurations: Configurations = Configurations::new();

    // verifica se no Config há as mínimas propriedades para o CLI rodar
    if authorization.profile_id.is_empty() {
        println!("\x1b[31m{}\x1b[0m", "Propriedade \"profile_id\" está vazia, vá até o Config.toml e preecha esta propriedade");
        return ExitCode::FAILURE;
    }

    if authorization.api_token.is_empty() {
        println!("\x1b[31m{}\x1b[0m","Propriedade \"api_token\" está vazia, vá até o Config.toml e preecha esta propriedade");
        return ExitCode::FAILURE;
    }

    if configurations.scopes.is_empty() {
        println!("\x1b[31m{}\x1b[0m", "Propriedade \"scopes\" está vazia, vá até o Config.toml e preecha esta propriedade");
        return ExitCode::FAILURE;
    }

    // Obtém as issues em progresso
    println!("Vamos obter as suas issues em progresso:");

    let issues_in_progress: Result<Vec<Issues::IssuesResponseData>, String> = get_in_progress_issues(
        &authorization.api_token, 
        &configurations.project_id
    ).await;

    let mut continue_without_issues: bool = false;

    let issues_in_progress: Vec<Issues::IssuesResponseData> = match issues_in_progress {
        Ok(issues) => issues,
        Err(_) => {
            continue_without_issues = QuestionnaireMain::show_confirm("Não conseguimos obter suas issues, deseja continuar?");
            Vec::new()
        }
    };

    if continue_without_issues == false {
        return ExitCode::FAILURE
    }

    println!("{:*<40}", "");

    // Inicia o questionário
    let answers = QuestionnaireMain::Questionnaire::start_questionnaire(
        issues_in_progress,
        configurations.scopes, 
        configurations.prefix, 
        configurations.default_target_branch,
    ).await;

    // Configura o objeto que iremos mandar para o space requsitando a abertura do Merge Request
    let merge_request_body:JsonValue = Utils::build_merge_request_body(
        &answers, 
        authorization.profile_id, 
        configurations.reviewers,
        configurations.repository
    );

    // Criação do Merge Request
    let merge_request_number: Result<i16, String> = MergeRequest::create_merge_request(
        &authorization.api_token, 
        &configurations.project_id,
        merge_request_body
    ).await;

    match merge_request_number {
        Ok(number) => {
            println!("Link para MR: https://multiplier.jetbrains.space/p/srp/reviews/{}/timeline", number);
        },
        Err(_) => {
            return ExitCode::FAILURE
        } 
    }
    
    let concatenated_number_issues: String = answers.issues.clone();
    let number_issues: Vec<&str> = concatenated_number_issues
        .split(" ")
        .filter(| issue | !issue.is_empty())
        .collect::<Vec<&str>>();

    
    if !number_issues.is_empty(){
        for number_issue in number_issues {

            let loading_update_status_issue = Loading::new(Spinner::new(vec!["...", "●..", ".●.", "..●"]));
        
            loading_update_status_issue.text(format!("Alterando status da issue: {number_issue:?}"));

            let result: Result<String, String> = Issues::update_status(&authorization.api_token, &configurations.project_id, &number_issue).await;

            match result {
                Ok(_) => {
                    loading_update_status_issue.text(format!("Status da Issue: {number_issue:?} alterado com sucesso"));
                    loading_update_status_issue.end()
                },
                Err(error_message) => {
                    loading_update_status_issue.fail(error_message);
                    loading_update_status_issue.end();
                    return ExitCode::FAILURE;
                },
            }
        }
    }

    ExitCode::SUCCESS
}

