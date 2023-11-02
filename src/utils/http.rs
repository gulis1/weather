use anyhow::Result;
use gloo::net::http::Request;

pub async fn http_get_string(url: &str) -> Result<String> {

    let response = Request::get(url)
        .send().await?
        .text().await?;

    Ok(response)
}