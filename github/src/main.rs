use chrono::DateTime;
use github::services::user_service;
use std::{env, process};

#[tokio::main]
async fn main() {
    let username = get_username_from_args();
    let user = user_service::get(username.as_str()).await;

    if let Some(user) = user {
        let created_at = format_date(&user.created_at);
        let updated_at = format_date(&user.updated_at);

        println!("Hello, {}!", user.name.unwrap_or(user.login));
        println!("You have {} followers", user.followers);
        println!("You have {} following", user.following);
        println!("You have {} public repos", user.public_repos);
        println!("You have {} public gists", user.public_gists);
        println!("You are a {} on GitHub", user.r#type);
        println!("Your GitHub profile is at {}", user.html_url);

        if let Some(created_at) = created_at {
            println!("Your GitHub profile was created at {}", created_at);
        }

        if let Some(updated_at) = updated_at {
            println!("Your GitHub profile was last updated at {}", updated_at);
        }
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

fn format_date(date: &str) -> Option<String> {
    let date = DateTime::parse_from_rfc3339(date);
    if let Ok(date) = date {
        return Some(date.format("%d %B %Y").to_string());
    }
    None
}
