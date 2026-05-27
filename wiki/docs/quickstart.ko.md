# 빠른 시작

## 설치

`Cargo.toml`에 의존성을 추가합니다.

```toml
[dependencies]
brainfuck-generator = "0.1.0"
```

## CLI 사용법

### 인코딩

텍스트 파일을 brainfuck 코드로 변환합니다.

```bash
cargo run -- config.toml encode input.txt
```

### 디코딩

brainfuck 코드를 실행하여 원본 텍스트를 복원합니다.

```bash
cargo run -- config.toml decode encoded.txt
```

## 라이브러리 사용법

```rust
use brainfuck_generator::{encode, decode, swap_chars, Config};

// 인코딩
let text = b"Hello, World!";
let brainfuck_code = encode(text);
let code_str = std::str::from_utf8(&brainfuck_code).unwrap();

// 디코딩
let decoded = decode(code_str);
assert_eq!(decoded, text);
```

## Config로 아류작 만들기

```rust
// config.toml 로드
let config: Config = toml::from_str(
    &std::fs::read_to_string("config.toml").unwrap()
).unwrap();

// 표준 brainfuck -> 커스텀 심볼로 치환
let mut code = std::str::from_utf8(&encode(b"Hi"))
    .unwrap().to_string();
swap_chars(&mut code, &config);

// 커스텀 심볼 -> 표준 brainfuck으로 복원
unswap_chars(&mut code, &config);
let result = decode(&code);
```
