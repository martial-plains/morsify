# Morsify

Morsify is a Rust crate designed for encoding and decoding Morse code. It provides a flexible and efficient way to convert text to Morse code and vice versa, with customizable options for different Morse code representations and character sets.

## Features

- **Encoding**: Convert plain text into Morse code with customizable symbols for dots, dashes, spaces, and separators.
- **Decoding**: Convert Morse code back into readable text using the provided configuration.
- **Customizable Character Sets**: Support for various character sets including Latin, Greek, Cyrillic, Arabic, and more, with customizable order during encoding and decoding.
- **Configurable Options**: Define how Morse code should be represented with options for symbols, handling invalid characters, and defining the order of character sets.

## Changes in v0.2.0

- **Customizable Character Set Order**: The `Options` struct now includes a `character_set_order` field, allowing you to define the order in which character sets are used during encoding and decoding.
- **Flexible Character Set Processing**: The `get_characters` function has been refactored to support a custom order and to allow set-specific customizations (e.g., adding a separator to the Latin set).

## Usage

To use Morsify, add it to your `Cargo.toml`:

```toml
[dependencies]
morsify = "0.2.0"
