use std::fs;
use std::path::Path;
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

/// Get the syncer.toml file
fn get_config_path() -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get current exe path");
    let exe_dir = exe_path.parent().expect("Failed to get exe directory");
    let config_path = exe_dir.join("syncer.toml");
    
    if config_path.exists() {
        config_path
    } else {
        panic!("Config file not found at {:?}", config_path);
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    sync_pairs: Vec<SyncPair>,
}

#[derive(Debug, Deserialize)]
struct SyncPair {
    source_folder: String,
    destination_folder: String,
}

fn main() {

    let config_path = get_config_path();
    let config = load_config(&config_path);
    println!("Loaded config: {:?}", config);

    for pair in config.sync_pairs {
        let source_folder = Path::new(&pair.source_folder);
        let destination_folder = Path::new(&pair.destination_folder);
        println!("\n\n#####################################################################################################################");
        println!("Syncing from {:?} to {:?}", source_folder, destination_folder);
        println!("#####################################################################################################################\n");
        sync(source_folder, destination_folder);       
    }
}


/// Load the configuration from a TOML file
fn load_config(path: &Path) -> Config {
    let config_content = fs::read_to_string(path).expect("Failed to read config file");
    toml::from_str(&config_content).expect("Failed to parse config file")
}

/// Copy a file from source to destination path
fn copy_file(source_path: &Path, destination_path: &Path) {
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
        println!("File updated from {:?} to {:?}\n", source_path, destination_path);
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
