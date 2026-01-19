//! News service for article and announcement APIs.

use std::collections::HashSet;
use std::sync::LazyLock;

use crate::error::{Error, Result};
use crate::http::{BaseClient, RequestOptions};
use crate::models::{GetArticleByPageRequest, GetArticleByPageResponse};

/// API endpoint for paginated article list.
const PATH_GET_ARTICLE_BY_PAGE: &str = "/dceapi/cms/info/articleByPage";

/// Valid column IDs for articles.
/// - 244: 业务公告与通知
/// - 245: 活动公告与通知
/// - 246: 交易所新闻-文媒
/// - 248: 媒体看大商所-文媒
/// - 1076: 今日提示
/// - 242: 新闻发布
static VALID_COLUMN_IDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert("244"); // 业务公告与通知
    set.insert("245"); // 活动公告与通知
    set.insert("246"); // 交易所新闻-文媒
    set.insert("248"); // 媒体看大商所-文媒
    set.insert("1076"); // 今日提示
    set.insert("242"); // 新闻发布
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
    /// # Valid Column IDs (官方文档定义)
    /// * `244` - 业务公告与通知
    /// * `245` - 活动公告与通知
    /// * `246` - 交易所新闻-文媒
    /// * `248` - 媒体看大商所-文媒
    /// * `1076` - 今日提示
    /// * `242` - 新闻发布
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
}
