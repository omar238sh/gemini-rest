use dotenv::dotenv;
use std::env;
mod cli;
use aii::ask;
use aii::print_response;
use clap::Parser;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = cli::AskCli::parse();
    let key = match env::var("key") {
        Ok(key) => key,
        Err(e) => panic!("Failed to get key in envfile: {}", e),
    };

    let response = ask(&key, &args.question).await.unwrap();

    print_response(response).await.unwrap();
}
