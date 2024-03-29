use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents result of checking whether the current session can be used to transfer a chat ownership to another user
pub trait TDCanTransferOwnershipResult: Debug + RObject {}

/// Represents result of checking whether the current session can be used to transfer a chat ownership to another user
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CanTransferOwnershipResult {
    #[doc(hidden)]
    _Default,
    /// Checks whether the current session can be used to transfer a chat ownership to another user
    #[serde(rename = "canTransferOwnership")]
    CanTransferOwnership(CanTransferOwnership),
    /// The session can be used
    #[serde(rename = "canTransferOwnershipResultOk")]
    Ok(CanTransferOwnershipResultOk),
    /// The 2-step verification needs to be enabled first
    #[serde(rename = "canTransferOwnershipResultPasswordNeeded")]
    PasswordNeeded(CanTransferOwnershipResultPasswordNeeded),
    /// The 2-step verification was enabled recently, user needs to wait
    #[serde(rename = "canTransferOwnershipResultPasswordTooFresh")]
    PasswordTooFresh(CanTransferOwnershipResultPasswordTooFresh),
    /// The session was created recently, user needs to wait
    #[serde(rename = "canTransferOwnershipResultSessionTooFresh")]
    SessionTooFresh(CanTransferOwnershipResultSessionTooFresh),
}

impl Default for CanTransferOwnershipResult {
    fn default() -> Self {
        CanTransferOwnershipResult::_Default
    }
}

impl RObject for CanTransferOwnershipResult {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            CanTransferOwnershipResult::CanTransferOwnership(t) => t.extra(),
            CanTransferOwnershipResult::Ok(t) => t.extra(),
            CanTransferOwnershipResult::PasswordNeeded(t) => t.extra(),
            CanTransferOwnershipResult::PasswordTooFresh(t) => t.extra(),
            CanTransferOwnershipResult::SessionTooFresh(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            CanTransferOwnershipResult::CanTransferOwnership(t) => t.client_id(),
            CanTransferOwnershipResult::Ok(t) => t.client_id(),
            CanTransferOwnershipResult::PasswordNeeded(t) => t.client_id(),
            CanTransferOwnershipResult::PasswordTooFresh(t) => t.client_id(),
            CanTransferOwnershipResult::SessionTooFresh(t) => t.client_id(),

            _ => None,
        }
    }
}

impl CanTransferOwnershipResult {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, CanTransferOwnershipResult::_Default)
    }
}

impl AsRef<CanTransferOwnershipResult> for CanTransferOwnershipResult {
    fn as_ref(&self) -> &CanTransferOwnershipResult {
        self
    }
}

/// The session can be used
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanTransferOwnershipResultOk {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CanTransferOwnershipResultOk {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCanTransferOwnershipResult for CanTransferOwnershipResultOk {}

impl CanTransferOwnershipResultOk {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CanTransferOwnershipResultOkBuilder {
        let mut inner = CanTransferOwnershipResultOk::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CanTransferOwnershipResultOkBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CanTransferOwnershipResultOkBuilder {
    inner: CanTransferOwnershipResultOk,
}

#[deprecated]
pub type RTDCanTransferOwnershipResultOkBuilder = CanTransferOwnershipResultOkBuilder;

impl CanTransferOwnershipResultOkBuilder {
    pub fn build(&self) -> CanTransferOwnershipResultOk {
        self.inner.clone()
    }
}

impl AsRef<CanTransferOwnershipResultOk> for CanTransferOwnershipResultOk {
    fn as_ref(&self) -> &CanTransferOwnershipResultOk {
        self
    }
}

impl AsRef<CanTransferOwnershipResultOk> for CanTransferOwnershipResultOkBuilder {
    fn as_ref(&self) -> &CanTransferOwnershipResultOk {
        &self.inner
    }
}

/// The 2-step verification needs to be enabled first
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanTransferOwnershipResultPasswordNeeded {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CanTransferOwnershipResultPasswordNeeded {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCanTransferOwnershipResult for CanTransferOwnershipResultPasswordNeeded {}

impl CanTransferOwnershipResultPasswordNeeded {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CanTransferOwnershipResultPasswordNeededBuilder {
        let mut inner = CanTransferOwnershipResultPasswordNeeded::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CanTransferOwnershipResultPasswordNeededBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CanTransferOwnershipResultPasswordNeededBuilder {
    inner: CanTransferOwnershipResultPasswordNeeded,
}

#[deprecated]
pub type RTDCanTransferOwnershipResultPasswordNeededBuilder =
    CanTransferOwnershipResultPasswordNeededBuilder;

impl CanTransferOwnershipResultPasswordNeededBuilder {
    pub fn build(&self) -> CanTransferOwnershipResultPasswordNeeded {
        self.inner.clone()
    }
}

impl AsRef<CanTransferOwnershipResultPasswordNeeded> for CanTransferOwnershipResultPasswordNeeded {
    fn as_ref(&self) -> &CanTransferOwnershipResultPasswordNeeded {
        self
    }
}

impl AsRef<CanTransferOwnershipResultPasswordNeeded>
    for CanTransferOwnershipResultPasswordNeededBuilder
{
    fn as_ref(&self) -> &CanTransferOwnershipResultPasswordNeeded {
        &self.inner
    }
}

/// The 2-step verification was enabled recently, user needs to wait
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanTransferOwnershipResultPasswordTooFresh {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Time left before the session can be used to transfer ownership of a chat, in seconds

    #[serde(default)]
    retry_after: i32,
}

impl RObject for CanTransferOwnershipResultPasswordTooFresh {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCanTransferOwnershipResult for CanTransferOwnershipResultPasswordTooFresh {}

impl CanTransferOwnershipResultPasswordTooFresh {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CanTransferOwnershipResultPasswordTooFreshBuilder {
        let mut inner = CanTransferOwnershipResultPasswordTooFresh::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CanTransferOwnershipResultPasswordTooFreshBuilder { inner }
    }

    pub fn retry_after(&self) -> i32 {
        self.retry_after
    }
}

#[doc(hidden)]
pub struct CanTransferOwnershipResultPasswordTooFreshBuilder {
    inner: CanTransferOwnershipResultPasswordTooFresh,
}

#[deprecated]
pub type RTDCanTransferOwnershipResultPasswordTooFreshBuilder =
    CanTransferOwnershipResultPasswordTooFreshBuilder;

impl CanTransferOwnershipResultPasswordTooFreshBuilder {
    pub fn build(&self) -> CanTransferOwnershipResultPasswordTooFresh {
        self.inner.clone()
    }

    pub fn retry_after(&mut self, retry_after: i32) -> &mut Self {
        self.inner.retry_after = retry_after;
        self
    }
}

impl AsRef<CanTransferOwnershipResultPasswordTooFresh>
    for CanTransferOwnershipResultPasswordTooFresh
{
    fn as_ref(&self) -> &CanTransferOwnershipResultPasswordTooFresh {
        self
    }
}

impl AsRef<CanTransferOwnershipResultPasswordTooFresh>
    for CanTransferOwnershipResultPasswordTooFreshBuilder
{
    fn as_ref(&self) -> &CanTransferOwnershipResultPasswordTooFresh {
        &self.inner
    }
}

/// The session was created recently, user needs to wait
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanTransferOwnershipResultSessionTooFresh {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Time left before the session can be used to transfer ownership of a chat, in seconds

    #[serde(default)]
    retry_after: i32,
}

impl RObject for CanTransferOwnershipResultSessionTooFresh {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCanTransferOwnershipResult for CanTransferOwnershipResultSessionTooFresh {}

impl CanTransferOwnershipResultSessionTooFresh {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CanTransferOwnershipResultSessionTooFreshBuilder {
        let mut inner = CanTransferOwnershipResultSessionTooFresh::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CanTransferOwnershipResultSessionTooFreshBuilder { inner }
    }

    pub fn retry_after(&self) -> i32 {
        self.retry_after
    }
}

#[doc(hidden)]
pub struct CanTransferOwnershipResultSessionTooFreshBuilder {
    inner: CanTransferOwnershipResultSessionTooFresh,
}

#[deprecated]
pub type RTDCanTransferOwnershipResultSessionTooFreshBuilder =
    CanTransferOwnershipResultSessionTooFreshBuilder;

impl CanTransferOwnershipResultSessionTooFreshBuilder {
    pub fn build(&self) -> CanTransferOwnershipResultSessionTooFresh {
        self.inner.clone()
    }

    pub fn retry_after(&mut self, retry_after: i32) -> &mut Self {
        self.inner.retry_after = retry_after;
        self
    }
}

impl AsRef<CanTransferOwnershipResultSessionTooFresh>
    for CanTransferOwnershipResultSessionTooFresh
{
    fn as_ref(&self) -> &CanTransferOwnershipResultSessionTooFresh {
        self
    }
}

impl AsRef<CanTransferOwnershipResultSessionTooFresh>
    for CanTransferOwnershipResultSessionTooFreshBuilder
{
    fn as_ref(&self) -> &CanTransferOwnershipResultSessionTooFresh {
        &self.inner
    }
}
