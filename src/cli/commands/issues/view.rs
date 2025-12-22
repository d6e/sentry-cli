use crate::api::SentryClient;
use crate::cli::args::OutputFormat;
use crate::error::Result;
use crate::output::{print_issue_detail, print_issue_json};

pub async fn view_issue(
    client: &SentryClient,
    issue_id: &str,
    output: OutputFormat,
) -> Result<()> {
    let issue = client.get_issue(issue_id).await?;

    match output {
        OutputFormat::Table => print_issue_detail(&issue),
        OutputFormat::Json => print_issue_json(&issue),
    }

    Ok(())
}
