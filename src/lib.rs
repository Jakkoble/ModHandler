use std::{
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process,
};

use crate::directory::{clear_directory, copy_directory, create_mods_dir};
use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    terminal::{Clear, ClearType},
};
use directory::mods_dir_path;

mod directory;

pub struct Profile {
    pub name: String,
    pub path: PathBuf,
    pub mods: usize,
}

pub enum Message<'a> {
    Success(),
    Error(&'a str),
    Instant,
}

pub fn divider() {
    println!("\n--------------------------------\n");
}

fn read_char() -> String {
    loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(KeyEvent {
                state: _,
                code,
                modifiers,
                kind,
            }) = event::read().unwrap()
            {
                if kind == event::KeyEventKind::Press {
                    if modifiers == KeyModifiers::NONE {
                        match code {
                            KeyCode::Char(c) => {
                                return c.to_string();
                            }
                            KeyCode::Esc => {
                                return "q".to_string();
                            }
                            KeyCode::Enter => {
                                return "ENTER".to_string();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

pub fn quit_program(message: Message) -> ! {
    match message {
        Message::Error(msg) => {
            println!("Error: {}", msg);
            println!("Press any key to exit...");
            read_char();
        }
        Message::Success() => {
            println!("\nDone! Press any key to exit...");
            read_char();
        }
        _ => {}
    }
    process::exit(0);
}

pub fn send_header() {
    Clear(ClearType::All);
    let header = "
  __  __           _ _   _                 _ _           
 |  \\/  | ___   __| | | | | __ _ _ __   __| | | ___ _ __ 
 | |\\/| |/ _ \\ / _` | |_| |/ _` | '_ \\ / _` | |/ _ \\ '__|
 | |  | | (_) | (_| |  _  | (_| | | | | (_| | |  __/ |   
 |_|  |_|\\___/ \\__,_|_| |_|\\__,_|_| |_|\\__,_|_|\\___|_|\n";
    println!("{}", header);
    println!("Your simple Minecraft Mod Manager.");
    println!("Version: {}\n", env!("CARGO_PKG_VERSION"));
    println!("If you have any questions or problems, please visit: https://github.com/jakkoble/ModHandler");
    divider();
}

pub fn fetch_profiles() -> Vec<Profile> {
    let profiles_path = "profiles";
    let profiles = fs::read_dir(profiles_path).unwrap_or_else(|_| {
        if let Ok(_) = fs::create_dir("profiles") {
            return match fs::read_dir(profiles_path) {
                Ok(dir) => dir,
                Err(_) => quit_program(Message::Error("Failed reading profiles directory.")),
            };
        }
        quit_program(Message::Error("Failed creating profiles directory."));
    });

    profiles
        .filter(|entry| {
            let dir = match entry {
                Ok(profile) => profile,
                Err(_) => quit_program(Message::Error("Failed reading profiles directory.")),
            };
            if let Ok(file_type) = dir.file_type() {
                return file_type.is_dir();
            }
            false
        })
        .map(|dir| {
            let profile_dir = match dir {
                Ok(profile) => profile,
                Err(_) => quit_program(Message::Error("Failed reading profiles directory.")),
            };

            let profile_name = profile_dir
                .file_name()
                .to_str()
                .unwrap_or("Unknown Profile name")
                .to_string();

            let profile_path = profile_dir.path();

            let mods = match fs::read_dir(&profile_path) {
                Ok(dir) => dir
                    .filter(|file| {
                        let file = match file {
                            Ok(file) => file,
                            Err(_) => {
                                quit_program(Message::Error("Failed reading profile directory."))
                            }
                        };

                        let file_type = match file.file_type() {
                            Ok(file_type) => file_type,
                            Err(_) => {
                                quit_program(Message::Error("Failed reading profile directory."))
                            }
                        };

                        file_type.is_file()
                            && file
                                .path()
                                .extension()
                                .unwrap_or_else(|| OsStr::new("Unknown"))
                                == "jar"
                    })
                    .count(),
                Err(_) => quit_program(Message::Error("Failed reading profile directory.")),
            };

            Profile {
                name: profile_name,
                path: profile_path,
                mods,
            }
        })
        .collect()
}

pub fn list_profiles(profiles: &Vec<Profile>) {
    let file_count = match fs::read_dir(&mods_dir_path()) {
        Ok(mods) => mods.count(),
        Err(err) => {
            println!("{}", err);
            if err.kind() == std::io::ErrorKind::NotFound {
                create_mods_dir();
                return;
            }
            0
        }
    };

    if file_count > 0 {
        println!(
            "0) Clear {} files from directory (No profile selected)",
            file_count
        );
    }

    for (index, profile) in profiles.iter().enumerate() {
        println!("{}) {} ({} Mods)", index + 1, profile.name, profile.mods);
    }
}

pub fn fetch_profile_input(profiles: &Vec<Profile>) -> &Profile {
    let selection = read_char();

    if selection == "q" {
        println!("Bye!");
        quit_program(Message::Instant);
    }

    let input = selection.parse::<usize>();

    if let Err(_) = input {
        println!("Please enter a valid number! Try again:");
        return fetch_profile_input(profiles);
    }

    let input = input.unwrap();

    if input == 0 {
        clear_directory(&mods_dir_path());
        println!("Mods directory cleared.");
        quit_program(Message::Success());
    }

    let selected_profile = match profiles.get(input - 1) {
        Some(profile) => profile,
        None => {
            println!("There is no profile with the number {}. Try again:", input);
            fetch_profile_input(profiles)
        }
    };

    return selected_profile;
}

pub fn copy_mods(source: &Path) {
    let mods_dir = create_mods_dir();

    let mut files = mods_dir.content.collect::<Vec<_>>();

    let files: Vec<&fs::DirEntry> = files
        .iter_mut()
        .map(|test| test.as_ref().unwrap())
        .collect();

    if !files.is_empty() {
        println!(
            "The mods directory is not empty. Do you want to clear it before continuing? (y/n)"
        );
        let input = read_char();
        if input == "y" {
            clear_directory(&mods_dir.path);
            println!("Mods directory cleared.");
        }
    }

    println!("Copying mods...");
    copy_directory(source, &mods_dir.path);
    quit_program(Message::Success());
}
