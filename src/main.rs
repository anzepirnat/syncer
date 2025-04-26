use std::fs;
use std::path::Path;
//use std::time::SystemTime;

fn main() {

    let source_folder = Path::new("/home/anzepirnat/DIY/syncer/izvor");
    let destination_folder = Path::new("/home/anzepirnat/DIY/syncer/ponor");
    //let mnt_folder = Path::new("/mnt");
    //let mnt_anime_folder = Path::new("/mnt/UbuntuServer1/anime");

    sync(source_folder, destination_folder);
}

/// Copy a file from source to destination path
fn copy_file(source_path: &Path, destination_path: &Path) {
    // Copy the file from source to destination
    fs::copy(source_path, destination_path).expect("Failed to copy file");
}

/// Check when file was last modified
fn check_modified(file_path: &Path) -> std::time::SystemTime {
    let metadata = fs::metadata(file_path).unwrap();
    let modified_time = metadata.modified().unwrap();
    return modified_time;
}

/// Updates the file if newer version exists
fn update_file (source_path: &Path, destination_path: &Path) {
    if check_modified(source_path) > check_modified(destination_path) {
        copy_file(source_path, destination_path);
        println!("File updated from {:?} to {:?}", source_path, destination_path);
    } else {
        println!("File is up to date.\n");
    }
}

/// Sync the files from source to destination path
fn sync(source_folder: &Path, destination_folder: &Path) {
    
    let entries = fs::read_dir(source_folder).unwrap();
    
    for entry in entries {
    
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let destination_path = destination_folder.join(file_name.to_str().unwrap());
        
        // This if statement makes it recursive; check all subfolders. It will catch folders.
        if entry.path().is_dir() {

            if !destination_path.exists() {
                fs::create_dir_all(&destination_path).unwrap();
                println!("Created directory {:?}", destination_path);
            }

            sync(&entry.path(), &destination_path);
        } else { // Only files will come here
            // If the file exists, check if it is newer, if not copy it
            if destination_path.exists() {
                println!("File exists");
                update_file(&entry.path(), &destination_path);
            } else {
                println!("File does not exist, will have to be copied");
                copy_file(&entry.path(), &destination_path);
                println!("File copied from {:?} to {:?}\n", entry.path(), destination_path);
            }
        }
    }
}
