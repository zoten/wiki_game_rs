// wiki_game main module

pub mod cli;
pub mod domain;
pub mod httpclient;
pub mod loggers;

use crate::wiki_game::loggers::console_logger::ConsoleLogger;
use domain::coordinator::execute;

use self::domain::coordinator::GameArgs;
use log::info;

static LOGGER: ConsoleLogger = ConsoleLogger;

pub struct SystemArgs {
    // debug level
    pub debug: log::LevelFilter,
}

pub async fn start() {
    let (ga, sa) = cli::parser::parse().await;
    set_logging(sa);

    print_config(&ga);

    execute(ga).await.expect("Execution failed  :(");
}

fn set_logging(sa: SystemArgs) {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(sa.debug))
        .unwrap();
}

fn print_config(ga: &GameArgs) {
    info!("Configuration:");
    info!("Base URL   :  [{}]", ga.base_url);
    info!("Start page :  [{}]", ga.start);
    info!("Target page:  [{}]", ga.target);
    info!("Workers    :  [{}]", ga.workers);
}
