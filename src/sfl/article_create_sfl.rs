use crate::{rpc::{ArticleCreateIn, ArticleOut}, common::AppError};

pub struct ArticleCreateSflCfgInfo<'a> {
    pub a: &'a str,
    pub b: i32,
}


pub struct ArticleCreateSflDeps {
    pub user_get_by_name_daf: Box<UserGetByNameDafTxT>,
	pub article_create_daf:  Box<ArticleCreateDafTxT>,
	pub tags_add_new_daf:  Box<TagsAddNewDafTxT>,
	pub tags_add_to_article_daf: Box<TagsAddToArticleDafTxT>,
	}


pub async fn article_create_sfl_c<ACFG>(
    cfg_src: impl Make<ACFG>,
    d: impl Deref<Target = FooArtSflDeps>,
    input: ArticleCreateIn,
    tx: &Tx<'_>,

) -> Result<ArticleOut, AppError>   
where
    ACFG: for<'a> RefInto<'a, ArticleCreateSflCfgInfo<'a>>,
{
	db := cfg_src()
	return dbpgx.sfl_with_transaction(db, func(
		ctx context.Context,
		tx pgx.Tx,
		req_ctx web.RequestContext,
		in rpc.ArticleCreateIn,
	) (rpc.ArticleOut, error) {
		err := in.validate()
		if err != nil {
			return rpc.article_out{}, err
		}
		username := req_ctx.username

		user, err := user_get_by_name_daf(ctx, tx, username)
		if err != nil {
			return rpc.article_out{}, err
		}

		article, err := in.to_article(user)
		if err != nil {
			return rpc.article_out{}, err
		}

		err = article_create_daf(ctx, tx, &article)
		if err != nil {
			return rpc.article_out{}, err
		}

		names := article.tag_list

		err = tags_add_new_daf(ctx, tx, names)
		if err != nil {
			return rpc.article_out{}, err
		}

		err = tags_add_to_article_daf(ctx, tx, names, article)
		if err != nil {
			return rpc.article_out{}, err
		}

		article_plus := model.article_plus__from_article(article, false, model.profile__from_user(user, false))
		article_out := rpc.article_out__from_model(article_plus)

		return article_out, nil
	})
}

