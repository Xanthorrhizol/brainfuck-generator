#[cfg(test)]
mod test;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    right: String,
    left: String,
    plus: String,
    minus: String,
    loop_start: String,
    loop_end: String,
    print: String,
}

fn encode_char(c: u8, is_minus: bool, is_referenced: bool) -> Vec<u8> {
    let mut result = Vec::new();
    if is_referenced {
        result.push(b'<');
    }
    if c > 10 {
        let a = (c as f32).sqrt() as u8;
        let b = c / a;
        let r = c % a;

        for _ in 0..a {
            result.push(b'+');
        }
        result.push(b'[');
        result.push(b'>');

        for _ in 0..b {
            if is_minus {
                result.push(b'-');
            } else {
                result.push(b'+');
            }
        }
        result.push(b'<');
        result.push(b'-');
        result.push(b']');
        result.push(b'>');

        for _ in 0..r {
            if is_minus {
                result.push(b'-');
            } else {
                result.push(b'+');
            }
        }
    } else {
        result.push(b'>');
        for _ in 0..c {
            if is_minus {
                result.push(b'-');
            } else {
                result.push(b'+');
            }
        }
    }
    result.push(b'.');

    result
}

pub fn encode(s: &str) -> Vec<u8> {
    let mut result = Vec::new();
    let mut tmp = 0u8;
    let mut is_first = true;
    for c in s.as_bytes() {
        result.extend(&encode_char(
            if *c > tmp { *c - tmp } else { tmp - *c },
            *c < tmp,
            !is_first,
        ));
        tmp = *c;
        if is_first {
            is_first = false;
        }
    }
    result
}

pub fn decode(s: &str) -> Vec<u8> {
    let mut result = Vec::new();
    let mut tmp = Vec::new();
    let mut idx = 0;
    tmp.push(0);
    let mut loop_stack = Vec::new();
    let mut loop_idx = 0;
    while loop_idx < s.len() {
        let c = s.as_bytes()[loop_idx];
        match c {
            b'>' => {
                idx += 1;
                if idx >= tmp.len() {
                    tmp.push(0);
                }
            }
            b'<' => idx -= 1,
            b'+' => tmp[idx] += 1,
            b'-' => tmp[idx] -= 1,
            b'[' => {
                loop_stack.push(loop_idx);
                if tmp[idx] == 0 {
                    loop_idx = loop_stack.pop().unwrap();
                }
            }
            b']' => {
                if tmp[idx] != 0 {
                    loop_idx = *loop_stack.last().unwrap();
                }
            }
            b'.' => {
                result.push(tmp[idx]);
            }
            _ => {}
        }
        loop_idx += 1;
    }
    result
}

fn validate_config(config: &Config) -> Result<(), String> {
    let Config {
        plus,
        minus,
        right,
        left,
        loop_start,
        loop_end,
        print,
    } = config;
    if plus == minus
        || plus == right
        || plus == left
        || plus == loop_start
        || plus == loop_end
        || minus == right
        || minus == left
        || minus == loop_start
        || minus == loop_end
        || right == left
        || right == loop_start
        || right == loop_end
        || left == loop_start
        || left == loop_end
        || print == plus
        || print == minus
        || print == right
        || print == left
        || print == loop_start
        || print == loop_end
    {
        return Err("invalid config".to_string());
    }
    Ok(())
}

pub fn swap_chars(s: &mut String, config: &Config) {
    validate_config(&config).expect("invalid config");
    let Config {
        plus,
        minus,
        right,
        left,
        loop_start,
        loop_end,
        print,
    } = config;
    *s = s.replace("+", plus);
    *s = s.replace("-", minus);
    *s = s.replace(">", right);
    *s = s.replace("<", left);
    *s = s.replace("[", loop_start);
    *s = s.replace("]", loop_end);
    *s = s.replace(".", print);
}

pub fn unswap_chars(s: &mut String, config: &Config) {
    validate_config(&config).expect("invalid config");
    let Config {
        plus,
        minus,
        right,
        left,
        loop_start,
        loop_end,
        print,
    } = config;
    let mut v = Vec::new();
    v.push((plus, "+"));
    v.push((minus, "-"));
    v.push((right, ">"));
    v.push((left, "<"));
    v.push((loop_start, "["));
    v.push((loop_end, "]"));
    v.push((print, "."));

    v.sort_by_key(|(x, _)| x.len());
    v.reverse();
    for (x, y) in v {
        *s = s.replace(x, y);
    }
}
