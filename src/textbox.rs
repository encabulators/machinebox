//! A client for utilizing the `textbox` machine
//!
//! **Textbox** processes text and performs natural language processing, sentiment analysis,
//! and entity and keyword extraction. For more information, see the [textbox docs](https://machinebox.io/docs/textbox)
use super::BoxClient;
use super::Result;
use reqwest;
use serde_json;
use Error;
use Kind;

/// Textbox represents a client capable of consuming the box's functionality
pub struct Textbox {
    url: String,
}

/// An analysis contains the results of a call to `check` on the textbox
#[derive(Serialize, Deserialize, Debug)]
pub struct Analysis {
    pub sentences: Vec<Sentence>,
    pub keywords: Vec<Keyword>,
}

/// A sentence identified in an `Analysis`.
///
/// It is a container for the raw text of the sentence, as well as an optional sentiment score and
/// a list of the entities discovered within.
#[derive(Serialize, Deserialize, Debug)]
pub struct Sentence {
    pub text: String,
    pub start: u32,
    pub end: u32,
    pub sentiment: f64,
    pub entities: Vec<Entity>,
}

/// An typed entity produced through an `Analysis`
///
/// Entities have a type, as well as the text that was captured as part of the analysis.
/// Be careful not to rely solely on entity identification for program logic.
/// The `text` field may still need additional processing post-analysis.
#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    #[serde(rename = "type")]
    pub entity_type: String,
    pub text: String,
    pub start: u32,
    pub end: u32,
}

/// Keywords are simple text tokens identified within sentences
#[derive(Serialize, Deserialize, Debug)]
pub struct Keyword {
    pub keyword: String,
}

impl Textbox {
    /// Creates a new textbox client
    ///
    /// # Arguments
    ///
    /// * `url` - The URL where the textbox machine is running
    pub fn new(url: &str) -> Textbox {
        Textbox {
            url: url.to_owned(),
        }
    }

    /// Check performs textual analysis of the input and returns the result in the form of
    /// an analysis struct.
    pub fn check(&self, text: &str) -> Result<Analysis> {
        let url = format!("{}/textbox/check", self.url());
        let params = [("text", text)];
        let client = reqwest::Client::new();
        match client.post(&url)
            .form(&params)
            .send()
            {
                Ok(mut response) => {
                    let raw = response.text()?;
                    let analysis: Analysis = serde_json::from_str(&raw)?;
                    Ok(analysis)
                },
                Err(e) => {
                    Err(Error { kind: Kind::Reqwest(e)} )
                }
            }
    }
}

impl BoxClient for Textbox {
    fn url(&self) -> &str {
        &self.url
    }
}