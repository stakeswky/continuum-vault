use data_encoding::HEXLOWER;
use hmac::{Hmac, Mac};
use serde_json::json;
use sha2::Sha256;
use std::time::Duration;

type HmacSha256 = Hmac<Sha256>;

pub struct WebhookEmitter {
    target_url: String,
    secret: String,
    client: reqwest::Client,
}

impl WebhookEmitter {
    pub fn new(target_url: String, secret: String) -> Self {
        Self {
            target_url,
            secret,
            client: reqwest::Client::new(),
        }
    }

    pub async fn emit_emergency_access_changed(
        &self,
        ea_uuid: &str,
        old: &str,
        new: &str,
    ) -> Result<(), reqwest::Error> {
        let body = json!({
            "event": "emergency_access.status_changed",
            "ea_uuid": ea_uuid,
            "old_status": old,
            "new_status": new,
            "occurred_at": chrono::Utc::now().to_rfc3339(),
            "nonce": uuid::Uuid::new_v4().to_string(),
        })
        .to_string();

        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .expect("HMAC accepts keys of any size");
        mac.update(body.as_bytes());
        let sig = HEXLOWER.encode(&mac.finalize().into_bytes());

        self.client
            .post(&self.target_url)
            .header("x-continuum-signature", format!("sha256={sig}"))
            .header("content-type", "application/json")
            .body(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

pub async fn poll_and_emit(emitter: WebhookEmitter) {
    let mut interval = tokio::time::interval(Duration::from_secs(10));
    let _emitter = emitter;
    let _emit = WebhookEmitter::emit_emergency_access_changed;

    loop {
        interval.tick().await;
    }
}
