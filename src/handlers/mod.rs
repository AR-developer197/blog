mod user;
mod posts;
mod comments;

pub use user::create_user_routes;
pub use posts::{get_post, get_posts, create_post, modify_post, delete_post};
pub use comments::{comments, create_comments, delete_comments};