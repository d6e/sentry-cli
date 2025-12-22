use crate::api::models::{ApiError, Issue, IssueUpdate, ListIssuesParams};
use crate::config::Config;
use crate::error::{Result, SentryCliError};
use reqwest::{Client, Response, StatusCode};
use url::Url;

pub struct SentryClient {
    client: Client,
    base_url: Url,
    auth_token: String,
    org_slug: String,
    verbose: bool,
}

impl SentryClient {
    pub fn new(
        config: &Config,
        org_override: Option<&str>,
        server_override: Option<&str>,
        token_override: Option<&str>,
        verbose: bool,
    ) -> Result<Self> {
        let auth_token = config.get_auth_token(token_override)?;
        let base_url_str = config.get_server_url(server_override);
        let org_slug = config.get_org(org_override)?;

        let base_url = Url::parse(&base_url_str)?;

        if verbose {
            eprintln!("[verbose] Server: {}", base_url);
            eprintln!("[verbose] Organization: {}", org_slug);
        }

        Ok(Self {
            client: Client::new(),
            base_url,
            auth_token,
            org_slug,
            verbose,
        })
    }

    fn api_url(&self, path: &str) -> Result<Url> {
        Ok(self.base_url.join(&format!("/api/0/{}", path))?)
    }

    fn log_request(&self, method: &str, url: &Url) {
        if self.verbose {
            eprintln!("[verbose] {} {}", method, url);
        }
    }

    fn log_response(&self, status: StatusCode) {
        if self.verbose {
            eprintln!("[verbose] Response: {}", status);
        }
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T> {
        let status = response.status();
        self.log_response(status);

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await.unwrap_or_default();
            let message = serde_json::from_str::<ApiError>(&error_body)
                .map(|e| e.detail)
                .unwrap_or(error_body);

            Err(match status {
                StatusCode::UNAUTHORIZED => SentryCliError::Auth(message),
                StatusCode::FORBIDDEN => SentryCliError::Forbidden(message),
                StatusCode::NOT_FOUND => SentryCliError::NotFound(message),
                StatusCode::TOO_MANY_REQUESTS => SentryCliError::RateLimited { retry_after: 60 },
                _ => SentryCliError::Api {
                    status: status.as_u16(),
                    message,
                },
            })
        }
    }

    /// Parse the Link header to find the next page cursor
    fn parse_next_cursor(link_header: Option<&str>) -> Option<String> {
        let link = link_header?;
        // Link header format: <url>; rel="previous"; results="false"; cursor="...", <url>; rel="next"; ...
        for part in link.split(',') {
            if part.contains("rel=\"next\"") && part.contains("results=\"true\"") {
                // Extract cursor value
                for segment in part.split(';') {
                    let segment = segment.trim();
                    if segment.starts_with("cursor=") {
                        return Some(segment.trim_start_matches("cursor=").trim_matches('"').to_string());
                    }
                }
            }
        }
        None
    }

    pub async fn list_issues(&self, params: ListIssuesParams) -> Result<Vec<Issue>> {
        let mut url = self.api_url(&format!("organizations/{}/issues/", self.org_slug))?;

        {
            let mut query = url.query_pairs_mut();

            if let Some(projects) = &params.project {
                for project in projects {
                    query.append_pair("project", project);
                }
            }

            if let Some(q) = &params.query {
                query.append_pair("query", q);
            }

            if let Some(status) = &params.status {
                query.append_pair("query", &format!("is:{}", status));
            }

            if let Some(sort) = &params.sort {
                query.append_pair("sort", sort);
            }

            if let Some(limit) = params.limit {
                query.append_pair("limit", &limit.to_string());
            }

            if let Some(cursor) = &params.cursor {
                query.append_pair("cursor", cursor);
            }
        }

        self.log_request("GET", &url);

        let response = self
            .client
            .get(url)
            .bearer_auth(&self.auth_token)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// List all issues with automatic pagination
    pub async fn list_all_issues(&self, params: ListIssuesParams) -> Result<Vec<Issue>> {
        let mut all_issues = Vec::new();
        let mut cursor: Option<String> = None;
        let mut page = 1;

        loop {
            let page_params = ListIssuesParams {
                project: params.project.clone(),
                query: params.query.clone(),
                status: params.status,
                sort: params.sort.clone(),
                limit: params.limit,
                cursor: cursor.clone(),
            };

            let mut url = self.api_url(&format!("organizations/{}/issues/", self.org_slug))?;

            {
                let mut query = url.query_pairs_mut();

                if let Some(projects) = &page_params.project {
                    for project in projects {
                        query.append_pair("project", project);
                    }
                }

                if let Some(q) = &page_params.query {
                    query.append_pair("query", q);
                }

                if let Some(status) = &page_params.status {
                    query.append_pair("query", &format!("is:{}", status));
                }

                if let Some(sort) = &page_params.sort {
                    query.append_pair("sort", sort);
                }

                if let Some(limit) = page_params.limit {
                    query.append_pair("limit", &limit.to_string());
                }

                if let Some(c) = &page_params.cursor {
                    query.append_pair("cursor", c);
                }
            }

            self.log_request("GET", &url);
            if self.verbose {
                eprintln!("[verbose] Fetching page {}...", page);
            }

            let response = self
                .client
                .get(url)
                .bearer_auth(&self.auth_token)
                .send()
                .await?;

            let status = response.status();
            self.log_response(status);

            if !status.is_success() {
                let error_body = response.text().await.unwrap_or_default();
                let message = serde_json::from_str::<ApiError>(&error_body)
                    .map(|e| e.detail)
                    .unwrap_or(error_body);

                return Err(SentryCliError::Api {
                    status: status.as_u16(),
                    message,
                });
            }

            // Get Link header before consuming response
            let link_header = response
                .headers()
                .get("link")
                .and_then(|v| v.to_str().ok())
                .map(String::from);

            let issues: Vec<Issue> = response.json().await?;
            let count = issues.len();
            all_issues.extend(issues);

            if self.verbose {
                eprintln!("[verbose] Got {} issues (total: {})", count, all_issues.len());
            }

            // Check for next page
            cursor = Self::parse_next_cursor(link_header.as_deref());
            if cursor.is_none() {
                break;
            }
            page += 1;
        }

        Ok(all_issues)
    }

    pub async fn get_issue(&self, issue_id: &str) -> Result<Issue> {
        let url = self.api_url(&format!(
            "organizations/{}/issues/{}/",
            self.org_slug, issue_id
        ))?;

        self.log_request("GET", &url);

        let response = self
            .client
            .get(url)
            .bearer_auth(&self.auth_token)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn update_issue(&self, issue_id: &str, update: IssueUpdate) -> Result<Issue> {
        let url = self.api_url(&format!(
            "organizations/{}/issues/{}/",
            self.org_slug, issue_id
        ))?;

        self.log_request("PUT", &url);

        let response = self
            .client
            .put(url)
            .bearer_auth(&self.auth_token)
            .json(&update)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn update_issues(&self, issue_ids: &[String], update: IssueUpdate) -> Result<()> {
        let mut url = self.api_url(&format!("organizations/{}/issues/", self.org_slug))?;

        {
            let mut query = url.query_pairs_mut();
            for id in issue_ids {
                query.append_pair("id", id);
            }
        }

        self.log_request("PUT", &url);

        let response = self
            .client
            .put(url)
            .bearer_auth(&self.auth_token)
            .json(&update)
            .send()
            .await?;

        let status = response.status();
        self.log_response(status);

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await.unwrap_or_default();
            Err(SentryCliError::Api {
                status: status.as_u16(),
                message: error_body,
            })
        }
    }

    pub async fn delete_issue(&self, issue_id: &str) -> Result<()> {
        let url = self.api_url(&format!(
            "organizations/{}/issues/{}/",
            self.org_slug, issue_id
        ))?;

        self.log_request("DELETE", &url);

        let response = self
            .client
            .delete(url)
            .bearer_auth(&self.auth_token)
            .send()
            .await?;

        let status = response.status();
        self.log_response(status);

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await.unwrap_or_default();
            Err(SentryCliError::Api {
                status: status.as_u16(),
                message: error_body,
            })
        }
    }

    pub async fn delete_issues(&self, issue_ids: &[String]) -> Result<()> {
        let mut url = self.api_url(&format!("organizations/{}/issues/", self.org_slug))?;

        {
            let mut query = url.query_pairs_mut();
            for id in issue_ids {
                query.append_pair("id", id);
            }
        }

        self.log_request("DELETE", &url);

        let response = self
            .client
            .delete(url)
            .bearer_auth(&self.auth_token)
            .send()
            .await?;

        let status = response.status();
        self.log_response(status);

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await.unwrap_or_default();
            Err(SentryCliError::Api {
                status: status.as_u16(),
                message: error_body,
            })
        }
    }

    pub async fn merge_issues(&self, primary_id: &str, other_ids: &[String]) -> Result<Issue> {
        let mut all_ids = vec![primary_id.to_string()];
        all_ids.extend(other_ids.iter().cloned());

        let mut url = self.api_url(&format!("organizations/{}/issues/", self.org_slug))?;

        {
            let mut query = url.query_pairs_mut();
            for id in &all_ids {
                query.append_pair("id", id);
            }
        }

        let update = IssueUpdate {
            merge: Some(true),
            ..Default::default()
        };

        self.log_request("PUT", &url);

        let response = self
            .client
            .put(url)
            .bearer_auth(&self.auth_token)
            .json(&update)
            .send()
            .await?;

        self.handle_response(response).await
    }
}
