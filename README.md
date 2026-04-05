# Brainfuck Generator

## Purpose

JUST FOR FUN

## Usage

1. Set config by edit `config.toml`
2. Enter the string into file
2. Run with following commands

**encode**

```bash
cargo run -- config.toml encode decoded.txt
```

**decode**

```bash
cargo run -- config.toml decode encoded.txt
```

## Caution

You can set config freely but you have to consider if the strings are simillar or repeats, it can cause error or runtime bug.
