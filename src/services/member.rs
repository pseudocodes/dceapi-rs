//! Member service for member ranking APIs.

use crate::error::Result;
use crate::http::{BaseClient, RequestOptions};
use crate::models::{DailyRankingRequest, DailyRankingResponse, PhaseRanking, PhaseRankingRequest};

/// API endpoint for daily ranking.
const PATH_GET_DAILY_RANKING: &str = "/dceapi/forward/publicweb/dailystat/memberDealPosi";

/// API endpoint for phase ranking.
const PATH_GET_PHASE_RANKING: &str = "/dceapi/forward/publicweb/phasestat/memberDealCh";

/// Member service for accessing member ranking data.
#[derive(Debug, Clone)]
pub struct MemberService {
    client: BaseClient,
}

impl MemberService {
    /// Create a new member service.
    pub fn new(client: BaseClient) -> Self {
        MemberService { client }
    }

    /// Get daily trading ranking.
    ///
    /// Returns volume, buy position, and sell position rankings for a contract on a specific date.
    ///
    /// # Arguments
    /// * `req` - Request with variety_id, contract_id, trade_date, and trade_type
    /// * `opts` - Optional request options
    pub async fn get_daily_ranking(
        &self,
        req: &DailyRankingRequest,
        opts: Option<RequestOptions>,
    ) -> Result<DailyRankingResponse> {
        self.client.do_post(PATH_GET_DAILY_RANKING, req, opts).await
    }

    /// Get phase (period) trading ranking.
    ///
    /// Returns member rankings for a date range.
    ///
    /// # Arguments
    /// * `req` - Request with variety, start_month, end_month, and trade_type
    /// * `opts` - Optional request options
    pub async fn get_phase_ranking(
        &self,
        req: &PhaseRankingRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<PhaseRanking>> {
        self.client.do_post(PATH_GET_PHASE_RANKING, req, opts).await
    }
}
