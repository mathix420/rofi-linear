use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

const ENDPOINT: &str = "https://api.linear.app/graphql";

#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ViewerData {
    pub viewer: Viewer,
}

#[derive(Debug, Deserialize)]
pub struct Viewer {
    #[allow(dead_code)]
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct TeamsData {
    pub teams: TeamsNodes,
}

#[derive(Debug, Deserialize)]
pub struct TeamsNodes {
    pub nodes: Vec<Team>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct IssueCreateData {
    #[serde(rename = "issueCreate")]
    pub issue_create: IssueCreateResult,
}

#[derive(Debug, Deserialize)]
pub struct IssueCreateResult {
    pub success: bool,
    pub issue: Option<Issue>,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    #[allow(dead_code)]
    pub id: String,
    pub identifier: String,
    pub url: String,
    pub title: String,
}

pub struct LinearClient {
    client: Client,
    api_key: String,
}

impl LinearClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
        }
    }

    async fn query<T: for<'de> Deserialize<'de>>(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
    ) -> Result<T> {
        let body = json!({
            "query": query,
            "variables": variables.unwrap_or(json!({}))
        });

        let response = self
            .client
            .post(ENDPOINT)
            .header("Authorization", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Linear API")?;

        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Linear API error ({}): {}", status, text);
        }

        let result: GraphQLResponse<T> = response
            .json()
            .await
            .context("Failed to parse Linear API response")?;

        if let Some(errors) = result.errors {
            let messages: Vec<_> = errors.iter().map(|e| e.message.as_str()).collect();
            anyhow::bail!("GraphQL errors: {}", messages.join(", "));
        }

        result.data.context("No data in response")
    }

    pub async fn get_viewer(&self) -> Result<Viewer> {
        const QUERY: &str = r#"
            query {
                viewer {
                    id
                    name
                    email
                }
            }
        "#;

        let data: ViewerData = self.query(QUERY, None).await?;
        Ok(data.viewer)
    }

    pub async fn get_teams(&self) -> Result<Vec<Team>> {
        const QUERY: &str = r#"
            query {
                teams {
                    nodes {
                        id
                        name
                        key
                    }
                }
            }
        "#;

        let data: TeamsData = self.query(QUERY, None).await?;
        Ok(data.teams.nodes)
    }

    pub async fn create_issue(
        &self,
        team_id: &str,
        title: &str,
        description: Option<&str>,
    ) -> Result<Issue> {
        const MUTATION: &str = r#"
            mutation CreateIssue($title: String!, $teamId: String!, $description: String) {
                issueCreate(input: { title: $title, teamId: $teamId, description: $description }) {
                    success
                    issue {
                        id
                        identifier
                        url
                        title
                    }
                }
            }
        "#;

        let variables = json!({
            "title": title,
            "teamId": team_id,
            "description": description
        });

        let data: IssueCreateData = self.query(MUTATION, Some(variables)).await?;

        if !data.issue_create.success {
            anyhow::bail!("Failed to create issue");
        }

        data.issue_create.issue.context("No issue returned")
    }
}
