use crate::Prompt;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fmt;
use std::fs::read_to_string;

struct Stats {
    min_number: f64,
    max_number: f64,
    unit_of_measure: HashMap<String, usize>,
    place: HashMap<String, usize>,
    adjective: HashMap<String, usize>,
    noun: HashMap<String, usize>,
}

// Macro for updating the count of a particular value inside one of the text maps
macro_rules! update_map {
    ( $self: ident, $map: ident, $prompt: ident) => {
        $self
            .$map
            .entry($prompt.$map.clone())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    };
}

impl Stats {
    fn new() -> Self {
        Self {
            min_number: f64::MAX,
            max_number: f64::MIN,
            unit_of_measure: HashMap::new(),
            place: HashMap::new(),
            adjective: HashMap::new(),
            noun: HashMap::new(),
        }
    }

    fn update_with_prompt(&mut self, prompt: &Prompt) {
        if prompt.number < self.min_number {
            self.min_number = prompt.number
        }
        if prompt.number > self.max_number {
            self.max_number = prompt.number
        }
        update_map!(self, unit_of_measure, prompt);
        update_map!(self, place, prompt);
        update_map!(self, adjective, prompt);
        update_map!(self, noun, prompt);
    }
}

// Finds the most common occurance of a value in a hash map
fn most_common(map: &HashMap<String, usize>) -> Option<(&String, &usize)> {
    map.iter().max_by(|v1, v2| v1.1.cmp(v2.1))
}

// Find the most common value in one of the text maps, and print that value and the count
macro_rules! print_most_common {
    ( $self:ident, $f: ident, $x:ident ) => {{
        if let Some((value, occurences)) = most_common(&$self.$x) {
            writeln!(
                $f,
                "Most common {}: {}. Occurences: {}",
                stringify!($x),
                value,
                occurences
            )?;
        }
    }};
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Min number: {}", self.min_number)?;
        writeln!(f, "Max number: {}", self.max_number)?;
        print_most_common!(self, f, unit_of_measure);
        print_most_common!(self, f, place);
        print_most_common!(self, f, adjective);
        print_most_common!(self, f, noun);
        Ok(())
    }
}

pub fn stats(file: &str) -> Result<()> {
    let mut stats = Stats::new();
    let file_contents =
        read_to_string(file).context(format!("Could not read from input file {}", file))?;
    file_contents
        .lines()
        .filter_map(|line| serde_json::from_str::<Prompt>(line).ok())
        .for_each(|prompt| stats.update_with_prompt(&prompt));
    print!("{}", stats);
    Ok(())
}
