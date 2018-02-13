extern crate reqwest;
extern crate serde;

use super::{Error, Kind, Result};
use serde::ser::Serialize;
use reqwest::StatusCode;

pub fn post_form_vars<S>(url: &str, vars: S) -> Result<String>
where
    S: Serialize,
{
    let client = reqwest::Client::new();
    let url = url.to_owned();
    match client.post(&url).form(&vars).send() {
        Ok(mut response) => {
            let raw = response.text()?;
            if response.status() != StatusCode::Ok {
                Err(Error {
                    kind: Kind::Machinebox(format!("HTTP {}: {}", response.status(), raw)),
                })
            } else {
                Ok(raw)
            }
        }
        Err(e) => Err(Error {
            kind: Kind::Reqwest(e),
        }),
    }
}

pub fn delete_with_response(url: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let url = url.to_owned();
    match client.delete(&url).send() {
        Ok(mut response) => {
            let raw = response.text()?;
            if response.status() != StatusCode::Ok {
                Err(Error {
                    kind: Kind::Machinebox(format!("HTTP {}: {}", response.status(), raw)),
                })
            } else {
                Ok(raw)
            }
        }
        Err(e) => Err(Error {
            kind: Kind::Reqwest(e),
        }),
    }
}

pub fn patch_json<S>(url: &str, payload: &S) -> Result<String>
where
    S: Serialize,
{
    let client = reqwest::Client::new();
    let url = url.to_owned();
    match client.patch(&url).json(payload).send() {
        Ok(mut response) => {
            let raw = response.text()?;
            if response.status() != StatusCode::Ok {
                Err(Error {
                    kind: Kind::Machinebox(format!("HTTP {}: {}", response.status(), raw)),
                })
            } else {
                Ok(raw)
            }
        }
        Err(e) => Err(Error {
            kind: Kind::Reqwest(e),
        }),
    }
}

pub fn post_multipart_file(url: &str, source_path: &str) -> Result<String> {
    let form = reqwest::multipart::Form::new().file("file", source_path)?;
    let client = reqwest::Client::new();
    let url = url.to_owned();
    match client.post(&url).multipart(form).send() {
        Ok(mut response) => {
            let raw = response.text()?;
            if response.status() != StatusCode::Ok {
                Err(Error {
                    kind: Kind::Machinebox(format!("HTTP {}: {}", response.status(), raw)),
                })
            } else {
                Ok(raw)
            }
        }
        Err(e) => Err(Error {
            kind: Kind::Reqwest(e),
        }),
    }
}

pub fn post_json<S>(url: &str, payload: &S) -> Result<String>
where
    S: Serialize,
{
    let client = reqwest::Client::new();
    let url = url.to_owned();
    match client.post(&url).json(payload).send() {
        Ok(mut response) => {
            let raw = response.text()?;
            if response.status() == StatusCode::Ok {
                Ok(raw)
            } else {
                Err(Error {
                    kind: Kind::Machinebox(raw),
                })
            }
        }
        Err(e) => Err(Error {
            kind: Kind::Reqwest(e),
        }),
    }
}
