use super::def::common;
use super::def::extension;
use crate::config;
use tonic::Response;
use xlog::debug;
pub struct Ext {
    config: config::Config,
}

impl Ext {
    pub fn new(config: config::Config) -> Self {
        Self { config }
    }
}

#[tonic::async_trait]
impl extension::ext_server::Ext for Ext {
    async fn config_ext(
        &self,
        request: tonic::Request<extension::ExtConfigHint>,
    ) -> Result<Response<common::Empty>, tonic::Status> {
        debug!("Set ext addr: {:?}", request);
        let inner = request.into_inner();
        let mut binding = self.config.file.lock().unwrap();
        binding.inner.exts.get_mut(&inner.id).unwrap().addr = inner.addr.clone();
        if let Some(expire) = inner.expire {
            let mut binding = self.config.runtime.lock().unwrap();
            binding
                .exts
                .insert(inner.id, config::ExtRuntimeInfo { expire });
        }
        Ok(Response::new(common::Empty {}))
    }
}
