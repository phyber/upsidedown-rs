// Translation tables
use std::collections::HashMap;

const PUNCTUATION: &'static [(char, char)] = &[
    ('.', '\u{02D9}'),
    ('[', ']'),
    (']', '['),
    ('{', '}'),
    ('}', '{'),
    ('(', ')'),
    (')', '('),
    ('?', '\u{00BF}'),
    ('!', '\u{00A1}'),
    ('\'', ','),
    (',', '\''),
    ('<', '>'),
    ('>', '<'),
    ('_', '\u{203E}'),
    ('\"', '\u{201E}'),
    ('\\', '\\'),
    (';', '\u{061B}'),        // ; => ؛
    ('\u{203F}', '\u{2040}'), // ‿ => ⁀
    ('\u{2045}', '\u{2046}'), // ⁅ => ⁆
    ('\u{2234}', '\u{2235}'), // ‴ => ‵
];

const UPSIDE_DOWN_ALPHA_UPPER: &'static [char] = &[
    '∀',
    'B', // FIXME
    'Ɔ',
    'D', // FIXME
    'Ǝ',
    'Ⅎ',
    'פ',
    'H',
    'I',
    'ſ',
    'K', // FIXME
    '˥',
    'W',
    'N',
    'O',
    'Ԁ',
    'Q', // FIXME
    'R', // FIXME
    'S',
    '┴',
    '∩',
    'Λ',
    'M',
    'X',
    '⅄',
    'Z',
];

const UPSIDE_DOWN_ALPHA_LOWER: &'static [char] = &[
    '\u{0250}',
    'q',
    '\u{0254}',
    'p',
    '\u{01DD}',
    '\u{025F}',
    '\u{0183}',
    '\u{0265}',
    '\u{0131}',
    '\u{027E}',
    '\u{029E}',
    '\u{05DF}',
    '\u{026F}',
    'u',
    'o',
    'd',
    'b',
    '\u{0279}',
    's',
    '\u{0287}',
    'n',
    '\u{028C}',
    '\u{028D}',
    'x',
    '\u{028E}',
    'z',
];

const UPSIDE_DOWN_NUMBER: &'static [char] = &[
    '0',
    'Ɩ',
    'ᄅ',
    'Ɛ',
    'ㄣ',
    'ϛ',
    '9',
    'ㄥ',
    '8',
    '6',
];

pub fn table() -> HashMap<String, String> {
    let mut table: HashMap<String, String> = HashMap::new();

    // Alphabet, upper followed by lower.
    let alphabet = (b'A' .. b'Z' + 1)  // u8
        .map(|c| c as char)            // convert to char
        .filter(|c| c.is_alphabetic()) // Filter alphabetics
        .collect::<Vec<_>>();          // Collect chars

    // Zip upper and lower translations together.
    let iter = UPSIDE_DOWN_ALPHA_UPPER.iter()
        .zip(UPSIDE_DOWN_ALPHA_LOWER.iter());


    // Alphabet translations
    for (old, (new_upper, new_lower)) in alphabet.iter().zip(iter) {
        // Upper
        table.insert(old.to_string(), new_upper.to_string());

        // Lower
        table.insert(old.to_lowercase().to_string(), new_lower.to_string());
    }

    // Number translations
    let numbers = (0 .. 9).into_iter().zip(UPSIDE_DOWN_NUMBER.iter());
    for (old, new) in numbers {
        table.insert(old.to_string(), new.to_string());
    }

    // Punctuation
    for (old, new) in PUNCTUATION {
        table.insert(old.to_string(), new.to_string());
    }

    return table;
}
