use std::sync::atomic::{AtomicI64, Ordering};

use kanal::AsyncReceiver;

use rapids_rs::{
    codecs::DynCodec,
    types::{IPCMessage, RPCMetadata},
};

use super::{payload_to_msg, ServiceImpl};

pub struct Service {
    state: AtomicI64,
    codec: DynCodec,
}

impl ServiceImpl for Service {
    async fn new(codec: DynCodec) -> anyhow::Result<Service> {
        Ok(Service {
            state: AtomicI64::new(0),
            codec,
        })
    }
}

impl Service {
    pub async fn add(
        &self,
        payload: serde_json::Value,
        metadata: RPCMetadata,
    ) -> anyhow::Result<()> {
        let amt = payload
            .as_object()
            .unwrap()
            .get("n")
            .unwrap()
            .as_i64()
            .unwrap();

        let res = self.state.fetch_add(amt, Ordering::SeqCst) + amt;

        let return_payload = serde_json::json!({
            "ok": true,
            "payload": {"result": res }
        });

        metadata
            .channel
            .send(payload_to_msg(return_payload, &metadata, self.codec)?)
            .await?;

        Ok(())
    }

    pub async fn reset_count(
        &self,
        payload: serde_json::Value,
        metadata: RPCMetadata,
    ) -> anyhow::Result<()> {
        let amt = payload.as_number().unwrap().as_i64().unwrap();

        self.state.store(amt, Ordering::SeqCst);

        let return_payload = serde_json::json!({
            "ok": true,
            "payload": null
        });

        metadata
            .channel
            .send(payload_to_msg(return_payload, &metadata, self.codec)?)
            .await?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn stream_add(
        &self,
        _: serde_json::Value,
        recv: AsyncReceiver<IPCMessage>,
        metadata: RPCMetadata,
    ) -> anyhow::Result<()> {
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

        let return_payload = serde_json::json!({
            "ok": true,
            "payload": { "result": result }
        });

        metadata
            .channel
            .send(payload_to_msg(return_payload, &metadata, self.codec)?)
            .await?;

        Ok(())
    }
}
