#[cfg(test)]
mod test;

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

pub fn decode(s: &str) -> String {
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
            _ => unreachable!(),
        }
        loop_idx += 1;
    }
    std::str::from_utf8(result.as_slice())
        .expect("invalid utf-8")
        .to_string()
}
