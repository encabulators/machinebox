use super::Result;
use super::{Error, Kind};

/// Represents a detected face in an image
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Face {
    /// Bounds and position of the detected face
    pub rect: Rect,
    /// ID of the face
    #[serde(default)]
    pub id: Option<String>,
    /// Trained/taught name of the face
    #[serde(default)]
    pub name: Option<String>,
    /// Indicates whether the face was matched
    pub matched: bool,
    /// Confidence rating of the match
    pub confidence: f64,
}

/// The bounds and position of a rectangle in which a face was detected
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rect {
    /// Vertical offset of the top of the rectangle
    pub top: isize,
    /// Horizontal offset of the left side of the rectangle
    pub left: isize,
    /// Width of the rectangle
    pub width: isize,
    /// Height of the rectangle
    pub height: isize,
}

/// The name and ID of a face detected as similar to a search target
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Similar {
    /// ID of the face
    pub id: String,
    /// Name of the detected face
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimilarResponseFull {
    pub success: bool,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub similar: Vec<Similar>,
}

/// Response from `facebox` when detecting similar faces
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimilarResponse {
    /// Vector of similar faces
    #[serde(default)]
    pub similar: Vec<Similar>,
}

impl Into<Result<SimilarResponse>> for SimilarResponseFull {
    fn into(self) -> Result<SimilarResponse> {
        if self.success {
            Ok(SimilarResponse { similar: self.similar })
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
pub struct CheckResponseFull {
    pub success: bool,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub faces: Vec<Face>,
}

/// This struct contains a vector of faces identified within the supplied image
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckResponse {
    /// List of identified faces
    pub faces: Vec<Face>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RenameRequest {
    pub name: String,
}


impl Into<Result<CheckResponse>> for CheckResponseFull {
    fn into(self) -> Result<CheckResponse> {
        if self.success {
            Ok(CheckResponse { faces: self.faces })
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

