mod game;

extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("rustman")
        .version("1.0")
        .author("viargentum <viargentum@riseup.net>")
        .arg(
            Arg::with_name("play")
                .short("p")
                .long("play")
                .help("Starts a game of rustman")
                .takes_value(false),
        )
        .get_matches();

    if matches.is_present("play") {
        game::main();
    }
}
