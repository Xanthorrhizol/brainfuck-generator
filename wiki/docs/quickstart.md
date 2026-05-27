# Quick Start

## Installation

Add the dependency to your `Cargo.toml`.

```toml
[dependencies]
brainfuck-generator = "0.1.0"
```

## CLI Usage

### Encoding

Convert a text file into brainfuck code.

```bash
cargo run -- config.toml encode input.txt
```

### Decoding

Execute brainfuck code to restore the original text.

```bash
cargo run -- config.toml decode encoded.txt
```

## Library Usage

```rust
use brainfuck_generator::{encode, decode, swap_chars, Config};

// Encode
let text = b"Hello, World!";
let brainfuck_code = encode(text);
let code_str = std::str::from_utf8(&brainfuck_code).unwrap();

// Decode
let decoded = decode(code_str);
assert_eq!(decoded, text);
```

## Creating a Variant with Config

```rust
// Load config.toml
let config: Config = toml::from_str(
    &std::fs::read_to_string("config.toml").unwrap()
).unwrap();

// Standard brainfuck -> custom symbols
let mut code = std::str::from_utf8(&encode(b"Hi"))
    .unwrap().to_string();
swap_chars(&mut code, &config);

// Custom symbols -> standard brainfuck
unswap_chars(&mut code, &config);
let result = decode(&code);
```
