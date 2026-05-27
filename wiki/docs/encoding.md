# Encoding Algorithm

## Overview

The encoder converts each byte into brainfuck code. Instead of simply repeating `+` N times, it uses nested loops for multiplication to optimize code length.

## Incremental Encoding

Only the **difference** between consecutive bytes is encoded.

For example, encoding `Hi` (72, 105):

1. First byte: 0 → 72, difference = 72
2. Second byte: 72 → 105, difference = 33

The second byte only needs to increment by 33 instead of 105, resulting in shorter code.

If the difference is negative (value decreasing), `-` is used instead of `+`.

## Depth Calculation

The optimal loop depth is calculated for a difference value `c`.

```
depth = 1             (when c < 8)
depth = c^(1/8) + 1   (when c >= 8)
```

- `depth = 1`: No loop, directly list `+`/`-`
- `depth = 2`: Single loop (e.g., `+++++[>++++<-]>++.`)
- `depth = 3`: Double nested loop

## Generated Code Structure (depth=2 example)

The difference value `c` is decomposed into `a * b + r` (where a approximates the square root of `c`).

```
+++...+++ [> +++...+++ <-]> ++...++ .
|-- a ---|  |--- b ---|    |-- r --|
```

1. Set `a` in an auxiliary cell
2. Loop: add `b` to the main cell, decrement auxiliary cell by 1
3. After loop, adjust remainder `r`
4. Output with `.`

This creates a value of size `a * b + r` using only `a + b + r` commands.
