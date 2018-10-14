# Working with temporary directories in Rust test

A zero dependencies crate to deal with temporary directories in tests.
To use it add

```
[dev-dependencies]
temp_testdir = "0.2"
```

## How to use

```
#[test]
fn should_delete_temp_dir() {
    let temp = TempDir::default();
    // You can use `temp` as a `Path`

    let file_path = PathBuf::from(temp.as_ref());
    file_path.push("hello.txt");

    let mut f = File::create(file_path.clone()).unwrap();

    f.write_all("Hello World!".as_bytes());

    my_app.process(&file_path);

    // Temp dir will be deleted at the end of the test
}
```

If you need to not delete the dir when test is done you can use

```
let temp = TempDir::default().permanent();
```

## Where the dirs are

All dirs will be in your system standard temp dir follow by
`rstest.<nr>` where `nr` is the lowest integer that can be
used to crate it.

You can change this behaviour by two envirorment variables:

- `RSTEST_TEMP_DIR_ROOT`: root of all temp dir (default system temp dir)
- `RSTEST_TEMP_DIR_ROOT_NAME`: prefix dir name (default system `rstest`)

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

