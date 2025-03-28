use reqwest::{Client, Error, Response};

pub async fn ask(key: &str, question: &str) -> Result<Response, Error> {
    let data = reqwest::Body::from(format!(
        r#"
        {{
          "contents": [{{
            "parts":[{{"text": "{question}"}}]
            }}]
            }}
        "#
    ));
    let client = Client::new();
    let link = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        key
    );

    let response = client
        .post(link)
        .header("Content-Type", "application/json")
        .body(data)
        .send()
        .await;

    response
}

pub async fn print_response(response: Response) -> Result<(), Error> {
    let text = response.text().await?;
    println!("{}", text);
    Ok(())
}
