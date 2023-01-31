// Translation tables
use std::collections::HashMap;

const ALPHABET: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const NUMBERS: &[(char, char)] = &[
    ('0', '0'),
    ('1', 'Ɩ'),
    ('2', 'ᄅ'),
    ('3', 'Ɛ'),
    ('4', 'ㄣ'),
    ('5', 'ϛ'),
    ('6', '9'),
    ('7', 'ㄥ'),
    ('8', '8'),
    ('9', '6'),
];

const PUNCTUATION: &[(char, char)] = &[
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

const UPSIDE_DOWN_ALPHA_UPPER: &[char] = &[
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

const UPSIDE_DOWN_ALPHA_LOWER: &[char] = &[
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

pub fn table() -> HashMap<char, char> {
    let mut table: HashMap<char, char> = HashMap::new();

    // Zip upper and lower translations together.
    let upper_lower = UPSIDE_DOWN_ALPHA_UPPER.iter()
        .zip(UPSIDE_DOWN_ALPHA_LOWER.iter());


    // Alphabet translations
    for (old, (new_upper, new_lower)) in ALPHABET.iter().zip(upper_lower) {
        // Upper
        table.insert(*old, *new_upper);

        // Lower
        table.insert(old.to_ascii_lowercase(), *new_lower);
    }

    // Number translations
    for (old, new) in NUMBERS {
        table.insert(*old, *new);
    }

    // Punctuation
    for (old, new) in PUNCTUATION {
        table.insert(*old, *new);
    }

    table
}
