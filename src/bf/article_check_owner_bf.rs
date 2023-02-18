use crate::model::ArticlePlus;
use anyhow::Result;

pub type ArticleCheckOwnerBfT = fn(article: ArticlePlus, username: String) -> Result<()>;

fn article_check_owner_bf(article: ArticlePlus, username: String) -> Result<()> {
    if article.author.username != username {
        return err_unauthorized_user.make(nil, err_msg_unauthorized_user, username);
    }
    return nil;
}
