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



async fn retry_async<F, Fut, Args, E, R>(mut f: F, args: Args) -> Result<R, E>
where
    F: FnMut(Args) -> Fut,
    Fut: Future<Output = Result<R, E>>,
    Args: Clone,
{
    let mut retries = 0;
    loop {
        match f(args.clone()).await {
            Ok(result) => return Ok(result),
            Err(err) => {
                retries += 1;
                if retries > 3 {
                    return Err(err)
                }
                let three_seconds = time::Duration::from_secs(3);
                thread::sleep(three_seconds);
            }
        }
    }
}
