extern crate machinebox;

use machinebox::textbox::Textbox;
use machinebox::BoxClient;

fn main() {
    if let Err(e) = perform_analysis() {
        println!("Failed to perform analysis: {}", e);
    }
}

fn perform_analysis() -> Result<(), machinebox::Error> {
    let tb = Textbox::new("http://localhost:8080");
    let bi = tb.info()?;

    println!("{:#?}", bi);

    let health = tb.health()?;
    println!("{:#?}", health);

    let live = tb.is_live()?;
    println!("{}", live);
    let ready = tb.is_ready()?;
    println!("{}", ready);

    let analysis = tb.check("Pay William $200 USD tomorrow")?;
    let money = analysis.sentences[0]
        .entities
        .iter()
        .find(|e| e.entity_type == "money");
    match money {
        Some(val) => println!("You specified {}", val.text),
        None => println!("You didn't indicate money"),
    }

    Ok(())
}
