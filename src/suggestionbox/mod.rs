//! A client for utilizing the `suggestionbox` machine
//!
//! **Suggestionbox** allows developers to create models, ask for predictions from those models,
//! and to train them by rewarding the predictions.
//!
//! For more information, see the [suggestionbox docs](https://machinebox.io/docs/suggestionbox)
use super::BoxClient;
use super::Result;
use reqwest;
use reqwest::StatusCode;
use serde_json;
use Error;
use Kind;

pub use self::types::{Choice, Feature, FeatureType, Model, ModelBuilder, ModelOptions, ModelStats};
pub use self::types::{Prediction, PredictionRequest, PredictionResponse, Reward};
use std::io::Write;
use std::collections::HashMap;

/// The client for the `suggestionbox` machinebox.
pub struct Suggestionbox {
    url: String,
}

impl Suggestionbox {
    /// Creates a new suggestionbox client
    pub fn new(url: &str) -> Suggestionbox {
        Suggestionbox {
            url: url.to_owned(),
        }
    }

    /// Creates a new model and returns a copy of the model as seen by the suggestion
    /// box, including the options used in model generation.
    pub fn create_model(&self, model: &Model) -> Result<Model> {
        let url = format!("{}/suggestionbox/models", self.url());
        let client = reqwest::Client::new();

        match client.post(&url).json(model).send() {
            Ok(mut response) => {
                let raw = response.text()?;
                if response.status() == StatusCode::Ok {
                    let newmodel: Model = serde_json::from_str(&raw)?;
                    Ok(newmodel)
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

    /// Deletes a model from the box. If the model doesn't exist, it will return
    /// an error of type `Machinebox` indicating an HTTP 404.
    pub fn delete_model(&self, id: &str) -> Result<()> {
        let url = format!("{}/suggestionbox/models/{}", self.url(), id);
        let client = reqwest::Client::new();
        match client.delete(&url).send() {
            Ok(response) => match response.status() {
                StatusCode::Ok => Ok(()),
                _ => Err(Error {
                    kind: Kind::Machinebox(format!("HTTP {}", response.status())),
                }),
            },
            Err(e) => Err(Error {
                kind: Kind::Reqwest(e),
            }),
        }
    }

    /// Retrieves a single model from the box
    pub fn get_model(&self, id: &str) -> Result<Model> {
        let url = format!("{}/suggestionbox/models/{}", self.url(), id);
        let client = reqwest::Client::new();
        match client.get(&url).send() {
            Ok(mut response) => {
                let raw = response.text()?;
                if response.status() != StatusCode::Ok {
                    Err(Error {
                        kind: Kind::Machinebox(format!("HTTP {}: {}", response.status(), raw)),
                    })
                } else {
                    let model: Model = serde_json::from_str(&raw)?;
                    Ok(model)
                }
            }
            Err(e) => Err(Error {
                kind: Kind::Reqwest(e),
            }),
        }
    }

    /// Lists all of the models currently managed by the suggestion box
    pub fn list_models(&self) -> Result<Vec<Model>> {
        let url = format!("{}/suggestionbox/models", self.url());
        let client = reqwest::Client::new();
        match client.get(&url).send() {
            Ok(mut response) => {
                let raw = response.text()?;
                let response: self::types::ModelList = serde_json::from_str(&raw)?;
                Ok(response.models)
            }
            Err(e) => Err(Error {
                kind: Kind::Reqwest(e),
            }),
        }
    }

    /// Obtains statistics about the given model
    pub fn get_model_stats(&self, id: &str) -> Result<ModelStats> {
        let url = format!("{}/suggestionbox/models/{}/stats", self.url(), id);
        let client = reqwest::Client::new();
        match client.get(&url).send() {
            Ok(mut response) => {
                let raw = response.text()?;
                if response.status() != StatusCode::Ok {
                    Err(Error {
                        kind: Kind::Machinebox(format!("HTTP {}: {}", response.status(), raw)),
                    })
                } else {
                    let stats: ModelStats = serde_json::from_str(&raw)?;
                    Ok(stats)
                }
            }
            Err(e) => Err(Error {
                kind: Kind::Reqwest(e),
            }),
        }
    }

    /// Asks the suggestionbox to make a prediction based upon the supplied list of features
    /// in the prediction request. Keep in mind that these features apply to the user for whom
    /// the prediction is being made, and don't imply any direct link to the features associated
    /// with model choices.
    pub fn predict(
        &self,
        model_id: &str,
        request: &PredictionRequest,
    ) -> Result<PredictionResponse> {
        let url = format!("{}/suggestionbox/models/{}/predict", self.url(), model_id);
        let client = reqwest::Client::new();
        match client.post(&url).json(request).send() {
            Ok(mut response) => {
                let raw = response.text()?;
                if response.status() != StatusCode::Ok {
                    Err(Error {
                        kind: Kind::Machinebox(format!("HTTP {}: {}", response.status(), raw)),
                    })
                } else {
                    let prediction: PredictionResponse = serde_json::from_str(&raw)?;
                    Ok(prediction)
                }
            }
            Err(e) => Err(Error {
                kind: Kind::Reqwest(e),
            }),
        }
    }

    /// Reward tells the suggestionbox about a successful prediction. Rewarding predictions
    /// is how this box learns to make better predictions over time. You will need to
    /// submit your reward within the `reward_expiration_seconds` timeout period of the model.
    pub fn reward(&self, model_id: &str, reward_id: &str, weight: f64) -> Result<()> {
        let reward = Reward {
            reward_id: reward_id.to_owned(),
            value: weight,
        };
        let url = format!("{}/suggestionbox/models/{}/rewards", self.url(), model_id);
        let client = reqwest::Client::new();
        match client.post(&url).json(&reward).send() {
            Ok(mut response) => {
                let raw = response.text()?;
                if response.status() != StatusCode::Ok {
                    Err(Error {
                        kind: Kind::Machinebox(format!("HTTP {}: {}", response.status(), raw)),
                    })
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(Error {
                kind: Kind::Reqwest(e),
            }),
        }
    }

    /// Fills the supplied buffer with the contents of the state file obtained
    /// from the machine box. This buffer can be any kind of `Write`, which includes
    /// empty vectors, files on disk, etc. The state file is binary. Returns the number
    /// of bytes written to the buffer.
    pub fn download_state<W>(&self, model_id: &str, buf: &mut W) -> Result<u64>
    where
        W: Write,
    {
        let url = format!("{}/suggestionbox/state/{}", self.url(), model_id);
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

    /// Submits the state file indicated by the `source_path` parameter to the suggestion box
    /// and returns the model originally contained in the state file
    pub fn post_state(&self, source_path: &str) -> Result<Model> {
        let url = format!("{}/suggestionbox/state", self.url());
        let form = reqwest::multipart::Form::new().file("state", source_path)?;
        let client = reqwest::Client::new();
        match client.post(&url).multipart(form).send() {
            Ok(mut response) => {
                let raw = response.text()?;
                if response.status() != StatusCode::Ok {
                    Err(Error {
                        kind: Kind::Machinebox(format!("HTTP {}: {}", response.status(), raw)),
                    })
                } else {
                    let model: Model = serde_json::from_str(&raw)?;
                    Ok(model)
                }
            }
            Err(e) => Err(Error {
                kind: Kind::Reqwest(e),
            }),
        }
    }

    /// Submits a URL to the suggestion box. The suggestion box will download the state
    /// contained in the file indicated by the URL and return the model from the state file
    pub fn post_state_url(&self, state_url: &str) -> Result<Model> {
        let url = format!("{}/suggestionbox/state", self.url());
        let mut params = HashMap::new();
        params.insert("url", state_url);
        let client = reqwest::Client::new();
        match client.post(&url).form(&params).send() {
            Ok(mut response) => {
                let raw = response.text()?;
                if response.status() != StatusCode::Ok {
                    Err(Error {
                        kind: Kind::Machinebox(format!("HTTP {}: {}", response.status(), raw)),
                    })
                } else {
                    let model: Model = serde_json::from_str(&raw)?;
                    Ok(model)
                }
            }
            Err(e) => Err(Error {
                kind: Kind::Reqwest(e),
            }),
        }
    }
}

impl BoxClient for Suggestionbox {
    fn url(&self) -> &str {
        &self.url
    }
}

mod types;

#[cfg(test)]
mod tests;
