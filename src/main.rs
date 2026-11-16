mod args;
mod errors;
mod game_loop;
mod game_state;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    println!("Welcome to Naughts and Crosses!");
    if args.two_player {
        println!("Starting two player game.");
        game_loop::two_player_loop();
    } else {
        println!("Starting single player game");
        println!("Not implemented.");
    }
}
