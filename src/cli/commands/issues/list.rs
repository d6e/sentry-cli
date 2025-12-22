use crate::api::models::{IssueStatus, ListIssuesParams};
use crate::api::SentryClient;
use crate::error::Result;
use crate::output::print_issues_table;

pub async fn list_issues(
    client: &SentryClient,
    project: Option<String>,
    status: Option<String>,
    query: Option<String>,
    sort: String,
    limit: u32,
) -> Result<()> {
    let status_filter = status.as_ref().and_then(|s| match s.to_lowercase().as_str() {
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
    };

    let issues = client.list_issues(params).await?;
    print_issues_table(&issues);

    Ok(())
}
