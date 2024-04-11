/* Coordinator module. Executes the strategy and awaits for results */
use futures_util::stream::FuturesUnordered;
use futures_util::StreamExt;
use itertools::Itertools;
use log::info;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::wiki_game::domain::explorer::explore;
use crate::wiki_game::domain::graph::LinksGraph;
use crate::wiki_game::domain::wiki::{to_full_wiki_url, to_relative_wiki_url};
use crate::wiki_game::httpclient::client::WikiClientScraper;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use tokio::select;
use tokio::sync::{mpsc, mpsc::Receiver, mpsc::Sender};

pub type FullWikiUrl = String;
pub type RelWikiUrl = String;

const LOG_AFTER: i32 = 10;

pub struct GameArgs {
    // start page
    pub start: String,
    // target page
    pub target: String,
    // base wiki url
    pub base_url: String,
    // Number of parallel workers
    pub workers: u8,
}

#[derive(Debug)]
pub struct LinksFoundContent {
    pub links: Vec<RelWikiUrl>,
}

#[derive(Debug)]
pub enum CoordinatorMessage {
    Links(LinksFoundContent),
}

pub type SearchState = Arc<Mutex<LinksGraph>>;

pub async fn execute(ga: GameArgs) -> Result<(), String> {
    let lg = LinksGraph::new();
    let state = Arc::new(Mutex::new(lg));

    info!("Starting coordinator");

    let client = WikiClientScraper::new();

    let start_full_url: FullWikiUrl = to_full_wiki_url(&ga.start, &ga.base_url);

    let start_rel_url: RelWikiUrl = to_relative_wiki_url(&ga.start, &ga.base_url);
    let target_rel_url: RelWikiUrl = to_relative_wiki_url(&ga.target, &ga.base_url);

    info!("Start rel [{start_rel_url}] Target rel [{target_rel_url}]");

    if is_target_page(&start_rel_url, &target_rel_url) {
        info!("Oh, come on pal!");
        return Ok(());
    }

    let mut current_links = vec![start_full_url];

    let (tx, mut rx): (Sender<CoordinatorMessage>, Receiver<CoordinatorMessage>) =
        mpsc::channel(1000);

    let start = SystemTime::now();

    let mut rng = thread_rng();
    let mut found: u8 = 0;
    let mut idx: i32 = 0;
    let mut futures_dead: i32 = 0;
    'outer: loop {
        if idx % LOG_AFTER == 0 {
            info!("*** Starting loop [{idx}] ***");
            info!("Futures dead    : {futures_dead}");
            info!("Current links # : {}", current_links.len());
        }
        idx += 1;
        let mut buf = FuturesUnordered::new();

        let len = current_links.len();
        current_links.shuffle(&mut rng);

        let chunks = current_links
            .iter()
            .map(|link| to_full_wiki_url(link, &ga.base_url))
            .collect::<Vec<String>>()
            .into_iter()
            .chunks(len / ga.workers as usize + 1);

        for chunk in chunks.into_iter() {
            let chunk = chunk.collect_vec();
            let handler = tokio::spawn(explore(
                client.clone(),
                ga.base_url.clone(),
                chunk,
                Arc::clone(&state),
                tx.clone(),
            ));
            buf.push(handler);
        }
        current_links = vec![];

        select! {
            recv = rx.recv() => {
                if let Some(msg) = recv {
                    match msg {
                        CoordinatorMessage::Links(mut content) => {
                            content.links.iter().for_each(|link| {
                                if is_target_page(link, &target_rel_url) {
                                    found += 1;
                                }
                            });

                            if break_condition(found, &ga) {
                                break 'outer;
                            }

                            current_links.append(&mut content.links);
                        }
                    }
                }
            }

            _resolved = buf.next() => {
                // future resolved, something to do?
                futures_dead += 1;
                if break_condition(found, &ga) {
                    break 'outer;
                }

                if current_links.len() == 0 {
                    info!("No more links to explore");
                    info!("Stopped at loop [{idx}]");
                    break 'outer;
                }
            }

        }
    }

    if found == 0 {
        info!("Target has not been found :(");
        return Err(String::from("Target has not been found :("));
    }

    let duration = start.elapsed().unwrap();

    let shortest_path = state
        .lock()
        .unwrap()
        .shortest_path_to_target(start_rel_url, target_rel_url)
        .unwrap();

    let duration_calculated = start.elapsed().unwrap().checked_sub(duration).unwrap();

    info!("Target has been found!");
    info!("[{}] hops needed", shortest_path.len());
    info!("{:?}", shortest_path);
    info!(
        "Search took [{}]s . Path calculation took [{}]ms",
        duration.as_secs(),
        duration_calculated.as_millis()
    );

    Ok(())
}

fn is_target_page(page: &str, target: &str) -> bool {
    page == target
}

/// found: we may want to stop after many paths converged to target
fn break_condition(found: u8, _ga: &GameArgs) -> bool {
    found >= 1
}
