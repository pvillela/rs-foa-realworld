use crate::common::AppError;
use crate::model::ArticlePlus;

pub type ArticleCheckOwnerBfT = Box<dyn Fn(ArticlePlus, String) -> Result<(), AppError>>;

pub fn article_check_owner_bf(article: ArticlePlus, username: String) -> Result<(), AppError> {
    if article.author.username != username {
        return Err(AppError::UnauthorizedUser { username });
    }
    Ok(())
}

fn _type_check() -> ArticleCheckOwnerBfT {
    Box::new(article_check_owner_bf)
}
