use crate::api::SentryClient;
use crate::error::Result;
use crate::output::print_issue_detail;

pub async fn view_issue(client: &SentryClient, issue_id: &str) -> Result<()> {
    let issue = client.get_issue(issue_id).await?;
    print_issue_detail(&issue);
    Ok(())
}
