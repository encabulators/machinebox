extern crate machinebox;

use machinebox::textbox::Textbox;
use machinebox::BoxClient;

fn main() {
    let tb = Textbox::new("http://localhost:8080");
    let bi = tb.info();
    println!("{:#?}", bi);

    let health = tb.health();
    println!("{:#?}", health);

    println!("{}", tb.is_live().unwrap());
    println!("{}", tb.is_ready().unwrap());

    let analysis = tb.check("Pay William $200 USD tomorrow");
    if let Ok(res) = analysis {
        let money = res.sentences[0].entities.iter().find(|e| e.entity_type == "money");
        match money {
            Some(val) => println!("You specified {}", val.text),
            None => println!("You didn't indicate money"),
        }
    }
}