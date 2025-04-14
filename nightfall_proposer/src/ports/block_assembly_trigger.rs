use async_trait::async_trait;

#[async_trait]
pub trait BlockAssemblyTrigger {
    async fn await_trigger(&self);
}
