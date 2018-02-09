//! # machinebox
//!
//! The `machinebox` create is a simple client SDK that allows Rust developers to consume the
//! features and functionality exposed by [machinebox](http://machinebox.io) boxes. For
//! more information on which boxes are available and their functionality, please check
//! the machinebox.io documentation.

#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use reqwest::StatusCode;
use std::fmt;

/// Represents an error communicating with a machinebox
#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

impl From<reqwest::Error> for Error {
    fn from(source: reqwest::Error) -> Self {
        Error {
            kind: Kind::Reqwest(source),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Error {
            kind: Kind::Serialization(source),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error {
            kind: Kind::Io(source),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            Kind::Machinebox(ref s) => fmt::Display::fmt(s, f),
            Kind::Serialization(ref e) => fmt::Display::fmt(e, f),
            Kind::Reqwest(ref e) => fmt::Display::fmt(e, f),
            Kind::Io(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "A machinebox client error occurred"
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match self.kind {
            Kind::Machinebox(_) => None,
            Kind::Serialization(ref e) => Some(e),
            Kind::Reqwest(ref e) => Some(e),
            Kind::Io(ref e) => Some(e),
        }
    }
}

/// Kind indicates the type of error that occurred. External consumers of this library
/// shouldn't have to deal with these values
#[derive(Debug)]
enum Kind {
    Reqwest(::reqwest::Error),
    Serialization(::serde_json::Error),
    Machinebox(String),
    Io(::std::io::Error),
}

type Result<T> = std::result::Result<T, Error>;

/// Provides information about a machinebox. All boxes, regardless of their type,
/// will provide this information in an `info()` call.
#[derive(Serialize, Deserialize, Debug)]
pub struct BoxInfo {
    pub success: bool,
    pub name: String,
    pub version: u64,
    pub build: String,
    pub status: String,
    pub plan: String,
    pub error: Option<String>,
}

/// Metadata about a particular box
#[derive(Serialize, Deserialize, Debug)]
pub struct BoxMetadata {
    pub boxname: String,
    pub build: String,
}

/// Structured error information returned when checking the health of a box
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BoxError {
    pub error: String,
    pub description: String,
}

/// Health details of a box.
#[derive(Serialize, Deserialize, Debug)]
pub struct Health {
    pub success: bool,
    pub hostname: String,
    pub metadata: BoxMetadata,
    pub errors: Vec<BoxError>,
}

/// BoxClient represents the methods that are available on all of the specialized
/// clients regardless of box type.
pub trait BoxClient {
    /// Provides information about the box
    fn info(&self) -> Result<BoxInfo> {
        let url = format!("{}/info", self.url());
        match reqwest::get(&url) {
            Ok(mut result) => {
                let raw = result.text()?;
                let bi: BoxInfo = serde_json::from_str(&raw)?;
                Ok(bi)
            }
            Err(e) => Err(Error {
                kind: Kind::Reqwest(e),
            }),
        }
    }

    /// Checks the health of the box
    fn health(&self) -> Result<Health> {
        let url = format!("{}/healthz", self.url());
        match reqwest::get(&url) {
            Ok(mut result) => {
                let raw = result.text()?;
                let health: Health = serde_json::from_str(&raw)?;
                Ok(health)
            }
            Err(e) => Err(Error {
                kind: Kind::Reqwest(e),
            }),
        }
    }

    /// Determines whether the box is live
    fn is_live(&self) -> Result<bool> {
        let url = format!("{}/liveness", self.url());
        match reqwest::get(&url) {
            Ok(response) => Ok(response.status() == StatusCode::Ok),
            Err(e) => Err(Error {
                kind: Kind::Reqwest(e),
            }),
        }
    }

    /// Determines if the box is ready. Some boxes may take a while to start up, so you can
    /// use this function to check if it is acceptable to start using the box-specific functionality
    fn is_ready(&self) -> Result<bool> {
        let url = format!("{}/readyz", self.url());
        match reqwest::get(&url) {
            Ok(response) => Ok(response.status() == StatusCode::Ok),
            Err(e) => Err(Error {
                kind: Kind::Reqwest(e),
            }),
        }
    }

    /// Indicates the URL of the box
    fn url(&self) -> &str;
}

pub mod textbox;
pub mod suggestionbox;

#[cfg(test)]
mod tests {
    extern crate mockito;

    use self::mockito::{mock, reset, SERVER_URL};
    use BoxClient;

    struct TestClient;

    impl BoxClient for TestClient {
        fn url(&self) -> &str {
            &SERVER_URL
        }
    }

    #[test]
    fn info_parses() {
        let mock = mock("GET", "/info")
            .with_body(
                r#"
            {
	"success": true,
	"name":    "tagbox",
	"version": 1,
	"build":   "27d1d38",
	"status":  "ready",
	"plan": "pro"
}"#,
            )
            .create();
        {
            let t = TestClient {};
            let info = t.info().unwrap();
            assert_eq!(info.success, true);
            assert_eq!(info.name, "tagbox");
            assert_eq!(info.version, 1);
            assert_eq!(info.build, "27d1d38");
            assert_eq!(info.status, "ready");
            assert_eq!(info.plan, "pro");
        }
        mock.assert();
    }

    #[test]
    fn health_parses_no_error() {
        let mock = mock("GET", "/healthz")
            .with_body(
                r#"
            {
	"success": true,
	"hostname": "83b1a33ef322",
	"metadata": {
		"boxname": "facebox",
		"build": "18f2361"
	},
	"errors": []
}"#,
            )
            .create();
        {
            let t = TestClient {};
            let health = t.health().unwrap();
            assert_eq!(health.success, true);
            assert_eq!(health.hostname, "83b1a33ef322");
            assert_eq!(health.errors, vec![]);
        }
        mock.assert();
        reset();
    }

    #[test]
    fn health_parses_with_error() {
        let mock = mock("GET", "/healthz")
            .with_body(
                r#"
            {
	"success": false,
	"hostname": "83b1a33ef322",
	"metadata": {
		"boxname": "facebox",
		"build": "18f2361"
	},
        "errors": [{
            "error": "Something went wrong",
            "description": "Something went wrong"
        }]
    }"#,
            )
            .create();
        {
            let t = TestClient {};
            let health = t.health().unwrap();
            assert_eq!(health.success, false);
            assert_eq!(health.hostname, "83b1a33ef322");
            assert_eq!(health.errors[0].error, "Something went wrong");
            assert_eq!(health.errors[0].description, "Something went wrong");
        }
        mock.assert();
        reset();
    }

    #[test]
    fn islive_checks_statuscode() {
        let mock = mock("GET", "/liveness").with_status(200).create();
        {
            let t = TestClient {};
            let live = t.is_live().unwrap();
            assert_eq!(live, true);
        }
        mock.assert();
    }

    #[test]
    fn ready_checks_statuscode() {
        let mock = mock("GET", "/readyz").with_status(200).create();
        {
            let t = TestClient {};
            let ready = t.is_ready().unwrap();
            assert_eq!(ready, true);
        }
        mock.assert();
    }

    #[test]
    fn ready_checks_statuscode_fail() {
        let mock = mock("GET", "/readyz").with_status(503).create();
        {
            let t = TestClient {};
            let ready = t.is_ready().unwrap();
            assert_eq!(ready, false);
        }
        mock.assert();
    }
}
