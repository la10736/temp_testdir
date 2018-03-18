use std::path::PathBuf;
use std::path::Path;
use std::ffi::OsStr;
use std::ops::Deref;

pub struct TempDir {
    path: PathBuf,
    destroy: bool,
}

static RSTEST_TEMP_DIR_ROOT: &'static str = "rstest";

impl TempDir {
    pub fn permanent(mut self) -> Self {
        self.destroy = false;
        self
    }

    fn new(mut path: PathBuf, destroy: bool) -> Self {
        Self::create_root(&path);
        while std::fs::create_dir(&path).is_err() {
            let val = {
                path.extension().unwrap_or(OsStr::new(""))
                    .to_str()
                    .and_then(|v| v.parse::<i32>().ok())
                    .unwrap_or(0) + 1
            };

            path.set_extension(val.to_string());
        }
        TempDir { path, destroy }
    }

    fn create_root(path: &Path) {
        if let Some(_parent) = path.parent() {
            std::fs::create_dir_all(&path).expect("Should create the parent dir");
        }
    }
}

fn rm(path: &Path) {
    let _ = std::fs::remove_dir_all(path);
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if self.destroy {
            rm(&self.path);
        }
    }
}

impl Default for TempDir {
    fn default() -> Self {
        let path = PathBuf::from(format!("/tmp/{}", RSTEST_TEMP_DIR_ROOT));
        Self::new(path, true)
    }
}

impl Deref for TempDir {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl AsRef<Path> for TempDir {
    fn as_ref(&self) -> &Path {
        self.path.as_path()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn default_tempdir_should_create_a_directory() {
        let temp = TempDir::default();

        assert!(temp.as_ref().is_dir())
    }

    #[test]
    fn default_tempdir_should_destroy_directory_after_go_out_of_scope() {
        let path;
        {
            let temp = TempDir::default();
            path = temp.as_ref().to_owned()
        }

        assert!(!path.exists())
    }

    #[test]
    fn tempdir_permanent_should_do_not_remove_dir() {
        let path;
        {
            let temp = TempDir::default().permanent();
            path = temp.as_ref().to_owned()
        }

        assert!(path.is_dir());
        rm(&path)
    }

    #[test]
    fn two_temp_dir_should_have_different_path() {
        let t1 = TempDir::default();
        let t2 = TempDir::default();

        assert_ne!(t1.as_ref(), t2.as_ref());
    }

    #[test]
    fn default_temp_should_destroy_also_content() {
        let path;
        {
            let temp = TempDir::default();
            path = temp.as_ref().to_owned();
            File::create(temp.join("somefile")).expect("Should create dir");
        }
        assert!(!path.exists());
        rm(&path);
    }
}
