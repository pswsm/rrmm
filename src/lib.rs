#![allow(dead_code)]
use reqwest::Url;
use scraper::{Html, Selector};

const SELECTOR: &str = "div.workshopItem > a.ugc";

/// Scrapes the Steam Rimworld workshop for the mod you've searched, or alternativeli shows the recommended ones if no text is provided
async fn search_workshop(mod_name: Option<&str>) -> Vec<String> {
    let selector: Selector = match Selector::parse(SELECTOR) {
        Ok(selector) => selector,
        Err(error) => panic!("{}", error),
    };
    let workshop_url: Result<Url, _> = match mod_name {
        Some(is_mod_name) => Url::parse_with_params(
            "https://steamcommunity.com/workshop/browse/?appid=294100",
            &[("searchText", is_mod_name)],
        ),
        None => Url::parse("https://steamcommunity.com/workshop/browse/?appid=294100"),
    };
    dbg!(&workshop_url);
    let response = reqwest::get(workshop_url.unwrap()).await;
    let response_text = response.unwrap().text().await;
    let html = Html::parse_document(response_text.unwrap().as_str());
    let scraped = html.select(&selector);

    scraped
        .into_iter()
        .map(|v| v.value().attr("data-publishedfileid").unwrap().to_string())
        .collect::<Vec<String>>()
    // return workshop_url.unwrap().as_str().to_owned();
    // return response.unwrap().status().is_success();
}

#[cfg(test)]
mod tests {
    use crate::search_workshop;

    #[tokio::test]
    async fn test_search_workshop() {
        assert_eq!(
            search_workshop(Some("vanilla expanded")).await,
            vec!["AAAAAAA".to_string()]
        )
    }

    #[tokio::test]
    async fn test_empty_search_workshop() {
        assert_eq!(search_workshop(None).await, vec!["AAAAAAA".to_string()])
    }
}
