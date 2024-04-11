# WikiGame

A naive Rust implementation of the [Wiki Game](https://en.wikipedia.org/wiki/Wikipedia:Wiki_Game) through a super simple orchestrated worker coordination.
Thanks to [nappa85](https://github.com/nappa85) for making me lose some hours this evening.

Brother of [wiki_game_ex](https://github.com/zoten/wiki_game_ex) (despite a different implementation)

## Run

``` bash
cargo run --release -- -w 250  -s "Minecraft" -b "https://m.wikipedia.org"
```

## Build

``` bash
cargo build --release
```

This will create a `wiki_game_rs` executable in your `target/release` directory

## Usage

``` bash
./wiki_game_rs -h
```

e.g.

``` bash
./wiki_game_rs -s Pokémon -t Super_Mario -w 250
```

## Notes

Hey, this is a game.

Also, I'm a really bad Rust developer and this has been written in some free time between days. This is by no way good code? This has few tests and mostly because I'm bad at Rust, a lot of probably avoidable `clone()` and some debatale practices. `clippy` is complaining too.
Deal with it, please.

Current implementation relies on a coordinator process that spawns explorer tasks, giving them a bunch of links to explore. Then, it acts as a dispatcher to the same number of explorer tasks, that are spawned again until the end of time.

Current implementation may not find the shortest path, since it stops after the first occurrence of target page. It is possible to improve it by tweaking the termination conditions.

There is no termination guard, be ready with `CTRL^C`. There is as little validation as possible, e.g. be sure the target page exists. Also the source, just to be sure.

This is still interestingly showing a couple of things about how to write decent software for a trivial scraping problem, how to write CLI tools in Rust and maybe other things I'll reason about.


## Things I'd like to add someday

 * [ ] other implementations of the game
   * [ ] an explore/exploit one
   * [ ] this could leverage traits and mock using traits
 * [ ] more tests and hex architecture for academic purpose

Will I? Probably not. But I had fun.
