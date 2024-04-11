// this is in fact both client and scraper but hey
use log::error;
use once_cell::sync::Lazy;
use reqwest::Client;
use scraper::{Html, Selector};

#[derive(Clone)]
pub struct WikiClientScraper {
    client: Client,
}

impl Default for WikiClientScraper {
    fn default() -> Self {
        Self::new()
    }
}

static LINK: Lazy<Selector> = Lazy::new(|| Selector::parse("a[href^='/wiki/'").unwrap());

impl WikiClientScraper {
    pub fn new() -> WikiClientScraper {
        let client = Client::new();
        WikiClientScraper { client: client }
    }
    /// GETs a specific page and returns all wiki links contained
    /// SKIPS special links (categories etc.), so basically each link containing ":"
    pub async fn get_links_from_page(&self, url: String) -> Result<Vec<String>, String> {
        let response = self.client.get(url.as_str()).send().await.map_err(|e| {
            error!("Error calling {url}: {e}");
            format!("Error calling {url}: {e}")
        })?;

        let text = response.text().await.map_err(|e| {
            error!("Error reading {url}: {e}");
            format!("Error reading {url}: {e}")
        })?;

        let mut links = WikiClientScraper::parse_page(text.as_str());
        links.dedup();

        Ok(links)
    }

    // separated just for testing, so I don't have to deal with async trait tests
    pub fn parse_page(page_content: &str) -> Vec<String> {
        let doc = Html::parse_document(page_content);
        let links = doc
            .select(&LINK)
            .map(|link_el| link_el.value().attr("href").unwrap().to_string())
            .filter(|link| !link.contains(':'))
            .collect();
        links
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[tokio::test]
    async fn parse_page_test() {
        let content = fs::read_to_string("assets/test/wiki/Test.html").unwrap();

        let links = WikiClientScraper::parse_page(&content);
        assert!(links.len() == 2);
    }
}
