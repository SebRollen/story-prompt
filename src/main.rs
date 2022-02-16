use anyhow::Result;
use clap::{App, AppSettings, Arg};
use serde::{Deserialize, Serialize};
use validator::Validate;

mod generate;
mod stats;

#[derive(Serialize, Deserialize, Validate)]
struct Prompt {
    number: f64,
    #[validate(length(min = 1, max = 50))]
    unit_of_measure: String,
    #[validate(length(min = 1, max = 50))]
    place: String,
    #[validate(length(min = 1, max = 50))]
    adjective: String,
    #[validate(length(min = 1, max = 50))]
    noun: String,
}

fn main() -> Result<()> {
    let matches = App::new("story-generator")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::new("file").short('f').default_value("valid-data.txt"))
        .subcommands(vec![
            App::new("generate")
                .about("Generate and print a story from JSON input, and store the input if valid")
                .arg(Arg::new("data").required(true)),
            App::new("stats").about("Calculate and print statistics about valid inputs"),
        ])
        .get_matches();

    let file = matches
        .value_of("file")
        .expect("Safe to unwrap as there is a default");

    match matches.subcommand() {
        Some(("generate", matches)) => {
            let data = matches
                .value_of("data")
                .expect("Safe to unwrap as DATA is required");

            generate::generate(data, file)
        }
        Some(("stats", _)) => stats::stats(file),
        _ => unreachable!("Subcommand is required"),
    }
}
