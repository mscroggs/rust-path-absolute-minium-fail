use std::{fs, path};

#[test]
fn test_open_file() {
    let file = path::PathBuf::from(file!());
    let dir = path::absolute(file.parent().unwrap()).unwrap();
    let file_path = format!("{}/file.txt", dir.display());
    println!("{}", file.display());
    println!("{}", dir.display());
    println!("{}", file_path);
    let contents = fs::read_to_string(file_path).unwrap();
    println!("{contents}");
}
