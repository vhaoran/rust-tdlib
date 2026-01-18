//use mongodb::bson::oid;
use crate::types::{File, Minithumbnail, Photo, Thumbnail};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashSet;


#[allow(dead_code)]
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
impl  StoryContentVideo{
    #[allow(dead_code)]
    pub fn new()->Self{
        Self{
            video:StoryVideo::default(),
            alternative_video:None,
        }
    }

    #[allow(dead_code)]
    pub fn builder()->StoryContentVideoBuild{
        StoryContentVideoBuild::new()
    }

    #[allow(dead_code)]
    pub fn video(&self) -> &StoryVideo {
        &self.video
    }
    #[allow(dead_code)]
    pub fn alternative_video(&self) -> Option<&StoryVideo> {
        self.alternative_video.as_ref()
    }

}
//-------------------------------------
pub struct StoryContentVideoBuild{
    inner:StoryContentVideo
}

impl StoryContentVideoBuild{
    #[allow(dead_code)]
    pub fn new()->Self{
        Self{
            inner:StoryContentVideo::new(),
        }
    }
    #[allow(dead_code)]
    pub fn video(&mut self, video: StoryVideo) -> &mut Self {
        self.inner.video = video;
        self
    }
    #[allow(dead_code)]
    pub fn alternative_video(&mut self, alternative_video: StoryVideo) -> &mut Self {
        self.inner.alternative_video = Some(alternative_video);
        self
    }
}

//-------------------------------------
#[allow(dead_code)]
#[derive(Clone, Default, Debug)]
pub struct StoryVideo {
    duration: f64,
    width: i32,
    height: i32,
    preload_prefix_size: i32,
    video: File,
    // #[serde(rename="mini_thumbnail")]
    mini_thumbnail: Option<Minithumbnail>,
    thumbnail: Option<Thumbnail>,
    has_stickers: bool,
    is_animation: bool,
}

//-------------------------------------
