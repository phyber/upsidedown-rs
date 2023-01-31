// Take a string and turn it upside down
use std::collections::HashMap;
use std::env;

mod translations;

type UpsideDownTranslation = HashMap<char, char>;

trait UpsideDown {
    fn upsidedown(&self) -> String;
    fn upsidedown_table(&self) -> UpsideDownTranslation;
}

impl UpsideDown for String {
    // Return a translation table for making a string upside down.
    fn upsidedown_table(&self) -> UpsideDownTranslation {
        translations::table()
    }

    // Take the string and flip it.
    fn upsidedown(&self) -> String {
        let table = self.upsidedown_table();

        self.chars()
            .rev()
            .map(|c| {
                let c = match table.get(&c) {
                    Some(c) => c,
                    None    => &c,
                };

                *c
            })
            .collect::<Vec<char>>()
            .iter()
            .collect::<String>()
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let orig = args.join(" ");
    println!("{}", orig.upsidedown());
}
