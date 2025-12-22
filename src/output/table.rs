use crate::api::models::Issue;
use chrono::{DateTime, Utc};
use colored::Colorize;
use comfy_table::{presets::UTF8_FULL_CONDENSED, Cell, Color, Table};

pub fn print_issues_table(issues: &[Issue]) {
    if issues.is_empty() {
        println!("No issues found.");
        return;
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_header(vec!["ID", "Short ID", "Title", "Status", "Events", "Last Seen"]);

    for issue in issues {
        let status_cell = match issue.status {
            crate::api::models::IssueStatus::Resolved => Cell::new("resolved").fg(Color::Green),
            crate::api::models::IssueStatus::Unresolved => Cell::new("unresolved").fg(Color::Red),
            crate::api::models::IssueStatus::Ignored => Cell::new("ignored").fg(Color::Yellow),
            crate::api::models::IssueStatus::Reprocessing => {
                Cell::new("reprocessing").fg(Color::Cyan)
            }
        };

        let title = truncate_string(&issue.title, 50);
        let last_seen = format_relative_time(&issue.last_seen);

        table.add_row(vec![
            Cell::new(&issue.id),
            Cell::new(&issue.short_id),
            Cell::new(title),
            status_cell,
            Cell::new(&issue.count),
            Cell::new(last_seen),
        ]);
    }

    println!("{table}");
    println!("Showing {} issue(s)", issues.len());
}

pub fn print_issue_detail(issue: &Issue) {
    let separator = "=".repeat(80);

    println!();
    println!("{}: {}", "Issue".bold(), issue.short_id.cyan());
    println!("{separator}");
    println!("{:<12} {}", "Title:".bold(), issue.title);
    println!("{:<12} {}", "Status:".bold(), format_status(&issue.status));
    println!("{:<12} {}", "Level:".bold(), issue.level);
    println!(
        "{:<12} {} ({})",
        "Project:".bold(),
        issue.project.name,
        issue.project.slug
    );
    println!(
        "{:<12} {}",
        "First Seen:".bold(),
        issue.first_seen.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!(
        "{:<12} {}",
        "Last Seen:".bold(),
        issue.last_seen.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!();
    println!(
        "{:<12} {} total ({} users affected)",
        "Events:".bold(),
        issue.count,
        issue.user_count
    );
    println!();

    if let Some(ref assigned) = issue.assigned_to {
        println!(
            "{:<12} {} ({})",
            "Assigned:".bold(),
            assigned.name,
            assigned.email.as_deref().unwrap_or(&assigned.actor_type)
        );
    } else {
        println!("{:<12} {}", "Assigned:".bold(), "Unassigned".dimmed());
    }

    if let Some(ref culprit) = issue.culprit {
        println!("{:<12} {}", "Culprit:".bold(), culprit);
    }

    println!();
    println!("{:<12} {}", "Link:".bold(), issue.permalink.blue());
    println!();
}

fn format_status(status: &crate::api::models::IssueStatus) -> String {
    match status {
        crate::api::models::IssueStatus::Resolved => "Resolved".green().to_string(),
        crate::api::models::IssueStatus::Unresolved => "Unresolved".red().to_string(),
        crate::api::models::IssueStatus::Ignored => "Ignored".yellow().to_string(),
        crate::api::models::IssueStatus::Reprocessing => "Reprocessing".cyan().to_string(),
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

fn format_relative_time(dt: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(*dt);

    if duration.num_seconds() < 60 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{} min ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{} hr ago", duration.num_hours())
    } else if duration.num_days() < 7 {
        format!("{} days ago", duration.num_days())
    } else {
        dt.format("%Y-%m-%d").to_string()
    }
}

pub fn print_success(message: &str) {
    println!("{} {}", "✓".green(), message);
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red(), message);
}
