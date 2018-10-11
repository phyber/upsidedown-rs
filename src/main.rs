// Take a string and turn it upside down
use std::collections::HashMap;
use std::env;

mod translations;

type UpsideDownTranslation = HashMap<String, String>;

trait UpsideDown {
    fn upsidedown(&self) -> String;
    fn upsidedown_table(&self) -> UpsideDownTranslation;
}

impl UpsideDown for String {
    // Return a translation table for making a string upside down.
    fn upsidedown_table(&self) -> UpsideDownTranslation {
        let table = translations::table();
        return table;
    }

    // Take the string and flip it.
    fn upsidedown(&self) -> String {
        let table = self.upsidedown_table();

        let mut new = String::new();

        for c in self.chars().rev() {
            let c = c.to_string();

            match table.get(&c) {
                Some(n) => new.push_str(n),
                None    => new.push_str(&c),
            }
        }

        return new;
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let orig = args.join(" ").to_string();
    println!("{}", orig.upsidedown());
}
