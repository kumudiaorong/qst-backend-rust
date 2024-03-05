use xlog::info;

use super::def::{common, daemon};
use crate::config;
use common::Empty;
use daemon::FastExtInfo;
use std::sync::Mutex;
use tonic::Response;
use std::collections::HashMap;
use tonic::Status;
pub struct Main {
    config: config::Config,
    handle: Mutex<HashMap<String, std::process::Child>>,
}

impl Main {
    pub fn new(config: config::Config) -> Self {
        Self {
            config,
            handle: Mutex::new(HashMap::new()),
        }
    }
}

#[tonic::async_trait]
impl daemon::main_server::Main for Main {
    async fn set_up(
        &self,
        _: tonic::Request<Empty>,
    ) -> std::result::Result<Response<daemon::FastConfig>, Status> {
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
        Ok(Response::new(daemon::FastConfig { fexts }))
    }
    async fn get_ext_addr(
        &self,
        request: tonic::Request<daemon::ExtHint>,
    ) -> std::result::Result<Response<daemon::ExtAddr>, Status> {
        let id = request.into_inner().id;
        let binding = self.config.file.lock().unwrap();
        let ext = binding.inner.exts.get(&id);
        let ext = ext.ok_or(Status::not_found("not found"))?;
        let r_info = self.config.runtime.lock().unwrap();
        if let Some(_) = r_info.exts.get(&id) {
            todo!("expire")
        }
        if ext.addr != "" {
            return Ok(Response::new(daemon::ExtAddr {
                addr: ext.addr.clone(),
            }));
        } else if self.handle.lock().unwrap().contains_key(&id) {
            return Err(Status::failed_precondition("wait"));
        } else {
            info!("start app: {:#?}", ext);
            match std::process::Command::new(&ext.exec)
            .current_dir(&ext.dir)
            .args([
                "--id",
                &id,
                "--uri",
                format!("http://127.0.0.1:50001").as_str(),
            ])
            .spawn(){
                Ok(handl) => {
                    self.handle.lock().unwrap().insert(id.clone(), handl);
                    return Err(Status::failed_precondition("retry"));
                }
                Err(e) => {
                    return Err(Status::aborted(format!("start: {}", e)));
                }
            }
        }
    }
}
