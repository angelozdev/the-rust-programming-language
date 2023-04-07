pub mod user_service {
    use crate::{models::User, services::utils};

    pub async fn get(username: &str) -> Option<User> {
        let url = format!("https://api.github.com/users/{}", username);
        println!("Fetching user from: {}", url);
        let client = reqwest::Client::new();
        let headers = utils::build_headers();

        let res = client.get(&url).headers(headers).send().await;

        match res {
            Ok(res) => utils::handle_success(res).await,
            Err(err) => {
                println!("Failed to fetch user: {}", err);
                None
            }
        }
    }
}
