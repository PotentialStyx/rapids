use std::sync::atomic::{AtomicI64, Ordering};

use kanal::AsyncReceiver;

use rapids_rs::{
    codecs::DynCodec,
    types::{IPCMessage, RPCMetadata},
};

use super::ServiceImpl;

pub struct Service {
    state: AtomicI64,
}

impl ServiceImpl for Service {
    async fn new(_: DynCodec) -> anyhow::Result<Service> {
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

    #[allow(clippy::too_many_arguments)]
    pub async fn stream_add(
        &self,
        _: serde_json::Value,
        recv: AsyncReceiver<IPCMessage>,
        _metadata: &RPCMetadata,
    ) -> anyhow::Result<serde_json::Value> {
        // TODO: deal with force close
        while let Ok(IPCMessage::Request(value)) = recv.recv().await {
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
}
