use serde_json::Value;
use tokio::io::{self, AsyncBufReadExt, BufReader};


#[tokio::main]
async fn main () {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        if let Ok(request) = serde_json::from_str::<Value>(&line) {
            handle_request(request).await;
        }
    }

}

async fn handle_request(_request: Value) {
    println!("Recived request: {:?}", _request);
}