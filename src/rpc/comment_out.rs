use crate::model::Comment;

pub struct CommentOut {
    pub comment: Comment,
}

impl CommentOut {
    pub fn from_model(comment: Comment) -> CommentOut {
        CommentOut { comment }
    }
}

pub struct CommentsOut {
    pub comments: Vec<CommentOut>,
}

impl CommentsOut {
    pub fn from_model(comments: Vec<Comment>) -> CommentsOut {
        CommentsOut {
            comments: comments
                .into_iter()
                .map(move |c| CommentOut::from_model(c))
                .collect(),
        }
    }
}
