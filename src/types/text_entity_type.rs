use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::{Debug, Display};

/// Represents a part of the text which must be formatted differently
pub trait TDTextEntityType: Debug + RObject {}

/// Represents a part of the text which must be formatted differently
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TextEntityType {
    #[doc(hidden)]
    _Default,
    /// A bank card number. The getBankCardInfo method can be used to get information about the bank card
    #[serde(rename = "textEntityTypeBankCardNumber")]
    BankCardNumber(TextEntityTypeBankCardNumber),
    /// A bold text
    #[serde(rename = "textEntityTypeBold")]
    Bold(TextEntityTypeBold),
    /// A bot command, beginning with "/"
    #[serde(rename = "textEntityTypeBotCommand")]
    BotCommand(TextEntityTypeBotCommand),
    /// A cashtag text, beginning with "$" and consisting of capital English letters (e.g., "$USD")
    #[serde(rename = "textEntityTypeCashtag")]
    Cashtag(TextEntityTypeCashtag),
    /// Text that must be formatted as if inside a code HTML tag
    #[serde(rename = "textEntityTypeCode")]
    Code(TextEntityTypeCode),
    /// An email address
    #[serde(rename = "textEntityTypeEmailAddress")]
    EmailAddress(TextEntityTypeEmailAddress),
    /// A hashtag text, beginning with "#"
    #[serde(rename = "textEntityTypeHashtag")]
    Hashtag(TextEntityTypeHashtag),
    /// An italic text
    #[serde(rename = "textEntityTypeItalic")]
    Italic(TextEntityTypeItalic),
    /// A media timestamp
    #[serde(rename = "textEntityTypeMediaTimestamp")]
    MediaTimestamp(TextEntityTypeMediaTimestamp),
    /// A mention of a user by their username
    #[serde(rename = "textEntityTypeMention")]
    Mention(TextEntityTypeMention),
    /// A text shows instead of a raw mention of the user (e.g., when the user has no username)
    #[serde(rename = "textEntityTypeMentionName")]
    MentionName(TextEntityTypeMentionName),
    /// A phone number
    #[serde(rename = "textEntityTypePhoneNumber")]
    PhoneNumber(TextEntityTypePhoneNumber),
    /// Text that must be formatted as if inside a pre HTML tag
    #[serde(rename = "textEntityTypePre")]
    Pre(TextEntityTypePre),
    /// Text that must be formatted as if inside pre, and code HTML tags
    #[serde(rename = "textEntityTypePreCode")]
    PreCode(TextEntityTypePreCode),
    /// A strikethrough text
    #[serde(rename = "textEntityTypeStrikethrough")]
    Strikethrough(TextEntityTypeStrikethrough),
    /// A text description shown instead of a raw URL
    #[serde(rename = "textEntityTypeTextUrl")]
    TextUrl(TextEntityTypeTextUrl),
    /// An underlined text
    #[serde(rename = "textEntityTypeUnderline")]
    Underline(TextEntityTypeUnderline),
    /// An HTTP URL
    #[serde(rename = "textEntityTypeUrl")]
    Url(TextEntityTypeUrl),
    //     add by whr
    #[serde(rename = "textEntityTypeCustomEmoji")]
    CustomEmoji(TextEntityTypeCustomEmoji),
    // add by whr
    #[serde(rename = "textEntityTypeSpoiler")]
    Spoiler(TextEntityTypeSpoiler),
    #[serde(rename = "textEntityTypeBlockQuote")]
    BlockQuote(TextEntityTypeBlockQuote),
    #[serde(rename = "textEntityTypeExpandableBlockQuote")]
    ExpandableBlockQuote(TextEntityTypeExpandableBlockQuote),
}

impl Default for TextEntityType {
    fn default() -> Self {
        TextEntityType::_Default
    }
}

impl RObject for TextEntityType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            TextEntityType::BankCardNumber(t) => t.extra(),
            TextEntityType::Bold(t) => t.extra(),         //
            TextEntityType::BotCommand(t) => t.extra(),   //
            TextEntityType::Cashtag(t) => t.extra(),      //
            TextEntityType::Code(t) => t.extra(),         //
            TextEntityType::EmailAddress(t) => t.extra(), //
            TextEntityType::Hashtag(t) => t.extra(),      //
            TextEntityType::Italic(t) => t.extra(),       //
            TextEntityType::MediaTimestamp(t) => t.extra(),
            TextEntityType::Mention(t) => t.extra(), //
            TextEntityType::MentionName(t) => t.extra(),
            TextEntityType::PhoneNumber(t) => t.extra(),
            TextEntityType::Pre(t) => t.extra(), //
            TextEntityType::PreCode(t) => t.extra(),
            TextEntityType::Strikethrough(t) => t.extra(), //
            TextEntityType::TextUrl(t) => t.extra(),       //
            TextEntityType::Underline(t) => t.extra(),     //
            TextEntityType::Url(t) => t.extra(),           //
            TextEntityType::CustomEmoji(t) => t.extra(),
            TextEntityType::Spoiler(t) => t.extra(), //
            TextEntityType::BlockQuote(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            TextEntityType::BankCardNumber(t) => t.client_id(),
            TextEntityType::Bold(t) => t.client_id(),
            TextEntityType::BotCommand(t) => t.client_id(),
            TextEntityType::Cashtag(t) => t.client_id(),
            TextEntityType::Code(t) => t.client_id(),
            TextEntityType::EmailAddress(t) => t.client_id(),
            TextEntityType::Hashtag(t) => t.client_id(),
            TextEntityType::Italic(t) => t.client_id(),
            TextEntityType::MediaTimestamp(t) => t.client_id(),
            TextEntityType::Mention(t) => t.client_id(),
            TextEntityType::MentionName(t) => t.client_id(),
            TextEntityType::PhoneNumber(t) => t.client_id(),
            TextEntityType::Pre(t) => t.client_id(),
            TextEntityType::PreCode(t) => t.client_id(),
            TextEntityType::Strikethrough(t) => t.client_id(),
            TextEntityType::TextUrl(t) => t.client_id(),
            TextEntityType::Underline(t) => t.client_id(),
            TextEntityType::Url(t) => t.client_id(),
            TextEntityType::CustomEmoji(t) => t.client_id(),
            TextEntityType::Spoiler(t) => t.client_id(),
            TextEntityType::BlockQuote(t) => t.client_id(),

            _ => None,
        }
    }
}

impl TextEntityType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, TextEntityType::_Default)
    }
}

impl AsRef<TextEntityType> for TextEntityType {
    fn as_ref(&self) -> &TextEntityType {
        self
    }
}

/// A bank card number. The getBankCardInfo method can be used to get information about the bank card
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeBankCardNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeBankCardNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeBankCardNumber {}

impl TextEntityTypeBankCardNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeBankCardNumberBuilder {
        let mut inner = TextEntityTypeBankCardNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeBankCardNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypeBankCardNumberBuilder {
    inner: TextEntityTypeBankCardNumber,
}

#[deprecated]
pub type RTDTextEntityTypeBankCardNumberBuilder = TextEntityTypeBankCardNumberBuilder;

impl TextEntityTypeBankCardNumberBuilder {
    pub fn build(&self) -> TextEntityTypeBankCardNumber {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeBankCardNumber> for TextEntityTypeBankCardNumber {
    fn as_ref(&self) -> &TextEntityTypeBankCardNumber {
        self
    }
}

impl AsRef<TextEntityTypeBankCardNumber> for TextEntityTypeBankCardNumberBuilder {
    fn as_ref(&self) -> &TextEntityTypeBankCardNumber {
        &self.inner
    }
}

/// A bold text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeBold {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeBold {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeBold {}

impl TextEntityTypeBold {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeBoldBuilder {
        let mut inner = TextEntityTypeBold::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeBoldBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypeBoldBuilder {
    inner: TextEntityTypeBold,
}

#[deprecated]
pub type RTDTextEntityTypeBoldBuilder = TextEntityTypeBoldBuilder;

impl TextEntityTypeBoldBuilder {
    pub fn build(&self) -> TextEntityTypeBold {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeBold> for TextEntityTypeBold {
    fn as_ref(&self) -> &TextEntityTypeBold {
        self
    }
}

impl AsRef<TextEntityTypeBold> for TextEntityTypeBoldBuilder {
    fn as_ref(&self) -> &TextEntityTypeBold {
        &self.inner
    }
}

/// A bot command, beginning with "/"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeBotCommand {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeBotCommand {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeBotCommand {}

impl TextEntityTypeBotCommand {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeBotCommandBuilder {
        let mut inner = TextEntityTypeBotCommand::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeBotCommandBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypeBotCommandBuilder {
    inner: TextEntityTypeBotCommand,
}

#[deprecated]
pub type RTDTextEntityTypeBotCommandBuilder = TextEntityTypeBotCommandBuilder;

impl TextEntityTypeBotCommandBuilder {
    pub fn build(&self) -> TextEntityTypeBotCommand {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeBotCommand> for TextEntityTypeBotCommand {
    fn as_ref(&self) -> &TextEntityTypeBotCommand {
        self
    }
}

impl AsRef<TextEntityTypeBotCommand> for TextEntityTypeBotCommandBuilder {
    fn as_ref(&self) -> &TextEntityTypeBotCommand {
        &self.inner
    }
}

/// A cashtag text, beginning with "$" and consisting of capital English letters (e.g., "$USD")
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeCashtag {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeCashtag {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeCashtag {}

impl TextEntityTypeCashtag {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeCashtagBuilder {
        let mut inner = TextEntityTypeCashtag::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeCashtagBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypeCashtagBuilder {
    inner: TextEntityTypeCashtag,
}

#[deprecated]
pub type RTDTextEntityTypeCashtagBuilder = TextEntityTypeCashtagBuilder;

impl TextEntityTypeCashtagBuilder {
    pub fn build(&self) -> TextEntityTypeCashtag {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeCashtag> for TextEntityTypeCashtag {
    fn as_ref(&self) -> &TextEntityTypeCashtag {
        self
    }
}

impl AsRef<TextEntityTypeCashtag> for TextEntityTypeCashtagBuilder {
    fn as_ref(&self) -> &TextEntityTypeCashtag {
        &self.inner
    }
}

/// Text that must be formatted as if inside a code HTML tag
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeCode {}

impl TextEntityTypeCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeCodeBuilder {
        let mut inner = TextEntityTypeCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeCodeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypeCodeBuilder {
    inner: TextEntityTypeCode,
}

#[deprecated]
pub type RTDTextEntityTypeCodeBuilder = TextEntityTypeCodeBuilder;

impl TextEntityTypeCodeBuilder {
    pub fn build(&self) -> TextEntityTypeCode {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeCode> for TextEntityTypeCode {
    fn as_ref(&self) -> &TextEntityTypeCode {
        self
    }
}

impl AsRef<TextEntityTypeCode> for TextEntityTypeCodeBuilder {
    fn as_ref(&self) -> &TextEntityTypeCode {
        &self.inner
    }
}

/// An email address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeEmailAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeEmailAddress {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeEmailAddress {}

impl TextEntityTypeEmailAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeEmailAddressBuilder {
        let mut inner = TextEntityTypeEmailAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeEmailAddressBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypeEmailAddressBuilder {
    inner: TextEntityTypeEmailAddress,
}

#[deprecated]
pub type RTDTextEntityTypeEmailAddressBuilder = TextEntityTypeEmailAddressBuilder;

impl TextEntityTypeEmailAddressBuilder {
    pub fn build(&self) -> TextEntityTypeEmailAddress {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeEmailAddress> for TextEntityTypeEmailAddress {
    fn as_ref(&self) -> &TextEntityTypeEmailAddress {
        self
    }
}

impl AsRef<TextEntityTypeEmailAddress> for TextEntityTypeEmailAddressBuilder {
    fn as_ref(&self) -> &TextEntityTypeEmailAddress {
        &self.inner
    }
}

/// A hashtag text, beginning with "#"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeHashtag {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeHashtag {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeHashtag {}

impl TextEntityTypeHashtag {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeHashtagBuilder {
        let mut inner = TextEntityTypeHashtag::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeHashtagBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypeHashtagBuilder {
    inner: TextEntityTypeHashtag,
}

#[deprecated]
pub type RTDTextEntityTypeHashtagBuilder = TextEntityTypeHashtagBuilder;

impl TextEntityTypeHashtagBuilder {
    pub fn build(&self) -> TextEntityTypeHashtag {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeHashtag> for TextEntityTypeHashtag {
    fn as_ref(&self) -> &TextEntityTypeHashtag {
        self
    }
}

impl AsRef<TextEntityTypeHashtag> for TextEntityTypeHashtagBuilder {
    fn as_ref(&self) -> &TextEntityTypeHashtag {
        &self.inner
    }
}

/// An italic text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeItalic {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeItalic {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeItalic {}

impl TextEntityTypeItalic {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeItalicBuilder {
        let mut inner = TextEntityTypeItalic::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeItalicBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypeItalicBuilder {
    inner: TextEntityTypeItalic,
}

#[deprecated]
pub type RTDTextEntityTypeItalicBuilder = TextEntityTypeItalicBuilder;

impl TextEntityTypeItalicBuilder {
    pub fn build(&self) -> TextEntityTypeItalic {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeItalic> for TextEntityTypeItalic {
    fn as_ref(&self) -> &TextEntityTypeItalic {
        self
    }
}

impl AsRef<TextEntityTypeItalic> for TextEntityTypeItalicBuilder {
    fn as_ref(&self) -> &TextEntityTypeItalic {
        &self.inner
    }
}

/// A media timestamp
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeMediaTimestamp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Timestamp from which a video/audio/video note/voice note playing must start, in seconds. The media can be in the content or the web page preview of the current message, or in the same places in the replied message

    #[serde(default)]
    media_timestamp: i32,
}

impl RObject for TextEntityTypeMediaTimestamp {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeMediaTimestamp {}

impl TextEntityTypeMediaTimestamp {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeMediaTimestampBuilder {
        let mut inner = TextEntityTypeMediaTimestamp::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeMediaTimestampBuilder { inner }
    }

    pub fn media_timestamp(&self) -> i32 {
        self.media_timestamp
    }
}

#[doc(hidden)]
pub struct TextEntityTypeMediaTimestampBuilder {
    inner: TextEntityTypeMediaTimestamp,
}

#[deprecated]
pub type RTDTextEntityTypeMediaTimestampBuilder = TextEntityTypeMediaTimestampBuilder;

impl TextEntityTypeMediaTimestampBuilder {
    pub fn build(&self) -> TextEntityTypeMediaTimestamp {
        self.inner.clone()
    }

    pub fn media_timestamp(&mut self, media_timestamp: i32) -> &mut Self {
        self.inner.media_timestamp = media_timestamp;
        self
    }
}

impl AsRef<TextEntityTypeMediaTimestamp> for TextEntityTypeMediaTimestamp {
    fn as_ref(&self) -> &TextEntityTypeMediaTimestamp {
        self
    }
}

impl AsRef<TextEntityTypeMediaTimestamp> for TextEntityTypeMediaTimestampBuilder {
    fn as_ref(&self) -> &TextEntityTypeMediaTimestamp {
        &self.inner
    }
}

/// A mention of a user by their username
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeMention {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeMention {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeMention {}

impl TextEntityTypeMention {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeMentionBuilder {
        let mut inner = TextEntityTypeMention::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeMentionBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypeMentionBuilder {
    inner: TextEntityTypeMention,
}

#[deprecated]
pub type RTDTextEntityTypeMentionBuilder = TextEntityTypeMentionBuilder;

impl TextEntityTypeMentionBuilder {
    pub fn build(&self) -> TextEntityTypeMention {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeMention> for TextEntityTypeMention {
    fn as_ref(&self) -> &TextEntityTypeMention {
        self
    }
}

impl AsRef<TextEntityTypeMention> for TextEntityTypeMentionBuilder {
    fn as_ref(&self) -> &TextEntityTypeMention {
        &self.inner
    }
}

/// A text shows instead of a raw mention of the user (e.g., when the user has no username)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeMentionName {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the mentioned user

    #[serde(default)]
    user_id: i64,
}

impl RObject for TextEntityTypeMentionName {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeMentionName {}

impl TextEntityTypeMentionName {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeMentionNameBuilder {
        let mut inner = TextEntityTypeMentionName::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeMentionNameBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct TextEntityTypeMentionNameBuilder {
    inner: TextEntityTypeMentionName,
}

#[deprecated]
pub type RTDTextEntityTypeMentionNameBuilder = TextEntityTypeMentionNameBuilder;

impl TextEntityTypeMentionNameBuilder {
    pub fn build(&self) -> TextEntityTypeMentionName {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<TextEntityTypeMentionName> for TextEntityTypeMentionName {
    fn as_ref(&self) -> &TextEntityTypeMentionName {
        self
    }
}

impl AsRef<TextEntityTypeMentionName> for TextEntityTypeMentionNameBuilder {
    fn as_ref(&self) -> &TextEntityTypeMentionName {
        &self.inner
    }
}

/// A phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypePhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypePhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypePhoneNumber {}

impl TextEntityTypePhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypePhoneNumberBuilder {
        let mut inner = TextEntityTypePhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypePhoneNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypePhoneNumberBuilder {
    inner: TextEntityTypePhoneNumber,
}

#[deprecated]
pub type RTDTextEntityTypePhoneNumberBuilder = TextEntityTypePhoneNumberBuilder;

impl TextEntityTypePhoneNumberBuilder {
    pub fn build(&self) -> TextEntityTypePhoneNumber {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypePhoneNumber> for TextEntityTypePhoneNumber {
    fn as_ref(&self) -> &TextEntityTypePhoneNumber {
        self
    }
}

impl AsRef<TextEntityTypePhoneNumber> for TextEntityTypePhoneNumberBuilder {
    fn as_ref(&self) -> &TextEntityTypePhoneNumber {
        &self.inner
    }
}

/// Text that must be formatted as if inside a pre HTML tag
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypePre {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypePre {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypePre {}

impl TextEntityTypePre {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypePreBuilder {
        let mut inner = TextEntityTypePre::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypePreBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypePreBuilder {
    inner: TextEntityTypePre,
}

#[deprecated]
pub type RTDTextEntityTypePreBuilder = TextEntityTypePreBuilder;

impl TextEntityTypePreBuilder {
    pub fn build(&self) -> TextEntityTypePre {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypePre> for TextEntityTypePre {
    fn as_ref(&self) -> &TextEntityTypePre {
        self
    }
}

impl AsRef<TextEntityTypePre> for TextEntityTypePreBuilder {
    fn as_ref(&self) -> &TextEntityTypePre {
        &self.inner
    }
}

/// Text that must be formatted as if inside pre, and code HTML tags
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypePreCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Programming language of the code; as defined by the sender

    #[serde(default)]
    language: String,
}

impl RObject for TextEntityTypePreCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypePreCode {}

impl TextEntityTypePreCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypePreCodeBuilder {
        let mut inner = TextEntityTypePreCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypePreCodeBuilder { inner }
    }

    pub fn language(&self) -> &String {
        &self.language
    }
}

#[doc(hidden)]
pub struct TextEntityTypePreCodeBuilder {
    inner: TextEntityTypePreCode,
}

#[deprecated]
pub type RTDTextEntityTypePreCodeBuilder = TextEntityTypePreCodeBuilder;

impl TextEntityTypePreCodeBuilder {
    pub fn build(&self) -> TextEntityTypePreCode {
        self.inner.clone()
    }

    pub fn language<T: AsRef<str>>(&mut self, language: T) -> &mut Self {
        self.inner.language = language.as_ref().to_string();
        self
    }
}

impl AsRef<TextEntityTypePreCode> for TextEntityTypePreCode {
    fn as_ref(&self) -> &TextEntityTypePreCode {
        self
    }
}

impl AsRef<TextEntityTypePreCode> for TextEntityTypePreCodeBuilder {
    fn as_ref(&self) -> &TextEntityTypePreCode {
        &self.inner
    }
}

/// A strikethrough text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeStrikethrough {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeStrikethrough {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeStrikethrough {}

impl TextEntityTypeStrikethrough {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeStrikethroughBuilder {
        let mut inner = TextEntityTypeStrikethrough::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeStrikethroughBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypeStrikethroughBuilder {
    inner: TextEntityTypeStrikethrough,
}

#[deprecated]
pub type RTDTextEntityTypeStrikethroughBuilder = TextEntityTypeStrikethroughBuilder;

impl TextEntityTypeStrikethroughBuilder {
    pub fn build(&self) -> TextEntityTypeStrikethrough {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeStrikethrough> for TextEntityTypeStrikethrough {
    fn as_ref(&self) -> &TextEntityTypeStrikethrough {
        self
    }
}

impl AsRef<TextEntityTypeStrikethrough> for TextEntityTypeStrikethroughBuilder {
    fn as_ref(&self) -> &TextEntityTypeStrikethrough {
        &self.inner
    }
}

/// A text description shown instead of a raw URL
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeTextUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// HTTP or tg:// URL to be opened when the link is clicked

    #[serde(default)]
    url: String,
}

impl RObject for TextEntityTypeTextUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeTextUrl {}

impl TextEntityTypeTextUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeTextUrlBuilder {
        let mut inner = TextEntityTypeTextUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeTextUrlBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct TextEntityTypeTextUrlBuilder {
    inner: TextEntityTypeTextUrl,
}

#[deprecated]
pub type RTDTextEntityTypeTextUrlBuilder = TextEntityTypeTextUrlBuilder;

impl TextEntityTypeTextUrlBuilder {
    pub fn build(&self) -> TextEntityTypeTextUrl {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<TextEntityTypeTextUrl> for TextEntityTypeTextUrl {
    fn as_ref(&self) -> &TextEntityTypeTextUrl {
        self
    }
}

impl AsRef<TextEntityTypeTextUrl> for TextEntityTypeTextUrlBuilder {
    fn as_ref(&self) -> &TextEntityTypeTextUrl {
        &self.inner
    }
}

/// An underlined text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeUnderline {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeUnderline {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeUnderline {}

impl TextEntityTypeUnderline {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeUnderlineBuilder {
        let mut inner = TextEntityTypeUnderline::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeUnderlineBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypeUnderlineBuilder {
    inner: TextEntityTypeUnderline,
}

#[deprecated]
pub type RTDTextEntityTypeUnderlineBuilder = TextEntityTypeUnderlineBuilder;

impl TextEntityTypeUnderlineBuilder {
    pub fn build(&self) -> TextEntityTypeUnderline {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeUnderline> for TextEntityTypeUnderline {
    fn as_ref(&self) -> &TextEntityTypeUnderline {
        self
    }
}

impl AsRef<TextEntityTypeUnderline> for TextEntityTypeUnderlineBuilder {
    fn as_ref(&self) -> &TextEntityTypeUnderline {
        &self.inner
    }
}

/// An HTTP URL
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeUrl {}

impl TextEntityTypeUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeUrlBuilder {
        let mut inner = TextEntityTypeUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeUrlBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypeUrlBuilder {
    inner: TextEntityTypeUrl,
}

#[deprecated]
pub type RTDTextEntityTypeUrlBuilder = TextEntityTypeUrlBuilder;

impl TextEntityTypeUrlBuilder {
    pub fn build(&self) -> TextEntityTypeUrl {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeUrl> for TextEntityTypeUrl {
    fn as_ref(&self) -> &TextEntityTypeUrl {
        self
    }
}

impl AsRef<TextEntityTypeUrl> for TextEntityTypeUrlBuilder {
    fn as_ref(&self) -> &TextEntityTypeUrl {
        &self.inner
    }
}

/// TextEntityTypeCustomEmoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeCustomEmoji {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(default)]
    custom_emoji_id: String,
}

impl RObject for TextEntityTypeCustomEmoji {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeCustomEmoji {}

impl TextEntityTypeCustomEmoji {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeCustomEmojiBuilder {
        let mut inner = TextEntityTypeCustomEmoji::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeCustomEmojiBuilder { inner }
    }
    pub fn custom_emoji_id(&self) -> &String {
        &self.custom_emoji_id
    }
}

#[doc(hidden)]
pub struct TextEntityTypeCustomEmojiBuilder {
    inner: TextEntityTypeCustomEmoji,
}

#[deprecated]
pub type RTDTextEntityTypeCustomEmojiBuilder = TextEntityTypeCustomEmojiBuilder;

impl TextEntityTypeCustomEmojiBuilder {
    pub fn build(&self) -> TextEntityTypeCustomEmoji {
        self.inner.clone()
    }
    pub fn custom_emoji_id<T>(&mut self, id: T) -> &mut Self
    where
        T: AsRef<str> + Display,
    {
        self.inner.custom_emoji_id = id.to_string();
        self
    }
}

impl AsRef<TextEntityTypeCustomEmoji> for TextEntityTypeCustomEmoji {
    fn as_ref(&self) -> &TextEntityTypeCustomEmoji {
        self
    }
}

impl AsRef<TextEntityTypeCustomEmoji> for TextEntityTypeCustomEmojiBuilder {
    fn as_ref(&self) -> &TextEntityTypeCustomEmoji {
        &self.inner
    }
}
//-----------end by whr--------------------------
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeSpoiler {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    // #[serde(default)]
    // #[serde(flatten)]
    // data: Option<Document>,
}

impl RObject for TextEntityTypeSpoiler {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeSpoiler {}

impl TextEntityTypeSpoiler {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeSpoilerBuilder {
        let mut inner = TextEntityTypeSpoiler::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeSpoilerBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypeSpoilerBuilder {
    inner: TextEntityTypeSpoiler,
}

#[deprecated]
pub type RTDTextEntityTypeSpoilerBuilder = TextEntityTypeSpoilerBuilder;

impl TextEntityTypeSpoilerBuilder {
    pub fn build(&self) -> TextEntityTypeSpoiler {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeSpoiler> for TextEntityTypeSpoiler {
    fn as_ref(&self) -> &TextEntityTypeSpoiler {
        self
    }
}

impl AsRef<TextEntityTypeSpoiler> for TextEntityTypeSpoilerBuilder {
    fn as_ref(&self) -> &TextEntityTypeSpoiler {
        &self.inner
    }
}
//-------------------------------------
/// A bold text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeBlockQuote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeBlockQuote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeBlockQuote {}

impl TextEntityTypeBlockQuote {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeBlockQuoteBuilder {
        let mut inner = TextEntityTypeBlockQuote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeBlockQuoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypeBlockQuoteBuilder {
    inner: TextEntityTypeBlockQuote,
}

#[deprecated]
pub type RTDTextEntityTypeBlockQuoteBuilder = TextEntityTypeBlockQuoteBuilder;

impl TextEntityTypeBlockQuoteBuilder {
    pub fn build(&self) -> TextEntityTypeBlockQuote {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeBlockQuote> for TextEntityTypeBlockQuote {
    fn as_ref(&self) -> &TextEntityTypeBlockQuote {
        self
    }
}

impl AsRef<TextEntityTypeBlockQuote> for TextEntityTypeBlockQuoteBuilder {
    fn as_ref(&self) -> &TextEntityTypeBlockQuote {
        &self.inner
    }
}
//-------------------------------------
/// A bold text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeExpandableBlockQuote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeExpandableBlockQuote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeExpandableBlockQuote {}

impl TextEntityTypeExpandableBlockQuote {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntityTypeExpandableBlockQuoteBuilder {
        let mut inner = TextEntityTypeExpandableBlockQuote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntityTypeExpandableBlockQuoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextEntityTypeExpandableBlockQuoteBuilder {
    inner: TextEntityTypeExpandableBlockQuote,
}

#[deprecated]
pub type RTDTextEntityTypeExpandableBlockQuoteBuilder = TextEntityTypeExpandableBlockQuoteBuilder;

impl TextEntityTypeExpandableBlockQuoteBuilder {
    pub fn build(&self) -> TextEntityTypeExpandableBlockQuote {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeExpandableBlockQuote> for TextEntityTypeExpandableBlockQuote {
    fn as_ref(&self) -> &TextEntityTypeExpandableBlockQuote {
        self
    }
}

impl AsRef<TextEntityTypeExpandableBlockQuote> for TextEntityTypeExpandableBlockQuoteBuilder {
    fn as_ref(&self) -> &TextEntityTypeExpandableBlockQuote {
        &self.inner
    }
}

#[test]
fn aa() {
    //---------------------
    let s = r#"
"㊗️㊗️㊗️㊗️㊗️㊗️㊗️㊗️㊗️㊗️㊗️㊗️\n\n【采集猫】V2.1.0版本已发布，请及时更新！\n\n🏃‍♂️🏃‍♂️🏃‍♂️🏃‍♂️🏃‍♂️🏃‍♂️🏃‍♂️🏃‍♂️🏃‍♂️🏃‍♂️🏃‍♂️🏃‍♂️\n\n更新内容如下（更新后无需做任何其他设置）：\n\n1️⃣优化了登陆流程，解决了登陆过程的一个BUG；\n2️⃣优化了重启软件后恢复缓冲区数据的逻辑，大大降低了软件请求飞机服务器的频次（图一）；\n3️⃣规则设置增加了模板，新增一条规则以后，将该条规则选中（整条规则变蓝色即为选中，鼠标点击一下该条规则的空白处即可选中），点击新增规则，那么新增加的规则将以选中的规则作为模板复制其“爬取类型”“历史还是当前还是所有”“替换规则”“转发群”，方便规则很多的用户快速复用规则参数，同时也新增了【全部删除】按钮，可以一键清空“来源群”或者“目标群”的所有内容；（图二）\n4️⃣将自动下载的受保护资源的保存期限延长到了一周（基于 恢复发送记录逻辑的更改 导致的）。\n5️⃣软件页面增加了一些操作提示，对新用户更友好。\n6️⃣规则设置底部增加了一个【查找】功能，支持模糊匹配，方便规则很多的用户快速定位想要修改的规则（图二）。\n7️⃣优化了合辑发送的逻辑，合辑发送功能更强大，进一步降低了合并合辑的时候的割裂可能；\n8️⃣其他更多细节，请更新软件后体验；\n\n❤️❤️本次更新针对了一些功能性缺失的修复，如无必要，可不用更新，🎯🎯\n推荐大家更新！\n❤️❤️\n\n============================\n\n更新方式：\n\n直接关闭采集猫软件，点击软件目录下的 update-crawler.exe，将会自动开始更新，更新结束后会提示已更新完成，完成后关闭更新软件，双击 软件目录下的ui-crawler.exe即可正常启动采集猫！\n\n============================\n\n新用户如何下载采集猫软件呢？\n\n点击 置顶 的消息，附件里面有个 update- crawler.exe，下载下来，在电脑某个盘新建一个文件夹，将该附件 移动进去，双击即可开始更新，更新完毕后，双击同目录下的 ui-cralwer.exe即可正常启动采集猫！（所有路径暂不支持中文，新建的文件夹名字请使用英文或数字）\n\n\n🔴🔴🔴唯一客服👍👍👍： @TGcaiji\n🔴🔴🔴官方群组：@aodipay (如果无法私聊客服，请加入群组@我们）\n🔴🔴🔴技术团队👍👍👍： @shixiong_da","entities":
    "#;
    println!("-----------{}-----------", s.len());
}
