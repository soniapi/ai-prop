use std::process::{Command, Child};
use std::time::Duration;
use tokio::time::sleep;
use serde_json::{json, Value};

struct ServerGuard {
    child: Child,
}

impl Drop for ServerGuard {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

async fn start_server() -> ServerGuard {
    let child = Command::new(env!("CARGO_BIN_EXE_server"))
        .spawn()
        .expect("Failed to start server");

    // Wait for server to start
    sleep(Duration::from_secs(2)).await;

    ServerGuard { child }
}

#[tokio::test]
async fn test_rest_api_proportions() {
    let _guard = start_server().await;
    let client = reqwest::Client::new();

    // test calculate_proportions
    let res = client.post("http://127.0.0.1:3000/calculate_proportions")
        .json(&json!({
            "overall": { "m": 100.0, "n": 200.0 },
            "group1": { "m": 40.0, "n": 100.0 },
            "group2": { "m": 60.0, "n": 100.0 }
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(res.status(), 200);

    let body: Value = res.json().await.expect("Failed to parse JSON");
    assert_eq!(body["p_population"].as_f64().unwrap(), 0.5);
    assert_eq!(body["p1"].as_f64().unwrap(), 0.4);
    assert_eq!(body["p2"].as_f64().unwrap(), 0.6);

    // test calculate_pooled_estimate
    let res = client.post("http://127.0.0.1:3000/calculate_pooled_estimate")
        .json(&json!({
            "n1": 100.0,
            "n2": 100.0,
            "p1": 0.6,
            "p2": 0.4
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(res.status(), 200);
    let body: Value = res.json().await.expect("Failed to parse JSON");
    assert_eq!(body["p"].as_f64().unwrap(), 0.5);

    // test calculate_z_statistics
    let res = client.post("http://127.0.0.1:3000/calculate_z_statistics")
        .json(&json!({
            "n1": 100.0,
            "n2": 100.0,
            "p1": 0.6,
            "p2": 0.4,
            "pooled_estimate": 0.5
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(res.status(), 200);
    let body: Value = res.json().await.expect("Failed to parse JSON");
    let z = body["z"].as_f64().unwrap();
    assert!((z - 2.828427).abs() < 0.0001);
}
