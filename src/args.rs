use clap::Parser;

#[derive(Parser)]
#[command(name = "naughts-and-crosses")]
#[command(version = "1.0")]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub two_player: bool,
}
