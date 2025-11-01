# First Word Extractor

A Rust command-line utility that extracts the first and second words from a given string input.

## Features

- Extracts the first word from a string
- Extracts the second word from a string
- Handles edge cases like empty strings and single-word inputs
- Memory efficient using string slices

## Algorithms

### First Word Extraction

The `first_word` function extracts the first word from a string using the following approach:

1. Converts the input string to bytes for efficient iteration
2. Iterates through each byte in the string
3. Returns the substring from the start up to (but not including) the first space character
4. If no space is found, returns the entire string as it's considered a single word

### Second Word Extraction

The `second_word` function extracts the second word using this approach:

1. Tracks word boundaries using start and end indices
2. Iterates through the string to find the second word
3. Identifies the second word as the sequence of characters between the first and second spaces
4. Handles edge cases where the input might have fewer than two words

## Prerequisites

- [mise](https://mise.jdx.dev/) - A version manager for multiple languages and tools

## Setup

1. Clone the repository
   ```bash
   git clone https://github.com/Brisinger/first_word.git
   cd first_word
   ```

2. Install Rust and tools using mise:
   ```bash
   mise install
   ```

   This will install:
   - Rust 1.85.0
   - rustfmt (for code formatting)
   - clippy (for linting)

## Development

### Building

To build the project with optimizations:
```bash
mise run build
```

### Running

To run the optimized release build:
```bash
cargo run --release
```

### Development Tasks

- Format code: `mise run format`
- Check for errors: `mise run check`
- Run linter: `mise run lint`
- Run all CI checks: `mise run ci`

## Usage

1. Run the program: `cargo run --release`
2. Enter a sentence when prompted
3. The program will display the first and second words

## Example

```bash
$ cargo run
   Compiling first_word v0.1.0 (/path/to/first_word)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/first_word`
Please enter a sentence: Hello World from Rust
First word: Hello
Second word: World
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

**Author**: [Shubhojit Dasgupta](https://github.com/Brisinger)
