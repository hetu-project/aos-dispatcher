const MNEMONIC_PHRASE: &str = "equal dragon fabric refuse stable cherry smoke allow alley easy never medal attend together lumber movie what sad siege weather matrix buffalo state shoot";
const DEFAULT_RELAY: &str = "ws://localhost:7010";

use model::JobAnswer;
use nostr::nips::nip06::FromMnemonic;
use nostr::nips::nip19::ToBech32;
use nostr::{Keys, Result};

use nostr_sdk::{Client, Event, EventBuilder, EventId, Filter, Kind, Metadata, RelayPoolNotification, SecretKey, SingleLetterTag, Tag, TagKind, Timestamp, Url};
use tokio::sync::mpsc;
use tracing::instrument::WithSubscriber;

use crate::opml::model::{create_opml_question, OpmlRequest};
use crate::server::server::SharedState;
use crate::tee::model::{create_question, query_latest_question, OperatorReq};
pub mod util;
pub mod model;
pub async fn subscription_service(
  server: SharedState,
  mut job_status_rx: mpsc::Receiver<JobAnswer>,
  mut dispatch_task_tx: mpsc::Sender<u32>,
  key: ed25519_dalek::SecretKey,
  relay_url: String,
){
  // let keys = Keys::from_mnemonic(MNEMONIC_PHRASE, None).unwrap();
  let secret_key = SecretKey::from_slice(key.as_ref()).unwrap();
  let keys = Keys::new(secret_key);

  let bech32_address = keys.public_key().to_bech32().unwrap();

  let client = Client::new(&keys);
  // let client = Client::default();
  client.add_relay(&relay_url).await.unwrap();
  client.connect().await;
  tracing::info!("connect relay {:#?} with {:#?}", &relay_url, bech32_address);
  let metadata = Metadata::new()
  .name("aos-dispatcher")
  .display_name("Aos Dispatcher")
  .website(Url::parse("https://github.com/hetu-project/aos-dispatcher").unwrap());

  let submit_client = client.clone();
  let job_status_submit = tokio::spawn(async move {


    while let Some(job_status) = job_status_rx.recv().await {
      tracing::info!("job status {:#?}", job_status);
      let event_id = job_status.event_id;
      let origin_event_filter = Filter::new().id(event_id);
      let job_request = submit_client
        .get_events_of(
          vec![origin_event_filter],
          nostr_sdk::EventSource::Relays { timeout: None, specific_relays: None }
        )
        .await;
      if let Ok(job_request) = job_request {
        match job_request.get(0) {
            Some(jq) => {
              let event = EventBuilder::job_result(jq.clone(), job_status.answer.clone(), 0,  None).unwrap();
              tracing::debug!("sending job result to nostr relay, {:#?}", event);
              submit_client.send_event_builder(event).await.unwrap();
              tracing::debug!("sended job result to nostr relay");
            },
            None => {
              tracing::error!("There is no event id {:#?} on relay", event_id);

            },
        }
      } else {
        tracing::error!("There is no event id {:#?} on relay", event_id);
      }

    }
  });

  let mut subscription = Filter::new()
  // .pubkey(keys.public_key())
  .kinds([Kind::JobRequest(5050)])
  // .since(Timestamp::now())
  // .kind(Kind::Custom(5050))
  // .limit(10)
  ;

  {
    let mut server = server.0.write().await;
    let mut conn = server.pg.get().expect("Failed to get a connection from pool");

    if let Ok(latest_question) = query_latest_question(&mut conn) {
      let time = latest_question.created_at.and_utc().timestamp();
      subscription = subscription.since(Timestamp::from(time as u64));
    } else {
      // subscription.since(Timestamp::now())
         
    }
  }



  client.subscribe(vec![subscription], None).await.unwrap();
  tracing::info!("Subscription ID: [auto-closing] start");

  let sub = client.handle_notifications(|notification|async{
    tracing::debug!("job notification {:#?}", notification);
    if let RelayPoolNotification::Event{
      event, ..
    }  = notification {
      // tracing::info!("job notification {:#?}", event);
      if event.kind() == Kind::JobRequest(5050) {

        // tracing::info!("receive task {:#?}", event);
        tracing::info!("receive task event {:#?}", event.id());
        // let uuid = uuid::Uuid::new_v4();
        let request_id =  event.id().to_string();
        // let mut e_model = None;
        // let mut e_prompt = None;

        let aos_task = util::AosTask::parse_event(&event).unwrap();
        tracing::debug!("start store task start {:#?}", request_id);

        {

          let mut server = server.0.write().await;
          let mut conn = server.pg.get().expect("Failed to get a connection from pool");
          let message = aos_task.prompt.unwrap_or_default();
          let message_id = event.id().to_string();
          let conversation_id = event.id().to_string();
          let model = aos_task.model.unwrap_or_default();
          let callback_url = event.id().to_string();

          // TODO: dispatch task to worker by websocket
          // dispatch_task_rx.send(2).await.unwrap();

          
          let q = create_question(
            &mut conn, 
            request_id.clone(),
            message.clone(),
            message_id,
            conversation_id,
            model.clone(), 
            callback_url.clone(),
          );

          if let Ok(q) =  q {
            // start dispatch tee task
            tracing::info!("store task success: {:#?}", q.request_id);


            tracing::debug!("emit dispatch task: {:#?}", q.request_id);
            dispatch_task_tx.send(1).await.unwrap_or_default();

            // let hash = &q.request_id;
            // let signature = server.sign(hash.as_bytes());       
            //   let op_req = OperatorReq {
            //     request_id: q.request_id.clone(),
            //     node_id: "".to_string(),
            //     model: model.clone(),
            //     prompt: message.clone(),
            //     prompt_hash: hash.into(),
            //     signature: signature.to_string(),
            //     params: aos_task.params.clone(),
            //     r#type: "TEE".to_string(),
            //   };
              // let next_work_name = server.tee_operator_collections.keys().next();
              // match  next_work_name {
              //   Some(work_name) => {
              //     server.send_tee_inductive_task(work_name.clone(), op_req).await;
              //     tracing::debug!("dispatched task {:#?}", request_id);
              //   },
              //   None => {
              //     tracing::warn!("there is no tee operator");
              //   },
              // }

              // // start dispatch opml task
              // tracing::debug!("dispatch opml task {:#?}", request_id);
              // let opml_request = OpmlRequest {
              //   model: model.clone(),
              //   prompt: message.clone(),
              //   req_id: request_id.clone(),
              //   callback: callback_url.clone(),
              // };
    
              // if let Err(e) = create_opml_question(&mut conn, request_id.clone(), &opml_request) {
              //   tracing::error!("Failed to store OPML question: {:?}", e);
              // }
    
    
              // // Send the request to the OPML server
              // if let Err(e) = server.send_opml_request(opml_request).await {
              //   tracing::error!("Failed to send OPML request: {:?}", e);
              // }
              
            }
            // tracing::debug!("dispatch task end {:#?}", request_id);
          }
      } else {
        tracing::info!("JobRequest other {:#?}", event.kind());
      }

    }
    Ok(false)
  }).await.unwrap();

  tracing::info!("Subscription ID: [auto-closing] end {:#?}", sub);
  job_status_submit.await.unwrap();
}