extern crate machinebox;
extern crate serde_json;

use std::thread;
use std::time::Duration;
use machinebox::facebox::Facebox;

fn main() {
    if let Err(e) = face_sample() {
        println!("Failed to run facebox samples: {}", e);
    }
}

fn face_sample() -> Result<(), machinebox::Error> {
    let facebox = Facebox::new("http://localhost:8080");

    let _res = facebox.teach_url("https://machinebox.io/samples/faces/john.jpg",
        "john.jpg", "John Lennon")?;
    println!("Taught facebox by URL, waiting...");

    thread::sleep(Duration::from_secs(4)); // Allow the box to update internal data

    let check_response = facebox.check_url("https://machinebox.io/samples/faces/thebeatles.jpg")?;
    println!("Performed face scan by URL.");

    let matched: Vec<_> = check_response.faces.iter().filter(|f| f.matched == true).collect();

    println!("{:#?}", matched);

    Ok(())
}
