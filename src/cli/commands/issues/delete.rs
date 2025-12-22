use crate::api::SentryClient;
use crate::error::Result;
use crate::output::print_success;
use std::io::{self, Write};

pub async fn delete_issues(
    client: &SentryClient,
    issue_ids: Vec<String>,
    confirm: bool,
) -> Result<()> {
    if !confirm {
        print!(
            "Are you sure you want to delete {} issue(s)? [y/N]: ",
            issue_ids.len()
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    if issue_ids.len() == 1 {
        client.delete_issue(&issue_ids[0]).await?;
        print_success(&format!("Issue {} deleted.", issue_ids[0]));
    } else {
        client.delete_issues(&issue_ids).await?;
        print_success(&format!("Deleted {} issues.", issue_ids.len()));
    }

    Ok(())
}
