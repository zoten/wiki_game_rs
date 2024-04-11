/* Parser utils for CLI execution
 */
use clap::Parser;

use crate::wiki_game::domain::coordinator::GameArgs;
use crate::wiki_game::SystemArgs;

use clap_num::number_range;

pub const DEFAULT_START: &str = "Minecraft";
pub const DEFAULT_TARGET: &str = "Adolf_Hitler";
pub const DEFAULT_BASE_URL: &str = "https://en.wikipedia.org";
pub const DEFAULT_WORKERS: &str = "5";
pub const DEFAULT_WORKERS_U8: u8 = 5;

#[derive(Parser, Clone)]
#[clap(author, version, about)]
#[command(version, about, long_about = None, name = "wiki-search")]
struct CliBuilder {
    /// Starting page for the search. Defaults to [/wiki/Minecraft]
    #[arg(short, long, default_value = DEFAULT_START)]
    start: Option<String>,
    /// Target page for the search. Defaults to [/wiki/Adolf_Hitler], because of the internets
    #[arg(short, long, default_value = DEFAULT_TARGET)]
    target: Option<String>,
    /// Target base wiki url for the search (will search only in that domain, e.g. https://en.wikipedia.org)
    #[arg(short, long, default_value = DEFAULT_BASE_URL)]
    base_url: Option<String>,

    /// Number of workers. Max 255, defaults to 5
    #[arg(short, long, default_value = DEFAULT_WORKERS, value_parser=less_than_255)]
    workers: Option<u8>,

    /// Turn debugging information on. Will log a bunch of stuff from external libraries, making it useless at the moment
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

impl Into<GameArgs> for CliBuilder {
    fn into(self) -> GameArgs {
        GameArgs {
            start: self.start.unwrap_or(String::from(DEFAULT_START)),
            target: self.target.unwrap_or(String::from(DEFAULT_TARGET)),
            workers: self.workers.unwrap_or(DEFAULT_WORKERS_U8),
            base_url: self.base_url.unwrap_or(String::from(DEFAULT_BASE_URL)),
        }
    }
}

impl Into<SystemArgs> for CliBuilder {
    fn into(self) -> SystemArgs {
        let level = match self.debug {
            0 => log::LevelFilter::Info,
            1 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Info,
        };
        SystemArgs { debug: level }
    }
}

pub async fn parse() -> (GameArgs, SystemArgs) {
    let cli = CliBuilder::parse();

    // todo fight with moving here
    let ga: GameArgs = cli.clone().into();
    let sa: SystemArgs = cli.into();

    (ga, sa)
}

fn less_than_255(s: &str) -> Result<u8, String> {
    number_range(s, 0, 255)
}
