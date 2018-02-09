extern crate machinebox;
extern crate serde_json;

use std::fs::File;
use machinebox::suggestionbox::{Feature, Model, PredictionRequest, Suggestionbox};

fn main() {
    if let Err(e) = suggestion_sample() {
        println!("Failed to run suggestionbox samples: {}", e);
    }
}

fn suggestion_sample() -> Result<(), machinebox::Error> {
    let mut model_file = File::open("examples/movie_model.json")?;
    let movie_model = Model::from_file(&mut model_file)?;
    println!(
        "Loaded {} choices from the movie demo model",
        movie_model.choices.len()
    );

    let sb = Suggestionbox::new("http://localhost:8080");

    let _ = sb.delete_model("movie_demo"); // Remove before we start

    let _res = sb.create_model(&movie_model)?;

    let request = PredictionRequest {
        inputs: vec![
            Feature::number("age", 42.0),
            Feature::list("favorite_genres", vec!["action", "christmas", "adventure"]),
            Feature::keyword("country", "USA"),
        ],
    };
    println!("Picking Die Hard from 1,000 predictions...");
    for _iter in 0..1000 {
        let predict = sb.predict("movie_demo", &request)?;
        let diehard = predict.choices.iter().find(|c| c.id == "diehard");
        match diehard {
            Some(ref prediction) => {
                sb.reward("movie_demo", &prediction.reward_id, 1.0)?;
            }
            None => {
                sb.reward("movie_demo", &predict.choices[0].reward_id, 1.0)?;
            }
        }
    }
    println!("Trained 1,000 Die Hard selections");

    let response = sb.predict("movie_demo", &request)?;
    println!("Prediction response post-training: {:#?}", response);

    let stats = sb.get_model_stats("movie_demo")?;
    println!("model stats - {:#?}", stats);

    let mut file = File::create("examples/movie_demo.suggestionbox")?;
    sb.download_state("movie_demo", &mut file)?;

    let _delres = sb.delete_model("movie_demo")?;

    Ok(())
}
