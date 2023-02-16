use crate::model::Tag;

pub struct TagsOut {
    pub tags: Vec<String>,
}

impl TagsOut {
    pub fn from_model(tags: Vec<Tag>) -> TagsOut {
        tags.into_iter().map(move |t| t.name).collect()
    }
}
