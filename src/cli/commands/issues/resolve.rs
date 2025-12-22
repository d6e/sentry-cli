use crate::api::models::{IssueStatus, IssueUpdate, StatusDetails};
use crate::api::SentryClient;
use crate::error::Result;
use crate::output::print_success;

pub async fn resolve_issues(
    client: &SentryClient,
    issue_ids: Vec<String>,
    in_release: Option<String>,
    in_next_release: bool,
) -> Result<()> {
    let status_details = if in_release.is_some() || in_next_release {
        Some(StatusDetails {
            in_release,
            in_next_release: if in_next_release { Some(true) } else { None },
            ignore_duration: None,
            ignore_count: None,
            ignore_until_escalating: None,
        })
    } else {
        None
    };

    let update = IssueUpdate {
        status: Some(IssueStatus::Resolved),
        status_details,
        ..Default::default()
    };

    if issue_ids.len() == 1 {
        let issue = client.update_issue(&issue_ids[0], update).await?;
        print_success(&format!("Issue {} resolved.", issue.short_id));
    } else {
        client.update_issues(&issue_ids, update).await?;
        print_success(&format!("Resolved {} issues.", issue_ids.len()));
    }

    Ok(())
}

pub async fn unresolve_issues(client: &SentryClient, issue_ids: Vec<String>) -> Result<()> {
    let update = IssueUpdate {
        status: Some(IssueStatus::Unresolved),
        ..Default::default()
    };

    if issue_ids.len() == 1 {
        let issue = client.update_issue(&issue_ids[0], update).await?;
        print_success(&format!("Issue {} unresolved.", issue.short_id));
    } else {
        client.update_issues(&issue_ids, update).await?;
        print_success(&format!("Unresolved {} issues.", issue_ids.len()));
    }

    Ok(())
}
