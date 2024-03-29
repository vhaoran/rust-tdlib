use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Part of the face, relative to which a mask is placed
pub trait TDMaskPoint: Debug + RObject {}

/// Part of the face, relative to which a mask is placed
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MaskPoint {
    #[doc(hidden)]
    _Default,
    /// The mask is placed relatively to the chin
    #[serde(rename = "maskPointChin")]
    Chin(MaskPointChin),
    /// The mask is placed relatively to the eyes
    #[serde(rename = "maskPointEyes")]
    Eyes(MaskPointEyes),
    /// The mask is placed relatively to the forehead
    #[serde(rename = "maskPointForehead")]
    Forehead(MaskPointForehead),
    /// The mask is placed relatively to the mouth
    #[serde(rename = "maskPointMouth")]
    Mouth(MaskPointMouth),
}

impl Default for MaskPoint {
    fn default() -> Self {
        MaskPoint::_Default
    }
}

impl RObject for MaskPoint {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            MaskPoint::Chin(t) => t.extra(),
            MaskPoint::Eyes(t) => t.extra(),
            MaskPoint::Forehead(t) => t.extra(),
            MaskPoint::Mouth(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            MaskPoint::Chin(t) => t.client_id(),
            MaskPoint::Eyes(t) => t.client_id(),
            MaskPoint::Forehead(t) => t.client_id(),
            MaskPoint::Mouth(t) => t.client_id(),

            _ => None,
        }
    }
}

impl MaskPoint {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, MaskPoint::_Default)
    }
}

impl AsRef<MaskPoint> for MaskPoint {
    fn as_ref(&self) -> &MaskPoint {
        self
    }
}

/// The mask is placed relatively to the chin
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MaskPointChin {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MaskPointChin {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMaskPoint for MaskPointChin {}

impl MaskPointChin {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MaskPointChinBuilder {
        let mut inner = MaskPointChin::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MaskPointChinBuilder { inner }
    }
}

#[doc(hidden)]
pub struct MaskPointChinBuilder {
    inner: MaskPointChin,
}

#[deprecated]
pub type RTDMaskPointChinBuilder = MaskPointChinBuilder;

impl MaskPointChinBuilder {
    pub fn build(&self) -> MaskPointChin {
        self.inner.clone()
    }
}

impl AsRef<MaskPointChin> for MaskPointChin {
    fn as_ref(&self) -> &MaskPointChin {
        self
    }
}

impl AsRef<MaskPointChin> for MaskPointChinBuilder {
    fn as_ref(&self) -> &MaskPointChin {
        &self.inner
    }
}

/// The mask is placed relatively to the eyes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MaskPointEyes {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MaskPointEyes {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMaskPoint for MaskPointEyes {}

impl MaskPointEyes {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MaskPointEyesBuilder {
        let mut inner = MaskPointEyes::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MaskPointEyesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct MaskPointEyesBuilder {
    inner: MaskPointEyes,
}

#[deprecated]
pub type RTDMaskPointEyesBuilder = MaskPointEyesBuilder;

impl MaskPointEyesBuilder {
    pub fn build(&self) -> MaskPointEyes {
        self.inner.clone()
    }
}

impl AsRef<MaskPointEyes> for MaskPointEyes {
    fn as_ref(&self) -> &MaskPointEyes {
        self
    }
}

impl AsRef<MaskPointEyes> for MaskPointEyesBuilder {
    fn as_ref(&self) -> &MaskPointEyes {
        &self.inner
    }
}

/// The mask is placed relatively to the forehead
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MaskPointForehead {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MaskPointForehead {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMaskPoint for MaskPointForehead {}

impl MaskPointForehead {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MaskPointForeheadBuilder {
        let mut inner = MaskPointForehead::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MaskPointForeheadBuilder { inner }
    }
}

#[doc(hidden)]
pub struct MaskPointForeheadBuilder {
    inner: MaskPointForehead,
}

#[deprecated]
pub type RTDMaskPointForeheadBuilder = MaskPointForeheadBuilder;

impl MaskPointForeheadBuilder {
    pub fn build(&self) -> MaskPointForehead {
        self.inner.clone()
    }
}

impl AsRef<MaskPointForehead> for MaskPointForehead {
    fn as_ref(&self) -> &MaskPointForehead {
        self
    }
}

impl AsRef<MaskPointForehead> for MaskPointForeheadBuilder {
    fn as_ref(&self) -> &MaskPointForehead {
        &self.inner
    }
}

/// The mask is placed relatively to the mouth
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MaskPointMouth {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MaskPointMouth {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMaskPoint for MaskPointMouth {}

impl MaskPointMouth {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MaskPointMouthBuilder {
        let mut inner = MaskPointMouth::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MaskPointMouthBuilder { inner }
    }
}

#[doc(hidden)]
pub struct MaskPointMouthBuilder {
    inner: MaskPointMouth,
}

#[deprecated]
pub type RTDMaskPointMouthBuilder = MaskPointMouthBuilder;

impl MaskPointMouthBuilder {
    pub fn build(&self) -> MaskPointMouth {
        self.inner.clone()
    }
}

impl AsRef<MaskPointMouth> for MaskPointMouth {
    fn as_ref(&self) -> &MaskPointMouth {
        self
    }
}

impl AsRef<MaskPointMouth> for MaskPointMouthBuilder {
    fn as_ref(&self) -> &MaskPointMouth {
        &self.inner
    }
}
