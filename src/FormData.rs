
use serde::de::Unexpected::Str;
use serde_json::{Result, Value};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageResponse {
    Welcome(Value),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Value),
    RoundSummary(Value),
    EndOfGame(Value),
    None
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeResult {
    Ok,
    Error(Value)
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
