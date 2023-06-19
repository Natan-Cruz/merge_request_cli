use inquire::list_option::ListOption;
use json::{JsonValue, object};
use regex::Regex;

use crate::questionnaire::QuestionnaireMain;

pub fn transform_string_to_vec(s: String) -> Vec<String> {
    return s.split("\n")
        .map(| mut word: &str | {
            let word: String = word.to_string();
            let regex: Regex = Regex::new(r"[^A-Za-z\-\_\/0-9]").unwrap();
            return regex.replace_all(&word, "").to_string()
        })
        .collect();
}

pub fn build_merge_request_body(answers: &QuestionnaireMain::Questionnaire, profile_id: String, reviewers: Vec<String>, repository: String) -> JsonValue {

    let mut title: String = String::new();

    if !answers.prefix.is_empty() {
        title.push_str("`");
        title.push_str(&answers.prefix);
        title.push_str("` ");
    }

    title.push_str(&answers.type_commit);
    title.push_str("(");
    title.push_str(&answers.scope_commit);
    title.push_str("): ");

    title.push_str(&answers.name);

    if !answers.issues.is_empty() {
        title.push_str(" - ");
        title.push_str(&answers.issues);
    }

    let reviewers: Vec<JsonValue> = reviewers
        .into_iter()
        .filter(| reviewerId | reviewerId != &profile_id ) 
        .map(| s | object!{ "profileId": s } )
        .collect::<Vec<JsonValue>>();

   return object!{
        "repository": repository,
        "sourceBranch": answers.current_branch.to_string(),
        "targetBranch": answers.target_branch.to_string(),
        "title": title,
        "description": answers.description.to_string(),
        "reviewers": reviewers
    };
}

pub fn formatter_issues(issues: &[ListOption<&String>]) -> String {
    let issues_number: String = issues.iter().map( | s | {
        return s.value.split(":").map(|s: &str| s.to_owned())
            .collect::<Vec<String>>()
            .swap_remove(0);
    }).collect::<Vec<String>>().join(" ");

    format!("{issues_number}")
}