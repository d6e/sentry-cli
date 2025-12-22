use crate::api::models::{IssueStatus, IssueUpdate, StatusDetails};
use crate::api::SentryClient;
use crate::error::Result;
use crate::output::print_success;

pub async fn ignore_issues(
    client: &SentryClient,
    issue_ids: Vec<String>,
    duration: Option<u64>,
    count: Option<u64>,
    until_escalating: bool,
) -> Result<()> {
    let status_details = if duration.is_some() || count.is_some() || until_escalating {
        Some(StatusDetails {
            in_release: None,
            in_next_release: None,
            ignore_duration: duration,
            ignore_count: count,
            ignore_until_escalating: if until_escalating { Some(true) } else { None },
        })
    } else {
        None
    };

    let update = IssueUpdate {
        status: Some(IssueStatus::Ignored),
        status_details,
        ..Default::default()
    };

    if issue_ids.len() == 1 {
        let issue = client.update_issue(&issue_ids[0], update).await?;
        let detail = if let Some(d) = duration {
            format!(" for {} minutes", d)
        } else if let Some(c) = count {
            format!(" until {} more events", c)
        } else if until_escalating {
            " until escalating".to_string()
        } else {
            String::new()
        };
        print_success(&format!("Issue {} ignored{}.", issue.short_id, detail));
    } else {
        client.update_issues(&issue_ids, update).await?;
        print_success(&format!("Ignored {} issues.", issue_ids.len()));
    }

    Ok(())
}
