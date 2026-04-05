use super::*;

#[test]
fn test() {
    let eng = "Hello, World!";
    let kor = "안녕, 세상!";

    let eng_encoded = std::str::from_utf8(&encode(eng)).unwrap().to_string();
    let kor_encoded = std::str::from_utf8(&encode(kor)).unwrap().to_string();

    println!("eng: {}", eng);
    println!("encode: {}", eng_encoded);
    println!("decode: {}", decode(&eng_encoded));
    println!("kor: {}", kor);
    println!("encode: {}", kor_encoded);
    println!("decode: {}", decode(&kor_encoded));
}
