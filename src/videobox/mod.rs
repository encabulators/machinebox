//! A client for utilizing the `videobox` machine
//!
//! **Videobox** sends individual frames of videos to other machine boxes for processing.
//! you must have one of `facebox`, `tagbox`, or `nudebox` running and available to use
//! videobox.
//!
//! For more information, see the [videobox docs](https://machinebox.io/docs/videobox)
use super::BoxClient;
use super::Result;
use serde_json;
use Error;
use Kind;

pub use self::types::{CheckOptions, CheckOptionsBuilder, Video, VideoAnalysis, Range, Item,
    Nudebox, Facebox, Tagbox, Status};
use super::utils::{delete_with_response, post_form_vars, get_json};

use self::types::{VideoResponse, VideoAnalysisResponse};

/// The client for the `videobox` machine box.
pub struct Videobox {
    url: String,
}

impl Videobox {
    /// Creates a new videobox client
    pub fn new(url: &str) -> Videobox {
        Videobox {
            url: url.to_owned()
        }
    }

    /// Begins processing the video at the given URL.
    /// Videobox is asynchronous, you must call `status` to check
    /// that the video processing operation has completed before calling `results` to
    /// get the results.
    pub fn check_url(&self, video_url: &str, options: CheckOptions) -> Result<Video> {
        let url = format!("{}/videobox/check", self.url());
        let mut params: Vec<(String,String)> = Vec::new();
        params.push(("url".to_owned(), video_url.to_owned()));

        for option in options.into_iter() {
            params.push(option.clone());
        }

        let s = post_form_vars(&url, &params)?;
        let video_result:VideoResponse = serde_json::from_str(&s)?;
        video_result.into()
    }

    /// Removes the processing results for a video
    pub fn delete(&self, id: &str) -> Result<()> {
        let url = format!("{}/videobox/results/{}", self.url(), id);
        let _ = delete_with_response(&url)?;

        Ok(())
    }

    /// Gets the results of a video processing operation
    /// This should be called after the video status is completed
    pub fn results(&self, id: &str) -> Result<VideoAnalysis> {
        let url = format!("{}/videobox/results/{}", self.url(), id);
        let s = get_json(&url)?;
        let analysis: VideoAnalysisResponse = serde_json::from_str(&s)?;
        analysis.into()
    }

    /// Checks the status of a video processing job
    pub fn status(&self, id: &str) -> Result<Video> {
        let url = format!("{}/videobox/status/{}", self.url(), id);
        let s = get_json(&url)?;
        let video: VideoResponse = serde_json::from_str(&s)?;
        video.into()
    }
}

impl BoxClient for Videobox {
    fn url(&self) -> &str {
        &self.url
    }
}

mod types;

#[cfg(test)]
mod tests;
