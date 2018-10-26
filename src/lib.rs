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
    use std::fs::{copy, create_dir_all};
    use std::path;
    use std::result;

    pub struct Paths {
        pub src: String,
        pub dest: String,
    }

    impl Paths {
        pub fn new(src: &str, dest: &str) -> Paths {
            Paths {src: String::from(src), dest: String::from(dest)}
        }
    }

    pub fn copy_path(paths: &Paths, file: &str) -> result::Result<(), &'static str> {
        let mut dest = paths.dest.clone();
        dest.push_str(&file);

       {
            let file_path = path::Path::new(&dest);
            let parent = match file_path.parent() {
            None => return result::Result::Err("can't find directory path"),
            Some(p) => p
        };
        if !parent.is_dir() {
            let _ = match parent.to_str() {
                None => return result::Result::Err("parent directory not valid string"),
                Some(s) => create_dir_all(&s)
            };
        }
       }

        let mut src= paths.src.clone();
        src.push_str(&file);

        match copy(src, dest) {
            Ok(_) => result::Result::Ok(()),
            Err(_) => result::Result::Err("Copy failed")
        }
    }
}
