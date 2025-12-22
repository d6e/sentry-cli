use crate::api::SentryClient;
use crate::error::Result;
use crate::output::print_success;

pub async fn merge_issues(
    client: &SentryClient,
    primary_id: String,
    other_ids: Vec<String>,
) -> Result<()> {
    let merged = client.merge_issues(&primary_id, &other_ids).await?;
    print_success(&format!(
        "Merged {} issue(s) into {}.",
        other_ids.len(),
        merged.short_id
    ));
    Ok(())
}
