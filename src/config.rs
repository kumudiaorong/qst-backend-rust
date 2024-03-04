use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use xcfg::File as XFile;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtInfo {
    pub name: String,
    pub prompt: String,
    pub dir: String,
    pub exec: String,
    pub addr: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtRuntimeInfo {
    pub expire: HashMap<String, i32>,
}
impl Default for ExtRuntimeInfo {
    fn default() -> Self {
        Self {
            expire: HashMap::new(),
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub exts: std::collections::HashMap<String, ExtInfo>,
}
impl Default for Info {
    fn default() -> Self {
        Self {
            exts: std::collections::HashMap::new(),
        }
    }
}
pub type Store = Arc<Mutex<XFile<Info>>>;
pub type Runtime = Arc<Mutex<ExtRuntimeInfo>>;
#[derive(Clone)]
pub struct Config {
    pub store: Store,
    pub runtime: Runtime,
}

pub fn init() -> (Config, Pin<Box<dyn Future<Output = ()>>>) {
    let path = dirs::home_dir().unwrap().join(".config/qst/backend.toml");
    let mut file: XFile<Info> = XFile::new().path(path.to_str().unwrap());
    let _ = file.load();
    let _ = file.save();
    let file = Arc::new(Mutex::new(file));
    let move_file = file.clone();
    let signal = async move {
        let saver = xcfg::keep::Saver::new(move_file);
        loop {
            match saver.run() {
                Ok(xcfg::keep::Action::TermSave) => {
                    break;
                }
                _ => {}
            }
        }
    };
    (
        Config {
            store: file.clone(),
            runtime: Arc::new(Mutex::new(ExtRuntimeInfo::default())),
        },
        Box::pin(signal),
    )
}
