//! # Morsify
//!
//! Morsify is a Rust crate for encoding and decoding Morse code. It provides a flexible and efficient way to convert
//! text into Morse code and vice versa, with customizable options for different Morse code representations and character sets.
//!
//! ## Features
//!
//! - **Encoding**: Convert plain text into Morse code with customizable symbols for dots, dashes, spaces, and separators.
//! - **Decoding**: Convert Morse code back into readable text using the provided configuration.
//! - **Customizable Character Sets**: Support for various character sets including Latin, Greek, Cyrillic, Arabic, and others.
//! - **Configurable Options**: Define how Morse code should be represented with options for symbols and handling invalid characters.
//!
//! ## Usage
//!
//! To use Morsify, include it in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! morsify = "0.1.0"
//! ```
//!
//! Then, use the `MorseCode` struct to encode and decode text. Here’s a basic example:
//!
//! ```rust
//! use morsify::{MorseCode, Options, MorseCharacterSet};
//!
//! // Create a new `MorseCode` instance with default options
//! let options = Options {
//!     dash: '-',
//!     dot: '.',
//!     space: '/',
//!     separator: ' ',
//!     invalid_char_callback: |c| c,
//!     priority: MorseCharacterSet::Latin,
//! };
//! let morse_code = MorseCode::new(options);
//!
//! // Encode a text message to Morse code
//! let encoded = morse_code.encode("Hello World");
//! println!("Encoded: {}", encoded);
//!
//! // Decode a Morse code message to text
//! let decoded = morse_code.decode(".... . .-.. .-.. --- / .-- --- .-. .-.. -..");
//! println!("Decoded: {}", decoded);
//! ```
//!
//! ## API Documentation
//!
//! For detailed information about the API, refer to the module documentation and individual methods of the `MorseCode` struct.
//!
//! ## License
//!
//! Morsify is licensed under the MIT License. See the `LICENSE` file for more details.

#![no_std]

use alloc::{
    collections::btree_map::BTreeMap,
    string::{String, ToString},
};
extern crate alloc;

/// A type alias for a map that associates Morse code characters with their string representations.
///
/// This alias defines a `BTreeMap` where:
/// - The key is a `MorseCharacterSet`, which represents a specific set or category of Morse code characters.
/// - The value is another `BTreeMap`, where:
///   - The key is a `char` representing a single character.
///   - The value is a `String` providing the string representation or description of the Morse code character.
///
/// This structure supports efficient storage and retrieval of Morse code characters and their associated string descriptions.
/// The use of `BTreeMap` ensures that the data is kept in sorted order, enabling efficient lookups.
type Characters = BTreeMap<MorseCharacterSet, BTreeMap<char, String>>;

/// Enumerates the different character sets used in Morse code.
///
/// Each variant represents a specific alphabet or character set that can be encoded or decoded.
/// The variants include common scripts as well as less common ones.
///
/// /// For more information about each character set, you can refer to the following links:
///
/// - [Latin Alphabet](https://en.wikipedia.org/wiki/Morse_code)
/// - [Extended Latin Characters](https://ham.stackexchange.com/questions/1379/international-characters-in-morse-code)
/// - [Cyrillic Alphabet](https://en.wikipedia.org/wiki/Russian_Morse_code)
/// - [Greek Alphabet](https://en.wikipedia.org/wiki/Morse_code_for_non-Latin_alphabets)
/// - [Hebrew Alphabet](https://en.wikipedia.org/wiki/Morse_code_for_non-Latin_alphabets)
/// - [Arabic Alphabet](https://en.wikipedia.org/wiki/Morse_code_for_non-Latin_alphabets)
/// - [Persian Alphabet](https://en.wikipedia.org/wiki/Morse_code_for_non-Latin_alphabets)
/// - [Japanese Characters](https://ja.wikipedia.org/wiki/%E3%83%A2%E3%83%BC%E3%83%AB%E3%82%B9%E7%AC%A6%E5%8F%B7#%E5%92%8C%E6%96%87%E3%83%A2%E3%83%BC%E3%83%AB%E3%82%B9%E7%AC%A6%E5%8F%B7)
/// - [Korean Characters](https://en.wikipedia.org/wiki/SKATS)
/// - [Thai Characters](https://th.wikipedia.org/wiki/รหัสมอร์ส)
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MorseCharacterSet {
    /// Represents an undefined character set.
    Undefined,
    /// Represents the Latin alphabet.
    Latin,
    /// Represents numerical digits.
    Numbers,
    /// Represents punctuation marks.
    Punctuation,
    /// Represents extended Latin characters.
    LatinExtended,
    /// Represents the Cyrillic alphabet.
    Cyrillic,
    /// Represents the Greek alphabet.
    Greek,
    /// Represents the Hebrew alphabet.
    Hebrew,
    /// Represents the Arabic alphabet.
    Arabic,
    /// Represents the Persian alphabet.
    Persian,
    /// Represents Japanese characters.
    Japanese,
    /// Represents Korean characters.
    Korean,
    /// Represents Thai characters.
    Thai,
}

/// Contains options for encoding and decoding Morse code.
///
/// This struct allows customization of Morse code encoding and decoding by specifying the characters used
/// for dashes, dots, spaces, separators, and invalid characters, as well as a priority character set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Options {
    /// Character used to represent a dash in Morse code.
    pub dash: char,
    /// Character used to represent a dot in Morse code.
    pub dot: char,
    /// Character used to represent a space between words in Morse code.
    pub space: char,
    /// Character used to separate Morse code characters.
    pub separator: char,
    /// Priority character set to use for encoding.
    pub priority: MorseCharacterSet,
    /// A function used to get represented an invalid Morse code character.
    pub invalid_char_callback: fn(char) -> char,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            dash: '-',
            dot: '.',
            space: '/',
            separator: ' ',
            invalid_char_callback: |c| c,
            priority: MorseCharacterSet::Latin,
        }
    }
}

/// A struct to manage Morse code operations including encoding and decoding.
///
/// This struct holds options and character mappings required for encoding and decoding Morse code.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MorseCode {
    /// Configuration options for encoding and decoding Morse code.
    ///
    /// This field specifies how Morse code should be represented, including the symbols used for dots, dashes,
    /// spaces, separators, and handling invalid characters. It also defines the priority character set to use.
    options: Options,

    /// A map of Morse code characters and their string representations.
    ///
    /// This field contains a `BTreeMap` where each key is a `MorseCharacterSet` indicating a specific set or category
    /// of Morse code characters, and each value is another `BTreeMap` mapping individual Morse code characters to their
    /// string representations or descriptions. This structure supports efficient storage and retrieval of Morse code data.
    characters: Characters,
}

impl Default for MorseCode {
    fn default() -> Self {
        let options = Options::default();
        Self {
            options,
            characters: get_characters(options),
        }
    }
}

impl MorseCode {
    /// Creates a new `MorseCode` instance with the given options.
    ///
    /// # Arguments
    ///
    /// * `options` - The options to use for encoding and decoding Morse code.
    ///
    /// # Returns
    ///
    /// A `MorseCode` instance configured with the provided options.
    #[must_use]
    pub fn new(options: Options) -> Self {
        let characters = get_characters(options);
        MorseCode {
            options,
            characters,
        }
    }

    /// Encodes the given text into Morse code using the struct’s options.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to encode.
    ///
    /// # Returns
    ///
    /// A `String` containing the encoded Morse code.
    pub fn encode<S: AsRef<str>>(&self, text: S) -> String {
        let mut result = String::new();

        let processed_text = text
            .as_ref()
            .replace(char::is_whitespace, &self.options.separator.to_string())
            .trim()
            .to_uppercase();

        for character in processed_text.chars() {
            let mut found = false;
            for set in self.characters.values() {
                if let Some(encoded) = set.get(&character) {
                    result.push_str(encoded);
                    found = true;
                    break;
                }
            }
            if !found {
                (self.options.invalid_char_callback)(character);
                result.push((self.options.invalid_char_callback)(character));
            }
            result.push(self.options.separator);
        }

        result = result
            .replace('0', &self.options.dot.to_string())
            .replace('1', &self.options.dash.to_string());

        if !result.is_empty() && result.ends_with(&self.options.separator.to_string()) {
            result.pop();
        }

        result
    }

    /// Decodes the given Morse code string into text using the struct’s options.
    ///
    /// # Arguments
    ///
    /// * `morse` - The Morse code string to decode.
    ///
    /// # Returns
    ///
    /// A `String` containing the decoded text.
    pub fn decode(&self, morse: &str) -> String {
        let swapped = swap_characters(self.options);

        morse
            .replace(char::is_whitespace, &self.options.separator.to_string()) // Replace whitespace with separator
            .trim() // Trim leading and trailing whitespace
            .split(self.options.separator) // Split by the separator
            .map(|characters| {
                swapped
                    .get(characters)
                    .copied()
                    .map_or_else(|| characters.to_string(), |c| c.to_string())
            })
            .collect::<String>() // Collect into a single String
    }
}

/// Generates a complete set of Morse code characters for various languages and symbols.
///
/// This function creates and returns a `Characters` mapping that includes Morse code representations
/// for Latin, numbers, punctuation, Latin extended characters, Cyrillic, Greek, Hebrew, Arabic,
/// Persian, Japanese, Korean, and Thai characters. Each set of characters is stored in a `BTreeMap`
/// and organized by the type of Morse character set.
fn base_characters() -> Characters {
    let mut characters = BTreeMap::new();
    let latin = latin_chars();
    characters.insert(MorseCharacterSet::Latin, latin);

    let numbers = numbers_chars();
    characters.insert(MorseCharacterSet::Numbers, numbers);

    let punctuation = punctuation_chars();
    characters.insert(MorseCharacterSet::Punctuation, punctuation);

    let latin_extended = latin_extended_chars();
    characters.insert(MorseCharacterSet::LatinExtended, latin_extended);

    let cyrillic = cyrillic_chars();
    characters.insert(MorseCharacterSet::Cyrillic, cyrillic);

    let greek = greek_chars();
    characters.insert(MorseCharacterSet::Greek, greek);

    let hebrew = hebrew_chars();
    characters.insert(MorseCharacterSet::Hebrew, hebrew);

    let arabic = arabic_chars();
    characters.insert(MorseCharacterSet::Arabic, arabic);

    let persian = persian_chars();
    characters.insert(MorseCharacterSet::Persian, persian);

    let japanese = japanese_chars();
    characters.insert(MorseCharacterSet::Japanese, japanese);

    let korean = korean_chars();
    characters.insert(MorseCharacterSet::Korean, korean);

    let thai = thai_chars();
    characters.insert(MorseCharacterSet::Thai, thai);

    characters
        .into_iter()
        .map(|(charset, char_map)| {
            (
                charset,
                char_map
                    .into_iter()
                    .map(|(char, morse_code)| (char, morse_code.to_string()))
                    .collect(),
            )
        })
        .collect::<Characters>()
}
/// Returns a `BTreeMap` of Morse code representations for Latin characters.
///
/// This function provides the Morse code mappings for Latin alphabet characters, where each key is a Latin character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Latin characters as keys and their Morse code representations as values.
fn latin_chars<'a>() -> BTreeMap<char, &'a str> {
    let mut latin = BTreeMap::new();
    latin.insert('A', "01");
    latin.insert('B', "1000");
    latin.insert('C', "1010");
    latin.insert('D', "100");
    latin.insert('E', "0");
    latin.insert('F', "0010");
    latin.insert('G', "110");
    latin.insert('H', "0000");
    latin.insert('I', "00");
    latin.insert('J', "0111");
    latin.insert('K', "101");
    latin.insert('L', "0100");
    latin.insert('M', "11");
    latin.insert('N', "10");
    latin.insert('O', "111");
    latin.insert('P', "0110");
    latin.insert('Q', "1101");
    latin.insert('R', "010");
    latin.insert('S', "000");
    latin.insert('T', "1");
    latin.insert('U', "001");
    latin.insert('V', "0001");
    latin.insert('W', "011");
    latin.insert('X', "1001");
    latin.insert('Y', "1011");
    latin.insert('Z', "1100");
    latin
}

/// Returns a `BTreeMap` of Morse code representations for digits (0-9).
///
/// This function provides the Morse code mappings for numeric digits, where each key is a digit and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with digits as keys and their Morse code representations as values.
fn numbers_chars<'a>() -> BTreeMap<char, &'a str> {
    let mut numbers = BTreeMap::new();
    numbers.insert('0', "11111");
    numbers.insert('1', "01111");
    numbers.insert('2', "00111");
    numbers.insert('3', "00011");
    numbers.insert('4', "00001");
    numbers.insert('5', "00000");
    numbers.insert('6', "10000");
    numbers.insert('7', "11000");
    numbers.insert('8', "11100");
    numbers.insert('9', "11110");
    numbers
}

/// Returns a `BTreeMap` of Morse code representations for punctuation marks.
///
/// This function provides the Morse code mappings for various punctuation marks, where each key is a punctuation mark
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with punctuation marks as keys and their Morse code representations as values.
fn punctuation_chars<'a>() -> BTreeMap<char, &'a str> {
    let mut punctuation = BTreeMap::new();
    punctuation.insert('.', "010101");
    punctuation.insert(',', "110011");
    punctuation.insert('?', "001100");
    punctuation.insert('\'', "011110");
    punctuation.insert('!', "101011");
    punctuation.insert('/', "10010");
    punctuation.insert('(', "10110");
    punctuation.insert(')', "101101");
    punctuation.insert('&', "01000");
    punctuation.insert(':', "111000");
    punctuation.insert(';', "101010");
    punctuation.insert('=', "10001");
    punctuation.insert('+', "01010");
    punctuation.insert('-', "100001");
    punctuation.insert('_', "001101");
    punctuation.insert('"', "010010");
    punctuation.insert('$', "0001001");
    punctuation.insert('@', "011010");
    punctuation.insert('¿', "00101");
    punctuation.insert('¡', "110001");
    punctuation
}

/// Returns a `BTreeMap` of Morse code representations for Latin Extended characters.
///
/// This function provides the Morse code mappings for Latin Extended characters, where each key is a Latin Extended character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Latin Extended characters as keys and their Morse code representations as values.
fn latin_extended_chars<'a>() -> BTreeMap<char, &'a str> {
    let mut latin_extended = BTreeMap::new();
    latin_extended.insert('Ã', "01101");
    latin_extended.insert('Á', "01101");
    latin_extended.insert('Å', "01101");
    latin_extended.insert('À', "01101");
    latin_extended.insert('Â', "01101");
    latin_extended.insert('Ä', "0101");
    latin_extended.insert('Ą', "0101");
    latin_extended.insert('Æ', "0101");
    latin_extended.insert('Ç', "10100");
    latin_extended.insert('Ć', "10100");
    latin_extended.insert('Ĉ', "10100");
    latin_extended.insert('Č', "110");
    latin_extended.insert('Ð', "00110");
    latin_extended.insert('È', "01001");
    latin_extended.insert('Ę', "00100");
    latin_extended.insert('Ë', "00100");
    latin_extended.insert('É', "00100");
    latin_extended.insert('Ê', "10010");
    latin_extended.insert('Ğ', "11010");
    latin_extended.insert('Ĝ', "11010");
    latin_extended.insert('Ĥ', "1111");
    latin_extended.insert('İ', "01001");
    latin_extended.insert('Ï', "10011");
    latin_extended.insert('Ì', "01110");
    latin_extended.insert('Ĵ', "01110");
    latin_extended.insert('Ł', "01001");
    latin_extended.insert('Ń', "11011");
    latin_extended.insert('Ñ', "11011");
    latin_extended.insert('Ó', "1110");
    latin_extended.insert('Ò', "1110");
    latin_extended.insert('Ö', "1110");
    latin_extended.insert('Ô', "1110");
    latin_extended.insert('Ø', "1110");
    latin_extended.insert('Ś', "0001000");
    latin_extended.insert('Ş', "01100");
    latin_extended.insert('Ș', "1111");
    latin_extended.insert('Š', "1111");
    latin_extended.insert('Ŝ', "00010");
    latin_extended.insert('ß', "000000");
    latin_extended.insert('Þ', "01100");
    latin_extended.insert('Ü', "0011");
    latin_extended.insert('Ù', "0011");
    latin_extended.insert('Ŭ', "0011");
    latin_extended.insert('Ž', "11001");
    latin_extended.insert('Ź', "110010");
    latin_extended.insert('Ż', "11001");
    latin_extended
}

/// Returns a `BTreeMap` of Morse code representations for Cyrillic characters.
///
/// This function provides the Morse code mappings for Cyrillic alphabet characters, where each key is a Cyrillic character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Cyrillic characters as keys and their Morse code representations as values.
fn cyrillic_chars<'a>() -> BTreeMap<char, &'a str> {
    let mut cyrillic = BTreeMap::new();
    cyrillic.insert('А', "01");
    cyrillic.insert('Б', "1000");
    cyrillic.insert('В', "011");
    cyrillic.insert('Г', "110");
    cyrillic.insert('Д', "100");
    cyrillic.insert('Е', "0");
    cyrillic.insert('Ж', "0001");
    cyrillic.insert('З', "1100");
    cyrillic.insert('И', "00");
    cyrillic.insert('Й', "0111");
    cyrillic.insert('К', "101");
    cyrillic.insert('Л', "0100");
    cyrillic.insert('М', "11");
    cyrillic.insert('Н', "10");
    cyrillic.insert('О', "111");
    cyrillic.insert('П', "0110");
    cyrillic.insert('Р', "010");
    cyrillic.insert('С', "000");
    cyrillic.insert('Т', "1");
    cyrillic.insert('У', "001");
    cyrillic.insert('Ф', "0010");
    cyrillic.insert('Х', "0000");
    cyrillic.insert('Ц', "1010");
    cyrillic.insert('Ч', "1110");
    cyrillic.insert('Ш', "1111");
    cyrillic.insert('Щ', "1101");
    cyrillic.insert('Ъ', "11011");
    cyrillic.insert('Ы', "1011");
    cyrillic.insert('Ь', "1001");
    cyrillic.insert('Э', "00100");
    cyrillic.insert('Ю', "0011");
    cyrillic.insert('Я', "0101");
    cyrillic.insert('Ї', "01110");
    cyrillic.insert('Є', "00100");
    cyrillic.insert('І', "00");
    cyrillic.insert('Ґ', "110");
    cyrillic
}

/// Returns a `BTreeMap` of Morse code representations for Greek characters.
///
/// This function provides the Morse code mappings for Greek alphabet characters, where each key is a Greek character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Greek characters as keys and their Morse code representations as values.
fn greek_chars<'a>() -> BTreeMap<char, &'a str> {
    let mut greek = BTreeMap::new();
    greek.insert('Α', "01");
    greek.insert('Β', "1000");
    greek.insert('Γ', "110");
    greek.insert('Δ', "100");
    greek.insert('Ε', "0");
    greek.insert('Ζ', "1100");
    greek.insert('Η', "0000");
    greek.insert('Θ', "1010");
    greek.insert('Ι', "00");
    greek.insert('Κ', "101");
    greek.insert('Λ', "0100");
    greek.insert('Μ', "11");
    greek.insert('Ν', "10");
    greek.insert('Ξ', "1001");
    greek.insert('Ο', "111");
    greek.insert('Π', "0110");
    greek.insert('Ρ', "010");
    greek.insert('Σ', "000");
    greek.insert('Τ', "1");
    greek.insert('Υ', "1011");
    greek.insert('Φ', "0010");
    greek.insert('Χ', "1111");
    greek.insert('Ψ', "1101");
    greek.insert('Ω', "011");
    greek
}

/// Returns a `BTreeMap` of Morse code representations for Hebrew characters.
///
/// This function provides the Morse code mappings for Hebrew script characters, where each key is a Hebrew character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Hebrew characters as keys and their Morse code representations as values.
fn hebrew_chars<'a>() -> BTreeMap<char, &'a str> {
    let mut hebrew = BTreeMap::new();
    hebrew.insert('א', "01");
    hebrew.insert('ב', "1000");
    hebrew.insert('ג', "110");
    hebrew.insert('ד', "100");
    hebrew.insert('ה', "111");
    hebrew.insert('ו', "0");
    hebrew.insert('ז', "1100");
    hebrew.insert('ח', "0000");
    hebrew.insert('ט', "001");
    hebrew.insert('י', "00");
    hebrew.insert('כ', "101");
    hebrew.insert('ל', "0100");
    hebrew.insert('מ', "11");
    hebrew.insert('נ', "10");
    hebrew.insert('ס', "1010");
    hebrew.insert('ע', "0111");
    hebrew.insert('פ', "0110");
    hebrew.insert('צ', "011");
    hebrew.insert('ק', "1101");
    hebrew.insert('ר', "010");
    hebrew.insert('ש', "000");
    hebrew.insert('ת', "1");
    hebrew
}

/// Returns a `BTreeMap` of Morse code representations for Arabic characters.
///
/// This function provides the Morse code mappings for Arabic script characters, where each key is an Arabic character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Arabic characters as keys and their Morse code representations as values.
fn arabic_chars<'a>() -> BTreeMap<char, &'a str> {
    let mut arabic = BTreeMap::new();
    arabic.insert('ا', "01");
    arabic.insert('ب', "1000");
    arabic.insert('ت', "1");
    arabic.insert('ث', "1010");
    arabic.insert('ج', "0111");
    arabic.insert('ح', "0000");
    arabic.insert('خ', "111");
    arabic.insert('د', "100");
    arabic.insert('ذ', "1100");
    arabic.insert('ر', "010");
    arabic.insert('ز', "1110");
    arabic.insert('س', "000");
    arabic.insert('ش', "1111");
    arabic.insert('ص', "1001");
    arabic.insert('ض', "0001");
    arabic.insert('ط', "001");
    arabic.insert('ظ', "1011");
    arabic.insert('ع', "0101");
    arabic.insert('غ', "110");
    arabic.insert('ف', "0010");
    arabic.insert('ق', "1101");
    arabic.insert('ك', "101");
    arabic.insert('ل', "0100");
    arabic.insert('م', "11");
    arabic.insert('ن', "10");
    arabic.insert('ه', "00100");
    arabic.insert('و', "011");
    arabic.insert('ي', "00");
    arabic.insert('ﺀ', "0");
    arabic
}

/// Returns a `BTreeMap` of Morse code representations for Persian characters.
///
/// This function provides the Morse code mappings for Persian script characters, where each key is a Persian character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Persian characters as keys and their Morse code representations as values.
fn persian_chars<'a>() -> BTreeMap<char, &'a str> {
    let mut persian = BTreeMap::new();
    persian.insert('ا', "01");
    persian.insert('ب', "1000");
    persian.insert('پ', "0110");
    persian.insert('ت', "1");
    persian.insert('ث', "1010");
    persian.insert('ج', "0111");
    persian.insert('چ', "1110");
    persian.insert('ح', "0000");
    persian.insert('خ', "1001");
    persian.insert('د', "100");
    persian.insert('ذ', "0001");
    persian.insert('ر', "010");
    persian.insert('ز', "1100");
    persian.insert('ژ', "110");
    persian.insert('س', "000");
    persian.insert('ش', "1111");
    persian.insert('ص', "0101");
    persian.insert('ض', "00100");
    persian.insert('ط', "001");
    persian.insert('ظ', "1011");
    persian.insert('ع', "111");
    persian.insert('غ', "0011");
    persian.insert('ف', "0010");
    persian.insert('ق', "111000");
    persian.insert('ک', "101");
    persian.insert('گ', "1101");
    persian.insert('ل', "0100");
    persian.insert('م', "11");
    persian.insert('ن', "10");
    persian.insert('و', "011");
    persian.insert('ه', "0");
    persian.insert('ی', "00");
    persian
}

/// Returns a `BTreeMap` of Morse code representations for Japanese characters.
///
/// This function provides the Morse code mappings for Japanese Katakana characters, where each key is a Japanese character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Japanese characters as keys and their Morse code representations as values.
fn japanese_chars<'a>() -> BTreeMap<char, &'a str> {
    let mut japanese = BTreeMap::new();
    japanese.insert('ア', "11011");
    japanese.insert('カ', "0100");
    japanese.insert('サ', "10101");
    japanese.insert('タ', "10");
    japanese.insert('ナ', "010");
    japanese.insert('ハ', "1000");
    japanese.insert('マ', "1001");
    japanese.insert('ヤ', "011");
    japanese.insert('ラ', "000");
    japanese.insert('ワ', "101");
    japanese.insert('イ', "01");
    japanese.insert('キ', "10100");
    japanese.insert('シ', "11010");
    japanese.insert('チ', "0010");
    japanese.insert('ニ', "1010");
    japanese.insert('ヒ', "11001");
    japanese.insert('ミ', "00101");
    japanese.insert('リ', "110");
    japanese.insert('ヰ', "01001");
    japanese.insert('ウ', "001");
    japanese.insert('ク', "0001");
    japanese.insert('ス', "11101");
    japanese.insert('ツ', "0110");
    japanese.insert('ヌ', "0000");
    japanese.insert('フ', "1100");
    japanese.insert('ム', "1");
    japanese.insert('ユ', "10011");
    japanese.insert('ル', "10110");
    japanese.insert('ン', "01010");
    japanese.insert('エ', "10111");
    japanese.insert('ケ', "1011");
    japanese.insert('セ', "01110");
    japanese.insert('テ', "01011");
    japanese.insert('ネ', "1101");
    japanese.insert('ヘ', "0");
    japanese.insert('メ', "10001");
    japanese.insert('レ', "111");
    japanese.insert('ヱ', "01100");
    japanese.insert('オ', "01000");
    japanese.insert('コ', "1111");
    japanese.insert('ソ', "1110");
    japanese.insert('ト', "00100");
    japanese.insert('ノ', "0011");
    japanese.insert('ホ', "100");
    japanese.insert('モ', "10010");
    japanese.insert('ヨ', "11");
    japanese.insert('ロ', "0101");
    japanese.insert('ヲ', "0111");
    japanese.insert('゛', "00");
    japanese.insert('゜', "00110");
    japanese.insert('。', "010100");
    japanese.insert('ー', "01101");
    japanese.insert('、', "010101");
    japanese.insert('（', "101101");
    japanese.insert('）', "010010");
    japanese
}

/// Returns a `BTreeMap` of Morse code representations for Korean characters.
///
/// This function provides the Morse code mappings for Korean Hangul characters, where each key is a Korean character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Korean characters as keys and their Morse code representations as values.
fn korean_chars<'a>() -> BTreeMap<char, &'a str> {
    let mut korean = BTreeMap::new();
    korean.insert('ㄱ', "0100");
    korean.insert('ㄴ', "0010");
    korean.insert('ㄷ', "1000");
    korean.insert('ㄹ', "0001");
    korean.insert('ㅁ', "11");
    korean.insert('ㅂ', "011");
    korean.insert('ㅅ', "110");
    korean.insert('ㅇ', "101");
    korean.insert('ㅈ', "0110");
    korean.insert('ㅊ', "1010");
    korean.insert('ㅋ', "1001");
    korean.insert('ㅌ', "1100");
    korean.insert('ㅍ', "111");
    korean.insert('ㅎ', "0111");
    korean.insert('ㅏ', "0");
    korean.insert('ㅑ', "00");
    korean.insert('ㅓ', "1");
    korean.insert('ㅕ', "000");
    korean.insert('ㅗ', "01");
    korean.insert('ㅛ', "10");
    korean.insert('ㅜ', "0000");
    korean.insert('ㅠ', "010");
    korean.insert('ㅡ', "100");
    korean.insert('ㅣ', "001");
    korean
}

/// Returns a `BTreeMap` of Morse code representations for Thai characters.
///
/// This function provides the Morse code mappings for Thai script characters, where each key is a Thai character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Thai characters as keys and their Morse code representations as values.   
fn thai_chars<'a>() -> BTreeMap<char, &'a str> {
    let mut thai = BTreeMap::new();
    thai.insert('ก', "110");
    thai.insert('ข', "1010");
    thai.insert('ค', "101");
    thai.insert('ง', "10110");
    thai.insert('จ', "10010");
    thai.insert('ฉ', "1111");
    thai.insert('ช', "1001");
    thai.insert('ซ', "1100");
    thai.insert('ญ', "0111");
    thai.insert('ด', "100");
    thai.insert('ต', "1");
    thai.insert('ถ', "10100");
    thai.insert('ท', "10011");
    thai.insert('น', "10");
    thai.insert('บ', "1000");
    thai.insert('ป', "0110");
    thai.insert('ผ', "1101");
    thai.insert('ฝ', "10101");
    thai.insert('พ', "01100");
    thai.insert('ฟ', "0010");
    thai.insert('ม', "11");
    thai.insert('ย', "1011");
    thai.insert('ร', "010");
    thai.insert('ล', "0100");
    thai.insert('ว', "011");
    thai.insert('ส', "000");
    thai.insert('ห', "0000");
    thai.insert('อ', "10001");
    thai.insert('ฮ', "11011");
    thai.insert('ฤ', "01011");
    thai.insert('ะ', "01000");
    thai.insert('า', "01");
    thai.insert('ิ', "00100");
    thai.insert('ี', "00");
    thai.insert('ึ', "00110");
    thai.insert('ื', "0011");
    thai.insert('ุ', "00101");
    thai.insert('ู', "1110");
    thai.insert('เ', "0");
    thai.insert('แ', "0101");
    thai.insert('ไ', "01001");
    thai.insert('โ', "111");
    thai.insert('ำ', "00010");
    thai.insert('่', "001");
    thai.insert('้', "0001");
    thai.insert('๊', "11000");
    thai.insert('๋', "01010");
    thai.insert('ั', "01101");
    thai.insert('็', "11100");
    thai.insert('์', "11001");
    thai.insert('ๆ', "10111");
    thai.insert('ฯ', "11010");
    thai
}

/// Retrieves a `Characters` map based on the given `Options` configuration.
///
/// This function generates a `Characters` map that includes Morse code representations for various character sets,
/// incorporating any options specified, such as a priority character set or custom separators and spaces.
///
/// # Parameters
/// - `options`: A configuration object containing options for character sets and Morse code representation.
///
/// # Returns
/// A `Characters` map where each key is a `MorseCharacterSet` and each value is a `BTreeMap` of characters and their Morse code representations.
fn get_characters(options: Options) -> Characters {
    let base_characters = base_characters();
    let mut characters = base_characters.clone();

    if let Some(priority_set) = base_characters.get(&options.priority) {
        characters.insert(MorseCharacterSet::Undefined, priority_set.clone());
    }

    if let Some(set_1) = base_characters.get(&MorseCharacterSet::Latin) {
        let mut new_set_1 = set_1.clone();
        new_set_1.insert(options.separator, options.space.to_string());
        characters.insert(MorseCharacterSet::Latin, new_set_1);
    }

    characters
        .into_iter()
        .map(|(key, value)| (key, value.into_iter().collect::<BTreeMap<char, String>>()))
        .collect::<Characters>()
}

/// Returns a `Characters` map with Morse code characters mapped to custom symbols based on the given `Options` configuration.
///
/// This function generates a `Characters` map by replacing Morse code symbols (dots and dashes) with custom symbols
/// specified in the `options` configuration.
///
/// # Parameters
/// - `options`: A configuration object containing custom symbols for dots and dashes.
///
/// # Returns
/// A `Characters` map where each key is a `MorseCharacterSet` and each value is a `BTreeMap` of characters and their updated Morse code representations.
fn get_mapped_characters(options: Options) -> Characters {
    let mut mapped = BTreeMap::new();
    let characters = get_characters(options);

    for (set, chars) in &characters {
        let mut new_set = BTreeMap::new();
        for (key, value) in chars {
            let mapped_value = value
                .replace('0', &options.dot.to_string())
                .replace('1', &options.dash.to_string());
            new_set.insert(*key, mapped_value);
        }
        mapped.insert(*set, new_set);
    }

    mapped
}

/// Returns a `BTreeMap` of Morse code representations swapped with their character mappings.
///
/// This function generates a mapping where Morse code representations are keys and the corresponding characters are values.
/// This is useful for reverse lookup of Morse code representations.
///
/// # Parameters
/// - `options`: A configuration object containing custom symbols for dots and dashes.
///
/// # Returns
/// A `BTreeMap` where each key is a Morse code representation and each value is the corresponding character.
fn swap_characters(options: Options) -> BTreeMap<String, char> {
    let mut swapped = BTreeMap::new();
    let mapped_characters = get_mapped_characters(options);

    for chars in mapped_characters.into_values() {
        for (key, value) in chars {
            swapped.entry(value).or_insert(key);
        }
    }

    swapped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_english_alphabet() {
        assert_eq!(MorseCode::default().encode("the quick brown fox jumps over the lazy dog"), "- .... . / --.- ..- .. -.-. -.- / -... .-. --- .-- -. / ..-. --- -..- / .--- ..- -- .--. ... / --- ...- . .-. / - .... . / .-.. .- --.. -.-- / -.. --- --.");
        assert_eq!(MorseCode::new(Options { dash: '–', dot: '•', space: '\\', ..Default::default() }).encode("the quick brown fox jumps over the lazy dog"), "– •••• • \\ ––•– ••– •• –•–• –•– \\ –••• •–• ––– •–– –• \\ ••–• ––– –••– \\ •––– ••– –– •––• ••• \\ ––– •••– • •–• \\ – •••• • \\ •–•• •– ––•• –•–– \\ –•• ––– ––•");
    }

    #[test]
    fn decodes_english_alphabet() {
        assert_eq!(MorseCode::default().decode("- .... . / --.- ..- .. -.-. -.- / -... .-. --- .-- -. / ..-. --- -..- / .--- ..- -- .--. ... / --- ...- . .-. / - .... . / .-.. .- --.. -.-- / -.. --- --."), "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG");
        assert_eq!(MorseCode::new(Options {dash: '–', dot: '•', space: '\\',..Default::default()}).decode("– •••• • \\ ––•– ••– •• –•–• –•– \\ –••• •–• ––– •–– –• \\ ••–• ––– –••– \\ •––– ••– –– •––• ••• \\ ––– •••– • •–• \\ – •••• • \\ •–•• •– ––•• –•–– \\ –•• ––– ––•"), "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG");
    }

    #[test]
    fn decodes_numbers() {
        assert_eq!(
            MorseCode::default()
                .decode("----- .---- ..--- ...-- ....- ..... -.... --... ---.. ----."),
            "0123456789"
        );
    }

    #[test]
    fn encodes_punctuation() {
        let morse_code = MorseCode::default();
        assert_eq!(
            morse_code.encode(".,?'!/("),
            ".-.-.- --..-- ..--.. .----. -.-.-- -..-. -.--."
        );
        assert_eq!(
            morse_code.encode(")&:;=¿¡"),
            "-.--.- .-... ---... -.-.-. -...- ..-.- --...-"
        );
    }

    #[test]
    fn decodes_punctuation() {
        let morse_code = MorseCode::default();
        assert_eq!(
            morse_code.decode(".-.-.- --..-- ..--.. .----. -.-.-- -..-. -.--."),
            ".,?'!/("
        );
        assert_eq!(
            morse_code.decode("-.--.- .-... ---... -.-.-. -...- ..-.- --...-"),
            ")&:;=¿¡"
        );
    }

    #[test]
    fn encodes_non_english_alphabet() {
        let morse_code = MorseCode::default();
        assert_eq!(
            morse_code.encode("ÃÁÅÀÂÄ"),
            ".--.- .--.- .--.- .--.- .--.- .-.-"
        );
        assert_eq!(
            morse_code.encode("ĄÆÇĆĈČ"),
            ".-.- .-.- -.-.. -.-.. -.-.. --."
        );
        assert_eq!(
            morse_code.encode("ĘÐÈËĘÉ"),
            "..-.. ..--. .-..- ..-.. ..-.. ..-.."
        );
        assert_eq!(
            morse_code.encode("ÊĞĜĤİÏ"),
            "-..-. --.-. --.-. ---- .-..- -..--"
        );
        assert_eq!(
            morse_code.encode("ÌĴŁŃÑÓ"),
            ".---. .---. .-..- --.-- --.-- ---."
        );
        assert_eq!(
            morse_code.encode("ÒÖÔØŚŞ"),
            "---. ---. ---. ---. ...-... .--.."
        );
        assert_eq!(
            morse_code.encode("ȘŠŜßÞÜ"),
            "---- ---- ...-. ... ... .--.. ..--"
        );
        assert_eq!(morse_code.encode("ÙŬŽŹŻ"), "..-- ..-- --..- --..-. --..-");
    }
}
