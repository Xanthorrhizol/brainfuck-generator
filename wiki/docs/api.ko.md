# API

## `encode(b: &[u8]) -> Vec<u8>`

바이트 배열을 brainfuck 코드로 인코딩합니다.

```rust
let code = encode(b"Hello");
let code_str = std::str::from_utf8(&code).unwrap();
```

- 입력: 임의의 바이트 배열 (UTF-8 텍스트, 바이너리 등)
- 출력: brainfuck 코드를 담은 바이트 벡터 (ASCII 범위)
- 이전 바이트와의 차이를 이용해 증분 인코딩하므로, 비슷한 값이 연속될수록 짧은 코드가 생성됩니다.

## `decode(s: &str) -> Vec<u8>`

brainfuck 코드를 실행하여 결과 바이트를 반환합니다.

```rust
let result = decode("+++++++++[>++++++++<-]>+.");
// result == [73] ('I')
```

- 입력: brainfuck 코드 문자열
- 출력: `.` 명령으로 출력된 바이트들의 벡터
- `,` 명령을 만나면 stdin에서 1바이트를 읽어 현재 셀에 저장합니다.

## `swap_chars(s: &mut String, config: &Config)`

표준 brainfuck 심볼을 Config에 정의된 커스텀 문자열로 치환합니다.

```rust
let config: Config = toml::from_str(toml_str).unwrap();
let mut code = "+++++.".to_string();
swap_chars(&mut code, &config);
```

- `encode`로 생성한 표준 brainfuck 코드에 적용합니다.
- Config 검증 실패 시 panic합니다.

## `unswap_chars(s: &mut String, config: &Config)`

Config에 정의된 커스텀 문자열을 표준 brainfuck 심볼로 복원합니다.

```rust
let config: Config = toml::from_str(toml_str).unwrap();
let mut code = custom_code.to_string();
unswap_chars(&mut code, &config);
let result = decode(&code);
```

- `decode` 전에 호출하여 커스텀 심볼을 표준으로 되돌립니다.
- 긴 토큰부터 먼저 치환하여 부분 문자열 충돌을 최소화합니다.

## `Config`

```rust
#[derive(Debug, Deserialize)]
pub struct Config {
    right: String,      // >
    left: String,       // <
    plus: String,       // +
    minus: String,      // -
    loop_start: String, // [
    loop_end: String,   // ]
    print: String,      // .
    input: String,      // ,
}
```

TOML 파일에서 `toml::from_str`로 역직렬화하여 사용합니다.
