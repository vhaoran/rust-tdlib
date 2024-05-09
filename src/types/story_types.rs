//use mongodb::bson::oid;
use crate::types::{File, Minithumbnail, Photo, Thumbnail};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashSet;

// storyContentPhoto
#[derive(Clone, Default, Debug)]
pub struct StoryContentPhoto {
    photo: Photo,
}

#[derive(Clone, Default, Debug)]
pub struct StoryContentVideo {
    video: StoryVideo,
    alternative_video: Option<StoryVideo>,
}

//-------------------------------------
#[derive(Clone, Default, Debug)]
pub struct StoryVideo {
    duration: i64,
    width: i32,
    height: i32,
    preload_prefix_size: i32,
    video: File,
    minithumbnail: Option<Minithumbnail>,
    thumbnail: Option<Thumbnail>,
    has_stickers: bool,
    is_animation: bool,
}

//-------------------------------------
