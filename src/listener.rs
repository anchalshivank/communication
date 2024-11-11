use async_trait::async_trait;
use std::error::Error;
use crate::conn::Conn;

#[async_trait]
pub trait Listener {

    async fn accept(&self) -> Result<Box<dyn Conn + Send + Sync>, Box<dyn Error + Send + Sync>>;

}
