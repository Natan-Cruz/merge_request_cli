use loading::{ Loading, Spinner };

use inquire::{
    Text, 
    Select,
    list_option::ListOption, 
    InquireError, 
    Confirm
};

use crate::{
    requests::Issues::{
        self,
        IssuesResponseData
    }, 
    libs::{
        GitFunctions,
        BranchCompleter,
        Utils
    }
};

#[derive(Debug)]
pub struct Questionnaire {
    pub type_commit: String,
    pub scope_commit: String,
    pub name: String,
    pub description: String,
    pub issues: String,
    pub prefix: String,
    pub current_branch: String,
    pub target_branch: String,
}

impl Questionnaire {

    pub async fn start_questionnaire(api_token: &String, scopes: Vec<String>, prefix: Vec<String>, default_target_branch: String) -> Self {

        let type_commit: String = show_type_commit();
        let scope_commit: String = show_scope_commit(scopes);
        
        let name: String = show_name();
        let description: String = show_description();

        // issues

        let loading = Loading::new(Spinner::new(vec!["...", "●..", ".●.", "..●"]));
        
        loading.text("Obtendo issues");

        let issues_in_progress: Issues::IssuesResponse = Issues::get_in_progress_issues(api_token).await;

        loading.end();

        let mut issues: String = String::new();

        if issues_in_progress.data.is_empty() {
            println!("ISSUES: Não há issues em progressos atrelados à você");
        } else{
            let issues_selected = show_issues(issues_in_progress.data).await;
            issues.push_str(&issues_selected)
        }

        let prefix = show_prefix(prefix);

        let current_branch: String = show_current_branch();
        let target_branch: String = show_target_branch(default_target_branch);

        show_confirm();

        Questionnaire { 
            type_commit,
            scope_commit,
            name,
            description,
            issues,
            prefix,
            current_branch,
            target_branch,
        }
    }
}

fn show_type_commit() -> String {

    let options: Vec<&str> = vec![
        "feature: Uma nova funcionalidade",
        "fix: Correção de bug",
        "docs: Alterações de documentação",
        "style: Mudanças que não afetam o sentido do código (espaços em branco, formatação, ponto e vírgula ausente, etc)",
        "refactor: Uma mudança de código que NÃO corrige bug ou adiciona funcionalidade",
        "perf: Alteração de código que melhora a performance",
        "test: Adiciona testes ausentes",
        "chore: Alterações que afetam o sistema de build, scripts, dependências externas, etc",
        "ci: Mudanças na configuração de arquivos do CI (actions, codepipeline, etc)",
        "revert: Reversão de commit",
        "WIP: Trabalho em progresso",
    ];

    let message ="Selecione o tipo de mudança que você está fazendo?";

    let type_commit = Select::new(message, options)
        .with_formatter(&|result: ListOption<&&str>| {
            let value_type_commit: String = result.value.split(":")
                .map(|s: &str| s.to_owned())
                .collect::<Vec<String>>()
                .swap_remove(0);

            format!("{value_type_commit}")
        })
        .prompt();

    match type_commit {
        Ok(type_commit) => {
            let value_type_commit: String = type_commit.split(":")
                .map(|s: &str| s.to_owned())
                .collect::<Vec<String>>()
                .swap_remove(0);

            return value_type_commit.to_owned()
        },
        Err(_) => panic!("Algo deu errado!")
    };
}

pub fn show_scope_commit(scopes: Vec<String>) -> String{

    let message = "Indique o ESCOPO desta mudança?";
    let scope: Result<String, InquireError> = Select::new(message, scopes)
        .prompt();

    match scope {
        Ok(scope) => return scope.to_owned(),
        Err(err) => panic!("Algo deu errado! {:?}", err)
    };
}

fn show_name() -> String {
    let message: &str = "Qual o nome da MR?";
    let result:Result<String, inquire::InquireError> = Text::new(&message)
        .prompt();

    match result {
        Ok(res) => return res,
        Err(_) => panic!("Algo deu errado!")
    };

}

fn show_description() -> String {
    let message: &str = "Qual a descrição da MR (opcional)?";
    let result: Result<String, inquire::InquireError> = Text::new(&message)
        .prompt();

    match result {
        Ok(res) => return res,
        Err(_) => panic!("Algo deu errado!")
    };
}

fn extract_issue_number(issue_title: &String) -> String {
    issue_title.split(":")
    .map(|s: &str| s.to_owned())
    .collect::<Vec<String>>()
    .swap_remove(0)
}

async fn show_issues(issues_in_progress: Vec<IssuesResponseData>) -> String {
    
    let options: Vec<String> = issues_in_progress.iter().map( | s | {
        format!("{}-T-{}: {}", s.projectRef.key.key, s.number, s.title)
    }).collect();

    let message: &str = "Qual(is) issues para esta MR?";

    let result: Result<Vec<String>, InquireError> = inquire::MultiSelect::new(&message, options)
        .with_formatter(&|issues: &[ListOption<&String>]| {
            let issues_number: String = issues
                .iter()
                .map( | s | extract_issue_number(s.value))
                .collect::<Vec<String>>()
                .join(" ");
        
            format!("{issues_number}")
        })
        .prompt();


    match result {
        Ok(issues_selected) => {
            let issues_number: String = issues_selected
                .iter()
                .map( | issue_selected | extract_issue_number(issue_selected))
                .collect::<Vec<String>>()
                .join(" ");
        
            return issues_number;
        },
        Err(_) => panic!("Algo deu errado!")
    };
}

fn show_prefix(mut prefix: Vec<String>) -> String {

    prefix.push("".to_string());
    prefix.rotate_right(1);

    let prefix: Result<String, InquireError> = Select::new("Qual é o prefixo?", prefix)
        .prompt();

    match prefix {
        Ok(prefix) => prefix.to_owned(),
        Err(_) => panic!("Algo deu errado!")
    }
}

fn show_current_branch() -> String {
    let git_current_branch: String = GitFunctions::get_current_branch();

    let current_branch: Result<String, InquireError> = Text::new("Qual a atual branch?")
        .with_autocomplete(BranchCompleter::BranchCompleter::default())
        .with_default(&git_current_branch)
        .prompt();

    match current_branch {
        Ok(target_branch) => return target_branch,
        Err(_) => panic!("Algo deu errado!")
    };
}

fn show_target_branch(default_target_branch: String) -> String {
    let target_branch: Result<String, InquireError> = Text::new("Qual a branch de destino?")
    .with_autocomplete(BranchCompleter::BranchCompleter::default())
    .with_default(&default_target_branch)
    .prompt();

    match target_branch {
        Ok(target_branch) => return target_branch,
        Err(_) => panic!("Algo deu errado!")
    };
}

fn show_confirm() -> bool {
    let ans: Result<bool, InquireError> = Confirm::new("Deseja continuar?")
        .with_default(true)
        .prompt();

    match ans {
        Ok(true) => return true,
        Ok(false) => std::process::exit(0x0100),
        Err(_) => panic!("Algo deu errado!"),
    }
}


   