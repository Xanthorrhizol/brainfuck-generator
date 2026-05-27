# Config

Config maps the 8 brainfuck commands to custom strings. Written in TOML format.

## Default Config

Uses standard brainfuck symbols as-is.

```toml
right = ">"
left = "<"
plus = "+"
minus = "-"
loop_start = "["
loop_end = "]"
print = "."
input = ","
```

## Fields

| Field | brainfuck Command | Description |
|-------|-------------------|-------------|
| `right` | `>` | Move pointer right |
| `left` | `<` | Move pointer left |
| `plus` | `+` | Increment cell |
| `minus` | `-` | Decrement cell |
| `loop_start` | `[` | Loop start |
| `loop_end` | `]` | Loop end |
| `print` | `.` | Output |
| `input` | `,` | Input |

## Rules

- All field values must be unique.
- Avoid making one field's value a substring of another. Substring relationships can cause unintended substitutions during `swap_chars`/`unswap_chars`.

## Example: Emoji

```toml
right = "🐸"
left = "🦀"
plus = "🔥"
minus = "💀"
loop_start = "🎠"
loop_end = "🎡"
print = "✨"
input = "📥"
```

## Example: Syusuk

```toml
right = "슈숙."
left = "슈슉."
plus = "슉."
minus = "시."
loop_start = "시발럼아."
loop_end = "시발롬아."
print = "슈슉 슈숙."
input = "슈숙 슈슉."
```
