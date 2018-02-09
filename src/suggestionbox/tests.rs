extern crate mockito;

use std;
use self::mockito::{mock, SERVER_URL};
use super::Suggestionbox;
use super::ModelBuilder;
use suggestionbox::Feature;
use suggestionbox::PredictionRequest;
use std::fs::File;

#[test]
fn create_model() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("POST", "/suggestionbox/models")
        .with_body(
            r#"{
	"success": true,
	"id": "model1",
	"name": "Articles",
	"options": {
		"reward_expiration_seconds": 120,
		"ngrams": 1,
		"skipgrams": 0,
		"epsilon": 0.3
	}
}"#,
        )
        .create();
    {
        let model = ModelBuilder::new()
            .named("test")
            .id("testmodel")
            .choice("choice1", vec![Feature::text("foo", "bar")])
            .choice("choice2", vec![Feature::text("baz", "foo")])
            .finish();

        let res = sb.create_model(&model);
        assert!(res.is_ok());
        if let Ok(outmodel) = res {
            assert_eq!(outmodel.id, Some("model1".to_string()));
            assert_eq!(outmodel.name, "Articles");
            assert_eq!(outmodel.options.unwrap().reward_expiration_seconds, 120);
        }
    }
    mock.assert();
}

#[test]
fn get_model() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("GET", "/suggestionbox/models/model1")
        .with_body(
            r#"{
            "id": "model1",
            "name": "Articles",
            "choices": [],
            "options": {
                "reward_expiration_seconds": 120,
                "ngrams": 1,
                "skipgrams": 0,
                "epsilon": 0.3
            }
    }"#,
        )
        .create();
    {
        let res = sb.get_model("model1");
        assert!(res.is_ok());
        if let Ok(outmodel) = res {
            assert_eq!(outmodel.id, Some("model1".to_string()));
            assert_eq!(outmodel.name, "Articles");
            let opts = outmodel.options.unwrap();
            assert_eq!(opts.reward_expiration_seconds, 120);
            assert_eq!(opts.ngrams, 1_i32);
        }
    }
    mock.assert();
}

#[test]
fn get_model_reports_failure() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("GET", "/suggestionbox/models/model1")
        .with_status(404)
        .create();
    {
        let res = sb.get_model("model1");
        assert!(res.is_err());
    }
    mock.assert();
}

#[test]
fn list_models() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("GET", "/suggestionbox/models")
        .with_body(
            r#"{
            "success": true,
            "models": [
                {
                    "id": "model1",
                    "name": "Articles",
                    "choices": [],
                    "options": {
                        "reward_expiration_seconds": 120,
                        "ngrams": 1,
                        "skipgrams": 0,
                        "epsilon": 0.3
                    }
                }
            ]
        }
        "#,
        )
        .create();
    {
        let res = sb.list_models();
        assert!(res.is_ok());
        if let Ok(models) = res {
            assert_eq!(models.len(), 1);
            let m0 = &models[0];
            assert_eq!(m0.id, Some("model1".to_owned()));
            assert_eq!(m0.name, "Articles");
            if let Some(ref opts) = m0.options {
                assert_eq!(opts.ngrams, 1_i32);
            }
        }
    }
    mock.assert();
}

#[test]
fn list_models_reports_failure() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("GET", "/suggestionbox/models")
        .with_status(500)
        .create();
    {
        let res = sb.list_models();
        assert!(res.is_err());
    }
    mock.assert();
}

#[test]
fn get_model_stats() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("GET", "/suggestionbox/models/model1/stats")
        .with_body(
            r#"{
            "predictions": 100,
            "rewards": 80,
            "reward_ratio": 0.8,
            "explores": 30,
            "exploits": 70,
            "explore_ratio": 0.3
        }"#,
        )
        .create();
    {
        let res = sb.get_model_stats("model1");
        assert!(res.is_ok());
        if let Ok(stats) = res {
            assert_eq!(stats.predictions, 100);
            assert_eq!(stats.rewards, 80);
            assert_eq!(stats.reward_ratio, 0.8);
            assert_eq!(stats.explores, 30);
            assert_eq!(stats.exploits, 70);
            assert_eq!(stats.explore_ratio, 0.3);
        }
    }
    mock.assert();
}

#[test]
fn get_model_stats_reports_failure() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("GET", "/suggestionbox/models/model1/stats")
        .with_status(500)
        .create();
    {
        let res = sb.get_model_stats("model1");
        assert!(res.is_err());
    }
    mock.assert();
}

#[test]
fn predict() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("POST", "/suggestionbox/models/model1/predict")
        .with_body(
            r#"{
            "choices": [
                {
                    "id": "choice3",
                    "score": 0.60,
                    "reward_id": "ad17da8d1d8bef78678dad"
                },
                {
                    "id": "choice1",
                    "score": 0.30,
                    "reward_id": "bef78678dadad17da8d1d8"
                },
                {
                    "id": "choice2",
                    "score": 0.10,
                    "reward_id": "8678dad8d1d8ad1bef77da"
                }
            ]
        }"#,
        )
        .create();
    {
        let request = PredictionRequest { inputs: Vec::new() };
        let res = sb.predict("model1", &request);
        assert!(res.is_ok());
        if let Ok(prediction) = res {
            assert_eq!(prediction.choices.len(), 3);
            assert_eq!(prediction.choices[0].id, "choice3");
            assert_eq!(prediction.choices[1].score, 0.30);
            assert_eq!(prediction.choices[2].reward_id, "8678dad8d1d8ad1bef77da");
        }
    }
    mock.assert();
}

#[test]
fn predict_reports_failure() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("POST", "/suggestionbox/models/model1/predict")
        .with_status(404)
        .create();
    {
        let request = PredictionRequest { inputs: Vec::new() };
        let res = sb.predict("model1", &request);
        assert!(res.is_err());
    }
    mock.assert();
}

#[test]
fn reward() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("POST", "/suggestionbox/models/model1/rewards")
        .with_status(200)
        .create();
    {
        let res = sb.reward("model1", "8678dad8d1d8ad1bef77da", 1.0);
        assert!(res.is_ok());
    }
    mock.assert();
}

#[test]
fn reward_reports_failure() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("POST", "/suggestionbox/models/model1/rewards")
        .with_status(404)
        .create();
    {
        let res = sb.reward("model1", "8678dad8d1d8ad1bef77da", 1.0);
        assert!(res.is_err());
    }
    mock.assert();
}

#[test]
fn download_state() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("GET", "/suggestionbox/state/model1")
        .with_body("1234512345")
        .create();
    {
        let mut buf: Vec<u8> = vec![];
        let res = sb.download_state("model1", &mut buf);
        assert!(res.is_ok());
        if let Ok(bytecount) = res {
            assert_eq!(bytecount, 10);
        }
    }
    mock.assert();
}

#[test]
fn download_state_reports_error() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("GET", "/suggestionbox/state/model1")
        .with_status(404)
        .create();
    {
        let mut buf: Vec<u8> = vec![];
        let res = sb.download_state("model1", &mut buf);
        assert!(res.is_err());
    }
    mock.assert();
}

#[test]
fn post_state() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("POST", "/suggestionbox/state")
        .with_body(
            r#"{
            "id": "model1",
            "name": "Articles",
            "choices": [],
            "options": {
                "reward_expiration_seconds": 120,
                "ngrams": 1,
                "skipgrams": 0,
                "epsilon": 0.3
            }
    }"#,
        )
        .create();
    {
        let _file = File::create("test.txt").unwrap();
        let res = sb.post_state("test.txt");
        assert!(res.is_ok());
        std::fs::remove_file("test.txt").unwrap();
        if let Ok(model) = res {
            assert_eq!(model.id, Some("model1".to_owned()));
            assert_eq!(model.name, "Articles");
        }
    }
    mock.assert();
}

#[test]
fn post_state_reports_failure() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("POST", "/suggestionbox/state")
        .with_status(500)
        .create();
    {
        let _file = File::create("test.txt").unwrap();
        let res = sb.post_state("test.txt");
        assert!(res.is_err());
        std::fs::remove_file("test.txt").unwrap();
    }
    mock.assert();
}

#[test]
fn post_state_url() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("POST", "/suggestionbox/state")
        .with_body(
            r#"{
            "id": "model1",
            "name": "Articles",
            "choices": [],
            "options": {
                "reward_expiration_seconds": 120,
                "ngrams": 1,
                "skipgrams": 0,
                "epsilon": 0.3
            }
    }"#,
        )
        .create();
    {
        let res = sb.post_state_url("http://this/is/a/url");
        assert!(res.is_ok());
        if let Ok(model) = res {
            assert_eq!(model.id, Some("model1".to_owned()));
            assert_eq!(model.name, "Articles");
        }
    }
    mock.assert();
}

#[test]
fn post_state_url_reports_error() {
    let sb = Suggestionbox::new(&SERVER_URL);
    let mock = mock("POST", "/suggestionbox/state")
        .with_status(500)
        .create();
    {
        let res = sb.post_state_url("http://this/is/a/url");
        assert!(res.is_err());
    }
    mock.assert();
}
