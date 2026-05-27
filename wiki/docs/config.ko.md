# Config

Config는 brainfuck의 8개 명령어를 원하는 문자열로 매핑합니다. TOML 형식으로 작성합니다.

## 기본 Config

표준 brainfuck 그대로 사용하는 설정입니다.

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

## 필드

| 필드 | brainfuck 명령어 | 설명 |
|------|------------------|------|
| `right` | `>` | 포인터 오른쪽 이동 |
| `left` | `<` | 포인터 왼쪽 이동 |
| `plus` | `+` | 셀 값 증가 |
| `minus` | `-` | 셀 값 감소 |
| `loop_start` | `[` | 루프 시작 |
| `loop_end` | `]` | 루프 끝 |
| `print` | `.` | 출력 |
| `input` | `,` | 입력 |

## 규칙

- 모든 필드의 값은 서로 달라야 합니다.
- 한 필드의 값이 다른 필드의 부분 문자열이 되지 않도록 주의하세요. 부분 문자열 관계가 있으면 `swap_chars`/`unswap_chars` 시 의도치 않은 치환이 발생할 수 있습니다.

## 예시: 이모지

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

## 예시: 슈숙어

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
