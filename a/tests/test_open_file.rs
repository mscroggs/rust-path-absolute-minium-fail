use std::fs;

#[test]
fn test_open_file() {
    let contents = fs::read_to_string("tests/file.txt").unwrap();
    println!("{contents}");
}
