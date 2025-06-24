use std::sync::Arc;

use kanal::AsyncSender;
use rapids_rs::{
    dispatch::ServiceHandler,
    types::{OutgoingMessage, RPCMetadata},
};
use std::marker::PhantomData;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

pub mod adder;

macro_rules! service_map {
    ( $( $name:ident ),* ) => {
        pub struct ServiceMap<T: ServiceHandler> {
            $(
                pub $name: Arc<$name::Service>,
                _hidden: PhantomData<T>,
            )*
        }

        impl<T: ServiceHandler> ServiceMap<T> {
            pub async fn new() -> anyhow::Result<Self> {
                Ok(ServiceMap {
                    $(
                        $name: Arc::new(<$name::Service as ServiceImpl<T>>::new().await?),
                    )*
                    _hidden: PhantomData {},
                })
            }
        }
    }
}

service_map!(adder);

pub trait ServiceImpl<T: ServiceHandler> {
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
            .send(<T>::payload_to_msg(
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
