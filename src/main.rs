use mod_handler::{
    copy_mods, divider, fetch_profile_input, fetch_profiles, list_profiles, quit_program,
    send_header, Message,
};

fn main() {
    send_header();

    if !cfg!(target_os = "windows") {
        println!("You are not using Windows. Visit for Guide: https://github.com/Jakkoble/ModHandler#specify-different-minecraft-directory");
        quit_program(Message::Success())
    }

    let profiles = fetch_profiles();
    if profiles.is_empty() {
        println!("I seems like you haven't created any profile yet. Go to the created \"profiles\" directory and create a new profile.");
        println!("For further information visit: https://github.com/jakkoble/ModHandler");
        quit_program(Message::Success())
    }

    println!("Your Profiles:");
    list_profiles(&profiles);

    divider();

    println!("Select the profile by typing the number: (type \"q\" to quit)");
    let selected_profile = fetch_profile_input(&profiles);
    copy_mods(&selected_profile.path);
}
