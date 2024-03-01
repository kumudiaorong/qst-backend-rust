use std::{
    sync::{Arc, Mutex},
    thread::spawn,
};

use serde::{Deserialize, Serialize};
use xcfg::File as XFile;
#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub exts: std::collections::HashMap<String, super::server::ExtInfo>,
}
impl Default for Info {
    fn default() -> Self {
        Self {
            exts: std::collections::HashMap::new(),
        }
    }
}
pub type Config = Arc<Mutex<XFile<Info>>>;
pub fn init() -> Config {
    let path = dirs::home_dir().unwrap().join(".config/qst/backend.toml");
    let mut file: XFile<Info> = XFile::new().path(path.to_str().unwrap());
    let _ = file.load();
    let _ = file.save();
    let file = Arc::new(Mutex::new(file));
    let move_file = file.clone();
    spawn(|| {
        let saver = xcfg::keep::Saver::new(move_file);
        loop {
            match saver.run() {
                Ok(xcfg::keep::Action::TermSave) => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    });
    file
}
