use model::JobAnswer;
use nostr::nips::nip19::ToBech32;
use nostr::Keys;

use nostr_sdk::{Client, EventBuilder, Filter, Kind, RelayPoolNotification, SecretKey, Timestamp};
use tokio::sync::mpsc;

use crate::server::server::SharedState;
pub mod model;
pub mod util;
pub async fn subscription_service(
    server: SharedState,
    mut job_status_rx: mpsc::Receiver<JobAnswer>,
    dispatch_task_tx: mpsc::Sender<u32>,
    key: ed25519_dalek::SecretKey,
    relay_url: String,
) -> anyhow::Result<()> {
    // let keys = Keys::from_mnemonic(MNEMONIC_PHRASE, None).unwrap();
    let secret_key =  SecretKey::from_slice(key.as_ref())?;
    let keys = Keys::new(secret_key);

    let bech32_address = keys.public_key().to_bech32().unwrap();

    let client = Client::new(&keys);
    // let client = Client::default();
    client.add_relay(&relay_url).await.unwrap();
    client.connect().await;
    tracing::info!("connect relay {:#?} with {:#?}", &relay_url, bech32_address);
    // let metadata = Metadata::new()
    // .name("aos-dispatcher")
    // .display_name("Aos Dispatcher")
    // .website(Url::parse("https://github.com/hetu-project/aos-dispatcher").unwrap());

    let submit_client = client.clone();
    let job_status_submit = tokio::spawn(async move {
        while let Some(job_status) = job_status_rx.recv().await {
            tracing::info!("job status {:#?}", job_status);
            let event_id = job_status.event_id;
            let origin_event_filter = Filter::new().id(event_id);
            let job_request = submit_client
                .get_events_of(
                    vec![origin_event_filter],
                    nostr_sdk::EventSource::Relays {
                        timeout: None,
                        specific_relays: None,
                    },
                )
                .await;
            if let Ok(job_request) = job_request {
                match job_request.get(0) {
                    Some(jq) => {
                        let event = EventBuilder::job_result(
                            jq.clone(),
                            job_status.answer.clone(),
                            0,
                            None,
                        )
                        .unwrap();
                        tracing::debug!("sending job result to nostr relay, {:#?}", event);
                        if let Err(e) = submit_client.send_event_builder(event).await {
                            tracing::error!("sended job result to nostr relay error{}", e);
                        }
                        tracing::debug!("sended job result to nostr relay");
                    }
                    None => {
                        tracing::error!("There is no event id {:#?} on relay", event_id);
                    }
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

    client.subscribe(vec![subscription], None).await.unwrap();
    tracing::info!("Subscription ID: [auto-closing] start");

    let sub = client
        .handle_notifications(|notification| async {
            tracing::debug!("job notification {:#?}", notification);
            if let RelayPoolNotification::Event { event, .. } = notification {
                // tracing::info!("job notification {:#?}", event);
                if event.kind() == Kind::JobRequest(5050) {
                    // tracing::info!("receive task {:#?}", event);
                    tracing::info!("receive task event {:#?}", event.id());
                    // let uuid = uuid::Uuid::new_v4();
                } else {
                    tracing::info!("JobRequest other {:#?}", event.kind());
                }
            }
            Ok(false)
        })
        .await
        .unwrap();

    tracing::info!("Subscription ID: [auto-closing] end {:#?}", sub);
    job_status_submit.await.unwrap();
    Ok(())
}
