use super::*;

#[test]
fn test() {
    let eng = "Hello, World!";
    println!("eng: {}", eng);

    let eng_encoded = std::str::from_utf8(&encode(eng)).unwrap().to_string();
    println!("encode: {}", eng_encoded);

    let eng_decoded = std::str::from_utf8(&decode(&eng_encoded))
        .unwrap()
        .to_string();
    println!("decode: {}", eng_decoded);

    let kor = "안녕, 세상!";
    println!("kor: {}", kor);
    let kor_encoded = std::str::from_utf8(&encode(kor)).unwrap().to_string();
    println!("encode: {}", kor_encoded);
    let kor_decoded = std::str::from_utf8(&decode(&kor_encoded))
        .unwrap()
        .to_string();
    println!("decode: {:?}", kor_decoded);
}
