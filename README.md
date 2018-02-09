[![Build Status](https://travis-ci.org/encabulators/machinebox.svg?branch=master)](https://travis-ci.org/encabulators/machinebox)

# Machinebox Client

**machinebox** is a Rust client for the [machinebox.io](http://machinebox.io) suite of machines.
It provides a simple means of accessing the machines, exposing common functionality to all machines as well
as box-specific functionality. Each "box" or "box type" is separated into its own module and struct.

## Modules
The following is a list of the modules corresponding to machinebox types and which ones
have been implemented so far:

- [X] Textbox
- [ ] Facebox
- [ ] Tagbox
- [ ] Videobox
- [ ] Nudebox
- [X] Suggestionbox
- [ ] Fakebox

## Usage
To use features, simply call the appropriate function on the corresponding box:

```rust
extern crate machinebox;

use machinebox::textbox::Textbox;
use machinebox::BoxClient;

// Make sure you actually have a textbox running here...
let tb = Textbox::new("http://localhost:8080");

let analysis = tb.check("Pay William $200 tomorrow");
    if let Ok(res) = analysis {
        let money = res.sentences[0].entities.iter().find(|e| e.entity_type == "money");
        match money {
            Some(val) => println!("You specified {}", val.text),
            None => println!("You didn't indicate money"),
        }
    }
```

The above code will analyse the phrase `Pay William $200 tomorrow`, flagging 
`tomorrow` as a date and `200` as an entity of type `money`.

## Notes
If you run the tests locally, you'll need `RUST_TEST_THREADS=1` because the mock HTTP server
persists throughout the lifetime of the test run.