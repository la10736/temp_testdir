extern crate temp_testdir;

use temp_testdir::TempDir;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::io::Read;

#[test]
fn should_delete_temp_dir() {
    let mut file_path;
    {
        let temp = TempDir::default();
        file_path = PathBuf::from(temp.as_ref());
        file_path.push("hello.txt");

        let mut f = File::create(file_path.clone()).unwrap();

        f.write_all("Hello World!".as_bytes()).unwrap();
    }
    // Should be deleted
    assert!(!file_path.is_file());
    assert!(!file_path.parent().unwrap().is_dir());
}

#[test]
fn should_not_delete_temp_dir() {
    let mut file_path;
    {
        let temp = TempDir::default().permanent();

        file_path = PathBuf::from(temp.as_ref());
        file_path.push("hello.txt");

        let mut f = File::create(file_path.clone()).unwrap();

        f.write_all("Hello World!".as_bytes()).unwrap();
    }
    let mut content = String::new();
    File::open(file_path).unwrap().read_to_string(&mut content).unwrap();
    assert_eq!("Hello World!", &content);
}
