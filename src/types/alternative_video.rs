use crate::types::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlternativeVideo {
    // int32 	width_
    // Video width.
    width: i32,

    // int32 	height_
    // Video height.
    height: i32,
    // string 	codec_
    // Codec used for video file encoding, for example, "h264", "h265", or "av1".
    codec: String,
    // object_ptr< file > 	hls_file_
    // HLS file describing the video.
    hls_file: Option<File>,
    // object_ptr< file > 	video_
    video: File,
}
