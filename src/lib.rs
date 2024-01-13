#![allow(dead_code)]
use reqwest::Url;

/// Scrapes the Steam Rimworld workshop for the mod you've searched, or alternativeli shows the recommended ones if no text is provided
async fn search_workshop<T: ToString>(mod_name: Option<T>) -> String {
    let workshop_url: Result<Url, _> = match mod_name {
        Some(is_mod_name) => Url::parse_with_params(
            "https://steamcommunity.com/workshop/browse/?appid=294100",
            &[("searchText", is_mod_name.to_string())],
        ),
        None => Url::parse("https://steamcommunity.com/workshop/browse/?appid=294100"),
    };
    return workshop_url.unwrap().as_str().to_owned();
}

#[cfg(test)]
mod tests {
    use crate::search_workshop;

    #[tokio::test]
    async fn test_search_workshop() {
        assert_eq!(
            search_workshop(Some("0000")).await,
            "https://steamcommunity.com/workshop/browse/?appid=294100&searchText=0000"
        )
    }
}
