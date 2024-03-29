use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains the description of an error in a Telegram Passport element
pub trait TDPassportElementErrorSource: Debug + RObject {}

/// Contains the description of an error in a Telegram Passport element
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportElementErrorSource {
    #[doc(hidden)]
    _Default,
    /// One of the data fields contains an error. The error will be considered resolved when the value of the field changes
    #[serde(rename = "passportElementErrorSourceDataField")]
    DataField(PassportElementErrorSourceDataField),
    /// The file contains an error. The error will be considered resolved when the file changes
    #[serde(rename = "passportElementErrorSourceFile")]
    File(PassportElementErrorSourceFile),
    /// The list of attached files contains an error. The error will be considered resolved when the list of files changes
    #[serde(rename = "passportElementErrorSourceFiles")]
    Files(PassportElementErrorSourceFiles),
    /// The front side of the document contains an error. The error will be considered resolved when the file with the front side changes
    #[serde(rename = "passportElementErrorSourceFrontSide")]
    FrontSide(PassportElementErrorSourceFrontSide),
    /// The reverse side of the document contains an error. The error will be considered resolved when the file with the reverse side changes
    #[serde(rename = "passportElementErrorSourceReverseSide")]
    ReverseSide(PassportElementErrorSourceReverseSide),
    /// The selfie with the document contains an error. The error will be considered resolved when the file with the selfie changes
    #[serde(rename = "passportElementErrorSourceSelfie")]
    Selfie(PassportElementErrorSourceSelfie),
    /// One of files with the translation of the document contains an error. The error will be considered resolved when the file changes
    #[serde(rename = "passportElementErrorSourceTranslationFile")]
    TranslationFile(PassportElementErrorSourceTranslationFile),
    /// The translation of the document contains an error. The error will be considered resolved when the list of translation files changes
    #[serde(rename = "passportElementErrorSourceTranslationFiles")]
    TranslationFiles(PassportElementErrorSourceTranslationFiles),
    /// The element contains an error in an unspecified place. The error will be considered resolved when new data is added
    #[serde(rename = "passportElementErrorSourceUnspecified")]
    Unspecified(PassportElementErrorSourceUnspecified),
}

impl Default for PassportElementErrorSource {
    fn default() -> Self {
        PassportElementErrorSource::_Default
    }
}

impl RObject for PassportElementErrorSource {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PassportElementErrorSource::DataField(t) => t.extra(),
            PassportElementErrorSource::File(t) => t.extra(),
            PassportElementErrorSource::Files(t) => t.extra(),
            PassportElementErrorSource::FrontSide(t) => t.extra(),
            PassportElementErrorSource::ReverseSide(t) => t.extra(),
            PassportElementErrorSource::Selfie(t) => t.extra(),
            PassportElementErrorSource::TranslationFile(t) => t.extra(),
            PassportElementErrorSource::TranslationFiles(t) => t.extra(),
            PassportElementErrorSource::Unspecified(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PassportElementErrorSource::DataField(t) => t.client_id(),
            PassportElementErrorSource::File(t) => t.client_id(),
            PassportElementErrorSource::Files(t) => t.client_id(),
            PassportElementErrorSource::FrontSide(t) => t.client_id(),
            PassportElementErrorSource::ReverseSide(t) => t.client_id(),
            PassportElementErrorSource::Selfie(t) => t.client_id(),
            PassportElementErrorSource::TranslationFile(t) => t.client_id(),
            PassportElementErrorSource::TranslationFiles(t) => t.client_id(),
            PassportElementErrorSource::Unspecified(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PassportElementErrorSource {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PassportElementErrorSource::_Default)
    }
}

impl AsRef<PassportElementErrorSource> for PassportElementErrorSource {
    fn as_ref(&self) -> &PassportElementErrorSource {
        self
    }
}

/// One of the data fields contains an error. The error will be considered resolved when the value of the field changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceDataField {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Field name

    #[serde(default)]
    field_name: String,
}

impl RObject for PassportElementErrorSourceDataField {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementErrorSource for PassportElementErrorSourceDataField {}

impl PassportElementErrorSourceDataField {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementErrorSourceDataFieldBuilder {
        let mut inner = PassportElementErrorSourceDataField::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementErrorSourceDataFieldBuilder { inner }
    }

    pub fn field_name(&self) -> &String {
        &self.field_name
    }
}

#[doc(hidden)]
pub struct PassportElementErrorSourceDataFieldBuilder {
    inner: PassportElementErrorSourceDataField,
}

#[deprecated]
pub type RTDPassportElementErrorSourceDataFieldBuilder = PassportElementErrorSourceDataFieldBuilder;

impl PassportElementErrorSourceDataFieldBuilder {
    pub fn build(&self) -> PassportElementErrorSourceDataField {
        self.inner.clone()
    }

    pub fn field_name<T: AsRef<str>>(&mut self, field_name: T) -> &mut Self {
        self.inner.field_name = field_name.as_ref().to_string();
        self
    }
}

impl AsRef<PassportElementErrorSourceDataField> for PassportElementErrorSourceDataField {
    fn as_ref(&self) -> &PassportElementErrorSourceDataField {
        self
    }
}

impl AsRef<PassportElementErrorSourceDataField> for PassportElementErrorSourceDataFieldBuilder {
    fn as_ref(&self) -> &PassportElementErrorSourceDataField {
        &self.inner
    }
}

/// The file contains an error. The error will be considered resolved when the file changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Index of a file with the error

    #[serde(default)]
    file_index: i32,
}

impl RObject for PassportElementErrorSourceFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementErrorSource for PassportElementErrorSourceFile {}

impl PassportElementErrorSourceFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementErrorSourceFileBuilder {
        let mut inner = PassportElementErrorSourceFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementErrorSourceFileBuilder { inner }
    }

    pub fn file_index(&self) -> i32 {
        self.file_index
    }
}

#[doc(hidden)]
pub struct PassportElementErrorSourceFileBuilder {
    inner: PassportElementErrorSourceFile,
}

#[deprecated]
pub type RTDPassportElementErrorSourceFileBuilder = PassportElementErrorSourceFileBuilder;

impl PassportElementErrorSourceFileBuilder {
    pub fn build(&self) -> PassportElementErrorSourceFile {
        self.inner.clone()
    }

    pub fn file_index(&mut self, file_index: i32) -> &mut Self {
        self.inner.file_index = file_index;
        self
    }
}

impl AsRef<PassportElementErrorSourceFile> for PassportElementErrorSourceFile {
    fn as_ref(&self) -> &PassportElementErrorSourceFile {
        self
    }
}

impl AsRef<PassportElementErrorSourceFile> for PassportElementErrorSourceFileBuilder {
    fn as_ref(&self) -> &PassportElementErrorSourceFile {
        &self.inner
    }
}

/// The list of attached files contains an error. The error will be considered resolved when the list of files changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceFiles {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementErrorSourceFiles {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementErrorSource for PassportElementErrorSourceFiles {}

impl PassportElementErrorSourceFiles {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementErrorSourceFilesBuilder {
        let mut inner = PassportElementErrorSourceFiles::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementErrorSourceFilesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PassportElementErrorSourceFilesBuilder {
    inner: PassportElementErrorSourceFiles,
}

#[deprecated]
pub type RTDPassportElementErrorSourceFilesBuilder = PassportElementErrorSourceFilesBuilder;

impl PassportElementErrorSourceFilesBuilder {
    pub fn build(&self) -> PassportElementErrorSourceFiles {
        self.inner.clone()
    }
}

impl AsRef<PassportElementErrorSourceFiles> for PassportElementErrorSourceFiles {
    fn as_ref(&self) -> &PassportElementErrorSourceFiles {
        self
    }
}

impl AsRef<PassportElementErrorSourceFiles> for PassportElementErrorSourceFilesBuilder {
    fn as_ref(&self) -> &PassportElementErrorSourceFiles {
        &self.inner
    }
}

/// The front side of the document contains an error. The error will be considered resolved when the file with the front side changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceFrontSide {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementErrorSourceFrontSide {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementErrorSource for PassportElementErrorSourceFrontSide {}

impl PassportElementErrorSourceFrontSide {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementErrorSourceFrontSideBuilder {
        let mut inner = PassportElementErrorSourceFrontSide::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementErrorSourceFrontSideBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PassportElementErrorSourceFrontSideBuilder {
    inner: PassportElementErrorSourceFrontSide,
}

#[deprecated]
pub type RTDPassportElementErrorSourceFrontSideBuilder = PassportElementErrorSourceFrontSideBuilder;

impl PassportElementErrorSourceFrontSideBuilder {
    pub fn build(&self) -> PassportElementErrorSourceFrontSide {
        self.inner.clone()
    }
}

impl AsRef<PassportElementErrorSourceFrontSide> for PassportElementErrorSourceFrontSide {
    fn as_ref(&self) -> &PassportElementErrorSourceFrontSide {
        self
    }
}

impl AsRef<PassportElementErrorSourceFrontSide> for PassportElementErrorSourceFrontSideBuilder {
    fn as_ref(&self) -> &PassportElementErrorSourceFrontSide {
        &self.inner
    }
}

/// The reverse side of the document contains an error. The error will be considered resolved when the file with the reverse side changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceReverseSide {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementErrorSourceReverseSide {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementErrorSource for PassportElementErrorSourceReverseSide {}

impl PassportElementErrorSourceReverseSide {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementErrorSourceReverseSideBuilder {
        let mut inner = PassportElementErrorSourceReverseSide::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementErrorSourceReverseSideBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PassportElementErrorSourceReverseSideBuilder {
    inner: PassportElementErrorSourceReverseSide,
}

#[deprecated]
pub type RTDPassportElementErrorSourceReverseSideBuilder =
    PassportElementErrorSourceReverseSideBuilder;

impl PassportElementErrorSourceReverseSideBuilder {
    pub fn build(&self) -> PassportElementErrorSourceReverseSide {
        self.inner.clone()
    }
}

impl AsRef<PassportElementErrorSourceReverseSide> for PassportElementErrorSourceReverseSide {
    fn as_ref(&self) -> &PassportElementErrorSourceReverseSide {
        self
    }
}

impl AsRef<PassportElementErrorSourceReverseSide> for PassportElementErrorSourceReverseSideBuilder {
    fn as_ref(&self) -> &PassportElementErrorSourceReverseSide {
        &self.inner
    }
}

/// The selfie with the document contains an error. The error will be considered resolved when the file with the selfie changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceSelfie {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementErrorSourceSelfie {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementErrorSource for PassportElementErrorSourceSelfie {}

impl PassportElementErrorSourceSelfie {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementErrorSourceSelfieBuilder {
        let mut inner = PassportElementErrorSourceSelfie::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementErrorSourceSelfieBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PassportElementErrorSourceSelfieBuilder {
    inner: PassportElementErrorSourceSelfie,
}

#[deprecated]
pub type RTDPassportElementErrorSourceSelfieBuilder = PassportElementErrorSourceSelfieBuilder;

impl PassportElementErrorSourceSelfieBuilder {
    pub fn build(&self) -> PassportElementErrorSourceSelfie {
        self.inner.clone()
    }
}

impl AsRef<PassportElementErrorSourceSelfie> for PassportElementErrorSourceSelfie {
    fn as_ref(&self) -> &PassportElementErrorSourceSelfie {
        self
    }
}

impl AsRef<PassportElementErrorSourceSelfie> for PassportElementErrorSourceSelfieBuilder {
    fn as_ref(&self) -> &PassportElementErrorSourceSelfie {
        &self.inner
    }
}

/// One of files with the translation of the document contains an error. The error will be considered resolved when the file changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceTranslationFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Index of a file with the error

    #[serde(default)]
    file_index: i32,
}

impl RObject for PassportElementErrorSourceTranslationFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementErrorSource for PassportElementErrorSourceTranslationFile {}

impl PassportElementErrorSourceTranslationFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementErrorSourceTranslationFileBuilder {
        let mut inner = PassportElementErrorSourceTranslationFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementErrorSourceTranslationFileBuilder { inner }
    }

    pub fn file_index(&self) -> i32 {
        self.file_index
    }
}

#[doc(hidden)]
pub struct PassportElementErrorSourceTranslationFileBuilder {
    inner: PassportElementErrorSourceTranslationFile,
}

#[deprecated]
pub type RTDPassportElementErrorSourceTranslationFileBuilder =
    PassportElementErrorSourceTranslationFileBuilder;

impl PassportElementErrorSourceTranslationFileBuilder {
    pub fn build(&self) -> PassportElementErrorSourceTranslationFile {
        self.inner.clone()
    }

    pub fn file_index(&mut self, file_index: i32) -> &mut Self {
        self.inner.file_index = file_index;
        self
    }
}

impl AsRef<PassportElementErrorSourceTranslationFile>
    for PassportElementErrorSourceTranslationFile
{
    fn as_ref(&self) -> &PassportElementErrorSourceTranslationFile {
        self
    }
}

impl AsRef<PassportElementErrorSourceTranslationFile>
    for PassportElementErrorSourceTranslationFileBuilder
{
    fn as_ref(&self) -> &PassportElementErrorSourceTranslationFile {
        &self.inner
    }
}

/// The translation of the document contains an error. The error will be considered resolved when the list of translation files changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceTranslationFiles {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementErrorSourceTranslationFiles {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementErrorSource for PassportElementErrorSourceTranslationFiles {}

impl PassportElementErrorSourceTranslationFiles {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementErrorSourceTranslationFilesBuilder {
        let mut inner = PassportElementErrorSourceTranslationFiles::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementErrorSourceTranslationFilesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PassportElementErrorSourceTranslationFilesBuilder {
    inner: PassportElementErrorSourceTranslationFiles,
}

#[deprecated]
pub type RTDPassportElementErrorSourceTranslationFilesBuilder =
    PassportElementErrorSourceTranslationFilesBuilder;

impl PassportElementErrorSourceTranslationFilesBuilder {
    pub fn build(&self) -> PassportElementErrorSourceTranslationFiles {
        self.inner.clone()
    }
}

impl AsRef<PassportElementErrorSourceTranslationFiles>
    for PassportElementErrorSourceTranslationFiles
{
    fn as_ref(&self) -> &PassportElementErrorSourceTranslationFiles {
        self
    }
}

impl AsRef<PassportElementErrorSourceTranslationFiles>
    for PassportElementErrorSourceTranslationFilesBuilder
{
    fn as_ref(&self) -> &PassportElementErrorSourceTranslationFiles {
        &self.inner
    }
}

/// The element contains an error in an unspecified place. The error will be considered resolved when new data is added
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceUnspecified {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementErrorSourceUnspecified {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementErrorSource for PassportElementErrorSourceUnspecified {}

impl PassportElementErrorSourceUnspecified {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementErrorSourceUnspecifiedBuilder {
        let mut inner = PassportElementErrorSourceUnspecified::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementErrorSourceUnspecifiedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PassportElementErrorSourceUnspecifiedBuilder {
    inner: PassportElementErrorSourceUnspecified,
}

#[deprecated]
pub type RTDPassportElementErrorSourceUnspecifiedBuilder =
    PassportElementErrorSourceUnspecifiedBuilder;

impl PassportElementErrorSourceUnspecifiedBuilder {
    pub fn build(&self) -> PassportElementErrorSourceUnspecified {
        self.inner.clone()
    }
}

impl AsRef<PassportElementErrorSourceUnspecified> for PassportElementErrorSourceUnspecified {
    fn as_ref(&self) -> &PassportElementErrorSourceUnspecified {
        self
    }
}

impl AsRef<PassportElementErrorSourceUnspecified> for PassportElementErrorSourceUnspecifiedBuilder {
    fn as_ref(&self) -> &PassportElementErrorSourceUnspecified {
        &self.inner
    }
}
