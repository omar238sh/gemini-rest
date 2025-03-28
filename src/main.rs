use dotenv::dotenv;
use std::env;

use aii::ask;
use aii::print_response;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let key = match env::var("key") {
        Ok(key) => key,
        Err(e) => panic!("Failed to get key in envfile: {}", e),
    };

    let response = ask(&key, "Hello gemini How are you").await.unwrap();
    print_response(response).await.unwrap();
}
