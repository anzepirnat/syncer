use std::fs;
use std::path::Path;
//use std::time::SystemTime;

fn main() {

    let source_folder = Path::new("/home/anzepirnat/DIY/syncer/izvor");
    let destination_folder = Path::new("/home/anzepirnat/DIY/syncer/ponor");

    sync(source_folder, destination_folder);
}

/// Copy a file from source to destination path
fn copy_file(source_path: &Path, destination_path: &Path) {
    // Copy the file from source to destination
    fs::copy(source_path, destination_path).expect("Failed to copy file");
    println!("File copied from {:?} to {:?}", source_path, destination_path);
}

/// Sync the files from source to destination path
fn sync(source_folder: &Path, destination_folder: &Path) {

    for entry in fs::read_dir(source_folder).unwrap() {
        let entry = entry.unwrap();
        println!("{:?}", entry.path());
        let file_name = entry.file_name();
        println!("{:?}", file_name);
        let destination_path = destination_folder.join(file_name.to_str().unwrap());
        println!("{:?}", destination_path);
        if destination_path.exists() {
            println!("File exists");
        } else {
            println!("File does not exist, will have to be copied");
            copy_file(&entry.path(), &destination_path);
        }
    }
}
