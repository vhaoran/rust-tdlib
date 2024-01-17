use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a local file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocalFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Local path to the locally available file part; may be empty

    #[serde(default)]
    path: String,
    /// True, if it is possible to download or generate the file

    #[serde(default)]
    can_be_downloaded: bool,
    /// True, if the file can be deleted

    #[serde(default)]
    can_be_deleted: bool,
    /// True, if the file is currently being downloaded (or a local copy is being generated by some other means)

    #[serde(default)]
    is_downloading_active: bool,
    /// True, if the local copy is fully available

    #[serde(default)]
    is_downloading_completed: bool,
    /// Download will be started from this offset. downloaded_prefix_size is calculated from this offset

    #[serde(default)]
    download_offset: i64,
    /// If is_downloading_completed is false, then only some prefix of the file starting from download_offset is ready to be read. downloaded_prefix_size is the size of that prefix in bytes

    #[serde(default)]
    downloaded_prefix_size: i64,
    /// Total downloaded file size, in bytes. Can be used only for calculating download progress. The actual file size may be bigger, and some parts of it may contain garbage

    #[serde(default)]
    downloaded_size: i32,
}

impl RObject for LocalFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl LocalFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> LocalFileBuilder {
        let mut inner = LocalFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        LocalFileBuilder { inner }
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn can_be_downloaded(&self) -> bool {
        self.can_be_downloaded
    }

    pub fn can_be_deleted(&self) -> bool {
        self.can_be_deleted
    }

    pub fn is_downloading_active(&self) -> bool {
        self.is_downloading_active
    }

    pub fn is_downloading_completed(&self) -> bool {
        self.is_downloading_completed
    }

    pub fn download_offset(&self) -> i64 {
        self.download_offset
    }

    pub fn downloaded_prefix_size(&self) -> i64 {
        self.downloaded_prefix_size
    }

    pub fn downloaded_size(&self) -> i32 {
        self.downloaded_size
    }
}

#[doc(hidden)]
pub struct LocalFileBuilder {
    inner: LocalFile,
}

#[deprecated]
pub type RTDLocalFileBuilder = LocalFileBuilder;

impl LocalFileBuilder {
    pub fn build(&self) -> LocalFile {
        self.inner.clone()
    }

    pub fn path<T: AsRef<str>>(&mut self, path: T) -> &mut Self {
        self.inner.path = path.as_ref().to_string();
        self
    }

    pub fn can_be_downloaded(&mut self, can_be_downloaded: bool) -> &mut Self {
        self.inner.can_be_downloaded = can_be_downloaded;
        self
    }

    pub fn can_be_deleted(&mut self, can_be_deleted: bool) -> &mut Self {
        self.inner.can_be_deleted = can_be_deleted;
        self
    }

    pub fn is_downloading_active(&mut self, is_downloading_active: bool) -> &mut Self {
        self.inner.is_downloading_active = is_downloading_active;
        self
    }

    pub fn is_downloading_completed(&mut self, is_downloading_completed: bool) -> &mut Self {
        self.inner.is_downloading_completed = is_downloading_completed;
        self
    }

    pub fn download_offset(&mut self, download_offset: i64) -> &mut Self {
        self.inner.download_offset = download_offset;
        self
    }

    pub fn downloaded_prefix_size(&mut self, downloaded_prefix_size: i64) -> &mut Self {
        self.inner.downloaded_prefix_size = downloaded_prefix_size;
        self
    }

    pub fn downloaded_size(&mut self, downloaded_size: i32) -> &mut Self {
        self.inner.downloaded_size = downloaded_size;
        self
    }
}

impl AsRef<LocalFile> for LocalFile {
    fn as_ref(&self) -> &LocalFile {
        self
    }
}

impl AsRef<LocalFile> for LocalFileBuilder {
    fn as_ref(&self) -> &LocalFile {
        &self.inner
    }
}
