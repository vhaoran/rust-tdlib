use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Message {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message identifier; unique for the chat to which the message belongs
    #[serde(default)]
    id: i64,
    /// Identifier of the sender of the message
    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    sender_id: MessageSender,
    /// Chat identifier
    #[serde(default)]
    chat_id: i64,
    /// The sending state of the message; may be null
    sending_state: Option<MessageSendingState>,
    /// The scheduling state of the message; may be null
    scheduling_state: Option<MessageSchedulingState>,
    /// True, if the message is outgoing
    #[serde(default)]
    is_outgoing: bool,
    /// True, if the message is pinned
    #[serde(default)]
    is_pinned: bool,
    /// True, if the message can be edited. For live location and poll messages this fields shows whether editMessageLiveLocation or stopPoll can be used with this message by the application
    // #[serde(default)]
    // can_be_edited: bool,
    /// True, if the message can be forwarded
    // #[serde(default)]
    // can_be_forwarded: bool,
    /// True, if content of the message can be saved locally or copied
    // #[serde(default)]
    // can_be_saved: bool,
    /// True, if the message can be deleted only for the current user while other users will continue to see it
    // #[serde(default)]
    // can_be_deleted_only_for_self: bool,
    /// True, if the message can be deleted for all users
    // #[serde(default)]
    // can_be_deleted_for_all_users: bool,
    /// True, if the message statistics are available
    // #[serde(default)]
    // can_get_statistics: bool,
    /// True, if the message thread info is available
    // #[serde(default)]
    // can_get_message_thread: bool,
    /// True, if chat members already viewed the message can be received through getMessageViewers
    // #[serde(default)]
    // can_get_viewers: bool,
    /// True, if media timestamp links can be generated for media timestamp entities in the message text, caption or web page description
    // #[serde(default)]
    // can_get_media_timestamp_links: bool,
    /// True, if media timestamp entities refers to a media in this message as opposed to a media in the replied message
    #[serde(default)]
    has_timestamped_media: bool,
    /// True, if the message is a channel post. All messages to channels are channel posts, all other messages are not channel posts
    #[serde(default)]
    is_channel_post: bool,
    /// True, if the message contains an unread mention for the current user
    #[serde(default)]
    is_topic_message: bool,

    /// True, if the message contains an unread mention for the current user
    #[serde(default)]
    contains_unread_mention: bool,
    /// Point in time (Unix timestamp) when the message was sent
    #[serde(default)]
    date: i32,
    /// Point in time (Unix timestamp) when the message was last edited
    #[serde(default)]
    edit_date: i32,
    /// Information about the initial message sender; may be null
    forward_info: Option<MessageForwardInfo>,
    /// Information about interactions with the message; may be null
    interaction_info: Option<MessageInteractionInfo>,
    /// If non-zero, the identifier of the chat to which the replied message belongs; Currently, only messages in the Replies chat can have different reply_in_chat_id and chat_id
    #[serde(default)]
    reply_in_chat_id: i64,
    /// If non-zero, the identifier of the message this message is replying to; can be the identifier of a deleted message
    #[serde(default)]
    reply_to_message_id: i64,
    /// If non-zero, the identifier of the message thread the message belongs to; unique within the chat to which the message belongs
    #[serde(default)]
    message_thread_id: i64,
    /// For self-destructing messages, the message's TTL (Time To Live), in seconds; 0 if none. TDLib will send updateDeleteMessages or updateMessageContent once the TTL expires
    #[serde(default)]
    ttl: i32,
    /// Time left before the message expires, in seconds. If the TTL timer isn't started yet, equals to the value of the ttl field
    #[serde(default)]
    ttl_expires_in: f32,
    /// If non-zero, the user identifier of the bot through which this message was sent
    #[serde(default)]
    via_bot_user_id: i64,
    /// For channel posts and anonymous group messages, optional author signature
    #[serde(default)]
    author_signature: String,
    /// Unique identifier of an album this message belongs to. Only audios, documents, photos and videos can be grouped together in albums
    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    media_album_id: i64,
    /// If non-empty, contains a human-readable description of the reason why access to this message must be restricted
    #[serde(default)]
    restriction_reason: String,
    /// Content of the message
    #[serde(skip_serializing_if = "MessageContent::_is_default")]
    content: MessageContent,
    /// Reply markup for the message; may be null
    reply_markup: Option<ReplyMarkup>,

    //-----------add by whr--------------------------
    #[serde(default)]
    sender_boost_count: i64,
    // whr-add
    unread_reactions: Option<serde_json::Value>,
    #[serde(default)]
    auto_delete_in: f64,
    #[serde(default)]
    self_destruct_in: f64,
    #[serde(default)]
    sender_business_bot_user_id: i64,

    #[serde(default)]
    is_from_offline: bool,
    #[serde(default)]
    can_be_replied_in_another_chat: bool,
    #[serde(default)]
    can_get_read_date: bool,
    #[serde(default)]
    can_get_added_reactions: bool,
    #[serde(default)]
    reactions: Option<serde_json::Value>,
    #[serde(default)]
    can_report_reactions: bool,

    #[serde(default)]
    saved_messages_topic_id: i64,
}

impl RObject for Message {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Message {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageBuilder {
        let mut inner = Message::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn sender_id(&self) -> &MessageSender {
        &self.sender_id
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn sending_state(&self) -> &Option<MessageSendingState> {
        &self.sending_state
    }

    pub fn scheduling_state(&self) -> &Option<MessageSchedulingState> {
        &self.scheduling_state
    }

    pub fn is_outgoing(&self) -> bool {
        self.is_outgoing
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }

    // pub fn can_be_edited(&self) -> bool {
    //     self.can_be_edited
    // }
    //
    // pub fn can_be_forwarded(&self) -> bool {
    //     self.can_be_forwarded
    // }
    //
    // pub fn can_be_saved(&self) -> bool {
    //     self.can_be_saved
    // }
    //
    // pub fn can_be_deleted_only_for_self(&self) -> bool {
    //     self.can_be_deleted_only_for_self
    // }
    //
    // pub fn can_be_deleted_for_all_users(&self) -> bool {
    //     self.can_be_deleted_for_all_users
    // }
    //
    // pub fn can_get_statistics(&self) -> bool {
    //     self.can_get_statistics
    // }
    //
    // pub fn can_get_message_thread(&self) -> bool {
    //     self.can_get_message_thread
    // }

    // pub fn can_get_viewers(&self) -> bool {
    //     self.can_get_viewers
    // }

    // pub fn can_get_media_timestamp_links(&self) -> bool {
    //     self.can_get_media_timestamp_links
    // }

    pub fn has_timestamped_media(&self) -> bool {
        self.has_timestamped_media
    }

    pub fn is_channel_post(&self) -> bool {
        self.is_channel_post
    }

    pub fn contains_unread_mention(&self) -> bool {
        self.contains_unread_mention
    }

    pub fn date(&self) -> i32 {
        self.date
    }

    pub fn edit_date(&self) -> i32 {
        self.edit_date
    }

    pub fn forward_info(&self) -> &Option<MessageForwardInfo> {
        &self.forward_info
    }

    pub fn interaction_info(&self) -> &Option<MessageInteractionInfo> {
        &self.interaction_info
    }

    pub fn reply_in_chat_id(&self) -> i64 {
        self.reply_in_chat_id
    }

    pub fn reply_to_message_id(&self) -> i64 {
        self.reply_to_message_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }

    pub fn ttl(&self) -> i32 {
        self.ttl
    }

    pub fn ttl_expires_in(&self) -> f32 {
        self.ttl_expires_in
    }

    pub fn via_bot_user_id(&self) -> i64 {
        self.via_bot_user_id
    }

    pub fn author_signature(&self) -> &String {
        &self.author_signature
    }

    pub fn media_album_id(&self) -> i64 {
        self.media_album_id
    }

    pub fn restriction_reason(&self) -> &String {
        &self.restriction_reason
    }

    pub fn content(&self) -> &MessageContent {
        &self.content
    }

    pub fn reply_markup(&self) -> &Option<ReplyMarkup> {
        &self.reply_markup
    }
}

#[doc(hidden)]
pub struct MessageBuilder {
    inner: Message,
}

#[deprecated]
pub type RTDMessageBuilder = MessageBuilder;

impl MessageBuilder {
    pub fn build(&self) -> Message {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
        self.inner.sender_id = sender_id.as_ref().clone();
        self
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn sending_state<T: AsRef<MessageSendingState>>(&mut self, sending_state: T) -> &mut Self {
        self.inner.sending_state = Some(sending_state.as_ref().clone());
        self
    }

    pub fn scheduling_state<T: AsRef<MessageSchedulingState>>(
        &mut self,
        scheduling_state: T,
    ) -> &mut Self {
        self.inner.scheduling_state = Some(scheduling_state.as_ref().clone());
        self
    }

    pub fn is_outgoing(&mut self, is_outgoing: bool) -> &mut Self {
        self.inner.is_outgoing = is_outgoing;
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }

    // pub fn can_be_edited(&mut self, can_be_edited: bool) -> &mut Self {
    //     self.inner.can_be_edited = can_be_edited;
    //     self
    // }
    //
    // pub fn can_be_forwarded(&mut self, can_be_forwarded: bool) -> &mut Self {
    //     self.inner.can_be_forwarded = can_be_forwarded;
    //     self
    // }

    // pub fn can_be_saved(&mut self, can_be_saved: bool) -> &mut Self {
    //     self.inner.can_be_saved = can_be_saved;
    //     self
    // }
    //
    // pub fn can_be_deleted_only_for_self(
    //     &mut self,
    //     can_be_deleted_only_for_self: bool,
    // ) -> &mut Self {
    //     self.inner.can_be_deleted_only_for_self = can_be_deleted_only_for_self;
    //     self
    // }
    //
    // pub fn can_be_deleted_for_all_users(
    //     &mut self,
    //     can_be_deleted_for_all_users: bool,
    // ) -> &mut Self {
    //     self.inner.can_be_deleted_for_all_users = can_be_deleted_for_all_users;
    //     self
    // }
    //
    // pub fn can_get_statistics(&mut self, can_get_statistics: bool) -> &mut Self {
    //     self.inner.can_get_statistics = can_get_statistics;
    //     self
    // }

    // pub fn can_get_message_thread(&mut self, can_get_message_thread: bool) -> &mut Self {
    //     self.inner.can_get_message_thread = can_get_message_thread;
    //     self
    // }
    //
    // pub fn can_get_viewers(&mut self, can_get_viewers: bool) -> &mut Self {
    //     self.inner.can_get_viewers = can_get_viewers;
    //     self
    // }
    //
    // pub fn can_get_media_timestamp_links(
    //     &mut self,
    //     can_get_media_timestamp_links: bool,
    // ) -> &mut Self {
    //     self.inner.can_get_media_timestamp_links = can_get_media_timestamp_links;
    //     self
    // }

    pub fn has_timestamped_media(&mut self, has_timestamped_media: bool) -> &mut Self {
        self.inner.has_timestamped_media = has_timestamped_media;
        self
    }

    pub fn is_channel_post(&mut self, is_channel_post: bool) -> &mut Self {
        self.inner.is_channel_post = is_channel_post;
        self
    }

    pub fn contains_unread_mention(&mut self, contains_unread_mention: bool) -> &mut Self {
        self.inner.contains_unread_mention = contains_unread_mention;
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }

    pub fn edit_date(&mut self, edit_date: i32) -> &mut Self {
        self.inner.edit_date = edit_date;
        self
    }

    pub fn forward_info<T: AsRef<MessageForwardInfo>>(&mut self, forward_info: T) -> &mut Self {
        self.inner.forward_info = Some(forward_info.as_ref().clone());
        self
    }

    pub fn interaction_info<T: AsRef<MessageInteractionInfo>>(
        &mut self,
        interaction_info: T,
    ) -> &mut Self {
        self.inner.interaction_info = Some(interaction_info.as_ref().clone());
        self
    }

    pub fn reply_in_chat_id(&mut self, reply_in_chat_id: i64) -> &mut Self {
        self.inner.reply_in_chat_id = reply_in_chat_id;
        self
    }

    pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
        self.inner.reply_to_message_id = reply_to_message_id;
        self
    }

    pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
        self.inner.message_thread_id = message_thread_id;
        self
    }

    pub fn ttl(&mut self, ttl: i32) -> &mut Self {
        self.inner.ttl = ttl;
        self
    }

    pub fn ttl_expires_in(&mut self, ttl_expires_in: f32) -> &mut Self {
        self.inner.ttl_expires_in = ttl_expires_in;
        self
    }

    pub fn via_bot_user_id(&mut self, via_bot_user_id: i64) -> &mut Self {
        self.inner.via_bot_user_id = via_bot_user_id;
        self
    }

    pub fn author_signature<T: AsRef<str>>(&mut self, author_signature: T) -> &mut Self {
        self.inner.author_signature = author_signature.as_ref().to_string();
        self
    }

    pub fn media_album_id(&mut self, media_album_id: i64) -> &mut Self {
        self.inner.media_album_id = media_album_id;
        self
    }

    pub fn restriction_reason<T: AsRef<str>>(&mut self, restriction_reason: T) -> &mut Self {
        self.inner.restriction_reason = restriction_reason.as_ref().to_string();
        self
    }

    pub fn content<T: AsRef<MessageContent>>(&mut self, content: T) -> &mut Self {
        self.inner.content = content.as_ref().clone();
        self
    }

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = Some(reply_markup.as_ref().clone());
        self
    }
}

impl AsRef<Message> for Message {
    fn as_ref(&self) -> &Message {
        self
    }
}

impl AsRef<Message> for MessageBuilder {
    fn as_ref(&self) -> &Message {
        &self.inner
    }
}
