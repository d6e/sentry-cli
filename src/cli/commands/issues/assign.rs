use crate::api::models::IssueUpdate;
use crate::api::SentryClient;
use crate::error::{Result, SentryCliError};
use crate::output::print_success;

pub async fn assign_issues(
    client: &SentryClient,
    issue_ids: Vec<String>,
    to: Option<String>,
    unassign: bool,
) -> Result<()> {
    let assigned_to = if unassign {
        Some(String::new()) // Empty string to unassign
    } else {
        match to {
            Some(assignee) => Some(assignee),
            None => {
                return Err(SentryCliError::Validation(
                    "Must specify --to <user> or --unassign".to_string(),
                ))
            }
        }
    };

    let update = IssueUpdate {
        assigned_to,
        ..Default::default()
    };

    if issue_ids.len() == 1 {
        let issue = client.update_issue(&issue_ids[0], update).await?;
        if unassign {
            print_success(&format!("Issue {} unassigned.", issue.short_id));
        } else {
            let assignee = issue
                .assigned_to
                .map(|a| a.name)
                .unwrap_or_else(|| "unknown".to_string());
            print_success(&format!(
                "Issue {} assigned to {}.",
                issue.short_id, assignee
            ));
        }
    } else {
        client.update_issues(&issue_ids, update).await?;
        if unassign {
            print_success(&format!("Unassigned {} issues.", issue_ids.len()));
        } else {
            print_success(&format!("Assigned {} issues.", issue_ids.len()));
        }
    }

    Ok(())
}
