use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a forwarded message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageForwardInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Origin of a forwarded message

    // #[serde(skip_serializing_if = "MessageForwardOrigin::_is_default")]
    // origin: MessageForwardOrigin,
    // add by whr
    #[serde(rename = "origin")]
    origin: MessageForwardOrigin,

    #[serde(default)]
    source: ForwardSource,

    /// Point in time (Unix timestamp) when the message was originally sent
    #[serde(default)]
    date: i32,
    /// The type of a public service announcement for the forwarded message

    #[serde(default)]
    public_service_announcement_type: String,
}

impl RObject for MessageForwardInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageForwardInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageForwardInfoBuilder {
        let mut inner = MessageForwardInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageForwardInfoBuilder { inner }
    }

    pub fn origin(&self) -> &MessageForwardOrigin {
        &self.origin
    }

    pub fn date(&self) -> i32 {
        self.date
    }

    pub fn public_service_announcement_type(&self) -> &String {
        &self.public_service_announcement_type
    }

    pub fn chat_id(&self) -> i64 {
        self.source.chat_id()
    }

    pub fn message_id(&self) -> i64 {
        self.source.message_id()
    }
}

#[doc(hidden)]
pub struct MessageForwardInfoBuilder {
    inner: MessageForwardInfo,
}

#[deprecated]
pub type RTDMessageForwardInfoBuilder = MessageForwardInfoBuilder;

impl MessageForwardInfoBuilder {
    pub fn build(&self) -> MessageForwardInfo {
        self.inner.clone()
    }

    pub fn origin<T: AsRef<MessageForwardOrigin>>(&mut self, origin: T) -> &mut Self {
        self.inner.origin = origin.as_ref().clone();
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }

    pub fn public_service_announcement_type<T: AsRef<str>>(
        &mut self,
        public_service_announcement_type: T,
    ) -> &mut Self {
        self.inner.public_service_announcement_type =
            public_service_announcement_type.as_ref().to_string();
        self
    }
}

impl AsRef<MessageForwardInfo> for MessageForwardInfo {
    fn as_ref(&self) -> &MessageForwardInfo {
        self
    }
}

impl AsRef<MessageForwardInfo> for MessageForwardInfoBuilder {
    fn as_ref(&self) -> &MessageForwardInfo {
        &self.inner
    }
}

//-------------------------------------
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForwardSource {
    // "chat_id" :-1002089733533 ,
    #[serde(default)]
    chat_id: i64,
    // "message_id" :10989076480 ,
    #[serde(default)]
    message_id: i64,

    /// #[serde(default)]
    sender_id: Option<MessageSender>,
    #[serde(default)]
    sender_name: String,
    #[serde(default)]
    date: i32,
    #[serde(default)]
    is_outgoing: bool,
}

impl ForwardSource {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
    pub fn message_id(&self) -> i64 {
        self.message_id
    }
    pub fn sender_name(&self) -> &String {
        &self.sender_name
    }
    pub fn date(&self) -> i32 {
        self.date
    }
    pub fn is_outgoing(&self) -> bool {
        self.is_outgoing
    }
}
