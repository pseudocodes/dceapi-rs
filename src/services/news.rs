//! News service for article and announcement APIs.

use std::collections::HashSet;
use std::sync::LazyLock;

use crate::error::{Error, Result};
use crate::http::{BaseClient, RequestOptions};
use crate::models::{ArticleDetail, GetArticleByPageRequest, GetArticleByPageResponse};

/// API endpoint for paginated article list.
const PATH_GET_ARTICLE_BY_PAGE: &str = "/dceapi/cms/info/articleByPage";

/// API endpoint for article detail.
const PATH_GET_ARTICLE_DETAIL: &str = "/dceapi/cms/info/articleDetail";

/// Valid column IDs for articles.
static VALID_COLUMN_IDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert("244");  // 交易所公告
    set.insert("245");  // 交易所通知
    set.insert("246");  // 交割信息
    set.insert("248");  // 会员服务系统公告
    set.insert("1076"); // 期权公告
    set.insert("242");  // 新闻
    set
});

/// Check if a column ID is valid.
pub fn is_valid_column_id(column_id: &str) -> bool {
    VALID_COLUMN_IDS.contains(column_id)
}

/// News service for accessing articles and announcements.
#[derive(Debug, Clone)]
pub struct NewsService {
    client: BaseClient,
}

impl NewsService {
    /// Create a new news service.
    pub fn new(client: BaseClient) -> Self {
        NewsService { client }
    }

    /// Get paginated article list.
    ///
    /// # Arguments
    /// * `req` - Request parameters including column_id, page_no, page_size
    /// * `opts` - Optional request options
    ///
    /// # Valid Column IDs
    /// * `244` - Exchange announcements (交易所公告)
    /// * `245` - Exchange notices (交易所通知)
    /// * `246` - Delivery information (交割信息)
    /// * `248` - Member service announcements (会员服务系统公告)
    /// * `1076` - Options announcements (期权公告)
    /// * `242` - News (新闻)
    pub async fn get_article_by_page(
        &self,
        mut req: GetArticleByPageRequest,
        opts: Option<RequestOptions>,
    ) -> Result<GetArticleByPageResponse> {
        // Validate column_id
        if !is_valid_column_id(&req.column_id) {
            return Err(Error::validation(
                "column_id",
                "invalid column_id, must be one of: 244, 245, 246, 248, 1076, 242",
            ));
        }

        // Apply default site_id if not set
        if req.site_id == 0 {
            req.site_id = 5;
        }

        self.client
            .do_post(PATH_GET_ARTICLE_BY_PAGE, &req, opts)
            .await
    }

    /// Get article detail by ID.
    ///
    /// # Arguments
    /// * `article_id` - The article ID to fetch
    /// * `opts` - Optional request options
    pub async fn get_article_detail(
        &self,
        article_id: &str,
        opts: Option<RequestOptions>,
    ) -> Result<ArticleDetail> {
        if article_id.is_empty() {
            return Err(Error::validation("article_id", "article_id is required"));
        }

        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Request<'a> {
            article_id: &'a str,
        }

        let req = Request { article_id };
        self.client.do_post(PATH_GET_ARTICLE_DETAIL, &req, opts).await
    }
}
