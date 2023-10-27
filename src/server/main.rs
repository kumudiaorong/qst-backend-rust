use xlog_rs::log;

use super::file::{daemon, defs};
use crate::config;
use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};

pub struct Main {
    config: Arc<Mutex<config::Config>>,
    handle: Mutex<std::collections::HashMap<String, std::process::Child>>,
}

impl Main {
    pub fn new(cfg: &Arc<Mutex<config::Config>>) -> Self {
        Self {
            config: Arc::clone(&cfg),
            handle: Mutex::new(std::collections::HashMap::new()),
        }
    }
}

#[tonic::async_trait]
impl daemon::main_server::Main for Main {
    async fn set_up(
        &self,
        _: tonic::Request<defs::Empty>,
    ) -> std::result::Result<tonic::Response<daemon::Prompt2Addr>, tonic::Status> {
        // let mut res = std::collections::HashMap::new();
        // for (_, v) in self.config.lock().file.exts.iter() {
        //     if v.addr != "" {
        //         res.insert(v.prompt.clone(), v.addr.clone());
        //     }
        // }
        // Ok(tonic::Response::new(server::SetUpResult {
        //     mresult: Some(server::set_up_result::Mresult::Ok(
        //         server::set_up_result::MOk { running: res },
        //     )),
        // }))

        Ok(tonic::Response::new(daemon::Prompt2Addr {
            running: self
                .config
                .lock()
                .unwrap()
                .borrow_mut()
                .file
                .exts
                .iter()
                .filter_map(|(id, v)| {
                    if v.addr != "" {
                        Some((
                            v.prompt.clone(),
                            daemon::ExtAddrWithId {
                                id: id.clone(),
                                addr: v.addr.clone(),
                            },
                        ))
                    } else {
                        None
                    }
                })
                .collect(),
        }))
    }
    async fn get_config(
        &self,
        _: tonic::Request<defs::Empty>,
    ) -> std::result::Result<tonic::Response<daemon::Config>, tonic::Status> {
        let exts = self.config.lock().unwrap().file.exts.clone();
        return Ok(tonic::Response::new(daemon::Config { exts }));
    }

    async fn set_config(
        &self,
        request: tonic::Request<daemon::Config>,
    ) -> std::result::Result<tonic::Response<defs::Empty>, tonic::Status> {
        self.config.lock().unwrap().file.exts = request.into_inner().exts;
        return Ok(tonic::Response::new(defs::Empty {}));
    }

    async fn get_ext_addr(
        &self,
        request: tonic::Request<daemon::ExtId>,
    ) -> std::result::Result<tonic::Response<daemon::ExtAddr>, tonic::Status> {
        let binding = self.config.lock().unwrap();
        let id = request.into_inner().id;
        let app = binding.file.exts.get(&id);
        if let Some(app) = app {
            if app.addr != "" {
                return Ok(tonic::Response::new(daemon::ExtAddr {
                    addr: app.addr.clone(),
                }));
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
                return Err(tonic::Status::unavailable("retry"));
            }
        }
        return Err(tonic::Status::not_found("not found"));
    }
}
