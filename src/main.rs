use std::fs;
use std::fs::{FileType, DirEntry};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = Path::new(".");
    findClosestDir("node_modules", path);
    Ok(())
}

fn findClosestDir(name: &str, path: &Path) -> Option<&Path>{
    if hasDir(path) {
        return Some(path);
    }

    let parent = path.parent();
    if isRootDir(parent) {
        return None;
    }

    return findClosestDir(name, parent)
}

fn hasDir(path: &Path) -> bool {
    for entry in fs::read_dir(".")? {
        let dir = entry?;
        dir.path().parent();
        println!("{:?}", dir.file_type());
        println!("{:?}", dir.path());
    }
}

fn isRootDir(path: Option<&Path>) -> bool {
    match path {
        Some(&Path) => true,
        None => false
    }
}