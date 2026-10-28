//! The OpenAI-compatible client names the output-token cap the way each
//! provider expects: `max_completion_tokens` for OpenAI itself, `max_tokens`
//! for other compatible servers.
use genesis_planner::llm::{LlmClient, LlmRequest, Message, OpenAiCompatClient};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::time::Duration;

/// Serve one valid chat completion and hand back the request body.
fn capture_once() -> (String, mpsc::Receiver<serde_json::Value>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut bytes = Vec::new();
        let mut chunk = [0u8; 4096];
        let header_end = loop {
            let n = stream.read(&mut chunk).unwrap();
            assert!(n > 0);
            bytes.extend_from_slice(&chunk[..n]);
            if let Some(p) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                break p + 4;
            }
        };
        let head = String::from_utf8_lossy(&bytes[..header_end]).to_lowercase();
        let length: usize = head
            .lines()
            .find_map(|l| l.strip_prefix("content-length:"))
            .map(|v| v.trim().parse().unwrap())
            .unwrap_or(0);
        while bytes.len() < header_end + length {
            let n = stream.read(&mut chunk).unwrap();
            assert!(n > 0);
            bytes.extend_from_slice(&chunk[..n]);
        }
        tx.send(serde_json::from_slice(&bytes[header_end..header_end + length]).unwrap())
            .unwrap();
        let body = r#"{"choices":[{"message":{"content":"{}"}}]}"#;
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        stream.write_all(response.as_bytes()).unwrap();
    });
    (url, rx)
}

async fn sent_body(provider: &str) -> serde_json::Value {
    let (url, rx) = capture_once();
    let request = LlmRequest {
        system: "Return JSON".into(),
        messages: vec![Message::user("evidence")],
        max_tokens: 77,
    };
    let reply = OpenAiCompatClient::new("k", "m", url, provider)
        .complete(&request)
        .await
        .unwrap();
    assert_eq!(reply, "{}");
    rx.recv_timeout(Duration::from_secs(5)).unwrap()
}

#[tokio::test]
async fn openai_uses_max_completion_tokens() {
    let body = sent_body("openai").await;
    assert_eq!(body["max_completion_tokens"], 77);
    assert!(body.get("max_tokens").is_none());
}

#[tokio::test]
async fn other_compatible_servers_keep_max_tokens() {
    for provider in ["gemini", "openai-compatible"] {
        let body = sent_body(provider).await;
        assert_eq!(body["max_tokens"], 77, "{provider}");
        assert!(body.get("max_completion_tokens").is_none(), "{provider}");
    }
}
