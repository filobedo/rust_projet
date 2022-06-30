
use serde_json::{Value};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageResponse {
    Welcome(Value),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Value),
    RoundSummary(Value),
    EndOfGame(Value),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeResult {
    Ok,
    Error(Value),
    Err(Value)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: u32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f32,
}
