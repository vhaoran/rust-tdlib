use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetForumTopic {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(default)]
    chat_id: i64,
    #[serde(default)]
    forum_topic_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetForumTopic {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetForumTopic {}

impl GetForumTopic {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetForumTopicBuilder {
        let mut inner = GetForumTopic::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getForumTopic".to_string();

        GetForumTopicBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
    pub fn forum_topic_id(&self) -> i64 {
        self.forum_topic_id
    }
}

#[doc(hidden)]
pub struct GetForumTopicBuilder {
    inner: GetForumTopic,
}

#[deprecated]
pub type RTDGetForumTopicBuilder = GetForumTopicBuilder;

impl GetForumTopicBuilder {
    pub fn build(&self) -> GetForumTopic {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
    // pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
    //     self.inner.message_thread_id = message_thread_id;
    //     self
    // }

    pub fn forum_topic_id(&mut self, forum_topic_id: i64) -> &mut Self {
        self.inner.forum_topic_id = forum_topic_id;
        self
    }
}

impl AsRef<GetForumTopic> for GetForumTopic {
    fn as_ref(&self) -> &GetForumTopic {
        self
    }
}

impl AsRef<GetForumTopic> for GetForumTopicBuilder {
    fn as_ref(&self) -> &GetForumTopic {
        &self.inner
    }
}
