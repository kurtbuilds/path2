use std::fmt::Formatter;
use std::ops::Div;

#[derive(Debug)]
struct PathBuf(std::path::PathBuf);

impl PathBuf {
    pub fn new() -> Self {
        PathBuf(std::path::PathBuf::new())
    }

    pub fn root() -> Self {
        PathBuf(std::path::PathBuf::from("/"))
    }
}

impl std::fmt::Display for PathBuf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string_lossy().as_ref())
    }
}

impl<T: ?Sized + AsRef<std::ffi::OsStr>> From<&T> for PathBuf {
    /// Converts a borrowed `OsStr` to a `PathBuf`.
    ///
    /// Allocates a [`PathBuf`] and copies the data into it.
    #[inline]
    fn from(s: &T) -> PathBuf {
        PathBuf(std::path::PathBuf::from(s.as_ref().to_os_string()))
    }
}

impl From<std::path::PathBuf> for PathBuf {
    fn from(path: std::path::PathBuf) -> Self {
        PathBuf(path)
    }
}

impl Into<std::path::PathBuf> for PathBuf {
    fn into(self) -> std::path::PathBuf {
        self.0
    }
}

impl Div<&'_ str> for PathBuf {
    type Output = Self;

    fn div(mut self, rhs: &str) -> Self::Output {
        self.0.push(rhs);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let p = PathBuf::root() / "foo" / "bar.txt";
        println!("{}", p);
        assert_eq!(1, 4);
    }
}