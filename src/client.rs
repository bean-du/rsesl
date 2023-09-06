use crate::event::{Event, EventHandler};
use crate::message::Message;
use crate::{connection::Connection, message::FormatType};
use anyhow::Result;
use tokio::net::TcpStream;
use tracing::{debug, error, info, trace, warn};

#[derive(Debug)]
pub struct Client {
    conn: Connection,
    addr: String,
    pwd: String,
    timeout: u64,
}

impl Client {
    #[tracing::instrument]
    pub async fn new_client(addr: String, pwd: String, timeout: u64) -> Result<Self> {
        let s = TcpStream::connect(addr.clone()).await?;

        let mut conn = Connection::new(s).await;

        let mut msg = conn.get_message().await?;

        let auth_str = msg.get_header("Content-Type");
        if auth_str != "auth/request" {
            error!("Unexpected AuthHeader");
            return Err(anyhow::anyhow!("Unexpected AuthHeader"));
        }

        let auth_command = format!("auth {}\n\n", pwd);
        conn.send(auth_command).await?;

        let mut msg = conn.get_message().await?;
        let s = msg.get_header("Reply-Text");
        if s.len() == 0 || s != "+OK accepted" {
            error!("Failed to authenticate");
            return Err(anyhow::anyhow!("Failed to authenticate"));
        }
        info!("Authenticated");

        let c = Client {
            conn,
            addr,
            pwd,
            timeout,
        };
        Ok(c)
    }

    #[tracing::instrument]
    pub async fn send(&mut self, data: String) -> Result<()> {
        self.conn.send(data).await?;
        Ok(())
    }

    #[tracing::instrument]
    pub async fn get_message(&mut self) -> Result<Message> {
        let msg = self.conn.get_message().await?;
        Ok(msg)
    }

    #[tracing::instrument]
    pub async fn api(&mut self, cmd: String) -> Result<Message> {
        let cmd = format!("api {}", cmd);
        self.conn.send(cmd).await?;
        let msg = self.conn.get_message().await?;
        Ok(msg)
    }

    #[tracing::instrument]
    pub async fn bigapi(&mut self, cmd: String) -> Result<()> {
        let cmd = format!("bigapi {}", cmd);
        self.conn.send(cmd).await?;
        Ok(())
    }

    #[tracing::instrument]
    pub async fn sendmsg(&mut self, msg: Message) -> Result<()> {
        let cmd = format!("sendmsg {}", msg.to_string());
        self.conn.send(cmd).await?;
        Ok(())
    }

    #[tracing::instrument]
    pub async fn set_event_format(&mut self, fmt: FormatType) -> Result<()> {
        let cmd = format!("events {} ALL", fmt);
        self.conn.send(cmd).await?;
        Ok(())
    }

    #[tracing::instrument]
    pub async fn reconnect(&mut self) -> Result<()> {
        let s = TcpStream::connect(self.addr.clone()).await?;

        let mut conn = Connection::new(s).await;

        let mut msg = conn.get_message().await?;

        let auth_str = msg.get_header("Content-Type");
        if auth_str != "auth/request" {
            return Err(anyhow::anyhow!("Unexpected AuthHeader"));
        }

        let auth_command = format!("auth {}\n\n", self.pwd);
        conn.send(auth_command).await?;

        let mut msg = conn.get_message().await?;
        let s = msg.get_header("Reply-Text");
        if s.len() == 0 || s != "+OK accepted" {
            println!("Failed to authenticate");
            return Err(anyhow::anyhow!("Failed to authenticate"));
        }
        println!("Authenticated");
        self.conn = conn;

        Ok(())
    }
}
