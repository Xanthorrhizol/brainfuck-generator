use super::*;

#[test]
fn test() {
    println!("########## TEST 1 ##########");

    let eng = "Hello, World!";
    println!("eng: {}", eng);

    let eng_encoded = std::str::from_utf8(&encode(eng.as_bytes()))
        .unwrap()
        .to_string();
    println!("encode: {}", eng_encoded);

    let eng_decoded = std::str::from_utf8(&decode(&eng_encoded))
        .unwrap()
        .to_string();
    println!("decode: {}", eng_decoded);

    println!("########## TEST 2 ##########");

    let kor = "안녕, 세상!";
    println!("kor: {}", kor);
    let kor_encoded = std::str::from_utf8(&encode(kor.as_bytes()))
        .unwrap()
        .to_string();
    println!("encode: {}", kor_encoded);
    let kor_decoded = std::str::from_utf8(&decode(&kor_encoded))
        .unwrap()
        .to_string();
    println!("decode: {:?}", kor_decoded);

    println!("########## TEST 3 ##########");

    let bytes = b"\x01\x08\x40\x60";
    println!("bytes: {:X?}", bytes);
    let bytes_encoded = std::str::from_utf8(&encode(bytes)).unwrap().to_string();
    println!("encode: {}", bytes_encoded);
    let bytes_decoded = decode(&bytes_encoded);
    println!("decode: {:X?}", bytes_decoded);
}
