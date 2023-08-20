use super::{quit_program, Message};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub fn clear_directory(path: &Path) {
    if fs::metadata(&path).is_err() {
        println!("Mods directory does not exist.");
        return;
    }

    let content = fs::read_dir(path).unwrap_or_else(|_| {
        quit_program(Message::Error("Failed reading mods directory."));
    });

    content.for_each(|entry| {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => quit_program(Message::Error("Failed reading mods directory.")),
        };

        let path = entry.path();

        if fs::metadata(&path).unwrap().is_dir() {
            if let Err(_) = fs::remove_dir_all(&path) {
                quit_program(Message::Error("\nFailed clearing mods directory!"));
            }
            return;
        }

        if let Err(_) = fs::remove_file(path) {
            quit_program(Message::Error("\nFailed clearing mods directory!"));
        }
    });
}

pub fn copy_directory(source: &Path, destination: &Path) {
    if fs::metadata(&source).is_err() {
        quit_program(Message::Error("Mods directory does not exist."));
    }

    let entries = match fs::read_dir(source) {
        Ok(entries) => entries,
        Err(_) => quit_program(Message::Error("Failed reading mods directory.")),
    };

    if fs::metadata(destination).is_err() {
        if let Err(_) = fs::create_dir_all(destination) {
            quit_program(Message::Error("Failed creating mods directory."));
        }
    }

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => quit_program(Message::Error("Failed reading mods directory.")),
        };

        let path = entry.path();
        let file_name = match path.file_name() {
            Some(file_name) => file_name,
            None => quit_program(Message::Error("Failed reading mods directory.")),
        };

        let destination_path = destination.join(file_name.to_str().unwrap());

        if fs::metadata(&path).unwrap().is_dir() {
            copy_directory(&path, &destination_path);
            continue;
        } else {
            fs::copy(&path, destination_path).unwrap_or_else(|_| {
                quit_program(Message::Error("Failed copying mods directory!"));
            });
        }
    }
}

pub struct ModDir {
    pub content: fs::ReadDir,
    pub path: PathBuf,
}

pub fn mods_dir_path() -> PathBuf {
    if let Ok(content) = fs::read_to_string("path.txt") {
        return Path::new(&content.trim().trim_end_matches("/")).join("mods");
    }
    let appdata_path = match env::var_os("APPDATA") {
        Some(os_path) => match os_path.to_str() {
            Some(path) => String::from(path),
            None => quit_program(Message::Error(
                "Failed parsing APPDATA environmental variable.",
            )),
        },
        None => quit_program(Message::Error("APPDATA environmental variable not found.")),
    };
    Path::new(&appdata_path).join(".minecraft").join("mods")
}

pub fn create_mods_dir() -> ModDir {
    let mods_path = mods_dir_path();
    if fs::metadata(&mods_path).is_err() {
        if let Err(err) = fs::create_dir(&mods_path) {
            eprintln!("{}", err);
            quit_program(Message::Error(
                "Failed creating mods directory. Have you probably installed Minecraft?",
            ));
        }
    }

    let dir = match fs::read_dir(&mods_path) {
        Ok(dir) => dir,
        Err(_) => quit_program(Message::Error("Failed to read mods directory.")),
    };

    return ModDir {
        content: dir,
        path: mods_path,
    };
}
