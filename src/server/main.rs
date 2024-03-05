use xlog::info;

use super::def::{common, daemon};
use crate::config;
use common::Empty;
use daemon::FastExtInfo;
use std::sync::Mutex;
use tonic::Response;
pub struct Main {
    config: config::Config,
    handle: Mutex<std::collections::HashMap<String, std::process::Child>>,
}

impl Main {
    pub fn new(config: config::Config) -> Self {
        {
            // let mut binding = config.lock().unwrap();
            // binding.inner.exts.insert(
            //     "test".to_string(),
            //     ExtInfo {
            //         name: "qst-e-a".to_string(),
            //         prompt: "a".to_string(),
            //         dir: "/home/kmdr/pro/qst-ext-appsearcher-rust/target/debug".to_string(),
            //         exec: "/home/kmdr/pro/qst-ext-appsearcher-rust/target/debug/qst-e-a"
            //             .to_string(),
            //         addr: "".to_string(),
            //     },
            // );
        }
        Self {
            config,
            handle: Mutex::new(std::collections::HashMap::new()),
        }
    }
}

#[tonic::async_trait]
impl daemon::main_server::Main for Main {
    async fn set_up(
        &self,
        _: tonic::Request<Empty>,
    ) -> std::result::Result<Response<daemon::FastConfig>, tonic::Status> {
        let binding = self.config.file.lock().unwrap();
        let fexts = binding
            .inner
            .exts
            .iter()
            .map(|(id, v)| {
                (
                    id.clone(),
                    FastExtInfo {
                        prompt: v.prompt.clone(),
                        addr: if v.addr != "" {
                            Some(v.addr.clone())
                        } else {
                            None
                        },
                    },
                )
            })
            .collect();
        info!("get config: {:#?}", fexts);
        Ok(Response::new(daemon::FastConfig { fexts }))
    }
    async fn get_ext_addr(
        &self,
        request: tonic::Request<daemon::ExtHint>,
    ) -> std::result::Result<Response<daemon::ExtAddr>, tonic::Status> {
        let id = request.into_inner().id;
        let binding = self.config.file.lock().unwrap();
        let ext = binding.inner.exts.get(&id);
        let ext = ext.ok_or(tonic::Status::not_found("not found"))?;
        let r_info = self.config.runtime.lock().unwrap();
        if let Some(_) = r_info.exts.get(&id) {
            todo!("expire")
        }
        if ext.addr != "" {
            return Ok(Response::new(daemon::ExtAddr {
                addr: ext.addr.clone(),
            }));
        } else if self.handle.lock().unwrap().contains_key(&id) {
            return Err(tonic::Status::failed_precondition("wait"));
        } else {
            info!("start app: {:#?}", ext);
            let handl = std::process::Command::new(&ext.exec)
                .current_dir(&ext.dir)
                .args([
                    "--id",
                    &id,
                    "--uri",
                    format!("http://127.0.0.1:50001").as_str(),
                ])
                .spawn()
                .unwrap_or_else(|e| panic!("fail to spawn{}", e));
            self.handle.lock().unwrap().insert(id.clone(), handl);
            return Err(tonic::Status::failed_precondition("retry"));
        }
    }
}
