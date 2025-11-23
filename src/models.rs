use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub public_repos: i64,
    pub followers: i64,
    pub following: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub stargazers_count: i64,
    pub language: Option<String>,
}

#[derive(tabled::Tabled)]
pub struct RepoRow {
    #[tabled(rename = "Repository Name")]
    pub name: String,
    #[tabled(rename = "Stars")]
    pub stars: i64,
    #[tabled(rename = "Language")]
    pub language: String,
    #[tabled(rename = "Description")]
    pub description: String,
}
