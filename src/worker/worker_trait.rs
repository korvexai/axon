use crate::event::bus::EventSender;
use crate::event::event::WorkerHealth;
use async_trait::async_trait;

#[async_trait]
pub trait Worker: Send + Sync {
    fn name(&self) -> &'static str;
    async fn start(&self, tx: EventSender) -> anyhow::Result<()>;
    async fn stop(&self);
    async fn health(&self) -> WorkerHealth;
}
