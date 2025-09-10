use actix_web::{delete, get, post, put, web, HttpResponse};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::transaction::{CreateTransaction, Transaction, TxnQuery, UpdateTransaction};

#[get("/transactions")]
pub async fn list_transactions(
    pool: web::Data<DbPool>,
    user: AuthUser,
    query: web::Query<TxnQuery>,
) -> Result<HttpResponse, AppError> {
    let q = query.into_inner();
    let rows = sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, user_id, category_id, amount, occurred_on, description
        FROM transactions
        WHERE user_id = $1
          AND ($2::uuid IS NULL OR category_id = $2)
          AND ($3::date IS NULL OR occurred_on >= $3)
          AND ($4::date IS NULL OR occurred_on <= $4)
        ORDER BY occurred_on DESC, id DESC
        "#,
    )
    .bind(user.0)
    .bind(q.category_id)
    .bind(q.start_date)
    .bind(q.end_date)
    .fetch_all(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(rows))
}

#[post("/transactions")]
pub async fn create_transaction(
    pool: web::Data<DbPool>,
    user: AuthUser,
    payload: web::Json<CreateTransaction>,
) -> Result<HttpResponse, AppError> {
    let id = Uuid::new_v4();
    // Ensure category belongs to user
    let owner = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(1) FROM categories WHERE id=$1 AND user_id=$2",
    )
    .bind(payload.category_id)
    .bind(user.0)
    .fetch_one(pool.get_ref())
    .await?;
    if owner == 0 {
        return Err(AppError::BadRequest("Invalid category for user".into()));
    }

    let rec = sqlx::query_as::<_, Transaction>(
        r#"INSERT INTO transactions (id, user_id, category_id, amount, occurred_on, description)
            VALUES ($1,$2,$3,$4,$5,$6)
            RETURNING id, user_id, category_id, amount, occurred_on, description"#,
    )
    .bind(id)
    .bind(user.0)
    .bind(payload.category_id)
    .bind(&payload.amount)
    .bind(payload.occurred_on)
    .bind(&payload.description)
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(rec))
}

#[put("/transactions/{id}")]
pub async fn update_transaction(
    pool: web::Data<DbPool>,
    user: AuthUser,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateTransaction>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    // Fetch current
    let mut current = sqlx::query_as::<_, Transaction>(
        "SELECT id, user_id, category_id, amount, occurred_on, description FROM transactions WHERE id=$1 AND user_id=$2",
    )
    .bind(id)
    .bind(user.0)
    .fetch_optional(pool.get_ref())
    .await?;

    let current = match current.take() {
        Some(t) => t,
        None => return Err(AppError::NotFound("Transaction not found".into())),
    };

    let category_id = payload.category_id.unwrap_or(current.category_id);
    // Verify category owner if changed
    if category_id != current.category_id {
        let owner = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(1) FROM categories WHERE id=$1 AND user_id=$2",
        )
        .bind(category_id)
        .bind(user.0)
        .fetch_one(pool.get_ref())
        .await?;
        if owner == 0 {
            return Err(AppError::BadRequest("Invalid category for user".into()));
        }
    }

    let amount = payload.amount.clone().unwrap_or(current.amount.clone());
    let occurred_on = payload.occurred_on.unwrap_or(current.occurred_on);
    let description = payload.description.clone().or(current.description.clone());

    let updated = sqlx::query_as::<_, Transaction>(
        r#"UPDATE transactions SET category_id=$1, amount=$2, occurred_on=$3, description=$4
           WHERE id=$5 AND user_id=$6
           RETURNING id, user_id, category_id, amount, occurred_on, description"#,
    )
    .bind(category_id)
    .bind(&amount)
    .bind(occurred_on)
    .bind(&description)
    .bind(id)
    .bind(user.0)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/transactions/{id}")]
pub async fn delete_transaction(
    pool: web::Data<DbPool>,
    user: AuthUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let res = sqlx::query("DELETE FROM transactions WHERE id=$1 AND user_id=$2")
        .bind(id)
        .bind(user.0)
        .execute(pool.get_ref())
        .await?;
    if res.rows_affected() == 0 {
        return Err(AppError::NotFound("Transaction not found".into()));
    }
    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_transactions)
        .service(create_transaction)
        .service(update_transaction)
        .service(delete_transaction);
}
