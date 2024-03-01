use super::def::common;
use super::def::extension;
use crate::config;
use xlog_rs::log;
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
        request: tonic::Request<extension::ExtAddrWithId>,
    ) -> std::result::Result<tonic::Response<common::Empty>, tonic::Status> {
        log::debug(format!("Set ext addr: {:?}", request).as_str());
        let inner = request.into_inner();
        let mut binding = self.config.lock().unwrap();
        binding.inner.exts.get_mut(&inner.id).unwrap().addr = inner.addr;
        Ok(tonic::Response::new(common::Empty {}))
    }
}
