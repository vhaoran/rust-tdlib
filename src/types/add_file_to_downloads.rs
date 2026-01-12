use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Downloads a file from the cloud. Download progress and completion of the download will be notified through updateFile updates
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddFileToDownloads {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    /// int32 	file_id_
    /// Identifier of the file to download.
    #[serde(default)]
    file_id: i32,

    /// int53 	chat_id_
    //? Chat identifier of the message with the file.
    #[serde(default)]
    chat_id: i64,
    // int53 	message_id_
    // Message identifier.
    /// int53 	message_id_
    /// Message identifier.
    #[serde(default)]
    message_id: i64,

    /// int32 	priority_
    /// Priority of the download (1-32). The higher the priority, the earlier the file will be downloaded. If the priorities of two files are equal, then the last one for which AddFileToDownloads was called will be downloaded first.
    #[serde(default)]
    priority: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddFileToDownloads {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddFileToDownloads {}

impl AddFileToDownloads {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AddFileToDownloadsBuilder {
        let mut inner = AddFileToDownloads::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addFileToDownloads".to_string();

        AddFileToDownloadsBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }

    pub fn priority(&self) -> i32 {
        self.priority
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct AddFileToDownloadsBuilder {
    inner: AddFileToDownloads,
}

#[deprecated]
pub type RTDAddFileToDownloadsBuilder = AddFileToDownloadsBuilder;

impl AddFileToDownloadsBuilder {
    pub fn build(&self) -> AddFileToDownloads {
        self.inner.clone()
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;
        self
    }

    pub fn priority(&mut self, priority: i32) -> &mut Self {
        self.inner.priority = priority;
        self
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }
}

impl AsRef<AddFileToDownloads> for AddFileToDownloads {
    fn as_ref(&self) -> &AddFileToDownloads {
        self
    }
}

impl AsRef<AddFileToDownloads> for AddFileToDownloadsBuilder {
    fn as_ref(&self) -> &AddFileToDownloads {
        &self.inner
    }
}
