//! tests/health_check.rs

mod common;

#[tokio::test]
async fn health_check_works() {
    let base = common::spawn_app().await;

    let response = reqwest::get(format!("{base}/health_check"))
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.text().await.unwrap(), "OK");
}
