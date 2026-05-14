#[path = "../src/continuum/webhooks.rs"]
mod webhooks;

#[tokio::test]
async fn webhook_emits_signed_payload() {
    let mut server = mockito::Server::new_async().await;
    let m = server
        .mock("POST", "/internal/vault-events")
        .match_header(
            "x-continuum-signature",
            mockito::Matcher::Regex("sha256=.+".into()),
        )
        .with_status(204)
        .create_async()
        .await;

    let emitter = webhooks::WebhookEmitter::new(
        format!("{}/internal/vault-events", server.url()),
        "test-secret".to_string(),
    );

    emitter
        .emit_emergency_access_changed("ea-uuid", "Invited", "Accepted")
        .await
        .unwrap();

    m.assert_async().await;
}
