use axum::extract::ws::Message;
use tokio::sync::mpsc;

use crate::{db::pg::{model::{Answer, JobResult}, util::{create_job_answer, create_job_result}}, server::server::SharedState};

use super::msg::{ConnectParams, JobResultParams, WsMethodMsg};

pub fn convert_to_msg(msg: &str) -> Result<WsMethodMsg, ()> {
  let method_msg =
      serde_json::from_str::<WsMethodMsg>(msg);
  match method_msg {
      Ok(m) => {
          Ok(m)
      }
      Err(e) => {
          Err(())
      }
  }
}

pub async fn connect_to_dispatcher(
    msg: &WsMethodMsg,
    mut tx: mpsc::Sender<Message>,
    server: SharedState,
) -> Result<(), ()>{
    let operator = msg.params.as_array().and_then(|p| {
        let a = p.get(0);
        if let Some(s) = a {
            let p = serde_json::from_value::<ConnectParams>(s.clone()).ok();
            return  p
        }
        None
    });
    if let Some(p) = operator {
        tracing::debug!("operator id {} connect", p.operator);
        let mut server = server.0.write().await;
        server.operator_channels.insert(p.operator, tx);
    }
    Ok(())
}

pub async fn receive_job_result(
    msg: &WsMethodMsg,
    mut tx: mpsc::Sender<Message>,
    server: SharedState,
) -> Result<(), ()>{
    let operator = msg.params.as_array().and_then(|p| {
        let a = p.get(0);
        if let Some(s) = a {
            let p = serde_json::from_value::<JobResultParams>(s.clone()).ok();
            return  p
        }
        None
    });
    if let Some(p) = operator {
        tracing::debug!("job of operator id {} connect saved", p.operator);
        let mut server = server.0.write().await;
        let jr = JobResult {
            id: p.id.clone(),
            job_id: p.id,
            operator: p.operator,
            result: todo!(),
            signature: "".into(),
            job_type: "".into(),
            created_at: chrono::Local::now().naive_local(),
        };
        let mut conn = server.pg.get().expect("Failed to get a connection from pool");

        let _ = create_job_result(&mut conn, &jr);
    }
    Ok(())
}
