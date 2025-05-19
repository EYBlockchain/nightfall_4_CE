use futures::future::join_all;

use crate::{domain::notifications::NotificationPayload, ports::notifier::Notifier};

pub struct DataPublisher {
    notifiers: Vec<Box<dyn Notifier>>,
}

impl DataPublisher {
    pub fn new() -> Self {
        Self {
            notifiers: Vec::new(),
        }
    }

    pub fn register_notifier(&mut self, notifier: Box<dyn Notifier>) {
        self.notifiers.push(notifier);
    }

    pub async fn publish(&self, notification: NotificationPayload) {
        let futures = self
            .notifiers
            .iter()
            .map(|notifier| notifier.notify(&notification));
        let results = join_all(futures).await;

        for result in results {
            match result {
                Ok(_) => log::debug!("Notification sent successfully"),
                Err(e) => log::warn!("Failed to send notification: {}", e),
            }
        }
    }
}

impl Default for DataPublisher {
    fn default() -> Self {
        Self::new()
    }
}
