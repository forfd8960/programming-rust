use std::error::Error;

pub mod utils;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromClient {
    Join { group_name: Arc<String> },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromServer {}
