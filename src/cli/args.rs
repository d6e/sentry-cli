use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sentry-cli")]
#[command(about = "CLI tool for managing Sentry issues", long_about = None)]
#[command(version)]
pub struct Cli {
    /// Sentry server URL (default: https://sentry.io)
    #[arg(long, global = true)]
    pub server: Option<String>,

    /// Organization slug
    #[arg(long, short, global = true)]
    pub org: Option<String>,

    /// Auth token (overrides env var and config)
    #[arg(long, global = true)]
    pub token: Option<String>,

    /// Enable verbose output
    #[arg(long, short, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage Sentry issues
    Issues {
        #[command(subcommand)]
        command: IssuesCommands,
    },
    /// Manage CLI configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand)]
pub enum IssuesCommands {
    /// List issues with optional filtering
    List {
        /// Filter by project slug(s), comma-separated
        #[arg(long, short)]
        project: Option<String>,

        /// Filter by status: unresolved, resolved, ignored
        #[arg(long, short)]
        status: Option<String>,

        /// Sentry search query string
        #[arg(long, short)]
        query: Option<String>,

        /// Sort by: date, new, freq, user
        #[arg(long, default_value = "date")]
        sort: String,

        /// Maximum number of results
        #[arg(long, default_value = "25")]
        limit: u32,
    },

    /// View detailed issue information
    View {
        /// Issue ID or short ID
        issue_id: String,
    },

    /// Resolve one or more issues
    Resolve {
        /// Issue ID(s) to resolve
        #[arg(required = true)]
        issue_ids: Vec<String>,

        /// Mark resolved in specific release
        #[arg(long)]
        in_release: Option<String>,

        /// Mark resolved in next release
        #[arg(long)]
        in_next_release: bool,
    },

    /// Unresolve one or more issues
    Unresolve {
        /// Issue ID(s) to unresolve
        #[arg(required = true)]
        issue_ids: Vec<String>,
    },

    /// Assign issue(s) to a user or team
    Assign {
        /// Issue ID(s) to assign
        #[arg(required = true)]
        issue_ids: Vec<String>,

        /// User email or team slug (prefix with "team:")
        #[arg(long)]
        to: Option<String>,

        /// Remove assignment instead
        #[arg(long)]
        unassign: bool,
    },

    /// Ignore issue(s)
    Ignore {
        /// Issue ID(s) to ignore
        #[arg(required = true)]
        issue_ids: Vec<String>,

        /// Ignore for N minutes
        #[arg(long)]
        duration: Option<u64>,

        /// Ignore until N more events
        #[arg(long)]
        count: Option<u64>,

        /// Ignore until escalating
        #[arg(long)]
        until_escalating: bool,
    },

    /// Delete issue(s)
    Delete {
        /// Issue ID(s) to delete
        #[arg(required = true)]
        issue_ids: Vec<String>,

        /// Skip confirmation prompt
        #[arg(long)]
        confirm: bool,
    },

    /// Merge multiple issues into one
    Merge {
        /// Primary issue ID (issues will be merged into this one)
        primary_id: String,

        /// Other issue IDs to merge
        #[arg(required = true)]
        other_ids: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Create default config file
    Init,

    /// Display current configuration
    Show,

    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },
}
