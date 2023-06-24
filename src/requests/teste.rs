pub async fn get_in_progress_issues(api_token: &String, project_id: &String, retry_attempts: &u8) -> IssuesResponse {

    let mut attempts: u8 = 0;
    let mut issues_in_progress = IssuesResponse{ data: Vec::new() };
    
    while &attempts < retry_attempts {

        let loading = Loading::new(Spinner::new(vec!["...", "●..", ".●.", "..●"]));
    
        loading.text("Obtendo issues");
        
        let result: Result<IssuesResponse, String> = _get_in_progress_issues(api_token, project_id, retry_attempts).await;

        match result {
            Ok(result) => {
                issues_in_progress = result;
            },
            Err(error_message) => {
                loading.fail(error_message);
            }
        }

        loading.end();

        if !issues_in_progress.data.is_empty() {
            break;
        }

        attempts += 1
    }

    if attempts == 3 {
        panic!("Ops, erro ao buscar issues, verifique sua internet");
    }

    return issues_in_progress
}