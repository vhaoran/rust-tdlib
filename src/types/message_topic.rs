#[derive(Debug, Clone, Copy,Deserialize, Serialize)]
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
    MessageTopicThread(TopicThread),
}
impl MessageTopic {
    pub fn builder() -> MessageTopicBuilder {
        MessageTopicBuilder::default()
    }
    pub fn id(&self) -> i64 {
        match self {
            MessageTopic::MessageTopicDirectMessages(TopicDirectMessages {
                direct_messages_chat_topic_id,
            }) => direct_messages_chat_topic_id.to_owned(),
            MessageTopic::MessageTopicForum(TopicForum { forum_topic_id }) => {
                forum_topic_id.to_owned()
            }
            MessageTopic::MessageTopicSavedMessages(TopicSavedMessages {
                saved_messages_topic_id,
            }) => saved_messages_topic_id.to_owned(),
            MessageTopic::MessageTopicThread(TopicThread { message_thread_id }) => {
                message_thread_id.to_owned()
            }
            _ => 0,
        }
    }
}

impl AsRef<MessageTopic> for MessageTopic {
    fn as_ref(&self) -> &MessageTopic {
        self
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct MessageTopicBuilder {
    inner: MessageTopic,
}

impl MessageTopicBuilder {
    pub fn default() -> Self {
        Self {
            inner: MessageTopic::_Default,
        }
    }
    pub fn new_of_direct_messages(thread_id: i64) -> Self {
        Self {
            inner: MessageTopic::MessageTopicDirectMessages(TopicDirectMessages {
                direct_messages_chat_topic_id: thread_id,
            }),
        }
    }

    pub fn new_of_forum(forum_topic_id: i64) -> Self {
        Self {
            inner: MessageTopic::MessageTopicForum(TopicForum { forum_topic_id }),
        }
    }

    pub fn new_of_saved_messages(saved_messages_topic_id: i64) -> Self {
        Self {
            inner: MessageTopic::MessageTopicSavedMessages(TopicSavedMessages {
                saved_messages_topic_id,
            }),
        }
    }

    pub fn new_of_topic_thread(thread_id: i64) -> Self {
        Self {
            inner: MessageTopic::MessageTopicThread(TopicThread {
                message_thread_id: thread_id,
            }),
        }
    }

    pub fn build(&self) -> MessageTopic {
        self.inner.clone()
    }
    pub fn message_topic_direct_messages(
        &mut self,
        message_topic_direct_messages: TopicDirectMessages,
    ) -> &mut Self {
        self.inner = MessageTopic::MessageTopicDirectMessages(message_topic_direct_messages);
        self
    }
    pub fn message_topic_forum(&mut self, message_topic_forum: TopicForum) -> &mut Self {
        self.inner = MessageTopic::MessageTopicForum(message_topic_forum);
        self
    }
    pub fn message_topic_saved_messages(
        &mut self,
        message_topic_saved_messages: TopicSavedMessages,
    ) -> &mut Self {
        self.inner = MessageTopic::MessageTopicSavedMessages(message_topic_saved_messages);
        self
    }
    pub fn message_topic_thread(&mut self, message_topic_thread: TopicThread) -> &mut Self {
        self.inner = MessageTopic::MessageTopicThread(message_topic_thread);
        self
    }
}

impl AsRef<MessageTopic> for MessageTopicBuilder {
    fn as_ref(&self) -> &MessageTopic {
        &self.inner
    }
}

//-------------------------------------
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct TopicDirectMessages {
    #[serde(rename = "messageTopic")]
    /// Unique identifier of the topic.
    // int53 	direct_messages_chat_topic_id_
    pub direct_messages_chat_topic_id: i64,
}
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct TopicDirectMessagesBuilder {
    inner: TopicDirectMessages,
}
impl TopicDirectMessagesBuilder {
    pub fn build(&self) -> TopicDirectMessages {
        self.inner.clone()
    }

    pub fn direct_messages_chat_topic_id(
        &mut self,
        direct_messages_chat_topic_id: i64,
    ) -> &mut Self {
        self.inner.direct_messages_chat_topic_id = direct_messages_chat_topic_id;
        self
    }
}

//-------------------------------------
#[derive(Debug, Clone, Copy,  Deserialize, Default, Serialize)]
pub struct TopicForum {
    #[serde(default)]
    // int53 	forum_topic_id_
    /// Unique identifier of the forum topic; all messages in a non-forum supergroup chats belongs to the General topic.
    pub forum_topic_id: i64,
}
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct TopicForumBuilder {
    inner: TopicForum,
}
impl TopicForumBuilder {
    pub fn build(&self) -> TopicForum {
        self.inner.clone()
    }

    pub fn forum_topic_id(&mut self, forum_topic_id: i64) -> &mut Self {
        self.inner.forum_topic_id = forum_topic_id;
        self
    }
}

//-------------------------------------
#[derive(Debug, Clone, Copy,  Deserialize, Default, Serialize)]
pub struct TopicSavedMessages {
    #[serde(default)]
    /// int53 	saved_messages_topic_id_
    /// Unique identifier of the Saved Messages topic.
    pub saved_messages_topic_id: i64,
}
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct TopicSavedMessagesBuilder {
    inner: TopicSavedMessages,
}
impl TopicSavedMessagesBuilder {
    pub fn build(&self) -> TopicSavedMessages {
        self.inner.clone()
    }

    pub fn saved_messages_topic_id(&mut self, saved_messages_topic_id: i64) -> &mut Self {
        self.inner.saved_messages_topic_id = saved_messages_topic_id;
        self
    }
}

//-------------------------------------
#[derive(Debug, Clone, Copy,  Deserialize, Default, Serialize)]
pub struct TopicThread {
    #[serde(default)]
    pub message_thread_id: i64,
}

//-------------------------------------
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct TopicThreadBuilder {
    inner: TopicThread,
}
impl TopicThreadBuilder {
    pub fn build(&self) -> TopicThread {
        self.inner.clone()
    }

    pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
        self.inner.message_thread_id = message_thread_id;
        self
    }
}
