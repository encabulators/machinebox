use super::Result;
use super::{Error, Kind};

/// A tag represents a single tag that describes an image. Depending on how you
/// obtained the tag, there might be a confidence score associated with it
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    /// The user-friendly description of the tag
    pub tag: String,
    /// If present, this is a probability score between 0 and 1. This is an Option to
    /// make an explicit difference between a 0 probability and a probability that was
    /// not supplied as part of the operation.
    #[serde(default)]
    pub confidence: Option<f64>,
    /// The ID is a unique identifier for the image supplied during a call to `teach`
    #[serde(default)]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckResponseFull {
    pub success: bool,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub custom_tags: Vec<Tag>,
}

impl Into<Result<CheckResponse>> for CheckResponseFull {
    fn into(self) -> Result<CheckResponse> {
        if self.success {
            Ok(CheckResponse {
                tags: self.tags,
                custom_tags: self.custom_tags,
            })
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

/// Response from calling `check` on an image
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckResponse {
    /// The list of identified tags
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// The list of custom (manually taught) tags identified with this image
    #[serde(default)]
    pub custom_tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeachResponse {
    pub success: bool,
    #[serde(default)]
    pub error: Option<String>,
}

impl Into<Result<()>> for TeachResponse {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarResponse {
    pub success: bool,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub similar: Vec<Tag>,
}

impl Into<Result<Vec<Tag>>> for SimilarResponse {
    fn into(self) -> Result<Vec<Tag>> {
        if self.success {
            Ok(self.similar)
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrainTag {
    pub tag: String,
    #[serde(default)]
    pub id: Option<String>,
    pub url: String,
}
