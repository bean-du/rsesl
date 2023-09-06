use crate::event::EventData;
use anyhow::{Error, Result};
use serde_json::{Map, Value};
use std::{collections::HashMap, fmt::Display, str::FromStr};
use thiserror;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    net::tcp::OwnedReadHalf,
};
use tracing::{debug, error, warn};
use urlencoding::decode;

type MIMEHeader = HashMap<String, Vec<String>>;

#[derive(thiserror::Error, Debug)]
pub enum MsgError {
    /// Not enough data is available to parse a message
    #[error("No data to parse")]
    Incomplete,

    /// Invalid message encoding
    #[error(transparent)]
    Other(Error),
}

#[derive(Debug, PartialEq)]
pub enum FormatType {
    Xml,
    Json,
    Plain,
}

impl FromStr for FormatType {
    type Err = MsgError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "xml" => Ok(FormatType::Xml),
            "json" => Ok(FormatType::Json),
            "plain" => Ok(FormatType::Plain),
            _ => Err(MsgError::Other(anyhow::anyhow!("Invalid FormatType"))),
        }
    }
}

impl Display for FormatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatType::Xml => write!(f, "xml"),
            FormatType::Json => write!(f, "json"),
            FormatType::Plain => write!(f, "plain"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ContentType {
    TextEventJson,
    TextDisconnectNotice,
    CommandReply,
    ApiResponse,
    TextEventPlain,
    AuthRequest,
}

impl FromStr for ContentType {
    type Err = MsgError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text/event-json" => Ok(ContentType::TextEventJson),
            "text/disconnect-notice" => Ok(ContentType::TextDisconnectNotice),
            "command/reply" => Ok(ContentType::CommandReply),
            "api/response" => Ok(ContentType::ApiResponse),
            "text/event-plain" => Ok(ContentType::TextEventPlain),
            "auth/request" => Ok(ContentType::AuthRequest),
            _ => Err(MsgError::Other(anyhow::anyhow!("Invalid ContentType"))),
        }
    }
}

impl Into<String> for ContentType {
    fn into(self) -> String {
        match self {
            ContentType::TextEventJson => "text/event-json".to_string(),
            ContentType::TextDisconnectNotice => "text/disconnect-notice".to_string(),
            ContentType::CommandReply => "command/reply".to_string(),
            ContentType::ApiResponse => "api/response".to_string(),
            ContentType::TextEventPlain => "text/event-plain".to_string(),
            ContentType::AuthRequest => "auth/request".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Message {
    pub header: Option<HashMap<String, String>>,
    pub event_data: Option<EventData>,
}

impl Message {
    pub fn new(headers: Option<HashMap<String, String>>, data: EventData) -> Self {
        Message {
            header: headers,
            event_data: Some(data),
        }
    }
    // parse text protocol message
    #[tracing::instrument]
    pub async fn parse(r: &mut BufReader<OwnedReadHalf>) -> Result<Message> {
        if r.buffer().len() == 0 {
            r.fill_buf().await?;
        }

        let mut header = HashMap::new();
        let mut content_length = None;
        let mut event_data: Option<EventData> = None;

        loop {
            let mut line = String::new();

            match r.read_line(&mut line).await {
                Ok(n) if n == 0 => {
                    error!("connection closed");
                    return Err(anyhow::anyhow!("connection closed"));
                }
                Ok(_) => {
                    if line.trim().is_empty() {
                        break; // end of headers
                    }
                    if let Some((field, value)) = parse_header_line(&line) {
                        header.insert(field.to_string(), value.to_string());
                    }
                }
                Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
                    break;
                }
                Err(e) => {
                    error!("failed to read from socket; err = {:?}", e);
                    return Err(anyhow::anyhow!("failed to read from socket"));
                }
            };
        }

        // parse Content-Length
        if let Some(content_length_str) = header.get("Content-Length") {
            if let Ok(length) = content_length_str.parse::<usize>() {
                content_length = Some(length);
            }
        }

        // read body
        let mut body = None;
        if let Some(length) = content_length {
            let mut content = vec![0u8; length];
            r.read_exact(&mut content).await.unwrap();
            body = Some(String::from_utf8_lossy(&content).to_string());
        }

        // get Content-Type
        if let Some(content_type) = header.get("Content-Type") {
            let msg_type = match ContentType::from_str(&content_type) {
                Ok(t) => {
                    // debug!("Content-Type: {:?}", t);
                    t
                }
                Err(e) => {
                    error!("failed to parse Content-Type; err = {:?}", e);
                    return Err(anyhow::anyhow!("Unsupported Content-Type"));
                }
            };
            // decode if value is url encoded and Content-Type is not text/event-json
            if msg_type != ContentType::TextEventJson {
                if let Some(data) = body {
                    let s: String = decode(&data)?.to_string();
                    body = Some(s);
                }
            }

            match msg_type {
                ContentType::TextEventJson => {
                    if let Some(body) = &body {
                        // parse body
                        match serde_json::from_str::<EventData>(&body) {
                            Ok(ed) => {
                                event_data = Some(ed);
                            }
                            Err(e) => {
                                error!("failed to parse body; err = {:?}", e);
                                return Err(anyhow::anyhow!("failed to parse body"));
                            }
                        }
                    }
                }
                ContentType::ApiResponse => {
                    if let Some(reply_text) = header.get("Reply-Text") {
                        if reply_text.contains("-ERR") {
                            return Err(anyhow::anyhow!("Received error response"));
                        }
                    }
                }
                ContentType::CommandReply => {
                    if let Some(body) = &body {
                        if body.contains("-ERR") {
                            error!("Received error response");
                            return Err(anyhow::anyhow!("Received error response"));
                        }
                    }
                }
                ContentType::TextDisconnectNotice => {
                    for (k, v) in header {
                        debug!("Message Header {}: {}", k, v);
                    }
                    error!("Received disconnect notice");
                    return Err(anyhow::anyhow!("Received disconnect notice"));
                }
                ContentType::TextEventPlain => {
                    if let Some(body) = &body {
                        let mut ed: EventData = HashMap::new();

                        body.split("\n").for_each(|line| {
                            if let Some((k, v)) = parse_header_line(line) {
                                ed.insert(k.to_string(), Value::String(v.to_string()));
                            }
                        });

                        event_data = Some(ed)
                    }
                }
                _ => {}
            }
        }

        Ok(Message {
            header: Some(header),
            event_data,
        })
    }

    #[tracing::instrument]
    pub fn get_header(&mut self, k: &str) -> String {
        let mut result = String::new();

        if let Some(h) = &self.header {
            if let Some(v) = h.get(k) {
                result = v.to_string();
            }
        }
        result
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        if let Some(h) = &self.header {
            result = generate_headers(h);
        }
        if let Some(ed) = &self.event_data {
            result.push_str(&generate_content(&serde_json::to_string(&ed).unwrap()));
        }
        write!(f, "{}", result)
    }
}

fn parse_header_line(line: &str) -> Option<(&str, &str)> {
    if let Some(pos) = line.find(':') {
        let field = line[..pos].trim();
        let value = line[pos + 1..].trim();
        Some((field, value))
    } else {
        None
    }
}

fn generate_headers(headers: &HashMap<String, String>) -> String {
    headers
        .iter()
        .map(|(field, value)| format!("{}: {}\n", field, value))
        .collect()
}

fn generate_content_length(content: &str) -> String {
    format!("Content-Length: {}\n", content.len())
}

fn generate_content(content: &str) -> String {
    format!("{}\n", content)
}
