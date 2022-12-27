use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns database statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDatabaseStatistics {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetDatabaseStatistics {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetDatabaseStatistics {}

impl GetDatabaseStatistics {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetDatabaseStatisticsBuilder {
        let mut inner = GetDatabaseStatistics::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getDatabaseStatistics".to_string();

        GetDatabaseStatisticsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetDatabaseStatisticsBuilder {
    inner: GetDatabaseStatistics,
}

#[deprecated]
pub type RTDGetDatabaseStatisticsBuilder = GetDatabaseStatisticsBuilder;

impl GetDatabaseStatisticsBuilder {
    pub fn build(&self) -> GetDatabaseStatistics {
        self.inner.clone()
    }
}

impl AsRef<GetDatabaseStatistics> for GetDatabaseStatistics {
    fn as_ref(&self) -> &GetDatabaseStatistics {
        self
    }
}

impl AsRef<GetDatabaseStatistics> for GetDatabaseStatisticsBuilder {
    fn as_ref(&self) -> &GetDatabaseStatistics {
        &self.inner
    }
}
