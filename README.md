# Morsify

Morsify is a Rust crate designed for encoding and decoding Morse code. It provides a flexible and efficient way to convert text to Morse code and vice versa, with customizable options for different Morse code representations and character sets.

## Features

- **Encoding**: Convert plain text into Morse code with customizable symbols for dots, dashes, spaces, and separators.
- **Decoding**: Convert Morse code back into readable text using the provided configuration.
- **Customizable Character Sets**: Support for various character sets including Latin, Greek, Cyrillic, Arabic, and more.
- **Configurable Options**: Define how Morse code should be represented with options for symbols and handling invalid characters.

## Usage

To use Morsify, add it to your `Cargo.toml`:

```toml
[dependencies]
morsify = "0.1.0"
```

Then, use the `MorseCode` struct to encode and decode text. Here’s a basic example:

```rust
use morsify::{MorseCode, Options, MorseCharacterSet};

// Create a new `MorseCode` instance with default options
let options = Options {
    dash: '-',
    dot: '.',
    space: '/',
    separator: ' ',
    invalid_char_callback: |c| c,
    priority: MorseCharacterSet::Latin,
};
let morse_code = MorseCode::new(options);

// Encode a text message to Morse code
let encoded = morse_code.encode("Hello World");
println!("Encoded: {}", encoded);

// Decode a Morse code message to text
let decoded = morse_code.decode(".... . .-.. .-.. --- / .-- --- .-. .-.. -..");
println!("Decoded: {}", decoded);
```

## API Documentation

For detailed information about the API, refer to the module documentation and individual methods of the `MorseCode` struct.

## License

Morsify is licensed under the MIT License. See the `LICENSE` file for more details.

## Character Sets Supported

- **Latin Alphabet**
- **Numerical Digits**
- **Punctuation Marks**
- **Extended Latin Characters**
- **Cyrillic Alphabet**
- **Greek Alphabet**
- **Hebrew Alphabet**
- **Arabic Alphabet**
- **Persian Alphabet**
- **Japanese Characters**
- **Korean Characters**
- **Thai Characters**

For more information about each character set, you can refer to the following links:

- [Latin Alphabet](https://en.wikipedia.org/wiki/Morse_code)
- [Extended Latin Characters](https://ham.stackexchange.com/questions/1379/international-characters-in-morse-code)
- [Cyrillic Alphabet](https://en.wikipedia.org/wiki/Russian_Morse_code)
- [Greek Alphabet](https://en.wikipedia.org/wiki/Morse_code_for_non-Latin_alphabets)
- [Hebrew Alphabet](https://en.wikipedia.org/wiki/Morse_code_for_non-Latin_alphabets)
- [Arabic Alphabet](https://en.wikipedia.org/wiki/Morse_code_for_non-Latin_alphabets)
- [Persian Alphabet](https://en.wikipedia.org/wiki/Morse_code_for_non-Latin_alphabets)
- [Japanese Characters](https://ja.wikipedia.org/wiki/%E3%83%A2%E3%83%BC%E3%83%AB%E3%82%B9%E7%AC%A6%E5%8F%B7#%E5%92%8C%E6%96%87%E3%83%A2%E3%83%BC%E3%83%AB%E3%82%B9%E7%AC%A6%E5%8F%B7)
- [Korean Characters](https://en.wikipedia.org/wiki/SKATS)
- [Thai Characters](https://th.wikipedia.org/wiki/รหัสมอร์ส)

