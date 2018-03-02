use reqwest;

use super::{Error, Kind, Result};
use serde::ser::Serialize;
use reqwest::StatusCode;
use reqwest::multipart::Form;
use reqwest::multipart::Part;
use std::io::Read;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawBoxResponse {
    success: bool,
    #[serde(default)]
    error: Option<String>,
}

impl Into<Result<()>> for RawBoxResponse {
   fn into(self) -> Result<()> {
       if self.success {
           Ok(())
       } else {
           let s = match self.error {
               Some(s) => s,
               None => "Request failed".to_owned(),
           };
           Err(Error {
               kind: Kind::Machinebox(s),
           })
       }
   }
}

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

fn post_multipart(url: &str, form: Form) -> Result<String> {
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

pub fn post_multipart_reader<T: Read + Send + 'static>(url: &str, reader: T) -> Result<String> {
    let part = Part::reader(reader).file_name("file");
    let form = reqwest::multipart::Form::new().part("file", part);
    post_multipart(url, form)
}

pub fn post_multipart_reader_parts<T: Read+Send+'static>(url: &str, reader: T, parts: Vec<(&'static str,&'static str)>) -> Result<String> {
    let rpart = Part::reader(reader).file_name("file");
    let mut form = reqwest::multipart::Form::new();
    form = form.part("file", rpart);
    for (k,v) in parts {
        form = form.part(k,Part::text(v));
    }
    post_multipart(url, form)
}

pub fn post_multipart_file(url: &str, source_path: &str) -> Result<String> {
    let form = reqwest::multipart::Form::new().file("file", source_path)?;
    post_multipart(url, form)
}

pub fn get_json(url: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let url = url.to_owned();
    match client.get(&url).send() {
        Ok(mut response) => {
            let raw = response.text()?;
            if response.status() == StatusCode::Ok {
                Ok(raw)
            } else {
                Err(Error {
                    kind: Kind::Machinebox(raw),
                })
            }
        },
        Err(e) => Err(Error {
            kind: Kind::Reqwest(e),
        })
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

#[derive(Debug, Serialize, Deserialize)]
pub struct URLWrapper {
    pub url: String,
}
