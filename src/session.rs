use crate::message::FormatType;
use crate::message::Message;
use anyhow::Result;
use tokio::{
    io::{AsyncWriteExt, BufReader, BufWriter},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
};
use tracing::{error, info};

#[derive(Debug)]
pub struct Session {
    writer: BufWriter<OwnedWriteHalf>,
    reader: BufReader<OwnedReadHalf>,
}

impl Session {
    pub async fn new(stream: TcpStream) -> Self {
        let (r, w) = stream.into_split();
        let writer = BufWriter::new(w);
        let reader = BufReader::new(r);

        Session { writer, reader }
    }

    #[tracing::instrument]
    pub async fn send(&mut self, data: String) -> Result<()> {
        let b = format!("{}\n\n", data.trim());

        self.writer.write_all(b.as_bytes()).await?;
        self.writer.flush().await?;

        Ok(())
    }

    #[tracing::instrument]
    pub async fn get_message(&mut self) -> Result<Message> {
        let mut reader = &mut self.reader;
        Message::parse(&mut reader).await
    }

    #[tracing::instrument]
    pub async fn api(&mut self, cmd: String) -> Result<Message> {
        let cmd = format!("api {}", cmd);
        self.send(cmd).await?;
        let msg = self.get_message().await?;
        Ok(msg)
    }

    #[tracing::instrument]
    pub async fn bigapi(&mut self, cmd: String) -> Result<()> {
        let cmd = format!("bigapi {}", cmd);
        self.send(cmd).await?;
        Ok(())
    }

    #[tracing::instrument]
    pub async fn sendmsg(&mut self, msg: String) -> Result<()> {
        let cmd = format!("sendmsg {}", msg);
        self.send(cmd).await?;
        Ok(())
    }

    #[tracing::instrument]
    pub async fn set_event_format(&mut self, fmt: FormatType) -> Result<()> {
        let cmd = format!("events {} ALL", fmt);
        self.send(cmd).await?;

        let mut msg = self.get_message().await?;
        let s = msg.get_header("Reply-Text");
        if s.len() == 0 || !s.contains("+OK") {
            error!("Failed to set format type");
            return Err(anyhow::anyhow!("Failed to set format type"));
        }
        Ok(())
    }

    #[tracing::instrument]
    pub async fn event_filter(&mut self, filters: Vec<String>) -> Result<()> {
        let filter = filters.join(" ");
        let cmd = format!("filter Event-Name {}", filter);
        self.send(cmd).await?;
        info!("data send successfully");

        let mut msg = self.get_message().await?;
        let s = msg.get_header("Reply-Text");
        if s.len() == 0 || !s.contains("+OK") {
            error!("Failed to set filter");
            return Err(anyhow::anyhow!("Failed to set filter"));
        }
        info!("Filter set successfully");
        Ok(())
    }

    pub async fn get_uuid(&mut self) -> Result<String> {
        let cmd = "create_uuid".to_string();
        let mut msg = self.api(cmd).await?;
        let uuid = msg.get_header("body");
        Ok(uuid)
    }

    #[tracing::instrument]
    pub async fn auth(&mut self, pwd: String) -> Result<()> {
        let auth_command = format!("auth {}\n\n", pwd);
        self.send(auth_command).await?;

        let mut msg = self.get_message().await?;
        let s = msg.get_header("Reply-Text");
        if s.len() == 0 || s != "+OK accepted" {
            error!("Failed to authenticate");
            return Err(anyhow::anyhow!("Failed to authenticate"));
        }
        info!("Authenticated");
        Ok(())
    }
}
