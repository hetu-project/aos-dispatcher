use axum::{extract::State, Json};
use serde_json::{json, Value};

use crate::{error::AppError, server::server::SharedState};

use super::{
    model::RegisterProjectReq,
    util::{Admin, AdminService},
};
pub async fn register(
    State(server): State<SharedState>,
    Json(req): Json<RegisterProjectReq>,
) -> anyhow::Result<Json<Value>, AppError> {
    let server = server.0.read().await;
    let mut conn = server.pg.get()?;
    let result = Admin::register_project(&mut conn, &req).await?;
    Ok(Json(json!({
      "result": result
    })))
}

pub async fn white_list(
    State(server): State<SharedState>,
) -> anyhow::Result<Json<Value>, AppError> {
    let server = server.0.read().await;
    let mut conn = server.pg.get()?;
    let result = Admin::project_list(&mut conn).await?;
    Ok(Json(json!({
      "result": result
    })))
}
