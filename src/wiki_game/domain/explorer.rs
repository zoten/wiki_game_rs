use std::time::Duration;

use log::error;
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;

use crate::wiki_game::domain::coordinator::LinksFoundContent;
use crate::wiki_game::httpclient::client::WikiClientScraper;

use super::coordinator::CoordinatorMessage::Links;
use super::coordinator::{CoordinatorMessage, FullWikiUrl, RelWikiUrl, SearchState};
use super::wiki::to_relative_wiki_url;

/// Explore!

pub async fn explore(
    client: WikiClientScraper,
    base_url: String,
    links: Vec<FullWikiUrl>,
    state: SearchState,
    tx: Sender<CoordinatorMessage>,
) {
    let mut result: Vec<RelWikiUrl> = vec![];
    for link in links.iter() {
        if let Ok(page_links) = client.get_links_from_page(link.clone()).await.map_err(|_| {
            error!("Couldn't get page [{link}]");
            format!("Couldn't get page [{link}]")
        }) {
            let mut graph = state.lock().unwrap();

            let mut add_to_result = page_links.clone();
            add_to_result.retain(|link| !graph.node_exists(link));

            // let's still add all the edges since we may have found new paths
            page_links.iter().for_each(|new_link| {
                let rel_link = to_relative_wiki_url(link, &base_url);
                graph.add_edge(&rel_link, new_link);
            });

            result.append(add_to_result.as_mut());
        }
        sleep(Duration::from_millis(1)).await;
    }

    result.dedup();

    match tx.send(Links(LinksFoundContent { links: result })).await {
        Ok(_) => (),
        Err(err) => {
            error!("Error tx to coordinator [{:?}]", err);
            ()
        }
    }

    // this is slightly hackish, since sometimes it seems the future dies before the message has been sent
    // probably I'd need to refactor the main loop
    sleep(Duration::from_millis(5)).await;

    ()
}
