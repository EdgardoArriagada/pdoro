use clap::Parser;

/// pdoro
#[derive(Debug, Parser)]
#[command(author, about, long_about = None)]
pub struct Args {
    /// time duration of session
    #[clap(long, short)]
    pub time: String,

    /// message to display
    #[clap(long, short)]
    pub message: Option<String>,

    /// remaining duration of session
    #[clap(long, short)]
    pub remaining: bool,
}
