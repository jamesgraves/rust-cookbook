use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};
use std::fs::{create_dir_all, remove_file};
use std::os::unix::fs::symlink;
use same_file::is_same_file;

fn contains_loop<P: AsRef<Path>>(path: P) -> io::Result<Option<(PathBuf, PathBuf)>> {
    let path = path.as_ref();
    let mut path_buf = path.to_path_buf();
    while path_buf.pop() {
        if is_same_file(&path_buf, path)? {
            return Ok(Some((path_buf, path.to_path_buf())));
        } else if let Some(looped_paths) = contains_loop(&path_buf)? {
            return Ok(Some(looped_paths));
        }
    }
    return Ok(None);
}

fn main() -> Result<(), Box<dyn Error>>{

    create_dir_all("/tmp/foo/bar/baz")?;
    let _ = remove_file("/tmp/foo/bar/baz/qux"); // Dont't care if the file doesn't exist yet.
    symlink("/tmp/foo", "/tmp/foo/bar/baz/qux")?;

    assert_eq!(
        contains_loop("/tmp/foo/bar/baz/qux/bar/baz").unwrap(),
        Some((
            PathBuf::from("/tmp/foo"),
            PathBuf::from("/tmp/foo/bar/baz/qux")
        ))
    );

    Ok(())
}
