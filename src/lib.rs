#![allow(dead_code)]
use core::panic;
use std::collections::HashMap;

use reqwest::Url;
use scraper::{Html, Selector};

const WKSHOP_ITEM_SELECTOR: &str = "div.workshopItem > a.ugc";
const AUTHOR_NAME_SELECTOR: &str =
    "div.workshopItem > div.workshopItemAuthorName > a.workshop_author_link";

struct Mod {
    author: String,
    id: usize,
}

fn parse_steam_workshop_url(mod_name: Option<&str>) -> Url {
    match mod_name {
        Some(is_mod_name) => Url::parse_with_params(
            "https://steamcommunity.com/workshop/browse/?appid=294100",
            &[("searchtext", is_mod_name)],
        )
        .unwrap(),
        None => Url::parse("https://steamcommunity.com/workshop/browse/?appid=294100").unwrap(),
    }
}

/// Scrapes the Steam Rimworld workshop for the mod you've searched, or alternativeli shows the recommended ones if no text is provided
async fn search_workshop(mod_name: Option<&str>) -> HashMap<&str, &str> {
    let workshop_item_selector: Selector = match Selector::parse(WKSHOP_ITEM_SELECTOR) {
        Ok(selector) => selector,
        Err(error) => panic!("{}", error),
    };
    let author_name_selector: Selector = match Selector::parse(AUTHOR_NAME_SELECTOR) {
        Ok(selector) => selector,
        Err(err) => panic!("{}", err),
    };

    let workshop_url: Url = parse_steam_workshop_url(mod_name);
    let response = reqwest::get(workshop_url).await;
    let response_text = response.unwrap().text().await;
    let html = Html::parse_document(response_text.unwrap().as_str());
    let scraped_workshop_items = html
        .select(&workshop_item_selector)
        .into_iter()
        .map(|wkitem| {
            wkitem
                .value()
                .attr("data-publishedfileid")
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>();
    let scraped_authors = html
        .select(&author_name_selector)
        .into_iter()
        .map(|wkauthor| wkauthor.value())
        .collect::<Vec<String>>();

    HashMap::from([("", "")])

    // scraped
    //     .into_iter()
    //     .map(|v| v.value().attr("data-publishedfileid").unwrap().to_string())
    //     .collect::<Vec<String>>()
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
