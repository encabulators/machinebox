//! A client for utilizing the `tagbox` machine
//!
//! **Tagbox** lets you identify the content of images by getting a list of ordered tags.
//!
//! For more information, see the [tagbox docs](https://machinebox.io/docs/tagbox)
use std::io::Write;
use super::BoxClient;
use super::Result;
use reqwest;
use reqwest::StatusCode;
use serde_json;
use Error;
use Kind;

pub use self::types::{CheckResponse, Tag};
use self::types::{CheckResponseFull, SimilarResponse, TeachResponse, TrainTag};

use super::utils::{delete_with_response, patch_json, post_form_vars, post_json,
                   post_multipart_file, post_multipart_reader, URLWrapper};
use std::io::Read;
use utils::RawBoxResponse;

/// The client for the `tagbox` machine box.
pub struct Tagbox {
    url: String,
}

impl Tagbox {
    /// Creates a new tagbox client connecting to the supplied URL.
    pub fn new(url: &str) -> Tagbox {
        Tagbox { url: url.to_owned() }
    }

    /// Gets the tags for the image to which `reader` points
    pub fn check<T: Read + Send + 'static>(&self, reader: T) -> Result<CheckResponse> {
        let url = format!("{}/tagbox/check", self.url());
        let raw = post_multipart_reader(&url, reader)?;
        let checkreply: CheckResponseFull = serde_json::from_str(&raw)?;
        checkreply.into()
    }

    /// Gets the tags for the image at `source_path`
    pub fn check_path(&self, source_path: &str) -> Result<CheckResponse> {
        let url = format!("{}/tagbox/check", self.url());
        let raw = post_multipart_file(&url, source_path)?;
        let checkreply: CheckResponseFull = serde_json::from_str(&raw)?;
        checkreply.into()
    }

    /// Gets the tags for the image contained in the base64 encoded data
    pub fn check_base64(&self, data: &str) -> Result<CheckResponse> {
        let url = format!("{}/tagbox/check", self.url());
        let params = [("base64", data)];
        let s = post_form_vars(&url, &params)?;
        let checkreply: CheckResponseFull = serde_json::from_str(&s)?;
        checkreply.into()
    }

    /// Gets the tags for the image at the given URL
    pub fn check_url(&self, image_url: &str) -> Result<CheckResponse> {
        let url = format!("{}/tagbox/check", self.url());
        let params = URLWrapper {
            url: image_url.to_owned(),
        };
        let s = post_json(&url, &params)?;
        let checkreply: CheckResponseFull = serde_json::from_str(&s)?;
        checkreply.into()
    }

    /// Teaches the tagbox the image with a custom tag at the specified URL
    pub fn teach_url(&self, image_url: &str, tag: &str, id: Option<String>) -> Result<()> {
        let url = format!("{}/tagbox/teach", self.url());
        let train = TrainTag {
            url: image_url.to_owned(),
            id: id,
            tag: tag.to_owned(),
        };
        let s = post_json(&url, &train)?;
        let teachreply: TeachResponse = serde_json::from_str(&s)?;
        teachreply.into()
    }

    /// Deletes a custom tag by its ID
    pub fn remove_custom_tag(&self, id: &str) -> Result<()> {
        let url = format!("{}/tagbox/teach/{}", self.url(), id);
        let s = delete_with_response(&url)?;
        let teachreply: TeachResponse = serde_json::from_str(&s)?;
        teachreply.into()
    }

    /// Renames a custom tag with the indicated ID
    pub fn rename_custom_tag(&self, id: &str, tag: &str) -> Result<()> {
        let url = format!("{}/tagbox/teach/{}", self.url(), id);
        let tag = Tag {
            tag: tag.to_owned(),
            id: None,
            confidence: None,
        };
        let s = patch_json(&url, &tag)?;
        let teachreply: TeachResponse = serde_json::from_str(&s)?;
        teachreply.into()
    }

    /// Checks the image file at `source_path` for similar images based on previously
    /// taught tags
    pub fn similar_file(&self, source_path: &str) -> Result<Vec<Tag>> {
        let url = format!("{}/tagbox/similar", self.url());
        let s = post_multipart_file(&url, source_path)?;
        let similar: SimilarResponse = serde_json::from_str(&s)?;
        similar.into()
    }

    /// Checks the image at the `image_url` for similar images based on previously
    /// taught tags.
    pub fn similar_url(&self, image_url: &str) -> Result<Vec<Tag>> {
        let url = format!("{}/tagbox/similar", self.url());
        let params = [("url", image_url)];
        let s = post_form_vars(&url, &params)?;
        let similar: SimilarResponse = serde_json::from_str(&s)?;
        similar.into()
    }

    /// Checks the image within the base64 encoded string for similar images based on
    /// previously taught tags.
    pub fn similar_base64(&self, data: &str) -> Result<Vec<Tag>> {
        let url = format!("{}/tagbox/similar", self.url());
        let params = [("base64", data)];
        let s = post_form_vars(&url, &params)?;
        let similar: SimilarResponse = serde_json::from_str(&s)?;
        similar.into()
    }

    /// Downloads the state of the tagbox into the `buf` buffer, returning
    /// the number of bytes written
    pub fn download_state<W>(&self, buf: &mut W) -> Result<u64>
    where
        W: Write,
    {
        let url = format!("{}/tagbox/state", self.url());
        let mut resp = reqwest::get(&url)?;
        if resp.status() != StatusCode::Ok {
            let raw = resp.text()?;
            Err(Error {
                kind: Kind::Machinebox(format!("HTTP {}: {}", resp.status(), raw)),
            })
        } else {
            let bytecount = resp.copy_to(buf)?;
            Ok(bytecount)
        }
    }

    /// Submits the state file indicated by the `source_path` parameter to the tagbox
    pub fn post_state(&self, source_path: &str) -> Result<()> {
        let url = format!("{}/tagbox/state", self.url());
        let raw = post_multipart_file(&url, source_path)?;
        let state_response:RawBoxResponse = serde_json::from_str(&raw)?;
        state_response.into()
    }

    /// Submits a state URL to the tagbox
    pub fn post_state_url(&self, state_url: &str) -> Result<()> {
        let url = format!("{}/tagbox/state", self.url());
        let params = [("url", state_url)];
        let raw = post_form_vars(&url, &params)?;
        let state_response:RawBoxResponse = serde_json::from_str(&raw)?;
        state_response.into()
    }
}

impl BoxClient for Tagbox {
    fn url(&self) -> &str {
        &self.url
    }
}

mod types;
