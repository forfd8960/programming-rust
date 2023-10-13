use async_std::{
    io::{self, prelude::BufReadExt, BufRead, WriteExt},
    stream::{Stream, StreamExt},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromClient {
    Join {
        group_name: Arc<String>,
    },
    Post {
        group_name: Arc<String>,
        message: Arc<String>,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromServer {
    Message {
        group_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}

pub async fn send_as_json<S, P>(outbound: &mut S, packet: &P) -> ChatResult<()>
where
    S: io::Write + Unpin,
    P: Serialize,
{
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');
    outbound.write_all(json.as_bytes()).await?;
    Ok(())
}

pub fn receive_as_json<S, P>(inbound: S) -> impl Stream<Item = ChatResult<P>>
where
    S: BufRead + Unpin,
    P: DeserializeOwned,
{
    inbound.lines().map(|line_res| -> ChatResult<P> {
        let line = line_res?;
        let parsed = serde_json::from_str::<P>(&line)?;
        Ok(parsed)
    })
}
