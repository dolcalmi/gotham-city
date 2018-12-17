#[macro_use]
extern crate clap;
use clap::App;

use reqwest;
use client_lib::ecdsa::keygen;

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let client = reqwest::Client::new();

    if let Some(matches) = matches.subcommand_matches("keygen-ecdsa") {
        let verbose = matches.is_present("verbose");
        keygen::get_master_key(&client, verbose);
    }
}
