//! A client for utilizing the `facebox` machine
//!
//! **Facebox** lets you identify faces within images.
//!
//! For more information, see the [facebox docs](https://machinebox.io/docs/facebox)
use super::BoxClient;
use super::Result;
use reqwest;
use reqwest::StatusCode;
use serde_json;
use Error;
use Kind;
use std::io::Read;

use self::types::{CheckResponseFull, SimilarResponseFull, RenameRequest};
pub use self::types::{CheckResponse, SimilarResponse, Face, Similar, Rect};

use super::utils::{delete_with_response, patch_json, post_form_vars, post_json,
                   post_multipart_file, get_json, post_multipart_reader, post_multipart_reader_parts,
                   RawBoxResponse, URLWrapper};
use std::io::Write;

/// The client for the `facebox` machine box.
pub struct Facebox {
    url: String,
}

impl Facebox {
    /// Creates a new facebox client connecting to the supplied URL.
    pub fn new(url: &str) -> Facebox {
        Facebox { url: url.to_owned() }
    }

    /// Identifies the faces in the reader image
    pub fn check<T: Read + Send + 'static>(&self, reader: T) -> Result<CheckResponse> {
        let url = format!("{}/facebox/check", self.url());
        let raw = post_multipart_reader(&url, reader)?;
        let checkreply: CheckResponseFull = serde_json::from_str(&raw)?;
        checkreply.into()
    }

    /// Identifies the faces in the image at the source path
    pub fn check_path(&self, source_path: &str) -> Result<CheckResponse> {
        let url = format!("{}/facebox/check", self.url());
        let raw = post_multipart_file(&url, source_path)?;
        let checkreply: CheckResponseFull = serde_json::from_str(&raw)?;
        checkreply.into()
    }

    /// Identifies the faces in the image in the supplied base64 encoded image
    pub fn check_base64(&self, data: &str) -> Result<CheckResponse> {
        let url = format!("{}/facebox/check", self.url());
        let params = [("base64", data)];
        let s = post_form_vars(&url, &params)?;
        let checkreply: CheckResponseFull = serde_json::from_str(&s)?;
        checkreply.into()
    }

    /// Identifies the faces im the image at the supplied URL
    pub fn check_url(&self, image_url: &str) -> Result<CheckResponse> {
        let url = format!("{}/facebox/check", self.url());
        let params = URLWrapper {
            url: image_url.to_owned(),
        };
        let s = post_json(&url, &params)?;
        let checkreply: CheckResponseFull = serde_json::from_str(&s)?;
        checkreply.into()
    }

    /// Returns a list of images that are similar to the one supplied by the reader
    pub fn similar<T: Read + Send + 'static>(&self, image: T) -> Result<SimilarResponse> {
        let url = format!("{}/facebox/similar", self.url());
        let raw = post_multipart_reader(&url, image)?;
        let similar_reply: SimilarResponseFull = serde_json::from_str(&raw)?;
        similar_reply.into()
    }

    /// Returns a list of images that are similar to the one indicated by the URL
    pub fn similar_url(&self, image_url: &str) -> Result<SimilarResponse> {
        let url = format!("{}/facebox/similar", self.url());
        let params = URLWrapper {
            url: image_url.to_owned(),
        };
        let s = post_json(&url, &params)?;
        let similar_reply: SimilarResponseFull = serde_json::from_str(&s)?;
        similar_reply.into()
    }

    /// Returns a list of images similar to the image identified by `id`
    pub fn similar_id(&self, id: &str) -> Result<SimilarResponse> {
        let url = format!("{}/facebox/similar?id={}", self.url(), id);
        let s = get_json(&url)?;
        let similar_reply: SimilarResponseFull = serde_json::from_str(&s)?;
        similar_reply.into()
    }

    /// Returns a list of images similar to the supplied base64 encoded image
    pub fn similar_base64(&self, data: &str) -> Result<SimilarResponse> {
        let url = format!("{}/facebox/similar", self.url());
        let params = [("base64", data)];
        let s = post_form_vars(&url, &params)?;
        let similar_reply: SimilarResponseFull = serde_json::from_str(&s)?;
        similar_reply.into()
    }

    /// Downloads the state of the faebox into the `buf` buffer, returning
    /// the number of bytes written
    pub fn download_state<W>(&self, buf: &mut W) -> Result<u64>
        where
            W: Write,
    {
        let url = format!("{}/facebox/state", self.url());
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

    /// Submits the state file indicated by the `source_path` parameter to the facebox
    pub fn post_state(&self, source_path: &str) -> Result<()> {
        let url = format!("{}/facebox/state", self.url());
        let raw = post_multipart_file(&url, source_path)?;
        let state_response:RawBoxResponse = serde_json::from_str(&raw)?;
        state_response.into()
    }

    /// Submits a state URL to the facebox
    pub fn post_state_url(&self, state_url: &str) -> Result<()> {
        let url = format!("{}/facebox/state", self.url());
        let params = [("url", state_url)];
        let raw = post_form_vars(&url, &params)?;
        let state_response:RawBoxResponse = serde_json::from_str(&raw)?;
        state_response.into()
    }

    /// Teaches facebox the face in the image contained in the `image` reader
    pub fn teach<T: Read + Send + 'static>(&self, image: T, id: &'static str,
                                           name: &'static str) -> Result<()> {
        let url = format!("{}/facebox/teach", self.url());
        let parts = vec![
            ("id", id),
            ("name", name)
        ];
        let raw = post_multipart_reader_parts(&url, image, parts)?;
        let teach_response: RawBoxResponse = serde_json::from_str(&raw)?;
        teach_response.into()
    }

    /// Teaches facebox the face in the image at the supplied URL
    pub fn teach_url(&self, image_url: &str, id: &str, name: &str) -> Result<()> {
        let url = format!("{}/facebox/teach", self.url());
        let params = [
            ("url", image_url.to_owned()),
            ("id", id.to_owned()),
            ("name", name.to_owned())
        ];
        let raw = post_form_vars(&url, &params)?;
        let teach_response: RawBoxResponse = serde_json::from_str(&raw)?;
        teach_response.into()
    }

    /// Removes the face with the supplied `id`
    pub fn remove(&self, id: &str) -> Result<()> {
        let url = format!("{}/facebox/teach/{}", self.url(), id);
        let raw = delete_with_response(&url)?;
        let remove_response: RawBoxResponse = serde_json::from_str(&raw)?;
        remove_response.into()
    }

    /// Renames the face associated with `id` to the new `name`
    pub fn rename(&self, id: &str, name: &str) -> Result<()> {
        let url = format!("{}/facebox/teach/{}", self.url(), id);
        let req = RenameRequest {
            name: name.to_owned()
        };
        let raw = patch_json(&url, &req)?;
        let rename_response: RawBoxResponse = serde_json::from_str(&raw)?;
        rename_response.into()
    }

    /// Renames all faces with the name `old_name` to `new_name`. This does not
    /// affect face IDs
    pub fn rename_all(&self, old_name: &str, new_name: &str) -> Result<()> {
        let url = format!("{}/facebox/rename", self.url());
        let params = [
            ("from", old_name),
            ("to", new_name)
        ];
        let raw = post_form_vars(&url, &params)?;
        let rename_response: RawBoxResponse = serde_json::from_str(&raw)?;
        rename_response.into()
    }
}

impl BoxClient for Facebox {
    fn url(&self) -> &str {
        &self.url
    }
}

mod types;
