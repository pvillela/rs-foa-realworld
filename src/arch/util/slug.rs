use slug::slugify;

pub fn slug(title: &str) -> String {
    slugify(title)
}
