#[cfg(test)]
mod tests {
    use build;

    #[test]
    fn can_create_paths() {
        let src = "src";
        let dest = "dest";

        let paths = build::Paths::new(src, dest);
        assert_eq!(src, paths.src);
        assert_eq!(dest, paths.dest);
    }
}

pub mod build {
    use std::fs::{copy};
    use std::io;
    
    pub struct Paths {
        pub src: String,
        pub dest: String,
    }

    impl Paths {
        pub fn new(src: &str, dest: &str) -> Paths {
            Paths {src: String::from(src), dest: String::from(dest)}
        }
    }

    pub fn copy_path(paths: &Paths, file: &str) -> io::Result<u64> {
        let mut dest = paths.dest.clone();
        dest.push_str(&file);

        let mut src= paths.src.clone();
        src.push_str(&file);

        copy(src, dest)
    }
}
