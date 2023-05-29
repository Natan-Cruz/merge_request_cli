
mod libs;
use libs::Utils;

use crate::libs::{
    Config::Config,
    Question::Question
};

mod requests;
use crate::requests::{
    MergeRequest
};

use json::JsonValue;

#[tokio::main]
async fn main() {
    let config: Config = Config::new();
    
    let answers: Question = Question::start_questionnaire();

    let merge_request_body:JsonValue = Utils::build_merge_request_body(answers, config.profile_id);

    MergeRequest::create_merge_request(config.api_token, merge_request_body).await;
}