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
    async fn set_ext_addr(
        &self,
        request: tonic::Request<extension::ExtAddrHint>,
    ) -> Result<Response<common::Empty>, tonic::Status> {
        debug!("Set ext addr: {:?}", request);
        let inner = request.into_inner();
        let mut binding = self.config.lock().unwrap();
        binding.inner.exts.get_mut(&inner.id).unwrap().addr = inner.addr;
        Ok(Response::new(common::Empty {}))
    }
    async fn set_expire(
        &self,
        request: tonic::Request<extension::ExpireHint>,
    ) -> Result<Response<common::Empty>, tonic::Status> {
        todo!()
    }
}
