use super::Result;
use serde_json;
use std::fs::File;
use std::io::Read;

/// A model represents a single model inside Suggestionbox
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    /// The ID of the model
    pub id: Option<String>,
    /// The name of the model
    pub name: String,
    /// Model options
    #[serde(default)]
    pub options: Option<ModelOptions>,
    /// The choices are the options this model will select from
    #[serde(default)]
    pub choices: Vec<Choice>,
}

impl Model {
    /// Convenience method to create a new model from a JSON file
    pub fn from_file(f: &mut File) -> Result<Model> {
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        let m: Model = serde_json::from_str(&contents)?;
        Ok(m)
    }
}

/// Allows for natural, fluent creation of new models for submission to the
/// suggestionbox.
/// # Examples
/// ```
/// use machinebox::suggestionbox::{ModelBuilder, Feature};
///
/// let newmodel = ModelBuilder::new()
///                 .named("My model")
///                 .id("model1")
///                 .choice(
///                    "article1",
///                    vec![Feature::text("title", "Machine Box releases new product")])
///                 .choice(
///                    "article2",
///                    vec![Feature::text("title", "The beatles reunite")])
///                 .finish();
/// ```
pub struct ModelBuilder {
    name: String,
    id: Option<String>,
    choices: Vec<Choice>,
    options: Option<ModelOptions>,
}

impl ModelBuilder {
    /// Creates a new modelbuilder with reasonable defaults set
    pub fn new() -> ModelBuilder {
        ModelBuilder {
            name: "default".to_owned(),
            id: None,
            choices: Vec::new(),
            options: None,
        }
    }

    /// Provides a name for the model
    pub fn named(mut self, name: &str) -> ModelBuilder {
        self.name = name.to_owned();
        self
    }

    /// Sets the ID of the model. If you do not set the ID, suggestionbox will assign
    /// you one automatically.
    pub fn id(mut self, id: &str) -> ModelBuilder {
        self.id = Some(id.to_owned());
        self
    }

    /// Adds a new choice to the model builder with the indicated list of features
    pub fn choice(mut self, id: &str, features: Vec<Feature>) -> ModelBuilder {
        self.choices.push(Choice {
            id: id.to_owned(),
            features: features,
        });
        self
    }

    /// Sets the options for the model
    pub fn options(mut self, options: ModelOptions) -> ModelBuilder {
        self.options = Some(options);
        self
    }

    /// Creates a new from the builder. As indicated by the name `finish`, this builder
    /// will be unusable after this method call as its values will have moved into the new
    /// `Model`.
    pub fn finish(self) -> Model {
        Model {
            name: self.name,
            id: self.id,
            choices: self.choices,
            options: self.options,
        }
    }
}

/// Various configuration parameters that can be used to tweak the behavior and learning
/// options of the suggestionbox model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelOptions {
    /// Determines the maximum length of time in seconds between when a prediction is presented
    /// to a client and when a reward is posted.
    #[serde(default)]
    pub reward_expiration_seconds: u64,
    /// Epsilon enables proportionate exploiting vs. exploring ratio.
    #[serde(default)]
    pub epsilon: f64,
    /// Soft max lambda enables adaptive exploiting vs exploring ratio.
    #[serde(default)]
    pub softmax_lambda: f64,
    /// The n-grams used for analysis
    #[serde(default)]
    pub ngrams: i32,
    /// skip-grams used for text analysis
    #[serde(default)]
    pub skipgrams: i32,
}

/// A choice represents a value that can be predicted for a user, and includes a set of features
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Choice {
    /// The ID of the choice
    pub id: String,
    /// List of features for this choice
    pub features: Vec<Feature>,
}

/// Tells suggestionbox how to treat the feature value when making predictions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FeatureType {
    /// Indicates a numerical feature
    #[serde(rename = "number")]
    Number,
    /// Indicates a textual feature that will be tokenized. Use `Keyword` if you don't want
    /// tokenization.
    #[serde(rename = "text")]
    Text,
    /// Indicates a non-tokenized textual feature. A `list` can be used to supply multiple
    /// keywords as a feature value
    #[serde(rename = "keyword")]
    Keyword,
    /// Indicates a list of non-tokenized keywords.
    #[serde(rename = "list")]
    List,
    /// Indicates a feature value that is a URL
    #[serde(rename = "image_url")]
    ImageURL,
    /// Indicates a feature value that is a binary image encoded with Base64
    #[serde(rename = "image_base64")]
    ImageBase64,
}

/// A feature is used to describe an input or a choice. For example, age:28 or location:"London"
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Feature {
    /// The feature's key
    pub key: String,
    /// The value of the feature
    pub value: String,
    /// The type of this feature
    #[serde(rename = "type")]
    pub feature_type: FeatureType,
}

impl Feature {
    /// Shortcut for producing a new text feature
    pub fn text(key: &str, text: &str) -> Feature {
        Feature {
            key: key.to_owned(),
            value: text.to_owned(),
            feature_type: FeatureType::Text,
        }
    }

    /// Shortcut for production a numerical feature
    pub fn number(key: &str, number: f64) -> Feature {
        Feature {
            key: key.to_owned(),
            value: format!("{}", number),
            feature_type: FeatureType::Number,
        }
    }

    /// Shortcut for producing a keyword feature
    pub fn keyword(key: &str, keyword: &str) -> Feature {
        Feature {
            key: key.to_owned(),
            value: keyword.to_owned(),
            feature_type: FeatureType::Keyword,
        }
    }

    /// Shortcut for producing a keyword list feature
    pub fn list(key: &str, list: Vec<&str>) -> Feature {
        Feature {
            key: key.to_owned(),
            value: list.join(","),
            feature_type: FeatureType::List,
        }
    }

    /// Shortcut for producing an image URL feature
    pub fn image_url(key: &str, url: &str) -> Feature {
        Feature {
            key: key.to_owned(),
            value: url.to_owned(),
            feature_type: FeatureType::ImageURL,
        }
    }

    /// Shortcut for producing a base64-encoded image feature
    pub fn image_base64(key: &str, data: &str) -> Feature {
        Feature {
            key: key.to_owned(),
            value: data.to_owned(),
            feature_type: FeatureType::ImageBase64,
        }
    }
}

/// Provides statistics for a model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelStats {
    /// The number of predictions this model has made
    pub predictions: u64,
    /// The number of rewards this model has _received_
    pub rewards: u64,
    /// The ratio between predictions and rewards
    pub reward_ratio: f64,
    /// The number of times the model has explored to learn new things
    pub explores: u64,
    /// The number of times the model has exploited learning
    pub exploits: u64,
    /// The ratio between exploring and exploiting
    pub explore_ratio: f64,
}

/// A wrapper for the suggestionbox response that includes a list of models
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelList {
    pub models: Vec<Model>,
    pub success: bool,
}

/// A prediction response produced by the suggestionbox
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PredictionResponse {
    /// List of predicted choices
    #[serde(default)]
    pub choices: Vec<Prediction>,
}

/// A prediction is a predicted choice
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prediction {
    /// The choice identifier being predicted
    pub id: String,
    /// The ID of the reward that should be submitted if this prediction turns out to be
    /// successful
    pub reward_id: String,
    /// The score is a numerical value indicating how this prediction relates to other
    /// predictions.
    #[serde(default)]
    pub score: f64,
}

/// A request for a prediction
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PredictionRequest {
    /// Inputs used in the prediction request
    pub inputs: Vec<Feature>,
}

/// A reward is used to inform the suggestionbox of a successful prediction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reward {
    /// The ID of the reward as presented during the prediction
    pub reward_id: String,
    /// The weight of the reward, usually `1`.
    pub value: f64,
}
