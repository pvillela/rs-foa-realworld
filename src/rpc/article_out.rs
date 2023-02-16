use crate::model::ArticlePlus;

pub struct ArticleOut {
    pub article: ArticlePlus,
}

pub struct ArticlesOut {
    pub articles: Vec<ArticleOut>,
    pub articles_count: usize,
}

impl ArticleOut {
    pub fn from_model(article: ArticlePlus) -> ArticleOut {
        ArticleOut { article }
    }
}

impl ArticlesOut {
    pub fn from_model(articles_plus: Vec<ArticlePlus>) -> ArticlesOut {
        let outs: Vec<ArticleOut> = articles_plus
            .into_iter()
            .map(move |a| ArticleOut::from_model(a))
            .collect();
        let len = outs.len();
        ArticlesOut {
            articles: outs,
            articles_count: len,
        }
    }
}
