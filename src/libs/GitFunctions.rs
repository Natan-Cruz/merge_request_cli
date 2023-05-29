use std::{
    process::{
        Output, 
        Command
    }, 
    string::FromUtf8Error
};

pub fn get_all_branchs() -> String {
    let all_branchs: Output = Command::new("git").args(["branch", "-a"]).output().unwrap();
    let all_branchs: Result<String, FromUtf8Error> = String::from_utf8(all_branchs.stdout);
    match all_branchs {
        Ok(all_branchs) => return all_branchs,
        Err(_) => panic!("Erro ao obter todas as branchs")
    };
}

pub fn get_current_branch() -> String {
    let git_current_branch: Output = Command::new("git").args(["rev-parse", "--abbrev-ref", "HEAD"]).output().unwrap();

    let git_current_branch: Result<String, FromUtf8Error> = String::from_utf8(git_current_branch.stdout);
    
    match git_current_branch {
        Ok(branch_name) => return branch_name.replace("\n", ""),
        Err(e) => panic!("Erro ao ler branch atual; ERROR: {}", e),
    };
}