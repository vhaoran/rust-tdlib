use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the types of chats to which notification settings are relevant
pub trait TDNotificationSettingsScope: Debug + RObject {}

/// Describes the types of chats to which notification settings are relevant
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NotificationSettingsScope {
    #[doc(hidden)]
    _Default,
    /// Notification settings applied to all channels when the corresponding chat setting has a default value
    #[serde(rename = "notificationSettingsScopeChannelChats")]
    ChannelChats(NotificationSettingsScopeChannelChats),
    /// Notification settings applied to all basic groups and supergroups when the corresponding chat setting has a default value
    #[serde(rename = "notificationSettingsScopeGroupChats")]
    GroupChats(NotificationSettingsScopeGroupChats),
    /// Notification settings applied to all private and secret chats when the corresponding chat setting has a default value
    #[serde(rename = "notificationSettingsScopePrivateChats")]
    PrivateChats(NotificationSettingsScopePrivateChats),
}

impl Default for NotificationSettingsScope {
    fn default() -> Self {
        NotificationSettingsScope::_Default
    }
}

impl RObject for NotificationSettingsScope {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            NotificationSettingsScope::ChannelChats(t) => t.extra(),
            NotificationSettingsScope::GroupChats(t) => t.extra(),
            NotificationSettingsScope::PrivateChats(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            NotificationSettingsScope::ChannelChats(t) => t.client_id(),
            NotificationSettingsScope::GroupChats(t) => t.client_id(),
            NotificationSettingsScope::PrivateChats(t) => t.client_id(),

            _ => None,
        }
    }
}

impl NotificationSettingsScope {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, NotificationSettingsScope::_Default)
    }
}

impl AsRef<NotificationSettingsScope> for NotificationSettingsScope {
    fn as_ref(&self) -> &NotificationSettingsScope {
        self
    }
}

/// Notification settings applied to all channels when the corresponding chat setting has a default value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationSettingsScopeChannelChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for NotificationSettingsScopeChannelChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNotificationSettingsScope for NotificationSettingsScopeChannelChats {}

impl NotificationSettingsScopeChannelChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> NotificationSettingsScopeChannelChatsBuilder {
        let mut inner = NotificationSettingsScopeChannelChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        NotificationSettingsScopeChannelChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct NotificationSettingsScopeChannelChatsBuilder {
    inner: NotificationSettingsScopeChannelChats,
}

#[deprecated]
pub type RTDNotificationSettingsScopeChannelChatsBuilder =
    NotificationSettingsScopeChannelChatsBuilder;

impl NotificationSettingsScopeChannelChatsBuilder {
    pub fn build(&self) -> NotificationSettingsScopeChannelChats {
        self.inner.clone()
    }
}

impl AsRef<NotificationSettingsScopeChannelChats> for NotificationSettingsScopeChannelChats {
    fn as_ref(&self) -> &NotificationSettingsScopeChannelChats {
        self
    }
}

impl AsRef<NotificationSettingsScopeChannelChats> for NotificationSettingsScopeChannelChatsBuilder {
    fn as_ref(&self) -> &NotificationSettingsScopeChannelChats {
        &self.inner
    }
}

/// Notification settings applied to all basic groups and supergroups when the corresponding chat setting has a default value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationSettingsScopeGroupChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for NotificationSettingsScopeGroupChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNotificationSettingsScope for NotificationSettingsScopeGroupChats {}

impl NotificationSettingsScopeGroupChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> NotificationSettingsScopeGroupChatsBuilder {
        let mut inner = NotificationSettingsScopeGroupChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        NotificationSettingsScopeGroupChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct NotificationSettingsScopeGroupChatsBuilder {
    inner: NotificationSettingsScopeGroupChats,
}

#[deprecated]
pub type RTDNotificationSettingsScopeGroupChatsBuilder = NotificationSettingsScopeGroupChatsBuilder;

impl NotificationSettingsScopeGroupChatsBuilder {
    pub fn build(&self) -> NotificationSettingsScopeGroupChats {
        self.inner.clone()
    }
}

impl AsRef<NotificationSettingsScopeGroupChats> for NotificationSettingsScopeGroupChats {
    fn as_ref(&self) -> &NotificationSettingsScopeGroupChats {
        self
    }
}

impl AsRef<NotificationSettingsScopeGroupChats> for NotificationSettingsScopeGroupChatsBuilder {
    fn as_ref(&self) -> &NotificationSettingsScopeGroupChats {
        &self.inner
    }
}

/// Notification settings applied to all private and secret chats when the corresponding chat setting has a default value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationSettingsScopePrivateChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for NotificationSettingsScopePrivateChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNotificationSettingsScope for NotificationSettingsScopePrivateChats {}

impl NotificationSettingsScopePrivateChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> NotificationSettingsScopePrivateChatsBuilder {
        let mut inner = NotificationSettingsScopePrivateChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        NotificationSettingsScopePrivateChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct NotificationSettingsScopePrivateChatsBuilder {
    inner: NotificationSettingsScopePrivateChats,
}

#[deprecated]
pub type RTDNotificationSettingsScopePrivateChatsBuilder =
    NotificationSettingsScopePrivateChatsBuilder;

impl NotificationSettingsScopePrivateChatsBuilder {
    pub fn build(&self) -> NotificationSettingsScopePrivateChats {
        self.inner.clone()
    }
}

impl AsRef<NotificationSettingsScopePrivateChats> for NotificationSettingsScopePrivateChats {
    fn as_ref(&self) -> &NotificationSettingsScopePrivateChats {
        self
    }
}

impl AsRef<NotificationSettingsScopePrivateChats> for NotificationSettingsScopePrivateChatsBuilder {
    fn as_ref(&self) -> &NotificationSettingsScopePrivateChats {
        &self.inner
    }
}
