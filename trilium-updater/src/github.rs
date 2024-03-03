use std::fmt::Display;

use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderName, ACCEPT, AUTHORIZATION};
use serde::Deserialize;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Deserialize)]
struct Release {
    draft: bool,
    prerelease: bool,
    tag_name: String,
}

pub async fn fetch_latest_tag(github_token: Option<impl Display>) -> Result<Option<String>> {
    let mut headers = HeaderMap::new();
    headers.append(ACCEPT, "application/vnd.github+json".parse()?);
    headers.append(
        HeaderName::from_static("x-github-api-version"),
        "2022-11-28".parse().unwrap(),
    );
    if let Some(token) = &github_token {
        headers.append(AUTHORIZATION, format!("Bearer {token}").parse()?);
    }
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .user_agent(USER_AGENT)
        .build()?;

    let url = "https://api.github.com/repos/zadam/trilium/releases";
    let res = client.get(url).send().await?;
    log::info!("GET {url} {}", res.status());
    let releases: Vec<Release> = res.json().await?;
    log::debug!("Releases: {releases:?}");

    let latest = releases
        .into_iter()
        .find(|release| !release.draft && !release.prerelease)
        .map(|release| release.tag_name);
    Ok(latest)
}
