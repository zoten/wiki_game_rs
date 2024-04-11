// use tokio::{select, sync::mpsc};
// use tracing::error;

use wiki_game_rs::execute;

#[tokio::main]
async fn main() {
    execute().await;
}
