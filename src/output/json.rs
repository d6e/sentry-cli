use crate::api::models::Issue;

pub fn print_issues_json(issues: &[Issue]) {
    let json = serde_json::to_string_pretty(issues).unwrap_or_else(|_| "[]".to_string());
    println!("{}", json);
}

pub fn print_issue_json(issue: &Issue) {
    let json = serde_json::to_string_pretty(issue).unwrap_or_else(|_| "{}".to_string());
    println!("{}", json);
}
