// Take a string and turn it upside down
use std::collections::HashMap;
use std::env;

type UpsideDownTranslation = HashMap<String, String>;

trait UpsideDown {
    fn upsidedown_table(&self) -> UpsideDownTranslation;
    fn upsidedown(&self) -> String;
}

impl UpsideDown for String {
    // Return a translation table for making a string upside down.
    fn upsidedown_table(&self) -> UpsideDownTranslation {
        let mut table = UpsideDownTranslation::new();

        // Alphabet
        table.insert("a".to_string(), "\u{0250}".to_string());
        table.insert("b".to_string(), "q".to_string());
        table.insert("c".to_string(), "\u{0254}".to_string());
        table.insert("d".to_string(), "p".to_string());
        table.insert("e".to_string(), "\u{01DD}".to_string());
        table.insert("f".to_string(), "\u{025F}".to_string());
        table.insert("g".to_string(), "\u{0183}".to_string());
        table.insert("h".to_string(), "\u{0265}".to_string());
        table.insert("i".to_string(), "\u{0131}".to_string());
        table.insert("j".to_string(), "\u{027E}".to_string());
        table.insert("k".to_string(), "\u{029E}".to_string());
        table.insert("l".to_string(), "\u{05DF}".to_string());
        table.insert("m".to_string(), "\u{026F}".to_string());
        table.insert("n".to_string(), "u".to_string());
        table.insert("o".to_string(), "o".to_string());
        table.insert("p".to_string(), "d".to_string());
        table.insert("q".to_string(), "b".to_string());
        table.insert("r".to_string(), "\u{0279}".to_string());
        table.insert("s".to_string(), "s".to_string());
        table.insert("t".to_string(), "\u{0287}".to_string());
        table.insert("u".to_string(), "n".to_string());
        table.insert("v".to_string(), "\u{028C}".to_string());
        table.insert("w".to_string(), "\u{028D}".to_string());
        table.insert("x".to_string(), "x".to_string());
        table.insert("y".to_string(), "\u{028E}".to_string());
        table.insert("z".to_string(), "z".to_string());

        // Punctuation
        table.insert(".".to_string(), "\u{02D9}".to_string());
        table.insert("[".to_string(), "]".to_string());
        table.insert("]".to_string(), "[".to_string());
        table.insert("{".to_string(), "}".to_string());
        table.insert("}".to_string(), "{".to_string());
        table.insert("(".to_string(), ")".to_string());
        table.insert(")".to_string(), "(".to_string());
        table.insert("?".to_string(), "\u{00BF}".to_string());
        table.insert("!".to_string(), "\u{00A1}".to_string());
        table.insert("'".to_string(), ",".to_string());
        table.insert(",".to_string(), "'".to_string());
        table.insert("<".to_string(), ">".to_string());
        table.insert(">".to_string(), "<".to_string());
        table.insert("_".to_string(), "\u{203E}".to_string());
        table.insert('"'.to_string(), "\u{201E}".to_string());
        table.insert('\\'.to_string(), '\\'.to_string());
        table.insert(";".to_string(), "\u{061B}".to_string());
        table.insert("\u{203F}".to_string(), "\u{2040}".to_string());
        table.insert("\u{2045}".to_string(), "\u{2046}".to_string());
        table.insert("\u{2234}".to_string(), "\u{2235}".to_string());

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
                None => new.push_str(&c),
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
