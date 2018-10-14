use std::path::PathBuf;
use std::path::Path;
use std::ffi::OsStr;
use std::ops::Deref;
use std::ffi::OsString;

/// Create a dir that will be removed.
pub struct TempDir {
    path: PathBuf,
    destroy: bool,
}

impl Default for TempDir {
    fn default() -> Self {
        let mut path = root();
        path.push(root_name());
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

pub static RSTEST_TEMP_DIR_ROOT_DEFAULT: &'static str = "rstest";

pub static ENV_RSTEST_TEMP_DIR_ROOT_NAME: &'static str = "RSTEST_TEMP_DIR_ROOT_NAME";
pub static ENV_RSTEST_TEMP_DIR_ROOT: &'static str = "RSTEST_TEMP_DIR_ROOT";

impl TempDir {
    /// Prevent dir delete
    pub fn permanent(mut self) -> Self {
        self.destroy = false;
        self
    }

    /// New Temp dir.
    pub fn new<P: AsRef<Path>>(path: P, destroy: bool) -> Self {
        let mut path = PathBuf::from(path.as_ref());
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

fn rm<P: AsRef<Path>>(path: P) {
    let _ = std::fs::remove_dir_all(path.as_ref());
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if self.destroy {
            rm(&self.path);
        }
    }
}

fn root_name() -> OsString {
    std::env::var_os(ENV_RSTEST_TEMP_DIR_ROOT_NAME)
        .unwrap_or(OsString::from(RSTEST_TEMP_DIR_ROOT_DEFAULT))
}

fn root() -> PathBuf {
    std::env::var_os(ENV_RSTEST_TEMP_DIR_ROOT)
        .map(|p| PathBuf::from(p))
        .unwrap_or(std::env::temp_dir())
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
    }

    #[derive(Default)]
    struct EnvState(std::collections::HashSet<String>);

    impl EnvState {
        pub fn add(&mut self, key: &str, value: &str) {
            std::env::set_var(key, value);
            self.0.insert(String::from(key));
        }
    }

    impl Drop for EnvState {
        fn drop(&mut self) {
            for k in &self.0 {
                std::env::remove_var(k);
            }
        }
    }

    #[test]
    fn should_resolve_rstest_temp_dir_root_name_in_env() {
        let mut env = EnvState::default();
        let new_root = "other_rstest_root";
        env.add(ENV_RSTEST_TEMP_DIR_ROOT_NAME, new_root);

        let last = OsString::from(TempDir::default().components().last().unwrap().as_os_str());

        assert!(last.into_string().unwrap().starts_with(new_root));
    }

    #[test]
    fn should_resolve_root_in_env() {
        let mut env = EnvState::default();
        let new_root = "this_test_root";
        env.add(ENV_RSTEST_TEMP_DIR_ROOT, new_root);

        let first = OsString::from(TempDir::default().components().nth(0).unwrap().as_os_str());

        assert_eq!(first.into_string().unwrap(), new_root);
        rm(new_root);
    }
}
