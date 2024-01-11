use super::def::extension;
use super::def::{common, daemon};
use crate::config;
use daemon::ExtInfo;
use std::sync::{Arc, Mutex};
use xlog_rs::log;
pub struct Ext {
    config: Arc<Mutex<config::Config>>,
}

impl Ext {
    pub fn new(cfg: &Arc<Mutex<config::Config>>) -> Self {
        Self {
            config: Arc::clone(&cfg),
        }
    }
}

#[tonic::async_trait]
impl extension::ext_server::Ext for Ext {
    async fn set_ext_addr(
        &self,
        request: tonic::Request<extension::ExtAddrWithId>,
    ) -> std::result::Result<tonic::Response<common::Empty>, tonic::Status> {
        log::debug(format!("Set ext addr: {:?}", request).as_str());
        let mut binding = self.config.lock().unwrap();
        let inner = request.into_inner();
        binding.file.exts.get_mut(&inner.id).unwrap().addr = inner.addr;
        Ok(tonic::Response::new(common::Empty {}))
    }
}
