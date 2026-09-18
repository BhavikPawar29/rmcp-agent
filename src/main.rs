use std::{fs, io::Write};

use serde_json::{Value, json};
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
    let id = _request["id"].clone();
    
    match _request["method"].as_str().unwrap_or_default() {
        "initialize" => {
            // Prepare the protocal handshanke response
            send_response(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocalVesion": "2026-09-18",
                    "capabilites": {"tools": {}},
                    "serverInfo": {"name": "bhavik-rmcp", "version": "0.1.0"},
                }
            }));
        }
        "tools/list" => {
                send_response(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "tools": [
                            {
                                "name": "analyze_production_log",
                                "description": "Reads local production-crash.log to find errors.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {},
                                    "required": []
                                }
                            }
                        ]
                    }
                }));
        }

        "tools/call" => {
            let log_content = fs::read_to_string("production-crash.log").
            unwrap_or_else(|_| "Error: production-crash.log not found.".to_string());

            send_response(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [
                        {
                            "type": "text",
                            "text": log_content
                        }
                    ]
                }
            }));
        }

        _ => {

        }
    }
}

fn send_response(response: Value) {

    let mut stdout = std::io::stdout();

    let output = serde_json::to_string(&response).unwrap();

    writeln!(stdout, "{}", output).unwrap();
    stdout.flush().unwrap();

}