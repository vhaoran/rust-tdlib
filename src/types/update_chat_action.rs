use crate::types::*;
// use super::RFunction;
// use super::RObject;
use crate::errors::*;
// use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
// #[serde(tag = "@type")]
pub struct UpdateChatAction {
    // #[doc(hidden)]
    // #[serde(rename(serialize = "@type", deserialize = "@type"))]
    // td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    /// Chat identifier
    chat_id: i64,
    /// If not 0, a message thread identifier in which the action was performed
    message_thread_id: i64,
    /// Identifier of a message sender performing the action
    sender_id: MessageSender,
    /// The action
    action: ChatAction,
}

impl RObject for UpdateChatAction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for UpdateChatAction {}

impl UpdateChatAction {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatActionBuilder {
        let mut inner = UpdateChatAction::default();
        // inner.td_name = "updateChatAction".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDUpdateChatActionBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }

    pub fn sender_id(&self) -> &MessageSender {
        &self.sender_id
    }

    pub fn action(&self) -> &ChatAction {
        &self.action
    }
}

#[doc(hidden)]
pub struct RTDUpdateChatActionBuilder {
    inner: UpdateChatAction,
}

impl RTDUpdateChatActionBuilder {
    pub fn build(&self) -> UpdateChatAction {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
        self.inner.message_thread_id = message_thread_id;
        self
    }

    pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
        self.inner.sender_id = sender_id.as_ref().clone();
        self
    }

    pub fn action<T: AsRef<ChatAction>>(&mut self, action: T) -> &mut Self {
        self.inner.action = action.as_ref().clone();
        self
    }
}

impl AsRef<UpdateChatAction> for UpdateChatAction {
    fn as_ref(&self) -> &UpdateChatAction {
        self
    }
}

impl AsRef<UpdateChatAction> for RTDUpdateChatActionBuilder {
    fn as_ref(&self) -> &UpdateChatAction {
        &self.inner
    }
}
