use crate::config::Config;
use crate::error::{Result, SentryCliError};
use crate::api::models::{ApiError, Issue, IssueUpdate, ListIssuesParams};
use reqwest::{Client, StatusCode};
use url::Url;

pub struct SentryClient {
    client: Client,
    base_url: Url,
    auth_token: String,
    org_slug: String,
}

impl SentryClient {
    pub fn new(
        config: &Config,
        org_override: Option<&str>,
        server_override: Option<&str>,
        token_override: Option<&str>,
    ) -> Result<Self> {
        let auth_token = config.get_auth_token(token_override)?;
        let base_url_str = config.get_server_url(server_override);
        let org_slug = config.get_org(org_override)?;

        let base_url = Url::parse(&base_url_str)?;

        Ok(Self {
            client: Client::new(),
            base_url,
            auth_token,
            org_slug,
        })
    }

    fn api_url(&self, path: &str) -> Result<Url> {
        Ok(self.base_url.join(&format!("/api/0/{}", path))?)
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();

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
        }

        let response = self
            .client
            .get(url)
            .bearer_auth(&self.auth_token)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn get_issue(&self, issue_id: &str) -> Result<Issue> {
        let url = self.api_url(&format!("organizations/{}/issues/{}/", self.org_slug, issue_id))?;

        let response = self
            .client
            .get(url)
            .bearer_auth(&self.auth_token)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn update_issue(&self, issue_id: &str, update: IssueUpdate) -> Result<Issue> {
        let url = self.api_url(&format!("organizations/{}/issues/{}/", self.org_slug, issue_id))?;

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

        let response = self
            .client
            .put(url)
            .bearer_auth(&self.auth_token)
            .json(&update)
            .send()
            .await?;

        let status = response.status();
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
        let url = self.api_url(&format!("organizations/{}/issues/{}/", self.org_slug, issue_id))?;

        let response = self
            .client
            .delete(url)
            .bearer_auth(&self.auth_token)
            .send()
            .await?;

        let status = response.status();
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

        let response = self
            .client
            .delete(url)
            .bearer_auth(&self.auth_token)
            .send()
            .await?;

        let status = response.status();
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
        // Merge is done by updating the primary issue with merge=true and providing other issue IDs
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
