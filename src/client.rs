use crate::{message::Message, session::Session};
use anyhow::Result;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use tracing::{debug, error, info, trace, warn};

#[derive(Debug)]
pub struct Client {
    addr: String,
    pub pwd: String,
}

impl Client {
    #[tracing::instrument]
    pub fn new(addr: String, pwd: String) -> Self {
        Client { addr, pwd }
    }

    pub async fn new_session(
        &self,
        tx: broadcast::Sender<Arc<Mutex<Message>>>,
        signal: broadcast::Receiver<bool>,
    ) -> Result<Session> {
        let s = TcpStream::connect(self.addr.clone()).await?;

        let s = Session::new(s, tx, signal).await;

        Ok(s)
    }
}
