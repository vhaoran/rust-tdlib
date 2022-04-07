use crate::types::{UpdateUserStatus, UserStatus, UserStatusOnline};
//
use crate::types::*;
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

#[test]
fn json_1() {
    //---------------------
    let s = r#"
    {
 "@type": "updateChatThemes" ,
 "chat_themes": [
  {
   "@type": "chatTheme" ,
   "name": "\ud83d\udc25" ,
   "light_settings": {
    "@type": "themeSettings" ,
    "accent_color": -14175887 ,
    "background": {
     "@type": "background" ,
     "id": "5784882925472317444" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "lp0prF8ISFAEAAAA_p385_CvG0w" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 155 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 10 ,
        "size": 20067 ,
        "expected_size": 20067 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhDFzqLglD1zuaFAaQfwZXAvNBgAAmYNAALKlThS4z4c8Z142qQBAAdtAAMiBA" ,
         "unique_id": "AQADZg0AAsqVOFJy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 20067
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 11 ,
       "size": 51194 ,
       "expected_size": 51194 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQxc6i4JQ9c7mhQGkH8GVwLzQYAAJmDQACypU4UuM-HPGdeNqkIgQ" ,
        "unique_id": "AgADZg0AAsqVOFI" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 51194
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        8895909 ,
        12180867 ,
        14014604 ,
        10864773
       ]
      } ,
      "intensity": 42 ,
      "is_inverted": false ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillFreeformGradient" ,
     "colors": [
      -1903140 ,
      -1248586 ,
      -1247555 ,
      -2230590
     ]
    } ,
    "animate_outgoing_message_fill": true ,
    "outgoing_message_accent_color": -11816859
   } ,
   "dark_settings": {
    "@type": "themeSettings" ,
    "accent_color": -8073627 ,
    "background": {
     "@type": "background" ,
     "id": "5784882925472317444" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "lp0prF8ISFAEAAAA_p385_CvG0w" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 155 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 10 ,
        "size": 20067 ,
        "expected_size": 20067 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhDFzqLglD1zuaFAaQfwZXAvNBgAAmYNAALKlThS4z4c8Z142qQBAAdtAAMiBA" ,
         "unique_id": "AQADZg0AAsqVOFJy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 20067
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 11 ,
       "size": 51194 ,
       "expected_size": 51194 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQxc6i4JQ9c7mhQGkH8GVwLzQYAAJmDQACypU4UuM-HPGdeNqkIgQ" ,
        "unique_id": "AgADZg0AAsqVOFI" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 51194
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        2440488 ,
        10854263 ,
        2965046 ,
        8164449
       ]
      } ,
      "intensity": 58 ,
      "is_inverted": true ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillFreeformGradient" ,
     "colors": [
      -9269190 ,
      -11694255 ,
      -13269399 ,
      -10842044
     ]
    } ,
    "animate_outgoing_message_fill": true ,
    "outgoing_message_accent_color": -8073627
   }
  }
 ],
 "@client_id":1
}
    "#;
    let src: UpdateChatAction = serde_json::from_str(s).unwrap();
    println!("-----------src: {:?}-----------", src);

    let src: TdType = serde_json::from_str(s).unwrap();
    println!("-----------src: {:?}-----------", src);
    //
}

#[test]
fn json_2() {
    //---------------------
    let s = r#"
    {
 "@type": "updateChatThemes" ,
 "chat_themes": [
  {
   "@type": "chatTheme" ,
   "name": "\ud83d\udc25" ,
   "light_settings": {
    "@type": "themeSettings" ,
    "accent_color": -14175887 ,
    "background": {
     "@type": "background" ,
     "id": "5784882925472317444" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "lp0prF8ISFAEAAAA_p385_CvG0w" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 155 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 10 ,
        "size": 20067 ,
        "expected_size": 20067 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhBI0Z1l-4tqU1ssUUFHDOAMPdcAAmYNAALKlThSzGdHUpCovZABAAdtAAMjBA" ,
         "unique_id": "AQADZg0AAsqVOFJy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 20067
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 11 ,
       "size": 51194 ,
       "expected_size": 51194 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQSNGdZfuLalNbLFFBRwzgDD3XAAJmDQACypU4UsxnR1KQqL2QIwQ" ,
        "unique_id": "AgADZg0AAsqVOFI" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 51194
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        8895909 ,
        12180867 ,
        14014604 ,
        10864773
       ]
      } ,
      "intensity": 42 ,
      "is_inverted": false ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillFreeformGradient" ,
     "colors": [
      -1903140 ,
      -1248586 ,
      -1247555 ,
      -2230590
     ]
    } ,
    "animate_outgoing_message_fill": true ,
    "outgoing_message_accent_color": -11816859
   } ,
   "dark_settings": {
    "@type": "themeSettings" ,
    "accent_color": -8073627 ,
    "background": {
     "@type": "background" ,
     "id": "5784882925472317444" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "lp0prF8ISFAEAAAA_p385_CvG0w" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 155 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 10 ,
        "size": 20067 ,
        "expected_size": 20067 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhBI0Z1l-4tqU1ssUUFHDOAMPdcAAmYNAALKlThSzGdHUpCovZABAAdtAAMjBA" ,
         "unique_id": "AQADZg0AAsqVOFJy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 20067
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 11 ,
       "size": 51194 ,
       "expected_size": 51194 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQSNGdZfuLalNbLFFBRwzgDD3XAAJmDQACypU4UsxnR1KQqL2QIwQ" ,
        "unique_id": "AgADZg0AAsqVOFI" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 51194
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        2440488 ,
        10854263 ,
        2965046 ,
        8164449
       ]
      } ,
      "intensity": 58 ,
      "is_inverted": true ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillFreeformGradient" ,
     "colors": [
      -9269190 ,
      -11694255 ,
      -13269399 ,
      -10842044
     ]
    } ,
    "animate_outgoing_message_fill": true ,
    "outgoing_message_accent_color": -8073627
   }
  } ,
  {
   "@type": "chatTheme" ,
   "name": "\u26c4" ,
   "light_settings": {
    "@type": "themeSettings" ,
    "accent_color": -12873797 ,
    "background": {
     "@type": "background" ,
     "id": "5785021373743104005" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "MIo6r0qGSFAFAAAAtL8TsDzNX60" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 148 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 12 ,
        "size": 27697 ,
        "expected_size": 27697 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhBI0XSTisZWM_BRQs1Mz2Djx7UAAk8HAAL5B0BQ7icOd7anY1YBAAdtAAMjBA" ,
         "unique_id": "AQADTwcAAvkHQFBy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 27697
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 13 ,
       "size": 100992 ,
       "expected_size": 100992 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQSNF0k4rGVjPwUULNTM9g48e1AAJPBwAC-QdAUO4nDne2p2NWIwQ" ,
        "unique_id": "AgADTwcAAvkHQFA" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 100992
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        8497648 ,
        10408948 ,
        11460306 ,
        12633324
       ]
      } ,
      "intensity": 41 ,
      "is_inverted": false ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillFreeformGradient" ,
     "colors": [
      -1771777 ,
      -852740 ,
      -1508613 ,
      -1576711
     ]
    } ,
    "animate_outgoing_message_fill": true ,
    "outgoing_message_accent_color": -12873797
   } ,
   "dark_settings": {
    "@type": "themeSettings" ,
    "accent_color": -11757585 ,
    "background": {
     "@type": "background" ,
     "id": "5785021373743104005" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "MIo6r0qGSFAFAAAAtL8TsDzNX60" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 148 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 12 ,
        "size": 27697 ,
        "expected_size": 27697 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhBI0XSTisZWM_BRQs1Mz2Djx7UAAk8HAAL5B0BQ7icOd7anY1YBAAdtAAMjBA" ,
         "unique_id": "AQADTwcAAvkHQFBy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 27697
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 13 ,
       "size": 100992 ,
       "expected_size": 100992 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQSNF0k4rGVjPwUULNTM9g48e1AAJPBwAC-QdAUO4nDne2p2NWIwQ" ,
        "unique_id": "AgADTwcAAvkHQFA" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 100992
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        6466222 ,
        2503489 ,
        8427911 ,
        2304827
       ]
      } ,
      "intensity": 57 ,
      "is_inverted": true ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillFreeformGradient" ,
     "colors": [
      -10768716 ,
      -11958892 ,
      -11696975 ,
      -11836513
     ]
    } ,
    "animate_outgoing_message_fill": true ,
    "outgoing_message_accent_color": -11757585
   }
  } ,
  {
   "@type": "chatTheme" ,
   "name": "\ud83d\udc8e" ,
   "light_settings": {
    "@type": "themeSettings" ,
    "accent_color": -4695855 ,
    "background": {
     "@type": "background" ,
     "id": "5951625624761139201" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "p-pXcflrmFIBAAAAvXYQk-mCwZU" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 155 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 14 ,
        "size": 21763 ,
        "expected_size": 21763 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhBI0SDpFlHSa45SR5lSsfgE7ccAAhkMAAJGz0BS0Lxxj6UesjUBAAdtAAMjBA" ,
         "unique_id": "AQADGQwAAkbPQFJy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 21763
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 15 ,
       "size": 93075 ,
       "expected_size": 93075 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQSNEg6RZR0muOUkeZUrH4BO3HAAIZDAACRs9AUtC8cY-lHrI1IwQ" ,
        "unique_id": "AgADGQwAAkbPQFI" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 93075
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        9944811 ,
        11659754 ,
        13021679 ,
        15710172
       ]
      } ,
      "intensity": 50 ,
      "is_inverted": false ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillSolid" ,
     "color": -13520415
    } ,
    "animate_outgoing_message_fill": false ,
    "outgoing_message_accent_color": -4695855
   } ,
   "dark_settings": {
    "@type": "themeSettings" ,
    "accent_color": -2524432 ,
    "background": {
     "@type": "background" ,
     "id": "5951625624761139201" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "p-pXcflrmFIBAAAAvXYQk-mCwZU" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 155 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 14 ,
        "size": 21763 ,
        "expected_size": 21763 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhBI0SDpFlHSa45SR5lSsfgE7ccAAhkMAAJGz0BS0Lxxj6UesjUBAAdtAAMjBA" ,
         "unique_id": "AQADGQwAAkbPQFJy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 21763
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 15 ,
       "size": 93075 ,
       "expected_size": 93075 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQSNEg6RZR0muOUkeZUrH4BO3HAAIZDAACRs9AUtC8cY-lHrI1IwQ" ,
        "unique_id": "AgADGQwAAkbPQFI" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 93075
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        2172493 ,
        6784419 ,
        1648459 ,
        11171002
       ]
      } ,
      "intensity": 58 ,
      "is_inverted": true ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillFreeformGradient" ,
     "colors": [
      -11182431 ,
      -10791004 ,
      -11970143 ,
      -13268576
     ]
    } ,
    "animate_outgoing_message_fill": true ,
    "outgoing_message_accent_color": -2524432
   }
  } ,
  {
   "@type": "chatTheme" ,
   "name": "\ud83d\udc68\u200d\ud83c\udfeb" ,
   "light_settings": {
    "@type": "themeSettings" ,
    "accent_color": -14633562 ,
    "background": {
     "@type": "background" ,
     "id": "5785191424383254532" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "CJNyxPMgSVAEAAAAvW9sMwc51cw" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 148 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 16 ,
        "size": 22921 ,
        "expected_size": 22921 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhBI0aOMv_1u2GZ4DbxghCQ-ppcAAigHAAKfq0lQ4MDuDgABFgXrAQAHbQADIwQ" ,
         "unique_id": "AQADKAcAAp-rSVBy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 22921
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 17 ,
       "size": 312605 ,
       "expected_size": 312605 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQSNGjjL_9bthmeA28YIQkPqaXAAIoBwACn6tJUODA7g4AARYF6yME" ,
        "unique_id": "AgADKAcAAp-rSVA" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 312605
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        13755285 ,
        9556684 ,
        7780582 ,
        9557658
       ]
      } ,
      "intensity": 50 ,
      "is_inverted": false ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillGradient" ,
     "top_color": -721949 ,
     "bottom_color": -2557979 ,
     "rotation_angle": 0
    } ,
    "animate_outgoing_message_fill": false ,
    "outgoing_message_accent_color": -11948438
   } ,
   "dark_settings": {
    "@type": "themeSettings" ,
    "accent_color": -8010013 ,
    "background": {
     "@type": "background" ,
     "id": "5785191424383254532" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "CJNyxPMgSVAEAAAAvW9sMwc51cw" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 148 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 16 ,
        "size": 22921 ,
        "expected_size": 22921 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhBI0aOMv_1u2GZ4DbxghCQ-ppcAAigHAAKfq0lQ4MDuDgABFgXrAQAHbQADIwQ" ,
         "unique_id": "AQADKAcAAp-rSVBy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 22921
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 17 ,
       "size": 312605 ,
       "expected_size": 312605 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQSNGjjL_9bthmeA28YIQkPqaXAAIoBwACn6tJUODA7g4AARYF6yME" ,
        "unique_id": "AgADKAcAAp-rSVA" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 312605
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        7184290 ,
        3034425 ,
        6716244 ,
        1721151
       ]
      } ,
      "intensity": 61 ,
      "is_inverted": true ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillFreeformGradient" ,
     "colors": [
      -12482947 ,
      -10645912 ,
      -12024717 ,
      -12223348
     ]
    } ,
    "animate_outgoing_message_fill": true ,
    "outgoing_message_accent_color": -8010013
   }
  } ,
  {
   "@type": "chatTheme" ,
   "name": "\ud83c\udf37" ,
   "light_settings": {
    "@type": "themeSettings" ,
    "accent_color": -2862973 ,
    "background": {
     "@type": "background" ,
     "id": "5785068300555780101" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "mP3FG_iwSFAFAAAA2AklJO978pA" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 155 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 18 ,
        "size": 23099 ,
        "expected_size": 23099 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhBI0R7ZUtvrkFbqpVi5SZ5Kh5wAAmILAAIlozhSlXtaYmjilD4BAAdtAAMjBA" ,
         "unique_id": "AQADYgsAAiWjOFJy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 23099
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 19 ,
       "size": 51705 ,
       "expected_size": 51705 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQSNEe2VLb65BW6qVYuUmeSoecAAJiCwACJaM4UpV7WmJo4pQ-IwQ" ,
        "unique_id": "AgADYgsAAiWjOFI" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 51705
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        15511182 ,
        15254915 ,
        15442113 ,
        13087211
       ]
      } ,
      "intensity": 50 ,
      "is_inverted": false ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillFreeformGradient" ,
     "colors": [
      -330793 ,
      -267309 ,
      -5672
     ]
    } ,
    "animate_outgoing_message_fill": true ,
    "outgoing_message_accent_color": -744874
   } ,
   "dark_settings": {
    "@type": "themeSettings" ,
    "accent_color": -1348722 ,
    "background": {
     "@type": "background" ,
     "id": "5785068300555780101" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "mP3FG_iwSFAFAAAA2AklJO978pA" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 155 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 18 ,
        "size": 23099 ,
        "expected_size": 23099 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhBI0R7ZUtvrkFbqpVi5SZ5Kh5wAAmILAAIlozhSlXtaYmjilD4BAAdtAAMjBA" ,
         "unique_id": "AQADYgsAAiWjOFJy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 23099
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 19 ,
       "size": 51705 ,
       "expected_size": 51705 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQSNEe2VLb65BW6qVYuUmeSoecAAJiCwACJaM4UpV7WmJo4pQ-IwQ" ,
        "unique_id": "AgADYgsAAiWjOFI" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 51705
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        12622837 ,
        5515835 ,
        15052663 ,
        5845028
       ]
      } ,
      "intensity": 58 ,
      "is_inverted": true ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillFreeformGradient" ,
     "colors": [
      -2456006 ,
      -4564150 ,
      -4566159 ,
      -6270594
     ]
    } ,
    "animate_outgoing_message_fill": true ,
    "outgoing_message_accent_color": -1348722
   }
  } ,
  {
   "@type": "chatTheme" ,
   "name": "\ud83d\udc9c" ,
   "light_settings": {
    "@type": "themeSettings" ,
    "accent_color": -7315764 ,
    "background": {
     "@type": "background" ,
     "id": "5784961613568147459" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "O-wmAfBPSFADAAAA4zINVfD_bro" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 155 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 20 ,
        "size": 21485 ,
        "expected_size": 21485 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhBI0QABUhzV3pxzu9d2WiH2UqZSAAIVFgACtlM4UgvDyM-nB1ErAQAHbQADIwQ" ,
         "unique_id": "AQADFRYAArZTOFJy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 21485
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 21 ,
       "size": 64363 ,
       "expected_size": 64363 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQSNEAAVIc1d6cc7vXdloh9lKmUgACFRYAArZTOFILw8jPpwdRKyME" ,
        "unique_id": "AgADFRYAArZTOFI" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 64363
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        15308995 ,
        15312086 ,
        12034025 ,
        15511668
       ]
      } ,
      "intensity": 57 ,
      "is_inverted": false ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillSolid" ,
     "color": -3778166
    } ,
    "animate_outgoing_message_fill": false ,
    "outgoing_message_accent_color": -7315764
   } ,
   "dark_settings": {
    "@type": "themeSettings" ,
    "accent_color": -5146404 ,
    "background": {
     "@type": "background" ,
     "id": "5784961613568147459" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "O-wmAfBPSFADAAAA4zINVfD_bro" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 155 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 20 ,
        "size": 21485 ,
        "expected_size": 21485 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhBI0QABUhzV3pxzu9d2WiH2UqZSAAIVFgACtlM4UgvDyM-nB1ErAQAHbQADIwQ" ,
         "unique_id": "AQADFRYAArZTOFJy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 21485
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 21 ,
       "size": 64363 ,
       "expected_size": 64363 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQSNEAAVIc1d6cc7vXdloh9lKmUgACFRYAArZTOFILw8jPpwdRKyME" ,
        "unique_id": "AgADFRYAArZTOFI" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 64363
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        8222680 ,
        4008280 ,
        10976357 ,
        7028568
       ]
      } ,
      "intensity": 59 ,
      "is_inverted": true ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillFreeformGradient" ,
     "colors": [
      -5613933 ,
      -5875602 ,
      -7711598 ,
      -10729077
     ]
    } ,
    "animate_outgoing_message_fill": true ,
    "outgoing_message_accent_color": -5146404
   }
  } ,
  {
   "@type": "chatTheme" ,
   "name": "\ud83c\udf84" ,
   "light_settings": {
    "@type": "themeSettings" ,
    "accent_color": -14050257 ,
    "background": {
     "@type": "background" ,
     "id": "6041986402319597570" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "-Xc-np9y2VMCAAAARKr0yNNPYW0" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 155 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 22 ,
        "size": 19643 ,
        "expected_size": 19643 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhBI0U86VH_7T8zgEdHi8_t0t0wAApQGAAIxtFlQ1iDhTdspoCsBAAdtAAMjBA" ,
         "unique_id": "AQADlAYAAjG0WVBy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 19643
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 23 ,
       "size": 104932 ,
       "expected_size": 104932 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQSNFPOlR_-0_M4BHR4vP7dLdMAAKUBgACMbRZUNYg4U3bKaArIwQ" ,
        "unique_id": "AgADlAYAAjG0WVA" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 104932
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        15310167 ,
        15651185 ,
        15637864 ,
        14733438
       ]
      } ,
      "intensity": 64 ,
      "is_inverted": false ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillGradient" ,
     "top_color": -135237 ,
     "bottom_color": -1335 ,
     "rotation_angle": 0
    } ,
    "animate_outgoing_message_fill": false ,
    "outgoing_message_accent_color": -673207
   } ,
   "dark_settings": {
    "@type": "themeSettings" ,
    "accent_color": -8538787 ,
    "background": {
     "@type": "background" ,
     "id": "6041986402319597570" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "-Xc-np9y2VMCAAAARKr0yNNPYW0" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 155 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 22 ,
        "size": 19643 ,
        "expected_size": 19643 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhBI0U86VH_7T8zgEdHi8_t0t0wAApQGAAIxtFlQ1iDhTdspoCsBAAdtAAMjBA" ,
         "unique_id": "AQADlAYAAjG0WVBy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 19643
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 23 ,
       "size": 104932 ,
       "expected_size": 104932 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQSNFPOlR_-0_M4BHR4vP7dLdMAAKUBgACMbRZUNYg4U3bKaArIwQ" ,
        "unique_id": "AgADlAYAAjG0WVA" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 104932
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        7178314 ,
        5187344 ,
        10714432 ,
        3953221
       ]
      } ,
      "intensity": 60 ,
      "is_inverted": true ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillFreeformGradient" ,
     "colors": [
      -5679063 ,
      -4222668 ,
      -6137057 ,
      -5876419
     ]
    } ,
    "animate_outgoing_message_fill": true ,
    "outgoing_message_accent_color": -8538787
   }
  } ,
  {
   "@type": "chatTheme" ,
   "name": "\ud83c\udfae" ,
   "light_settings": {
    "@type": "themeSettings" ,
    "accent_color": -11109917 ,
    "background": {
     "@type": "background" ,
     "id": "5785007509588672513" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "kO4jyq55SFABAAAA0WEpcLfahXk" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 156 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 24 ,
        "size": 21843 ,
        "expected_size": 21843 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhBI0dbu26awaPMlu0sEu70ZtBQAAgIJAAKL3DlSrk7AAAH1xexaAQAHbQADIwQ" ,
         "unique_id": "AQADAgkAAovcOVJy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 21843
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 25 ,
       "size": 78338 ,
       "expected_size": 78338 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQSNHW7tumsGjzJbtLBLu9GbQUAAICCQACi9w5Uq5OwAAB9cXsWiME" ,
        "unique_id": "AgADAgkAAovcOVI" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 78338
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        4247006 ,
        11306733 ,
        15303896 ,
        15717209
       ]
      } ,
      "intensity": 52 ,
      "is_inverted": false ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillGradient" ,
     "top_color": -547 ,
     "bottom_color": -3115 ,
     "rotation_angle": 0
    } ,
    "animate_outgoing_message_fill": false ,
    "outgoing_message_accent_color": -11109917
   } ,
   "dark_settings": {
    "@type": "themeSettings" ,
    "accent_color": -10261265 ,
    "background": {
     "@type": "background" ,
     "id": "5785007509588672513" ,
     "is_default": false ,
     "is_dark": true ,
     "name": "kO4jyq55SFABAAAA0WEpcLfahXk" ,
     "document": {
      "@type": "document" ,
      "file_name": "pattern.tgv" ,
      "mime_type": "application/x-tgwallpattern" ,
      "thumbnail": {
       "@type": "thumbnail" ,
       "format": {
        "@type": "thumbnailFormatPng"
       } ,
       "width": 156 ,
       "height": 320 ,
       "file": {
        "@type": "file" ,
        "id": 24 ,
        "size": 21843 ,
        "expected_size": 21843 ,
        "local": {
         "@type": "localFile" ,
         "path": "" ,
         "can_be_downloaded": true ,
         "can_be_deleted": false ,
         "is_downloading_active": false ,
         "is_downloading_completed": false ,
         "download_offset": 0 ,
         "downloaded_prefix_size": 0 ,
         "downloaded_size": 0
        } ,
        "remote": {
         "@type": "remoteFile" ,
         "id": "AAMCBQADFQABYhBI0dbu26awaPMlu0sEu70ZtBQAAgIJAAKL3DlSrk7AAAH1xexaAQAHbQADIwQ" ,
         "unique_id": "AQADAgkAAovcOVJy" ,
         "is_uploading_active": false ,
         "is_uploading_completed": true ,
         "uploaded_size": 21843
        }
       }
      } ,
      "document": {
       "@type": "file" ,
       "id": 25 ,
       "size": 78338 ,
       "expected_size": 78338 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "EAACAgUAAxUAAWIQSNHW7tumsGjzJbtLBLu9GbQUAAICCQACi9w5Uq5OwAAB9cXsWiME" ,
        "unique_id": "AgADAgkAAovcOVI" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 78338
       }
      }
     } ,
     "type": {
      "@type": "backgroundTypePattern" ,
      "fill": {
       "@type": "backgroundFillFreeformGradient" ,
       "colors": [
        7314621 ,
        1980258 ,
        9272509 ,
        5649495
       ]
      } ,
      "intensity": 59 ,
      "is_inverted": true ,
      "is_moving": false
     }
    } ,
    "outgoing_message_fill": {
     "@type": "backgroundFillFreeformGradient" ,
     "colors": [
      -11620925 ,
      -9745981 ,
      -1810250 ,
      -1933003
     ]
    } ,
    "animate_outgoing_message_fill": false ,
    "outgoing_message_accent_color": -10261265
   }
  }
 ] ,
 "@client_id": 1
}
    "#;

    let src: TdType = serde_json::from_str(s).unwrap();
    println!("-----------src: {:?}-----------", src);
    //
}

#[test]
fn animate_emoji_1() {
    //---------------------
    let s = r#"
    {
 "@type": "updateNewMessage" ,
 "message": {
  "@type": "message" ,
  "id": 544250789888 ,
  "sender_id": {
   "@type": "messageSenderUser" ,
   "user_id": 5177652190
  } ,
  "chat_id": -1001560428294 ,
  "is_outgoing": false ,
  "is_pinned": false ,
  "can_be_edited": false ,
  "can_be_forwarded": true ,
  "can_be_saved": true ,
  "can_be_deleted_only_for_self": false ,
  "can_be_deleted_for_all_users": false ,
  "can_get_statistics": false ,
  "can_get_message_thread": true ,
  "can_get_viewers": false ,
  "can_get_media_timestamp_links": true ,
  "has_timestamped_media": true ,
  "is_channel_post": false ,
  "contains_unread_mention": false ,
  "date": 1649141467 ,
  "edit_date": 0 ,
  "interaction_info": {
   "@type": "messageInteractionInfo" ,
   "view_count": 0 ,
   "forward_count": 0 ,
   "reply_info": {
    "@type": "messageReplyInfo" ,
    "reply_count": 0 ,
    "recent_replier_ids": [ ] ,
    "last_read_inbox_message_id": 0 ,
    "last_read_outbox_message_id": 0 ,
    "last_message_id": 0
   }
  } ,
  "reply_in_chat_id": 0 ,
  "reply_to_message_id": 0 ,
  "message_thread_id": 544250789888 ,
  "ttl": 0 ,
  "ttl_expires_in": 0.000000 ,
  "via_bot_user_id": 0 ,
  "author_signature": "" ,
  "media_album_id": "0" ,
  "restriction_reason": "" ,
  "content": {
   "@type": "messageAnimatedEmoji" ,
   "animated_emoji": {
    "@type": "animatedEmoji" ,
    "sticker": {
     "@type": "sticker" ,
     "set_id": "1258816259751983" ,
     "width": 320 ,
     "height": 320 ,
     "emoji": "\ud83d\udc98" ,
     "is_animated": true ,
     "is_mask": false ,
     "outline": [
      {
       "@type": "closedVectorPath" ,
       "commands": [
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 226.250000 ,
          "y": 326.875000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 200.000000 ,
          "y": 270.625000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 197.500000 ,
          "y": 270.000000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 195.000000 ,
          "y": 270.625000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 181.250000 ,
          "y": 286.250000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 178.750000 ,
          "y": 288.750000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 152.500000 ,
          "y": 315.625000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 140.000000 ,
          "y": 286.875000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 120.000000 ,
          "y": 266.250000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 86.875000 ,
          "y": 231.875000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 46.250000 ,
          "y": 190.000000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 37.500000 ,
          "y": 140.625000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 35.000000 ,
          "y": 126.250000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 35.000000 ,
          "y": 110.000000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 40.625000 ,
          "y": 95.625000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 41.250000 ,
          "y": 94.375000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 60.625000 ,
          "y": 67.500000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 53.125000 ,
          "y": 70.000000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 49.375000 ,
          "y": 71.250000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 47.500000 ,
          "y": 79.375000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 41.250000 ,
          "y": 80.625000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 6.250000 ,
          "y": 87.500000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": -3.125000 ,
          "y": 14.375000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 8.750000 ,
          "y": 9.375000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 26.250000 ,
          "y": 1.250000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 74.375000 ,
          "y": 7.500000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 76.250000 ,
          "y": 31.250000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 76.875000 ,
          "y": 37.500000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 75.625000 ,
          "y": 43.125000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 71.875000 ,
          "y": 48.125000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 70.000000 ,
          "y": 50.625000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 63.750000 ,
          "y": 50.625000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 65.000000 ,
          "y": 53.125000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 66.875000 ,
          "y": 56.875000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 69.375000 ,
          "y": 60.625000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 73.125000 ,
          "y": 61.250000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 77.500000 ,
          "y": 61.875000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 88.125000 ,
          "y": 56.250000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 93.125000 ,
          "y": 56.250000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 121.875000 ,
          "y": 54.375000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 143.125000 ,
          "y": 70.625000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 160.625000 ,
          "y": 91.875000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 160.625000 ,
          "y": 91.875000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 165.625000 ,
          "y": 85.000000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 166.250000 ,
          "y": 85.000000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 171.250000 ,
          "y": 78.750000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 176.875000 ,
          "y": 73.125000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 183.125000 ,
          "y": 68.750000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 207.500000 ,
          "y": 51.875000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 240.625000 ,
          "y": 51.250000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 263.125000 ,
          "y": 71.250000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 290.000000 ,
          "y": 95.625000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 288.750000 ,
          "y": 135.625000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 275.625000 ,
          "y": 166.250000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 273.125000 ,
          "y": 172.500000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 261.250000 ,
          "y": 189.375000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 261.875000 ,
          "y": 191.250000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 263.125000 ,
          "y": 195.000000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 280.000000 ,
          "y": 202.500000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 283.750000 ,
          "y": 205.000000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 297.500000 ,
          "y": 215.625000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 320.000000 ,
          "y": 236.875000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 313.750000 ,
          "y": 256.875000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 312.500000 ,
          "y": 262.500000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 306.875000 ,
          "y": 265.000000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 305.625000 ,
          "y": 269.375000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 304.375000 ,
          "y": 273.125000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 312.500000 ,
          "y": 276.250000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 308.125000 ,
          "y": 281.875000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 303.750000 ,
          "y": 288.125000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 287.500000 ,
          "y": 277.500000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 283.125000 ,
          "y": 282.500000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 281.250000 ,
          "y": 285.000000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 281.875000 ,
          "y": 288.750000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 280.625000 ,
          "y": 291.250000
         }
        } ,
        {
         "@type": "vectorPathCommandCubicBezierCurve" ,
         "start_control_point": {
          "@type": "point" ,
          "x": 275.625000 ,
          "y": 305.000000
         } ,
         "end_control_point": {
          "@type": "point" ,
          "x": 265.000000 ,
          "y": 316.875000
         } ,
         "end_point": {
          "@type": "point" ,
          "x": 250.000000 ,
          "y": 316.875000
         }
        }
       ]
      }
     ] ,
     "thumbnail": {
      "@type": "thumbnail" ,
      "format": {
       "@type": "thumbnailFormatWebp"
      } ,
      "width": 128 ,
      "height": 128 ,
      "file": {
       "@type": "file" ,
       "id": 111 ,
       "size": 5326 ,
       "expected_size": 5326 ,
       "local": {
        "@type": "localFile" ,
        "path": "" ,
        "can_be_downloaded": true ,
        "can_be_deleted": false ,
        "is_downloading_active": false ,
        "is_downloading_completed": false ,
        "download_offset": 0 ,
        "downloaded_prefix_size": 0 ,
        "downloaded_size": 0
       } ,
       "remote": {
        "@type": "remoteFile" ,
        "id": "AAMCAQADFQABYkvJ8FdUeMWjXIw3HCI_1NLgAmcAAhQIAALjeAQAAVx1YkXB5I5ZAQAHbQADIwQ" ,
        "unique_id": "AQADFAgAAuN4BAABcg" ,
        "is_uploading_active": false ,
        "is_uploading_completed": true ,
        "uploaded_size": 5326
       }
      }
     } ,
     "sticker": {
      "@type": "file" ,
      "id": 110 ,
      "size": 14116 ,
      "expected_size": 14116 ,
      "local": {
       "@type": "localFile" ,
       "path": "" ,
       "can_be_downloaded": true ,
       "can_be_deleted": false ,
       "is_downloading_active": false ,
       "is_downloading_completed": false ,
       "download_offset": 0 ,
       "downloaded_prefix_size": 0 ,
       "downloaded_size": 0
      } ,
      "remote": {
       "@type": "remoteFile" ,
       "id": "CAACAgEAAxUAAWJLyfBXVHjFo1yMNxwiP9TS4AJnAAIUCAAC43gEAAFcdWJFweSOWSME" ,
       "unique_id": "AgADFAgAAuN4BAAB" ,
       "is_uploading_active": false ,
       "is_uploading_completed": true ,
       "uploaded_size": 14116
      }
     }
    } ,
    "fitzpatrick_type": 0
   } ,
   "emoji": "\ud83d\udc98"
  }
 } ,
 "@client_id": 1
}

    "#;
    let src: TdType = serde_json::from_str(s).unwrap();
    println!("-----------src: {:?}-----------", src);
}

#[test]
fn a_3() {
    //---------------------
    let s = r#"
{
 "@type": "updateNewMessage" ,
 "message": {
  "@type": "message" ,
  "id": 55269392384 ,
  "sender_id": {
   "@type": "messageSenderUser" ,
   "user_id": 2035680002
  } ,
  "chat_id": -1001180873406 ,
  "is_outgoing": false ,
  "is_pinned": false ,
  "can_be_edited": false ,
  "can_be_forwarded": true ,
  "can_be_saved": true ,
  "can_be_deleted_only_for_self": false ,
  "can_be_deleted_for_all_users": false ,
  "can_get_statistics": false ,
  "can_get_message_thread": true ,
  "can_get_viewers": false ,
  "can_get_media_timestamp_links": true ,
  "has_timestamped_media": true ,
  "is_channel_post": false ,
  "contains_unread_mention": false ,
  "date": 1649148784 ,
  "edit_date": 0 ,
  "interaction_info": {
   "@type": "messageInteractionInfo" ,
   "view_count": 0 ,
   "forward_count": 0 ,
   "reply_info": {
    "@type": "messageReplyInfo" ,
    "reply_count": 0 ,
    "recent_replier_ids": [] ,
    "last_read_inbox_message_id": 0 ,
    "last_read_outbox_message_id": 0 ,
    "last_message_id": 0
   }
  }
  "reply_in_chat_id": 0 ,
  "reply_to_message_id": 0 ,
  "message_thread_id": 55269392384 ,
  "ttl": 2678400 ,
  "ttl_expires_in": 2678398.949641 ,
  "via_bot_user_id": 0 ,
  "author_signature": "" ,
  "media_album_id": "0" ,
  "restriction_reason": "" ,
  "content": {
   "@type": "messageAudio" ,
   "audio": {
    "@type": "audio" ,
    "duration": 312 ,
    "title": "\u8d77\u98ce\u4e86(\u65e7\u7248)" ,
    "performer": "\u4e70\u8fa3\u6912\u4e5f\u7528\u5238" ,
    "file_name": "\u8d77\u98ce\u4e86(\u65e7\u7248)-\u4e70\u8fa3\u6912\u4e5f\u7528\u5238-\u8d77\u98ce\u4e86_(\u65e7\u7248).mp3" ,
    "mime_type": "audio/mpeg" ,
    "audio": {
     "@type": "file" ,
     "id": 17621 ,
     "size": 5011893 ,
     "expected_size": 5011893 ,
     "local": {
      "@type": "localFile" ,
      "path": "" ,
      "can_be_downloaded": true ,
      "can_be_deleted": false ,
      "is_downloading_active": false ,
      "is_downloading_completed": false ,
      "download_offset": 0 ,
      "downloaded_prefix_size": 0 ,
      "downloaded_size": 0
     } ,
     "remote": {
      "@type": "remoteFile" ,
      "id": "CQACAgUAAx0CRmKyvgACzeViTANSy7HQGO5R5XYgFTV7tBfFyAAC3Q0AApESeFXDmmy2zXPtyyME" ,
      "unique_id": "AgAD3Q0AApESeFU" ,
      "is_uploading_active": false ,
      "is_uploading_completed": true ,
      "uploaded_size": 5011893
     }
    }
   } ,
   "caption": {
    "@type": "formattedText" ,
    "text": "Artist: \u4e70\u8fa3\u6912\u4e5f\u7528\u5238\nAlbum: \u8d77\u98ce\u4e86 (\u65e7\u7248)\nDuration: 05:12  Type: mp3\n\u00a9 From: @VmomoVBot" ,
    "entities": [
     {
      "@type": "textEntity" ,
      "offset": 0 ,
      "length": 6 ,
      "type": {
       "@type": "textEntityTypeBold"
      }
     } ,
     {
      "@type": "textEntity" ,
      "offset": 15 ,
      "length": 5 ,
      "type": {
       "@type": "textEntityTypeBold"
      }
     } ,
     {
      "@type": "textEntity" ,
      "offset": 31 ,
      "length": 8 ,
      "type": {
       "@type": "textEntityTypeBold"
      }
     } ,
     {
      "@type": "textEntity" ,
      "offset": 41 ,
      "length": 5 ,
      "type": {
       "@type": "textEntityTypeMediaTimestamp" ,
       "media_timestamp": 312
      }
     } ,
     {
      "@type": "textEntity" ,
      "offset": 48 ,
      "length": 4 ,
      "type": {
       "@type": "textEntityTypeBold"
      }
     } ,
     {
      "@type": "textEntity" ,
      "offset": 58 ,
      "length": 8 ,
      "type": {
       "@type": "textEntityTypeItalic"
      }
     } ,
     {
      "@type": "textEntity" ,
      "offset": 66 ,
      "length": 10 ,
      "type": {
       "@type": "textEntityTypeTextUrl" ,
       "url": "https://t.me/VmomoVBot"
      }
     } ,
     {
      "@type": "textEntity" ,
      "offset": 66 ,
      "length": 10 ,
      "type": {
       "@type": "textEntityTypeItalic"
      }
     }
    ]
   }
  } ,
  "reply_markup": {
   "@type": "replyMarkupInlineKeyboard" ,
   "rows": [
    [
     {
      "@type": "inlineKeyboardButton" ,
      "text": "\ud83d\uddd1" ,
      "type": {
       "@type": "inlineKeyboardButtonTypeCallback" ,
       "data": "Y2xvc2VANTA5OTQ3MDE0Ng=="
      }
     } ,
     {
      "@type": "inlineKeyboardButton" ,
      "text": "\ud83d\udcd6" ,
      "type": {
       "@type": "inlineKeyboardButtonTypeCallback" ,
       "data": "bHlyaWNjNjlmODczZTA1YThmNDE4YzlmZTE0M2U3ZGY5MzU1Mw=="
      }
     }
    ]
   ]
  }
 } ,
 "@client_id": 1
    "#;

    let src: TdType = serde_json::from_str(s).unwrap();
    println!("-----------src: {:?}-----------", src);
}

#[test]
fn a_m_4() {
    //---------------------
    let s = r#"
    {
 "@type": "updateChatVideoChat",
 "chat_id": -1001587868058,
 "video_chat": {
  "@type": "videoChat",
  "group_call_id": 3,
  "has_participants": true
 },
 "@client_id": 1
}
    "#;

    let src: TdType = serde_json::from_str(s).unwrap();
    println!("-----------src: {:?}-----------", src);
}

#[test]
fn a_5() {
    //---------------------
    let s = r#"
    {
 "@type": "updateGroupCall",
 "group_call": {
  "@type": "groupCall",
  "id": 3,
  "title": "",
  "scheduled_start_date": 0,
  "enabled_start_notification": false,
  "is_active": true,
  "is_joined": false,
  "need_rejoin": false,
  "can_be_managed": false,
  "participant_count": 1,
  "loaded_all_participants": false,
  "recent_speakers": [
   {
    "@type": "groupCallRecentSpeaker",
    "participant_id": {
     "@type": "messageSenderUser",
     "user_id": 1555806518
    },
    "is_speaking": false
   }
  ],
  "is_my_video_enabled": false,
  "is_my_video_paused": false,
  "can_enable_video": true,
  "mute_new_participants": false,
  "can_toggle_mute_new_participants": false,
  "record_duration": 0,
  "is_video_recorded": false,
  "duration": 0
 },
 "@client_id": 1
}
    "#;

    let src: TdType = serde_json::from_str(s).unwrap();
    println!("-----------src: {:?}-----------", src);
}

#[test]
fn a_6() {
    let s = r#"
    {
 "@type": "updateChatMessageTtl" ,
 "chat_id": -1001401975183 ,
 "message_ttl": 2678400 ,
 "@client_id": 1
}

    "#;
    let src: TdType = serde_json::from_str(s).unwrap();
    println!("-----------src: {:?}-----------", src);
}

#[test]
fn a_7() {
    let s = r#"
    {
         "@type": "updateChatAction" ,
         "chat_id": -1001682824094 ,
         "message_thread_id": 0 ,
         "sender_id": {
              "@type": "messageSenderUser" ,
              "user_id": 1666120241
         } ,
         "action": {
            "@type": "chatActionChoosingSticker"
         } ,
         "@client_id": 1
    }
    "#;
    let src: TdType = serde_json::from_str(s).unwrap();
    println!("-----------src: {:?}-----------", src);
}

#[test]
fn a_9() {
    //---------------------
    // `suggestedActionCheckPassword`
    let s = r#"
    {
     "@type": "updateSuggestedActions" ,
     "added_actions": [
      {
       "@type": "suggestedActionCheckPassword"
      }
     ] ,
     "removed_actions": [ ] ,
     "@client_id": 1
    }
    "#;
    let src: TdType = serde_json::from_str(s).unwrap();
    println!("-----------src: {:?}-----------", src);
}

#[test]
fn a_10() {
    //---------------------
    // updateChatHasProtectedContent
    let s = r#"
        {
         "@type": "updateChatHasProtectedContent" ,
         "chat_id": -1001284752833 ,
         "has_protected_content": false ,
         "@client_id": 1
        }
    "#;
    let src: TdType = serde_json::from_str(s).unwrap();
    println!("-----------src: {:?}-----------", src);
}
