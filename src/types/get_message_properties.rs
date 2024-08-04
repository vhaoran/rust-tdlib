use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageProperties {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat the message belongs to

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message to get

    #[serde(default)]
    message_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMessageProperties {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMessageProperties {}

impl GetMessageProperties {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetMessagePropertiesBuilder {
        let mut inner = GetMessageProperties::default();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.td_type = "getMessageProperties".to_string();

        GetMessagePropertiesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct GetMessagePropertiesBuilder {
    inner: GetMessageProperties,
}

#[deprecated]
pub type RTDGetMessagePropertiesBuilder = GetMessagePropertiesBuilder;

impl GetMessagePropertiesBuilder {
    pub fn build(&self) -> GetMessageProperties {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }
}

impl AsRef<GetMessageProperties> for GetMessageProperties {
    fn as_ref(&self) -> &GetMessageProperties {
        self
    }
}

impl AsRef<GetMessageProperties> for GetMessagePropertiesBuilder {
    fn as_ref(&self) -> &GetMessageProperties {
        &self.inner
    }
}
