use super::common::{Actor, ProjectRef};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: String,
    pub short_id: String,
    pub title: String,
    pub status: IssueStatus,
    pub level: String,
    #[serde(default)]
    pub count: String,
    #[serde(default)]
    pub user_count: u64,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub permalink: String,
    pub project: ProjectRef,
    pub assigned_to: Option<Actor>,
    #[serde(default)]
    pub is_bookmarked: bool,
    #[serde(default)]
    pub is_subscribed: bool,
    #[serde(default)]
    pub has_seen: bool,
    #[serde(default)]
    pub metadata: IssueMetadata,
    #[serde(default)]
    pub culprit: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IssueStatus {
    Resolved,
    Unresolved,
    Ignored,
    #[serde(rename = "reprocessing")]
    Reprocessing,
}

impl std::fmt::Display for IssueStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueStatus::Resolved => write!(f, "resolved"),
            IssueStatus::Unresolved => write!(f, "unresolved"),
            IssueStatus::Ignored => write!(f, "ignored"),
            IssueStatus::Reprocessing => write!(f, "reprocessing"),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct IssueMetadata {
    pub value: Option<String>,
    pub filename: Option<String>,
    pub function: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<IssueStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_seen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bookmarked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_window: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<StatusDetails>,
}

impl Default for IssueUpdate {
    fn default() -> Self {
        Self {
            status: None,
            assigned_to: None,
            has_seen: None,
            is_bookmarked: None,
            merge: None,
            ignore_duration: None,
            ignore_count: None,
            ignore_window: None,
            status_details: None,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_release: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_next_release: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_until_escalating: Option<bool>,
}

#[derive(Debug, Default, Clone)]
pub struct ListIssuesParams {
    pub project: Option<Vec<String>>,
    pub query: Option<String>,
    pub status: Option<IssueStatus>,
    pub sort: Option<String>,
    pub limit: Option<u32>,
    pub cursor: Option<String>,
}
