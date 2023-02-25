use alice_http::{ALICE_DID, BOB_DID};
use hyper::body::HttpBody;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let m = didcomm_rs::Message::new()
        .from(ALICE_DID)
        .to(&[BOB_DID])
        .body(json!(["hey Bob"]).to_string().as_str());

    let m = m.as_raw_json().unwrap();

    dbg!(&m);

    let client = hyper::Client::new();
    let req = hyper::Request::builder()
        .uri("http://localhost:3010")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(hyper::Body::from(m))?;

    let mut res = client.request(req).await?;

    let res_body = res.body_mut().data().await.unwrap().unwrap();
    let res_body = String::from_utf8_lossy(&res_body);
    println!("Response body: {}", res_body);

    Ok(())
}
