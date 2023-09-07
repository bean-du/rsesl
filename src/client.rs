use crate::session::Session;
use anyhow::Result;
use tokio::net::TcpStream;
use tracing::{debug, error, info, trace, warn};

#[derive(Debug)]
pub struct Client {
    addr: String,
    pwd: String,
}

impl Client {
    #[tracing::instrument]
    pub fn new(addr: String, pwd: String) -> Self {
        Client { addr, pwd }
    }

    pub async fn new_session(&self) -> Result<Session> {
        let s = TcpStream::connect(self.addr.clone()).await?;

        let mut conn = Session::new(s).await;

        let mut msg = conn.get_message().await?;

        let auth_str = msg.get_header("Content-Type");

        if auth_str != "auth/request" {
            error!("Unexpected AuthHeader");
            return Err(anyhow::anyhow!("Unexpected AuthHeader"));
        }

        conn.auth(self.pwd.clone()).await?;

        Ok(conn)
    }
}
