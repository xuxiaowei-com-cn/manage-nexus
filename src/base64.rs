/**
 * string 转 base64
 * <p>
 * 来自：https://magiclen.org/base64-stream/
 */
pub fn string_to_base64(data: String) -> String {
    use std::io::{Cursor, Read};

    use base64_stream::ToBase64Reader;

    let data_vec = data.as_bytes().to_vec();

    let mut reader = ToBase64Reader::new(Cursor::new(data_vec));

    let mut result = String::new();

    reader.read_to_string(&mut result).unwrap();

    return result;
}

/**
 * str 转 base64
 * <p>
 * 来自：https://magiclen.org/base64-stream/
 */
pub fn str_to_base64(data: &str) -> String {
    use std::io::{Cursor, Read};

    use base64_stream::ToBase64Reader;

    let data_vec = data.as_bytes().to_vec();

    let mut reader = ToBase64Reader::new(Cursor::new(data_vec));

    let mut result = String::new();

    reader.read_to_string(&mut result).unwrap();

    return result;
}


#[test]
fn string_to_base64_test() {
    let tmp = "Hi there, this is a simple sentence used for testing this crate. I hope all cases are correct.";
    let data = tmp.to_string();
    let result = string_to_base64(data);
    assert_eq!("SGkgdGhlcmUsIHRoaXMgaXMgYSBzaW1wbGUgc2VudGVuY2UgdXNlZCBmb3IgdGVzdGluZyB0aGlzIGNyYXRlLiBJIGhvcGUgYWxsIGNhc2VzIGFyZSBjb3JyZWN0Lg==", result);
}

#[test]
fn str_to_base64_test() {
    let data = "Hi there, this is a simple sentence used for testing this crate. I hope all cases are correct.";
    let result = str_to_base64(data);
    assert_eq!("SGkgdGhlcmUsIHRoaXMgaXMgYSBzaW1wbGUgc2VudGVuY2UgdXNlZCBmb3IgdGVzdGluZyB0aGlzIGNyYXRlLiBJIGhvcGUgYWxsIGNhc2VzIGFyZSBjb3JyZWN0Lg==", result);
}
