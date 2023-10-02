use reqwest::Client;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let mut event_stream = client
        .get("http://example.com/sse_endpoint")
        .header("Accept", "text/event-stream")
        .send()
        .await?
        .bytes_stream();

    while let Some(event) = event_stream.next().await {
        match event {
            Ok(chunk) => {
                let data = String::from_utf8_lossy(&chunk);
                println!("Received event: {}", data);
            }
            Err(err) => {
                eprintln!("Error receiving event: {:?}", err);
            }
        }
    }

    Ok(())
}