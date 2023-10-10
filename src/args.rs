use clap::Parser;

/// pdoro
#[derive(Debug, Parser)]
#[command(author, about, long_about = None)]
pub struct Args {
    /// time duration of session
    #[clap(long, short)]
    pub time: Option<String>,

    /// message to display
    #[clap(long, short)]
    pub message: Option<String>,

    /// remaining duration of session
    #[clap(long, short)]
    pub remaining: bool,

    /// start pdoro sercer
    #[clap(long, short)]
    pub start_server: bool,

    /// halt pomodoro counter
    #[clap(long)]
    pub halt_counter: bool,

    /// pause pomodoro counter
    #[clap(long)]
    pub pause_counter: bool,

    /// resume pomodoro counter
    #[clap(long)]
    pub resume_counter: bool,

    /// toggle pause/resume pomodoro counter
    #[clap(long, short)]
    pub pause_resume_counter: bool,
}
