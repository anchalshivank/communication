use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Conn {

    async fn send(&mut self, data: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn receive(&mut self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>>;
    async fn close(&self);

}
