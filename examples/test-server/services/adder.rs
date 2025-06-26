use std::sync::atomic::{AtomicI64, Ordering};

use kanal::{AsyncReceiver, AsyncSender};

use rapids::types::{IncomingMessage, OutgoingMessage, RPCMetadata};

use super::ServiceImpl;
use anyhow::format_err;

pub struct Service {
    state: AtomicI64,
}

impl ServiceImpl for Service {
    async fn new() -> anyhow::Result<Service> {
        Ok(Service {
            state: AtomicI64::new(0),
        })
    }
}

impl Service {
    pub async fn add(
        &self,
        payload: serde_json::Value,
        _metadata: &RPCMetadata,
    ) -> anyhow::Result<serde_json::Value> {
        let amt = payload
            .as_object()
            .unwrap()
            .get("n")
            .unwrap()
            .as_i64()
            .unwrap();

        let res = self.state.fetch_add(amt, Ordering::SeqCst) + amt;

        let return_payload = serde_json::json!({"result": res });
        if amt == 6 {
            return Err(format_err!("test"));
        }

        Ok(return_payload)
    }

    pub async fn reset_count(
        &self,
        payload: serde_json::Value,
        _metadata: &RPCMetadata,
    ) -> anyhow::Result<serde_json::Value> {
        let amt = payload.as_number().unwrap().as_i64().unwrap();

        self.state.store(amt, Ordering::SeqCst);

        let return_payload = serde_json::json!(null);

        Ok(return_payload)
    }

    pub async fn upload_add(
        &self,
        _: serde_json::Value,
        recv: AsyncReceiver<IncomingMessage>,
        _metadata: &RPCMetadata,
    ) -> anyhow::Result<serde_json::Value> {
        // TODO: deal with force close
        while let Ok(IncomingMessage::Request(value)) = recv.recv().await {
            let amt = value
                .as_object()
                .unwrap()
                .get("n")
                .unwrap()
                .as_i64()
                .unwrap();

            self.state.fetch_add(amt, Ordering::SeqCst);
        }

        let result = self.state.load(Ordering::SeqCst);

        let return_payload = serde_json::json!({ "result": result });

        Ok(return_payload)
    }

    pub async fn stream_add(
        &self,
        _: serde_json::Value,
        recv: AsyncReceiver<IncomingMessage>,
        send: AsyncSender<OutgoingMessage>,
        metadata: &RPCMetadata,
    ) -> anyhow::Result<()> {
        // TODO: deal with force close
        while let Ok(IncomingMessage::Request(value)) = recv.recv().await {
            let amt = value
                .as_object()
                .unwrap()
                .get("n")
                .unwrap()
                .as_i64()
                .unwrap();

            let res = self.state.fetch_add(amt, Ordering::SeqCst) + amt;

            Self::send_payload(send.clone(), serde_json::json!({ "result": res }), metadata)
                .await?;
        }

        Ok(())
    }

    pub async fn subscription_add(
        &self,
        payload: serde_json::Value,
        send: AsyncSender<OutgoingMessage>,
        metadata: &RPCMetadata,
    ) -> anyhow::Result<()> {
        let amts = payload.as_array().unwrap();

        for amt in amts {
            let amt = amt.as_i64().unwrap();
            let res = self.state.fetch_add(amt, Ordering::SeqCst) + amt;

            Self::send_payload(send.clone(), serde_json::json!({ "result": res }), metadata)
                .await?;
        }

        Ok(())
    }
}
