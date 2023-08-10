use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed fetching news articles")]
    RequestFailed(ureq::Error),
    #[error("Failed converting response to string")]
    ResponseToStringError(std::io::Error),
    #[error("Failed parsing json")]
    JSONParseError(serde_json::Error),
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| NewsApiError::RequestFailed(e))?
        .into_string()
        .map_err(|e| NewsApiError::ResponseToStringError(e))?;

    let articles: Articles =
        serde_json::from_str(&response).map_err(|e| NewsApiError::JSONParseError(e))?;

    Ok(articles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
