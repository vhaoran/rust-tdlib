use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddedProxy {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Proxy server IP address
    proxy: Proxy,

    /// int32 	id_
    /// Unique identifier of the proxy.
    id: i32,

    /// int32 	last_used_date_
    /// Point in time (Unix timestamp) when the proxy was last used; 0 if never.
    last_used_date: i32,

    /// True, if the proxy needs to be enabled
    #[serde(default)]
    is_enabled: bool,

    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_type: String,
}

impl RObject for AddedProxy {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddedProxy {}

impl AddedProxy {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AddedProxyBuilder {
        let mut inner = AddedProxy::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addedProxy".to_string();

        AddedProxyBuilder { inner }
    }
    pub fn proxy(&self) -> &Proxy {
        &self.proxy
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn last_used_date(&self) -> i32 {
        self.last_used_date
    }
    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }
}

#[doc(hidden)]
pub struct AddedProxyBuilder {
    inner: AddedProxy,
}

// #[deprecated]
// pub type RTDAddedProxyBuilder = AddedProxyBuilder;

impl AddedProxyBuilder {
    pub fn build(&self) -> AddedProxy {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }
    pub fn last_used_date(&mut self, last_used_date: i32) -> &mut Self {
        self.inner.last_used_date = last_used_date;
        self
    }
    pub fn is_enabled(&mut self, is_enabled: bool) -> &mut Self {
        self.inner.is_enabled = is_enabled;
        self
    }
    pub fn proxy(&mut self, proxy: Proxy) -> &mut Self {
        self.inner.proxy = proxy;
        self
    }
}

impl AsRef<AddedProxy> for AddedProxy {
    fn as_ref(&self) -> &AddedProxy {
        self
    }
}

impl AsRef<AddedProxy> for AddedProxyBuilder {
    fn as_ref(&self) -> &AddedProxy {
        &self.inner
    }
}

mod t {
    use crate::types::AddedProxy;

    #[test]
    fn test_added_proxy() {
        let s = r#"
    {
    "@type":"addedProxy",
    "id":2,
    "last_used_date":1775965797,
    "is_enabled":true,
    "proxy":{
        "@type":"proxy",
        "server":"127.0.0.1",
        "port":7890,
        "type":{"@type":"proxyTypeSocks5","username":"","password":""}
    },
    "@extra":"3845d607-421d-475f-b0cb-14f4f10fa64f",
    "@client_id":1
    }        
        "#;
        let r = AddedProxy::from_json(s);
        println!("-----------{r:?}-----------",);
    }
}
