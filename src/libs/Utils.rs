use inquire::list_option::ListOption;
use json::{JsonValue, object};
use regex::Regex;

use super::Question::Question;

pub fn transform_string_to_vec(s: String) -> Vec<String> {
    return s.split("\n")
        .map(| mut word: &str | {
            let word: String = word.to_string();
            let regex: Regex = Regex::new(r"[^A-Za-z\-\_\/0-9]").unwrap();
            return regex.replace_all(&word, "").to_string()
        })
        .collect();
}

pub fn build_merge_request_body(answers: &Question, profile_id: String) -> JsonValue {

    let mut title: String = String::new();

    if answers.is_draft {
        title.push_str("`");
        title.push_str("DRAFT");
        title.push_str("` ");
    } else if !answers.priority.is_empty() {
        title.push_str("`");
        title.push_str(&answers.priority);
        title.push_str("` ");
    }

    title.push_str(&answers.type_commit);
    title.push_str("(");
    title.push_str(&answers.scope_commit);
    title.push_str("): ");

    title.push_str(&answers.name);
    title.push_str(" - ");
    title.push_str(&answers.issues);


    let mut reviewers: Vec<&str> = vec![
        "3Qy8hG2WRFXI",
        "NWfFk1zYP6U",
        "jHxkJ18dV9H",
        "1Tag2d0WZh3N",
        "4TvgBF4CTFgI",
        "2APly31xzeO9",
        "ccrB10uUJtz"
    ];

    let reviewers: Vec<JsonValue> = reviewers
        .into_iter()
        .filter(|s| {
            if answers.is_draft {
                return false 
            } else {
                return s.to_owned() != profile_id
            }
        }) 
        .map(| s | {
            return object!{ "profileId": s }
        })
        .collect::<Vec<JsonValue>>();



   return object!{
        "repository": "front",
        "sourceBranch": answers.current_branch.to_owned(),
        "targetBranch": answers.target_branch.to_owned(),
        "title": title,
        "description": answers.description.to_owned(),
        "reviewers": reviewers
    };
}

pub fn formatter_issues(issues: &[ListOption<&String>]) -> String {
    let issues_numer: String = issues.iter().map( | s | {
        return s.value.split(":").map(|s: &str| s.to_owned())
            .collect::<Vec<String>>()
            .swap_remove(0);
    }).collect::<Vec<String>>().join("; ");

    format!("{issues_numer}")
}