pub mod wiki_game;

use wiki_game::start;

pub async fn execute() {
    start().await;
}
