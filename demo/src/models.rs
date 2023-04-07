#[derive(serde::Deserialize, Debug)]
pub struct User {
    pub avatar_url: String,
    pub bio: Option<String>,
    pub blog: String,
    pub company: Option<String>,
    pub created_at: String,
    pub email: Option<String>,
    pub events_url: String,
    pub followers_url: String,
    pub followers: i64,
    pub following_url: String,
    pub following: i64,
    pub gists_url: String,
    pub gravatar_id: String,
    pub hireable: Option<bool>,
    pub html_url: String,
    pub id: i64,
    pub location: String,
    pub login: String,
    pub name: Option<String>,
    pub node_id: String,
    pub organizations_url: String,
    pub public_gists: i64,
    pub public_repos: i64,
    pub r#type: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub site_admin: bool,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub twitter_username: Option<String>,
    pub updated_at: String,
    pub url: String,
}