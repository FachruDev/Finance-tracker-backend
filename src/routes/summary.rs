use actix_web::{get, web, HttpResponse};
use rust_decimal::Decimal;
use serde::Serialize;

use crate::auth::AuthUser;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::category::CategoryBreakdownItem;
use crate::services::summary_service as svc;
use crate::response as resp;

#[derive(Debug, Serialize)]
pub struct MonthlySummary {
    pub year: i32,
    pub month: u32,
    pub total_income: Decimal,
    pub total_expense: Decimal,
    pub balance: Decimal,
    pub category_breakdown: Vec<CategoryBreakdownItem>,
}

#[get("/summary/month")]
pub async fn monthly_summary(
    pool: web::Data<DbPool>,
    user: AuthUser,
    query: web::Query<SummaryQuery>,
) -> Result<HttpResponse, AppError> {
    let res = svc::monthly_summary(pool.get_ref(), user.0, query.year, query.month).await?;
    Ok(resp::ok(res))
}

#[derive(Debug,serde::Deserialize)]
pub struct SummaryQuery {
    pub year: i32,
    pub month: u32,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(monthly_summary);
}
