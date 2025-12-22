mod api;
mod cli;
mod config;
mod error;
mod output;

use clap::Parser;
use cli::args::{Cli, Commands, ConfigCommands, IssuesCommands};
use cli::commands::{config as config_cmd, issues};
use config::load_config;
use output::print_error;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        print_error(&e.to_string());
        std::process::exit(1);
    }
}

async fn run() -> error::Result<()> {
    let cli = Cli::parse();
    let config = load_config();

    match cli.command {
        Commands::Issues { command } => {
            let client = api::SentryClient::new(
                &config,
                cli.org.as_deref(),
                cli.server.as_deref(),
                cli.token.as_deref(),
                cli.verbose,
            )?;

            match command {
                IssuesCommands::List {
                    project,
                    status,
                    query,
                    sort,
                    limit,
                    output,
                    all,
                } => {
                    issues::list_issues(&client, project, status, query, sort, limit, output, all)
                        .await?;
                }
                IssuesCommands::View { issue_id, output } => {
                    issues::view_issue(&client, &issue_id, output).await?;
                }
                IssuesCommands::Resolve {
                    issue_ids,
                    in_release,
                    in_next_release,
                } => {
                    issues::resolve_issues(&client, issue_ids, in_release, in_next_release).await?;
                }
                IssuesCommands::Unresolve { issue_ids } => {
                    issues::unresolve_issues(&client, issue_ids).await?;
                }
                IssuesCommands::Assign {
                    issue_ids,
                    to,
                    unassign,
                } => {
                    issues::assign_issues(&client, issue_ids, to, unassign).await?;
                }
                IssuesCommands::Ignore {
                    issue_ids,
                    duration,
                    count,
                    until_escalating,
                } => {
                    issues::ignore_issues(&client, issue_ids, duration, count, until_escalating)
                        .await?;
                }
                IssuesCommands::Delete { issue_ids, confirm } => {
                    issues::delete_issues(&client, issue_ids, confirm).await?;
                }
                IssuesCommands::Merge {
                    primary_id,
                    other_ids,
                } => {
                    issues::merge_issues(&client, primary_id, other_ids).await?;
                }
            }
        }
        Commands::Config { command } => match command {
            ConfigCommands::Init => {
                config_cmd::init_config()?;
            }
            ConfigCommands::Show => {
                config_cmd::show_config()?;
            }
            ConfigCommands::Set { key, value } => {
                config_cmd::set_config(&key, &value)?;
            }
        },
    }

    Ok(())
}
