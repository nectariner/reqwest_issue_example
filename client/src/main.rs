use std::{net::IpAddr, time::Duration};

use reqwest::header::{ACCEPT, CONTENT_TYPE};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let local_addr = IpAddr::from([0, 0, 0, 0]);
    let client = reqwest::Client::builder()
        .local_address(local_addr)
        .pool_max_idle_per_host(0)
        .connect_timeout(Duration::new(999999, 0))
        .build()
        .unwrap();
    dbg!(&client);

    let res = client
        .get("http://0.0.0.0:8001/")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "*")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    info!(res);
}
