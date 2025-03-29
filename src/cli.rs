use clap::Parser;

#[derive(Parser)]
#[command(name = "AskAi", version = "1.0", about = "Ask Gemini in terminal")]
pub struct AskCli {
    #[arg(short, long, default_value = "User")]
    pub name: String,

    pub question: String,
}
