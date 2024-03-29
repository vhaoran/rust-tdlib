use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the way the text needs to be parsed for TextEntities
pub trait TDTextParseMode: Debug + RObject {}

/// Describes the way the text needs to be parsed for TextEntities
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TextParseMode {
    #[doc(hidden)]
    _Default,
    /// The text uses HTML-style formatting. The same as Telegram Bot API "HTML" parse mode
    #[serde(rename = "textParseModeHTML")]
    HTML(TextParseModeHTML),
    /// The text uses Markdown-style formatting
    #[serde(rename = "textParseModeMarkdown")]
    Markdown(TextParseModeMarkdown),
}

impl Default for TextParseMode {
    fn default() -> Self {
        TextParseMode::_Default
    }
}

impl RObject for TextParseMode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            TextParseMode::HTML(t) => t.extra(),
            TextParseMode::Markdown(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            TextParseMode::HTML(t) => t.client_id(),
            TextParseMode::Markdown(t) => t.client_id(),

            _ => None,
        }
    }
}

impl TextParseMode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, TextParseMode::_Default)
    }
}

impl AsRef<TextParseMode> for TextParseMode {
    fn as_ref(&self) -> &TextParseMode {
        self
    }
}

/// The text uses HTML-style formatting. The same as Telegram Bot API "HTML" parse mode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextParseModeHTML {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextParseModeHTML {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextParseMode for TextParseModeHTML {}

impl TextParseModeHTML {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextParseModeHTMLBuilder {
        let mut inner = TextParseModeHTML::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextParseModeHTMLBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TextParseModeHTMLBuilder {
    inner: TextParseModeHTML,
}

#[deprecated]
pub type RTDTextParseModeHTMLBuilder = TextParseModeHTMLBuilder;

impl TextParseModeHTMLBuilder {
    pub fn build(&self) -> TextParseModeHTML {
        self.inner.clone()
    }
}

impl AsRef<TextParseModeHTML> for TextParseModeHTML {
    fn as_ref(&self) -> &TextParseModeHTML {
        self
    }
}

impl AsRef<TextParseModeHTML> for TextParseModeHTMLBuilder {
    fn as_ref(&self) -> &TextParseModeHTML {
        &self.inner
    }
}

/// The text uses Markdown-style formatting
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextParseModeMarkdown {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Version of the parser: 0 or 1  Telegram Bot API "Markdown" parse mode, 2  Telegram Bot API "MarkdownV2" parse mode

    #[serde(default)]
    version: i32,
}

impl RObject for TextParseModeMarkdown {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextParseMode for TextParseModeMarkdown {}

impl TextParseModeMarkdown {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextParseModeMarkdownBuilder {
        let mut inner = TextParseModeMarkdown::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextParseModeMarkdownBuilder { inner }
    }

    pub fn version(&self) -> i32 {
        self.version
    }
}

#[doc(hidden)]
pub struct TextParseModeMarkdownBuilder {
    inner: TextParseModeMarkdown,
}

#[deprecated]
pub type RTDTextParseModeMarkdownBuilder = TextParseModeMarkdownBuilder;

impl TextParseModeMarkdownBuilder {
    pub fn build(&self) -> TextParseModeMarkdown {
        self.inner.clone()
    }

    pub fn version(&mut self, version: i32) -> &mut Self {
        self.inner.version = version;
        self
    }
}

impl AsRef<TextParseModeMarkdown> for TextParseModeMarkdown {
    fn as_ref(&self) -> &TextParseModeMarkdown {
        self
    }
}

impl AsRef<TextParseModeMarkdown> for TextParseModeMarkdownBuilder {
    fn as_ref(&self) -> &TextParseModeMarkdown {
        &self.inner
    }
}
