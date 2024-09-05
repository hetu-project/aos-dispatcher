use axum::{BoxError, debug_handler, extract, Json};
use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::db::pg::model::Operator;
use crate::db::pg::util::{create_operator, get_operator_by_id, query_operators};
use crate::server::server::SharedState;
use serde_json::json;
use crate::db::pg;

use crate::operator::model::{OperatorInfoReq, OperatorRegisterReq, OperatorRegisterResp};

pub fn sample_range_of_operators(ops: &mut Vec<Operator>) {
  let count = ops.len();
  let range_max = 0xffffff;
  let range_min = 0x000000;
  let diff = range_max - range_min;
  // let range_per = (range_max - range_min) / count;
  for (pos ,op) in ops.iter_mut().enumerate() {
    let start = range_min + ((pos * diff)/count);
    let end = range_min + (((pos+1) * diff)/count);
    op.start = format!("{:06x}", start).into();
    op.end = format!("{:06x}", end).into();
  }
}

#[debug_handler]
pub async fn register(State(server): State<SharedState>, Json(req): Json<OperatorRegisterReq>) -> Json<serde_json::Value> {
  tracing::debug!("register operator");
  let id: String = req.address.to_string();
  let operator = Operator { 
    id: req.params.operator.clone(),
    name: format!("Operator {}", &id).into(), 
    address: req.params.operator.clone(),
    start: "".into(),
    end: "".into(), 
    operator_type: "".into(),
    status: "".into(), 
    created_at: chrono::Local::now().naive_local(), 
  };
  let mut server = server.0.write().await;
  let keys = &server.nostr_keys;
  let mut conn = server.pg.get().expect("Failed to get a connection from pool");
  let r = create_operator(&mut conn, &operator);

  if let Err(e) = r {
    return Json(json!({
      "code": 500,
      "message": e.to_string(),
      "result": null
    }))
  }

  // let list = vec![operator];
  let mut list = query_operators(&mut conn).unwrap();
  sample_range_of_operators(&mut list);

  let insert_operators = pg::util::sync_operators_info(&mut conn, &list).expect("Error saving new question");
  Json(json!({
    "code": 200,
    "result": insert_operators,
}))
}



#[debug_handler]
pub async fn operator_info(State(server): State<SharedState>, Json(req): Json<OperatorInfoReq>) -> Json<serde_json::Value> {
  tracing::debug!("operator info");
  let operator_id = req.operator;
  let mut server = server.0.write().await;
  // let keys = &server.nostr_keys;
  let mut conn = server.pg.get().expect("Failed to get a connection from pool");
  let operator = get_operator_by_id(&mut conn, &operator_id).ok();
  Json(json!({
    "code": 200,
    "result": operator,
}))
}

