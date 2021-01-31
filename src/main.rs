use std::fs;
use std::path::Path;


fn main() -> std::io::Result<()> {
    let path = fs::canonicalize(".")?;

    match find_closest_dir("node_modules", path.as_path()) {
        Some(target_path) => println!("Dir found {:?}", target_path),
        None => println!("No dir found")
    }
    Ok(())
}

fn find_closest_dir<'a>(name: &str, path: &'a Path) -> Option<&'a Path>{
    if has_dir(name, path) {
        return Option::from(path);
    }

    // If has parent lets check it
    let parent = path.parent();
    match parent {
        Some(parent_path) => find_closest_dir(name, parent_path),
        None => None
    }
}

fn has_dir(name: &str, path: &Path) -> bool {
    for entry in fs::read_dir(path).unwrap() {
        let dir = entry.unwrap();
        if dir.file_name() == name {
            return true;
        }
    }

    false
}