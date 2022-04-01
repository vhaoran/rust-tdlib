use crate::types::{UpdateUserStatus, UserStatus, UserStatusOnline};
//
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[test]
fn a_1() {
    let src = super::Update::UserStatus(
        UpdateUserStatus::builder()
            .user_id(10_i64)
            .status(UserStatus::Online(
                UserStatusOnline::builder().expires(500).build(),
            ))
            .build(),
    );
    let s = serde_json::to_string(&src).unwrap_or("no data".to_string());
    println!("-----------{}-----------", s);
}

#[test]
fn t1_1() {
    // #[serde(rename = "@type")]
    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(tag = "@type")]
    pub enum XType {
        #[serde(rename(serialize = "updateUserStatus", deserialize = "updateUserStatus"))]
        A(XA),
    }
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct XA {
        pub id: i64,
    }
    //-------------------------------------
    let src = XType::A(XA { id: 1 });
    let s = serde_json::to_string(&src).unwrap_or("no data".to_string());
    println!("-----------{}-----------", s);
}
