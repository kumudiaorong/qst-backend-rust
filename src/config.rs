use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use xcfg::File as XFile;

mod inner {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ExtInfo {
        pub name: String,
        pub prompt: String,
        pub dir: String,
        pub exec: String,
        pub addr: String,
    }
    #[derive(Debug, Deserialize, Serialize, Default)]
    pub struct Info {
        pub exts: HashMap<String, ExtInfo>,
    }
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ExtRuntimeInfo {
        pub expire: i32,
    }
    #[derive(Debug, Clone, Default)]
    pub struct RuntimeInfo {
        pub exts: HashMap<String, ExtRuntimeInfo>,
    }
}
pub use inner::ExtRuntimeInfo;
use xlog::warn;
#[derive(Debug, Clone)]
pub struct Config {
    pub file: Arc<Mutex<XFile<inner::Info>>>,
    pub runtime: Arc<Mutex<inner::RuntimeInfo>>,
}
impl Config {
    fn from_path(value: PathBuf) -> Result<Self, xcfg::Error> {
        let path = value.to_str().unwrap();
        let mut file: XFile<inner::Info> = XFile::default().path(path);
        let _ = warn!(res, file.load(), "fail to load config");
        file.save()?;
        let file = Arc::new(Mutex::new(file));
        Ok(Self {
            file,
            runtime: Arc::new(Mutex::new(inner::RuntimeInfo::default())),
        })
    }
    pub fn save(self) -> Result<(), xcfg::Error> {
        let mut file = self.file.lock().unwrap();
        let runtime = self.runtime.lock().unwrap();
        file.inner.exts.iter_mut().for_each(|(k, v)| {
            if let Some(expire) = runtime.exts.get(k) {
                if expire.expire < 0 {
                    v.addr.clear();
                }
            } else {
                v.addr.clear();
            }
        });
        file.save()
    }
}

pub fn init() -> Result<Config, xcfg::Error> {
    let path = dirs::home_dir().unwrap().join(".config/qst/backend.toml");
    let config = Config::from_path(path)?;
    Ok(config)
}
