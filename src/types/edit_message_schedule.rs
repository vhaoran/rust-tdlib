// use crate::errors::Result;
// use crate::types::*;
// use uuid::Uuid;
//
// /// Edits the text of a message (or a text of a game message). Returns the edited message after the edit is completed on the server side
// #[derive(Debug, Clone, Default, Serialize, Deserialize)]
// pub struct EditMessageSchedulingState {
//     #[doc(hidden)]
//     #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
//     extra: Option<String>,
//     #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
//     client_id: Option<i32>,
//     /// The chat the message belongs to
//
//     #[serde(default)]
//     chat_id: i64,
//     #[serde(default)]
//     message_id: i64,
//     #[serde(default)]
//     scheduling_state: MessageSchedulingState,
//
//     #[serde(rename(serialize = "@type"))]
//     td_type: String,
// }
//
// impl RObject for EditMessageSchedulingState {
//     #[doc(hidden)]
//     fn extra(&self) -> Option<&str> {
//         self.extra.as_deref()
//     }
//     #[doc(hidden)]
//     fn client_id(&self) -> Option<i32> {
//         self.client_id
//     }
// }
//
// impl RFunction for EditMessageSchedulingState {}
//
// impl EditMessageSchedulingState {
//     pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
//         Ok(serde_json::from_str(json.as_ref())?)
//     }
//     pub fn builder() -> EditMessageSchedulingStateBuilder {
//         let mut inner = EditMessageSchedulingState::default();
//         inner.extra = Some(Uuid::new_v4().to_string());
//
//         inner.td_type = "EditMessageSchedulingState".to_string();
//
//         EditMessageSchedulingStateBuilder { inner }
//     }
//
//     pub fn chat_id(&self) -> i64 {
//         self.chat_id
//     }
//
//     pub fn message_id(&self) -> i64 {
//         self.message_id
//     }
//
//     pub fn scheduling_state(&self) -> &MessageSchedulingState {
//         &self.scheduling_state
//     }
// }
//
// #[doc(hidden)]
// pub struct EditMessageSchedulingStateBuilder {
//     inner: EditMessageSchedulingState,
// }
//
// #[deprecated]
// pub type RTDEditMessageSchedulingStateBuilder = EditMessageSchedulingStateBuilder;
//
// impl EditMessageSchedulingStateBuilder {
//     pub fn build(&self) -> EditMessageSchedulingState {
//         self.inner.clone()
//     }
//
//     pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
//         self.inner.chat_id = chat_id;
//         self
//     }
//
//     pub fn message_id(&mut self, message_id: i64) -> &mut Self {
//         self.inner.message_id = message_id;
//         self
//     }
//
//     pub fn scheduling_state(&mut self, st: MessageSchedulingState) -> &mut Self {
//         self.inner.scheduling_state = st;
//         self
//     }
// }
//
// impl AsRef<EditMessageSchedulingState> for EditMessageSchedulingState {
//     fn as_ref(&self) -> &EditMessageSchedulingState {
//         self
//     }
// }
//
// impl AsRef<EditMessageSchedulingState> for EditMessageSchedulingStateBuilder {
//     fn as_ref(&self) -> &EditMessageSchedulingState {
//         &self.inner
//     }
// }
