use crate::client::tdlib_client::TdJson;
use crate::client::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::OnceCell;
use tracing::debug;

#[derive(Clone, Default, Debug)]
pub struct FnStats {
    pub fn_name: String,
    pub count: i64,
}

/// key: client-uuid
type IType = HashMap<String, Vec<FnStats>>;

async fn instance() -> &'static Arc<Mutex<IType>> {
    static INSTANCE: OnceCell<Arc<Mutex<IType>>> = OnceCell::const_new();
    INSTANCE
        .get_or_init(|| async {
            let m = IType::new();
            Arc::new(Mutex::new(m))
        })
        .await
}

pub struct VFnStats;

impl VFnStats {
    pub async fn push_json<T: AsRef<str> + std::fmt::Display>(uuid: T, json: String) {
        let uuid = uuid.to_string();
        let tp = match json::parse(json.as_str()) {
            Ok(m) => {
                if m["@type"].is_string() {
                    m["@type"].to_string()
                } else {
                    "".to_string()
                }
            }
            _ => "".to_string(),
        };

        if !tp.is_empty() {
            Self::push_of(uuid.as_str(), tp.as_str(), 1).await;
        }
    }
}

impl VFnStats {
    pub async fn get_copy_sorted() -> IType {
        let mut a = {
            let a = self::instance().await.clone();
            let m = a.lock().await;
            m.clone()
        };
        for (_, v) in a.iter_mut() {
            v.sort_by(|x, y| y.count.cmp(&x.count));
        }
        a
    }

    pub async fn push_of_client<T: AsRef<str> + std::fmt::Display>(
        client: &Client<TdJson>,
        fn_name: T,
        count: usize,
    ) {
        let fn_name = fn_name.to_string();

        let uuid = client.uuid().to_owned();
        Self::push_of(&uuid, fn_name.as_str(), count).await;
    }

    pub async fn push_of(uuid: &str, fn_name: &str, count: usize) {
        let a = self::instance().await.clone();
        let mut m = a.lock().await;
        let uuid = uuid.to_string();
        let stats = m.entry(uuid).or_insert(Vec::new());

        let row = stats.iter_mut().find(|x| x.fn_name.as_str() == fn_name);

        if let Some(v) = row {
            v.count += count as i64;
        } else {
            stats.push(FnStats {
                fn_name: fn_name.to_string(),
                count: count as i64,
            });
        }
    }
}
