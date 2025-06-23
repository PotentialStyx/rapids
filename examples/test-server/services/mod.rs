use std::sync::Arc;

use axum::{body::Bytes, extract::ws::Message};

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

use rapids_rs::{
    codecs::{Codec, DynCodec},
    types::{Header, RPCMetadata, RequestInner, TransportRequestMessage},
    utils::generate_id,
};

pub mod example;

macro_rules! service_map {
    ( $( $name:ident ),* ) => {
        pub struct ServiceMap {
            $(
                pub $name: Arc<$name::Service>,
            )*
        }

        impl ServiceMap {
            pub async fn new(codec: DynCodec) -> anyhow::Result<Self> {
                Ok(ServiceMap {
                    $(
                        $name: Arc::new(<$name::Service as ServiceImpl>::new(codec).await?),
                    )*
                })
            }
        }
    }
}

service_map!(example);

pub trait ServiceImpl {
    fn new(
        codec: DynCodec,
    ) -> impl std::future::Future<Output = anyhow::Result<Self>> + Send + Sync
    where
        Self: Sized;
}

pub(crate) fn payload_to_msg<T: Codec>(
    payload: serde_json::Value,
    metadata: &RPCMetadata,
    codec: T,
) -> anyhow::Result<Message> {
    debug!(
        stream_id = metadata.stream_id,
        to = metadata.client_id,
        "Sent {}",
        payload.as_object().unwrap().get("payload").unwrap()
    );

    let message = TransportRequestMessage {
        header: Header {
            stream_id: metadata.stream_id.clone(),
            control_flags: 0b01000,
            id: generate_id(),
            to: metadata.client_id.clone(),
            from: "SERVER".to_string(),
            seq: metadata.seq,
            ack: 1,
        },
        inner: RequestInner::Request { payload },
    };

    Ok(Message::Binary(Bytes::from_owner(
        codec.encode_to_vec(&message)?,
    )))
}
