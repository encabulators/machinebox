extern crate machinebox;
extern crate serde_json;

use std::{thread, time};
use std::time::Duration;
use machinebox::Error;
use machinebox::tagbox::{CheckResponse, Tagbox};

fn main() {
    if let Err(e) = tag_sample() {
        println!("Failed to run suggestionbox samples: {}", e);
    }
}

fn tag_sample() -> Result<(), machinebox::Error> {
    let tagbox = Tagbox::new("http://localhost:8080");

    println!("Analyzing towerbridge.jpg...");
    let tower = tagbox.check_url("https://machinebox.io/samples/images/towerbridge.jpg")?;

    println!("tower tags: \n{:#?}", tower);

    println!("teaching about monkeys...");
    let _ = tagbox.teach_url(
        "https://machinebox.io/samples/images/monkey.jpg",
        "monkeys",
        Some("monkey.jpg".to_owned()),
    )?;
    println!("Taught about monkeys, waiting a few seconds for index to refresh...");

    thread::sleep(Duration::from_secs(5));
    let monkeycheck = tagbox.check_url("https://machinebox.io/samples/images/monkey.jpg")?;
    println!("taught monkey result - {:#?}.", monkeycheck);

    let _rename = tagbox.rename_custom_tag("monkey.jpg", "floobers")?;

    let monkeycheck2 = tagbox.check_url("https://machinebox.io/samples/images/monkey.jpg")?;
    println!("{:#?}", monkeycheck2.custom_tags);

    // Remove monkeys tag
    let _ = tagbox.remove_custom_tag("monkey.jpg")?;
    Ok(())
}
