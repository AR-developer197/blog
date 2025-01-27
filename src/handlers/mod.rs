mod user;
mod posts;
mod comments;

pub use user::create_user_routes;
pub use posts::create_post_routes;
pub use comments::{comments, create_comments, delete_comments};