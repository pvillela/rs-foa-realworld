/// Defines criteria for selection of articles
pub struct ArticleCriteria {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited_by: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
