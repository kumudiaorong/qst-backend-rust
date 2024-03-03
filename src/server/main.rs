use xlog_rs::log;

use super::def::{common, daemon};
use crate::config;
use common::Empty;
use daemon::ExtInfo;
use daemon::FastExtInfo;
use std::sync::Mutex;
pub struct Main {
    config: config::Config,
    handle: Mutex<std::collections::HashMap<String, std::process::Child>>,
}

impl Main {
    pub fn new(config: config::Config) -> Self {
        {
            let mut binding = config.lock().unwrap();
            binding.inner.exts.insert(
                "test".to_string(),
                ExtInfo {
                    name: "qst-e-a".to_string(),
                    prompt: "a".to_string(),
                    dir: "/home/kmdr/pro/qst-ext-appsearcher-rust/target/debug".to_string(),
                    exec: "/home/kmdr/pro/qst-ext-appsearcher-rust/target/debug/qst-e-a"
                        .to_string(),
                    addr: "".to_string(),
                },
            );
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
    ) -> std::result::Result<tonic::Response<daemon::FastConfig>, tonic::Status> {
        let binding = self.config.lock().unwrap();
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
        log::info(format!("get config: {:#?}", fexts).as_str());
        Ok(tonic::Response::new(daemon::FastConfig { fexts }))
    }
    async fn get_ext_addr(
        &self,
        request: tonic::Request<daemon::ExtId>,
    ) -> std::result::Result<tonic::Response<daemon::ExtAddr>, tonic::Status> {
        let id = request.into_inner().id;
        let binding = self.config.lock().unwrap();
        let app = binding.inner.exts.get(&id);
        if let Some(app) = app {
            if app.addr != "" {
                return Ok(tonic::Response::new(daemon::ExtAddr {
                    addr: app.addr.clone(),
                }));
            } else if self.handle.lock().unwrap().contains_key(&id) {
                return Err(tonic::Status::failed_precondition("wait"));
            } else {
                log::info(format!("start app: {:#?}", app).as_str());
                let handl = std::process::Command::new(&app.exec)
                    .current_dir(&app.dir)
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
        return Err(tonic::Status::not_found("not found"));
    }
}
