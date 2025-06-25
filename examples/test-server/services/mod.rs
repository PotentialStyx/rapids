use std::sync::Arc;

use kanal::AsyncSender;
use rapids_rs::{
    types::{OutgoingMessage, RPCMetadata},
    utils,
};
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

pub mod adder;

macro_rules! service_map {
    ( $( $name:ident ),* ) => {
        pub struct ServiceMap {
            $(
                pub $name: Arc<$name::Service>,
            )*
        }

        impl ServiceMap {
            pub async fn new() -> anyhow::Result<Self> {
                Ok(ServiceMap {
                    $(
                        $name: Arc::new(<$name::Service as ServiceImpl>::new().await?),
                    )*
                })
            }
        }
    }
}

service_map!(adder);

pub trait ServiceImpl {
    fn new() -> impl std::future::Future<Output = anyhow::Result<Self>> + Send + Sync
    where
        Self: Sized;

    async fn send_payload(
        channel: AsyncSender<OutgoingMessage>,
        payload: serde_json::Value,
        metadata: &RPCMetadata,
    ) -> anyhow::Result<()> {
        let message = serde_json::json!({ "ok": true, "payload": payload });

        channel
            .send(utils::payload_to_msg(
                rapids_rs::types::ProcedureRes::Response(message),
                metadata,
                false,
                false,
            ))
            .await
            .expect("TODO: handle this");

        Ok(())
    }
}
