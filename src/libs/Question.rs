use inquire::{
    Text, 
    Select,
    list_option::ListOption, InquireError, Confirm
};

use super::{GitFunctions, BranchCompleter};

#[derive(Debug)]
pub struct Question {
    pub type_commit: String,
    pub scope_commit: String,
    pub name: String,
    pub description: String,
    pub issues: String,
    pub is_draft: bool,
    pub priority: String,
    pub current_branch: String,
    pub target_branch: String,
}

impl Question {

    pub fn start_questionnaire() -> Self {
        let type_commit: String = show_type_commit();
        let scope_commit: String = show_scope_commit();
        
        let name: String = show_name();
        let description: String = show_description();
        let issues: String = show_issues();

        let is_draft: bool = show_is_draft();

        let mut priority: String = String::new();

        if is_draft == false {
            priority = show_priority();
        }

        let current_branch: String = show_current_branch();
        let target_branch: String = show_target_branch();

        show_confirm();

        Question { 
            type_commit,
            scope_commit,
            name,
            description,
            issues,
            is_draft,
            priority,
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

pub fn show_scope_commit() -> String{
    let scope_options: Vec<&str> = vec![
        "customer",
        "customer-visualization",
        "customer-registration",
        "customer-listing",
        "business-group",
        "core",
        "order",
        "report",
        "my-day",
        "schedule",
        "opportunity-map",
        "financial-credit"
    ];

    let message = "Indique o ESCOPO desta mudança (opcional)?";
    let scope: Result<&str, InquireError> = Select::new(message, scope_options)
        .prompt();

    match scope {
        Ok(scope) => return scope.to_owned(),
        Err(_) => panic!("Algo deu errado!")
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

fn show_issues() -> String {
    let message: &str = "Qual(is) issues para esta MR?";
    let result:Result<String, inquire::InquireError> = Text::new(&message)
        .prompt();

    match result {
        Ok(res) => return res,
        Err(_) => panic!("Algo deu errado!")
    };
}


fn show_is_draft() -> bool {
    let answer: Result<bool, InquireError> = Confirm::new("Está em draft")
        .with_default(false)
        .prompt();

    match answer {
        Ok(boolean) => boolean,
        Err(_) => panic!("Algo deu errado!"),
    }
}

fn show_priority() -> String {
    let options = vec![
        "",
        "Priority",
        "Priority 1",
        "Priority 2",
        "Priority 3",
    ];

    let priority: Result<&str, InquireError> = Select::new("Qual a prioridade?", options)
        .prompt();

    match priority {
        Ok(priority) => priority.to_owned(),
        Err(_) => panic!("Algo deu errado!")
    }
}

fn show_current_branch() -> String {
    let git_current_branch: String = GitFunctions::get_current_branch();

    let current_branch: Result<String, InquireError> = Text::new("Qual a atual branch?")
        .with_default(&git_current_branch)
        .prompt();

    match current_branch {
        Ok(target_branch) => return target_branch,
        Err(_) => panic!("Algo deu errado!")
    };
}

fn show_target_branch() -> String {
    let target_branch: Result<String, InquireError> = Text::new("Qual a branch de destino?")
    .with_autocomplete(BranchCompleter::BranchCompleter::default())
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


   