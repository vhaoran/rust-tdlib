use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the type of a chat
pub trait TDChatType: Debug + RObject {}

/// Describes the type of a chat
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InviteLinkChatType  {
    #[doc(hidden)]
    _Default,
    #[serde(rename="inviteLinkChatTypeBasicGroup")]
    InviteLinkChatTypeBasicGroup,
    #[serde(rename="inviteLinkChatTypeChannel")]
    InviteLinkChatTypeChannel,
    #[serde(rename="inviteLinkChatTypeSupergroup")]
    InviteLinkChatTypeSupergroup,
}

impl Default for InviteLinkChatType{
    fn default() -> Self {
        Self::_Default
    }
}

// impl RObject for InviteLinkChatType {
//     #[doc(hidden)]
//     fn extra(&self) -> Option<&str> {
//         match self {
//             ChatType::BasicGroup(t) => t.extra(),
//             ChatType::Private(t) => t.extra(),
//             ChatType::Secret(t) => t.extra(),
//             ChatType::Supergroup(t) => t.extra(),
//
//             _ => None,
//         }
//     }
//     #[doc(hidden)]
//     fn client_id(&self) -> Option<i32> {
//         match self {
//             ChatType::BasicGroup(t) => t.client_id(),
//             ChatType::Private(t) => t.client_id(),
//             ChatType::Secret(t) => t.client_id(),
//             ChatType::Supergroup(t) => t.client_id(),
//
//             _ => None,
//         }
//     }
// }

impl InviteLinkChatType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InviteLinkChatType::_Default)
    }
}

impl AsRef<InviteLinkChatType> for InviteLinkChatType{
    fn as_ref(&self) -> &InviteLinkChatType {
        self
    }
}
