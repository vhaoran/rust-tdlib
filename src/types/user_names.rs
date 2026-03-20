use crate::types::RObject;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserNames {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    ///     array< string > 	active_usernames_
    ///     List of active usernames;
    /// the first one must be shown as the primary username.
    /// The order of active usernames can be changed with reorderActiveUsernames, reorderBotActiveUsernames or reorderSupergroupActiveUsernames.
    #[serde(default)]
    active_usernames: Vec<String>,

    /// array< string > 	disabled_usernames_
    /// List of currently disabled usernames;
    /// the username can be activated with toggleUsernameIsActive,
    /// toggleBotUsernameIsActive, or toggleSupergroupUsernameIsActive.
    #[serde(default)]
    disabled_usernames: Vec<String>,

    ///
    /// string 	editable_username_
    /// The active username,
    /// which can be changed with setUsername or setSupergroupUsername.
    /// Information about other active usernames can be received using getCollectibleItemInfo.
    ///
    #[serde(default)]
    editable_username: String,
    #[serde(default)]
    collectible_usernames: Vec<String>,

    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_type: String,
}

impl RObject for UserNames {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl UserNames {
    pub fn from_json<S: AsRef<str>>(json: S) -> crate::errors::Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserNamesBuilder {
        let mut inner = UserNames::default();
        // inner.extra = Some(Uuid::new_v4().to_string());
        inner.td_type = "usernames".to_string();

        UserNamesBuilder { inner }
    }
    pub fn active_usernames(&self) -> &Vec<String> {
        &self.active_usernames
    }
    pub fn disabled_usernames(&self) -> &Vec<String> {
        &self.disabled_usernames
    }
    pub fn editable_username(&self) -> &String {
        &self.editable_username
    }
}

#[doc(hidden)]
pub struct UserNamesBuilder {
    inner: UserNames,
}

#[deprecated]
pub type RTDUserNamesBuilder = UserNamesBuilder;

impl UserNamesBuilder {
    pub fn build(&self) -> UserNames {
        self.inner.clone()
    }
    pub fn active_usernames(&mut self, l: Vec<String>) -> &mut Self {
        self.inner.active_usernames = l;
        self
    }
    pub fn disabled_usernames(&mut self, l: Vec<String>) -> &mut Self {
        self.inner.disabled_usernames = l;
        self
    }
    pub fn editable_username(&mut self, n: String) -> &mut Self {
        self.inner.editable_username = n;
        self
    }
}

impl AsRef<UserNames> for UserNames {
    fn as_ref(&self) -> &UserNames {
        self
    }
}

impl AsRef<UserNames> for UserNamesBuilder {
    fn as_ref(&self) -> &UserNames {
        &self.inner
    }
}
