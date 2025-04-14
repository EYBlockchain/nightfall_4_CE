use crate::{domain::notifications::NotificationPayload, ports::notifier::Notifier};
use async_trait::async_trait;
use reqwest::Client;

pub struct WebhookNotifier {
    pub endpoint: String,
    client: Client,
}

impl WebhookNotifier {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Notifier for WebhookNotifier {
    async fn notify(&self, payload: &NotificationPayload) -> Result<(), String> {
        let res = self
            .client
            .post(&self.endpoint)
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(format!("HTTP request failed with status: {}", res.status()))
        }
    }
}
