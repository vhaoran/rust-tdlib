use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Creates a new invite link for a chat. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right in the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateForumTopic {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    #[serde(default)]
    chat_id: i64,
    /// name
    #[serde(default)]
    name: String,
    #[serde(default)]
    icon: Option<ForumTopicIcon>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CreateForumTopic {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreateForumTopic {}

impl CreateForumTopic {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CreateForumTopicBuilder {
        let mut inner = CreateForumTopic::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createForumTopic".to_string();

        CreateForumTopicBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn icon(&self) -> &Option<ForumTopicIcon> {
        &self.icon
    }
}

#[doc(hidden)]
pub struct CreateForumTopicBuilder {
    inner: CreateForumTopic,
}

#[deprecated]
pub type RTDCreateForumTopicBuilder = CreateForumTopicBuilder;

impl CreateForumTopicBuilder {
    pub fn build(&self) -> CreateForumTopic {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn icon(&mut self, icon: ForumTopicIcon) -> &mut Self {
        self.inner.icon = Some(icon);
        self
    }
}

impl AsRef<CreateForumTopic> for CreateForumTopic {
    fn as_ref(&self) -> &CreateForumTopic {
        self
    }
}

impl AsRef<CreateForumTopic> for CreateForumTopicBuilder {
    fn as_ref(&self) -> &CreateForumTopic {
        &self.inner
    }
}
/*
MessagesManager.cpp:20731
*/