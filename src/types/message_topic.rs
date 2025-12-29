#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageTopic {
    #[doc(hidden)]
    _Default,
    #[serde(rename = "messageTopicDirectMessages")]
    MessageTopicDirectMessages(TopicDirectMessages),
    #[serde(rename = "messageTopicForum")]
    MessageTopicForum(TopicForum),
    #[serde(rename = "messageTopicSavedMessages")]
    MessageTopicSavedMessages(TopicSavedMessages),
    #[serde(rename = "messageTopicThread")]
    MessageTopicThread(TopicThreadMessage),
}

//-------------------------------------
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopicDirectMessages {
    #[serde(rename = "messageTopic")]
    /// Unique identifier of the topic.
    // int53 	direct_messages_chat_topic_id_
    pub direct_messages_chat_topic_id: i64,
}

//-------------------------------------
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopicForum {
    #[serde(default)]
    // int53 	forum_topic_id_
    /// Unique identifier of the forum topic; all messages in a non-forum supergroup chats belongs to the General topic.
    pub forum_topic_id: i64,
}

//-------------------------------------
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopicSavedMessages {
    #[serde(default)]
    /// int53 	saved_messages_topic_id_
    /// Unique identifier of the Saved Messages topic.
    pub saved_messages_topic_id: i64,
}

//-------------------------------------
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopicThreadMessage {
    #[serde(default)]
    pub message_thread_id: i64,
}
/*
69451382784

*/