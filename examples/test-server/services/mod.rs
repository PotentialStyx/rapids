use std::sync::Arc;

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

pub mod example;

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

service_map!(example);

pub trait ServiceImpl {
    fn new() -> impl std::future::Future<Output = anyhow::Result<Self>> + Send + Sync
    where
        Self: Sized;
}
