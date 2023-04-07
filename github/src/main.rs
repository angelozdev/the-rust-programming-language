use github::services::user_service;
use std::{env, process};

#[tokio::main]
async fn main() {
    let username = get_username_from_args();
    let user = user_service::get(username.as_str()).await;

    if let Some(user) = user {
        println!("Hello {}!", user.name.unwrap_or(user.login));
        println!("You have {} followers", user.followers);
        println!("You have {} following", user.following);
        println!("You have {} public repos", user.public_repos);
        println!("You have {} public gists", user.public_gists);
        println!("You are a {} on GitHub", user.r#type);
        println!("Your GitHub profile is at {}", user.html_url);
        println!("Your GitHub profile image is at {}", user.avatar_url);
        println!("Your GitHub profile was created at {}", user.created_at);
        println!(
            "Your GitHub profile was last updated at {}",
            user.updated_at
        );
    } else {
        println!("User not found");
    }
}

fn get_username_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1].is_empty() {
        println!("Usage: cargo run <username>");
        process::exit(1);
    }

    args[1].clone()
}
