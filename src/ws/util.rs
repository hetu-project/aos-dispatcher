use anyhow::anyhow;
use axum::extract::ws::Message;
use serde_json::json;
use tokio::sync::mpsc;

use crate::{
    db::pg::{model::JobResult, util::create_job_result},
    server::server::SharedState,
};

use super::msg::{ConnectParams, JobResultParams, WsMethodMsg};

pub async fn handle_command_msg(msg: &String, _tx: mpsc::Sender<Message>) -> anyhow::Result<()> {
    let method_msg = convert_to_msg(msg)?;
    tracing::debug!("receive {:#?}", &method_msg.method);
    let method = method_msg.method.ok_or(anyhow!("the method is empty"))?;
    tracing::debug!("Receive method msg {:#?}", method);
    match method.as_str() {
        "connect" => {}
        "job_result" => {}
        _ => {}
    }
    Ok(())
}

pub fn convert_to_msg(msg: &str) -> anyhow::Result<WsMethodMsg> {
    let method_msg =
        serde_json::from_str::<WsMethodMsg>(msg).map_err(|e| anyhow!("convert msg error {}", e));
    method_msg
}

pub async fn connect_to_dispatcher(
    msg: &WsMethodMsg,
    tx: mpsc::Sender<Message>,
    server: SharedState,
) -> Result<String, ()> {
    let operator = msg.params.clone().and_then(|p| {
        p.as_array().and_then(|v| {
            let a = v.get(0);
            if let Some(s) = a {
                let p = serde_json::from_value::<ConnectParams>(s.clone()).ok();
                return p;
            }
            None
        })
    });
    if let Some(p) = operator {
        tracing::debug!("operator id {} connect", p.operator);
        let mut server = server.0.write().await;
        server.operator_channels.insert(p.operator.clone(), tx);
        return Ok(p.operator.clone());
    }
    Err(())
}

pub async fn receive_job_result(
    msg: &WsMethodMsg,
    _tx: mpsc::Sender<Message>,
    server: SharedState,
) -> anyhow::Result<()> {
    tracing::debug!("receive job result");
    let result = msg.params.clone().and_then(|p| {
        p.as_array().and_then(|v| {
            let a = v.get(0);
            if let Some(s) = a {
                let p = serde_json::from_value::<JobResultParams>(s.clone()).ok();
                return p;
            }
            None

        })
    });
    if let Some(p) = result {
        tracing::debug!("job of operator id {} connect saved", p.operator);
        let server = server.0.write().await;
        let jr = JobResult {
            id: format!(
                "{}_{}_{}",
                p.operator.clone(),
                p.job_id.clone(),
                p.tag.clone().unwrap_or_default()
            ),
            job_id: p.job_id.clone(),
            operator: p.operator,
            result: p.result.into(),
            signature: "".into(),
            job_type: "".into(),
            tag: p.tag.unwrap_or_default(),
            clock: json!({
                "1": "2",
            }),
            created_at: chrono::Local::now().naive_local(),
        };
        let mut conn = server
            .pg
            .get()?;

        let _ = create_job_result(&mut conn, &jr);
    } else {
        tracing::error!("there is no job result");
    }
    Ok(())
}
