use crate::api::models::{IssueStatus, ListIssuesParams};
use crate::api::SentryClient;
use crate::cli::args::OutputFormat;
use crate::error::Result;
use crate::output::{print_issues_json, print_issues_table};

pub async fn list_issues(
    client: &SentryClient,
    project: Option<String>,
    status: Option<String>,
    query: Option<String>,
    sort: String,
    limit: u32,
    output: OutputFormat,
    all: bool,
) -> Result<()> {
    let status_filter = status
        .as_ref()
        .and_then(|s| match s.to_lowercase().as_str() {
            "resolved" => Some(IssueStatus::Resolved),
            "unresolved" => Some(IssueStatus::Unresolved),
            "ignored" => Some(IssueStatus::Ignored),
            _ => None,
        });

    let projects = project.map(|p| p.split(',').map(|s| s.trim().to_string()).collect());

    let params = ListIssuesParams {
        project: projects,
        query,
        status: status_filter,
        sort: Some(sort),
        limit: Some(limit),
        cursor: None,
    };

    let issues = if all {
        client.list_all_issues(params).await?
    } else {
        client.list_issues(params).await?
    };

    match output {
        OutputFormat::Table => print_issues_table(&issues),
        OutputFormat::Json => print_issues_json(&issues),
    }

    Ok(())
}
