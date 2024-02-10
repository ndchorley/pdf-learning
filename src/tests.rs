use std::{fs::File, io::Read};

#[test]
fn can_read_a_file() {
    let mut buffer = [0; 100_000];

    let mut file = File::open("sample.pdf").unwrap();

    let bytes_read = file.read(&mut buffer).unwrap();

    assert_ne!(bytes_read, 0);
}
