mod args;
mod game_state;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    println!("Welcome to Naughts and Crosses!");
    if args.two_player {
        println!("Starting two player game");
    } else {
        println!("Starting single player game");
    }
    println!("{}", args.two_player);
}
