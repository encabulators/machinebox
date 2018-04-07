use super::Result;
use super::{Error, Kind};
use std::collections::HashMap;
use std::str::FromStr;
use std;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VideoResponse {
    success: bool,
    #[serde(default)]
    error: Option<String>,
    id: String,
    #[serde(default)]
    status: Option<String>,
    #[serde(default, rename="downloadTotal", skip_serializing_if = "Option::is_none")]
    download_total: Option<i64>,
    #[serde(default, rename="downloadComplete", skip_serializing_if = "Option::is_none")]
    download_complete: Option<i64>,
    #[serde(default, rename="downloadCompleteEstimate", skip_serializing_if = "Option::is_none")]
    download_complete_estimate: Option<String>,
    #[serde(default, rename="framesCount", skip_serializing_if = "Option::is_none")]
    frames_count: Option<isize>,
    #[serde(default, rename="framesComplete", skip_serializing_if = "Option::is_none")]
    frames_complete: Option<isize>,
    #[serde(default, rename="lastFrameBase64", skip_serializing_if = "Option::is_none")]
    last_frame_base64: Option<String>,
    #[serde(default, rename="millisecondsComplete", skip_serializing_if = "Option::is_none")]
    milliseconds_complete: Option<isize>,
    #[serde(default)]
    expires: Option<String>,
}

/// Indicates the status of a video processing job
#[derive(Debug, PartialEq)]
pub enum Status {
    Pending,
    Downloading,
    Processing,
    Complete,
    Failed,
    Unknown,
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, <Self as FromStr>::Err> {
        match s {
            "pending" => Ok(Status::Pending),
            "downloading" => Ok(Status::Downloading),
            "processing" => Ok(Status::Processing),
            "complete" => Ok(Status::Complete),
            "failed" => Ok(Status::Failed),
            "unknown" => Ok(Status::Unknown),
            _ => Err(())
        }
    }
}

/// Represents a video
pub struct Video {
    pub id: String,
    pub status: Status,
    pub download_total: i64,
    pub download_complete: i64,
    pub download_complete_estimate: String,
    pub frames_count: isize,
    pub frames_complete: isize,
    pub last_frame_base64: String,
    pub milliseconds_complete: isize,
    pub expires: String,
}

impl Into<Result<Video>> for VideoResponse {
    fn into(self) -> Result<Video> {
        if self.success {
            let st = self.status.unwrap_or("unknown".to_owned());
            Ok(Video {
                id: self.id,
                status: Status::from_str(&st).unwrap(),
                //status: self.status.unwrap_or("unknown".to_owned()),
                download_total: self.download_total.unwrap_or(0),
                download_complete: self.download_complete.unwrap_or(0),
                download_complete_estimate: self.download_complete_estimate.unwrap_or("unknown".to_owned()),
                frames_count: self.frames_count.unwrap_or(0),
                frames_complete: self.frames_complete.unwrap_or(0),
                last_frame_base64: self.last_frame_base64.unwrap_or("".to_owned()),
                milliseconds_complete: self.milliseconds_complete.unwrap_or(0),
                expires: self.expires.unwrap_or("".to_owned())
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


/// Represents the set of options to be passed when invoking `check` to start
/// video analysis.
pub struct CheckOptions {
    kvs: HashMap<String, String>
}

impl IntoIterator for CheckOptions {
    type Item = (String,String);
    type IntoIter = ::std::collections::hash_map::IntoIter<String,String>;

    fn into_iter(self) -> Self::IntoIter {
        self.kvs.into_iter()
    }
}

/// A builder that allows for fluent creation of check options
/// # Examples
/// ```
/// use machinebox::videobox::CheckOptionsBuilder;
///
/// let opts = CheckOptionsBuilder::new()
///     .result_duration("1h0m0s")
///     .skip_frames(2)
///     .skip_seconds(3)
///     .frame_width(100)
///     .frame_height(120)
///     .facebox_threshold(0.75)
///     .tagbox_threshold(0.7)
///     .nudebox_threshold(0.2)
///     .finish();
/// ```
pub struct CheckOptionsBuilder {
    kvs: HashMap<String, String>,
}

impl CheckOptionsBuilder {
    /// Creates a new check options builder
    pub fn new() -> Self {
        CheckOptionsBuilder {
            kvs: HashMap::new(),
        }
    }

    /// Sets the duration the results should be kept in video box before being
    /// garbage collected
    pub fn result_duration(mut self, duration: &str) -> Self {
        self.kvs.insert("resultDuration".to_owned(), duration.to_owned());
        self
    }

    /// The number of frames to skip between extractions
    pub fn skip_frames(mut self, frames: isize) -> Self {
        self.kvs.insert("skipframes".to_owned(), frames.to_string());
        self
    }

    /// The number of seconds to skip between frame extractions
    pub fn skip_seconds(mut self, seconds: isize) -> Self {
        self.kvs.insert("skipseconds".to_owned(), seconds.to_string());
        self
    }

    /// Sets the width of the frame to extract
    pub fn frame_width(mut self, width: isize) -> Self {
        self.kvs.insert("frameWidth".to_owned(), width.to_string());
        self
    }

    /// Sets the height of the frame to extract
    pub fn frame_height(mut self, height:isize) -> Self {
        self.kvs.insert("frameHeight".to_owned(), height.to_string());
        self
    }

    /// Sets the number of frames to process concurrently
    pub  fn frame_concurrency(mut self, concurrency: isize) -> Self {
        self.kvs.insert("frameConcurrency".to_owned(), concurrency.to_string());
        self
    }

    /// Sets the minimum confidence threshold of facebox matches for the frame
    /// to be included in the results
    pub fn facebox_threshold(mut self, threshold: f64) -> Self {
        self.kvs.insert("faceboxThreshold".to_owned(), threshold.to_string());
        self
    }

    /// Sets the tagbox include value (e.g. 'all' or 'custom')
    pub fn tagbox_include(mut self, include: &str) -> Self {
        self.kvs.insert("tagboxInclude".to_owned(), include.to_owned());
        self
    }

    /// Sets the minimum confidence threshold of tagbox matches for the frame to
    /// be included in the results
    pub fn tagbox_threshold(mut self, threshold: f64) -> Self {
        self.kvs.insert("tagboxThreshold".to_owned(), threshold.to_string());
        self
    }

    /// Sets the minimum confidence threshold of nudebox matches for the frame to
    /// be included in the results
    pub fn nudebox_threshold(mut self, threshold: f64) -> Self {
        self.kvs.insert("nudeboxThreshold".to_owned(), threshold.to_string());
        self
    }

    /// Convert the builder into a set of check options ready for submission to the
    /// `check` function.
    pub fn finish(self) -> CheckOptions {
        CheckOptions {
            kvs: self.kvs
        }
    }
}

/// An item is a single entity that was discovered at one or many instances
/// within a video
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub key: String,
    #[serde(default)]
    pub instances: Vec<Range>,
}

/// A range describes a period of time within a video
#[derive(Serialize, Deserialize, Debug)]
pub struct Range {
    pub start: isize,
    pub end: isize,
    pub start_ms: isize,
    pub end_ms: isize,
    #[serde(default)]
    pub confidence: Option<f64>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoAnalysisResponse {
    success: bool,
    #[serde(default)]
    error: Option<String>,

    ready: bool,
    #[serde(default)]
    facebox: Option<Facebox>,
    #[serde(default)]
    tagbox: Option<Tagbox>,
    #[serde(default)]
    nudebox: Option<Nudebox>,
}

impl Into<Result<VideoAnalysis>> for VideoAnalysisResponse {
    fn into(self) -> Result<VideoAnalysis> {
        if self.success {
            Ok(VideoAnalysis{
                ready: self.ready,
                facebox: self.facebox,
                tagbox: self.tagbox,
                nudebox: self.nudebox
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

/// The results of a video analysis performed by calling `check`
pub struct VideoAnalysis {
    /// Indicates whether the results are ready
    pub ready: bool,
    /// Facebox analysis results
    pub facebox: Option<Facebox>,
    /// Tagbox analysis results
    pub tagbox: Option<Tagbox>,
    /// Nudebox analysis results
    pub nudebox: Option<Nudebox>,
}

/// Facebox-specific results
#[derive(Serialize, Deserialize, Debug)]
pub struct Facebox {
    pub faces: Vec<Item>,
    #[serde(rename = "errorsCount")]
    pub errors_count: isize,
    #[serde(default, rename = "lastError")]
    pub last_error: Option<String>,
}

/// Tagbox-specific results
#[derive(Serialize, Deserialize, Debug)]
pub struct Tagbox {
    pub tags: Vec<Item>,
    #[serde(rename = "errorsCount")]
    pub error_count: isize,
    #[serde(rename = "lastError", default)]
    pub last_error: Option<String>,
}

/// Nudebox-specific results
#[derive(Serialize, Deserialize, Debug)]
pub struct Nudebox {
    pub nudity: Vec<Item>,
    #[serde(rename = "errorsCount")]
    pub error_count: isize,
    #[serde(rename = "lastError", default)]
    pub last_error: Option<String>,
}