# Introduction

Brainfuck Generator is a Rust library that encodes text into [brainfuck](https://en.wikipedia.org/wiki/Brainfuck) code and decodes brainfuck code back into text.

## Features

- **Encoding**: Convert arbitrary byte arrays into brainfuck code
- **Decoding**: Execute brainfuck code to restore the original bytes
- **Config-based symbol substitution**: Replace the 8 brainfuck commands (`+`, `-`, `>`, `<`, `[`, `]`, `.`, `,`) with custom strings to create your own esoteric language variant

## What is brainfuck?

brainfuck is an esoteric programming language that operates with only 8 commands.

| Command | Action |
|---------|--------|
| `>` | Move pointer right |
| `<` | Move pointer left |
| `+` | Increment current cell |
| `-` | Decrement current cell |
| `[` | Jump past matching `]` if current cell is 0 |
| `]` | Jump back to matching `[` if current cell is not 0 |
| `.` | Output current cell as a byte |
| `,` | Read one byte from stdin into current cell |
