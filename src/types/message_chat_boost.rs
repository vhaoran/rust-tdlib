#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageChatBoost {
    #[serde(default)]
    pub boost_count: i64,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageChatShared {
    #[serde(default)]
    chat_id: i64,
    #[serde(default)]
    button_id: i64,
}
