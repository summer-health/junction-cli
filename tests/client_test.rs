use junction::client::JunctionClient;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_sends_api_key_header() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/link/providers"))
        .and(header("x-vital-api-key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"providers": []})))
        .expect(1)
        .mount(&server)
        .await;

    let client = JunctionClient::from_parts(server.uri(), "test-key".into());
    let result: serde_json::Value = client.get_raw("/v2/link/providers").await.unwrap();
    assert_eq!(result, json!({"providers": []}));
}

#[tokio::test]
async fn get_json_parses_response() {
    let server = MockServer::start().await;

    let body = json!({
        "sleep": [{
            "id": "sleep-1",
            "user_id": "user-1",
            "duration": 28800,
            "source": { "provider": "oura" }
        }]
    });

    Mock::given(method("GET"))
        .and(path("/v2/summary/sleep/user-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .mount(&server)
        .await;

    let client = JunctionClient::from_parts(server.uri(), "key".into());
    let result: serde_json::Value = client
        .get_raw("/v2/summary/sleep/user-1?start_date=2024-01-01")
        .await
        .unwrap();
    assert!(result["sleep"].is_array());
}

#[tokio::test]
async fn post_json_sends_body() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v2/link/token"))
        .and(header("x-vital-api-key", "my-key"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"link_token": "tok_abc123"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = JunctionClient::from_parts(server.uri(), "my-key".into());
    let body = json!({"user_id": "user-1"});
    let result: serde_json::Value = client.post_json("/v2/link/token", &body).await.unwrap();
    assert_eq!(result["link_token"], "tok_abc123");
}

#[tokio::test]
async fn error_response_returns_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/summary/profile/bad-id"))
        .respond_with(
            ResponseTemplate::new(404).set_body_json(json!({"detail": "User not found"})),
        )
        .mount(&server)
        .await;

    let client = JunctionClient::from_parts(server.uri(), "key".into());
    let err = client.get_raw("/v2/summary/profile/bad-id").await.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("404"), "Error should contain status code: {msg}");
}

#[tokio::test]
async fn delete_sends_request() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v2/some/resource"))
        .and(header("x-vital-api-key", "del-key"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let client = JunctionClient::from_parts(server.uri(), "del-key".into());
    let resp = client.delete("/v2/some/resource").await.unwrap();
    assert_eq!(resp.status(), 204);
}

#[tokio::test]
async fn server_error_returns_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/link/providers"))
        .respond_with(
            ResponseTemplate::new(500)
                .set_body_json(json!({"detail": "Internal server error"})),
        )
        .mount(&server)
        .await;

    let client = JunctionClient::from_parts(server.uri(), "key".into());
    let err = client.get_raw("/v2/link/providers").await.unwrap_err();
    assert!(err.to_string().contains("500"));
}
