use crate::domain::notifications::NotificationPayload;
use async_trait::async_trait;

#[async_trait]
pub trait Notifier: Send + Sync {
    async fn notify(&self, payload: &NotificationPayload) -> Result<(), String>;
}
