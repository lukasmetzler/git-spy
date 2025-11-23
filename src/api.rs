use crate::models::{Repository, User};
use reqwest::{Client, Error};

fn get_github_client() -> Client {
    Client::builder()
        .user_agent("git-spy-cli")
        .build()
        .expect("Could not create the HTTP client")
}

pub async fn fetch_user(username: &str) -> Result<User, Error> {
    let client = get_github_client();
    let url = format!("https://api.github.com/users/{}", username);

    let user = client.get(&url).send().await?.json::<User>().await?;

    Ok(user)
}

pub async fn fetch_repos(username: &str) -> Result<Vec<Repository>, Error> {
    let client = get_github_client();
    let url = format!("https://api.github.com/users/{}/repos", username);

    let repos = client
        .get(&url)
        .send()
        .await?
        .json::<Vec<Repository>>()
        .await?;

    Ok(repos)
}
