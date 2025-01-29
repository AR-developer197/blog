mod comment;

struct Comment {
    comment_id: Option<i32>,
    post_id: Option<i32>,
    user_id: Option<i32>,
    body: String
}

struct Body {
    comment: Comment,
    token: String
}