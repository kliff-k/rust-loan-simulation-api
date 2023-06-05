use azeventhubs::producer::{EventHubProducerClient, EventHubProducerClientOptions, SendEventOptions};
use crate::model::{Hub, RetornoSimulacao};

/// Registra evento em Event Hub
pub async fn envia_evento_hub(payload: &RetornoSimulacao, hub_settings: &Hub){
    let mut producer_client =
        EventHubProducerClient::from_connection_string(
            hub_settings.connection_string.to_owned(),
            hub_settings.hub_name.to_owned(),
            EventHubProducerClientOptions::default()
        ).await.unwrap();

    let event = serde_json::to_string(&payload).unwrap();
    let options = SendEventOptions::new();
    producer_client.send_event(event, options).await.unwrap();

    producer_client.close().await.unwrap();
}