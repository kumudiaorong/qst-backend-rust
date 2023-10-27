use serde::{Deserialize, Serialize};
use xlog_rs::log;
#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub exts: std::collections::HashMap<String, super::server::ExtInfo>,
}

#[derive(Debug)]
pub struct Config {
    last_update: Option<std::time::SystemTime>,
    pub by_prompt: std::collections::HashMap<String, String>,
    pub file: File,
}
impl Config {
    pub fn new() -> Config {
        let dir = std::env!("HOME").to_string() + "/.config/qst";
        let (sf, f) = match std::fs::File::open(dir.clone() + "/backend.yaml") {
            Ok(f) => {
                let sf = f;
                let f: File = serde_yaml::from_reader(&sf).unwrap();
                (sf, f)
            }
            Err(_) => {
                std::fs::create_dir_all(&dir)
                    .unwrap_or_else(|_| todo!("Failed to create ~/.config/qst/ directory"));
                let sf = std::fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(dir.clone() + "/backend.yaml")
                    .unwrap_or_else(|_| todo!("Failed to create ~/.config/qst/backend.yaml file"));
                let f = File {
                    exts: std::collections::HashMap::new(),
                };
                serde_yaml::to_writer(&sf, &f).unwrap_or_else(|_| {
                    todo!("Failed to create default ~/.config/qst/backend.yaml file")
                });
                (sf, f)
            }
        };
        Self {
            last_update: sf.metadata().unwrap().modified().map_or(None, |t| Some(t)),
            by_prompt: f
                .exts
                .iter()
                .map(|(k, v)| (v.prompt.clone(), k.clone()))
                .collect(),
            file: f,
        }
    }
    fn file() -> std::fs::File {
        let dir = std::env!("HOME").to_string() + "/.config/qst";
        std::fs::File::open(dir.clone() + "/backend.yaml").unwrap_or_else(|_| {
            std::fs::create_dir_all(&dir)
                .unwrap_or_else(|_| todo!("Failed to create ~/.config/qst/ directory"));
            let f = std::fs::File::create(dir.clone() + "/backend.yaml")
                .unwrap_or_else(|_| todo!("Failed to create ~/.config/qst/backend.yaml file"));
            serde_yaml::to_writer(
                &f,
                &File {
                    exts: std::collections::HashMap::new(),
                },
            )
            .unwrap_or_else(|_| todo!("Failed to create default ~/.config/qst/backend.yaml file"));
            f
        })
    }
    pub fn update(&mut self) {
        let dir = std::env!("HOME").to_string() + "/.config/qst";
        match std::fs::File::open(dir.clone() + "/backend.yaml") {
            Ok(f) => {
                if let Some(t) = self.last_update {
                    if t == f.metadata().unwrap().modified().unwrap() {
                        return;
                    }
                }
                self.last_update = f.metadata().unwrap().modified().map_or(None, |t| Some(t));
                let f: File = serde_yaml::from_reader(&f).unwrap();
                self.by_prompt = f
                    .exts
                    .iter()
                    .map(|(k, v)| (v.prompt.clone(), k.clone()))
                    .collect();
                self.file = f;
            }
            Err(_) => {
                std::fs::create_dir_all(&dir)
                    .unwrap_or_else(|_| todo!("Failed to create ~/.config/qst/ directory"));
                let sf = std::fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(dir.clone() + "/backend.yaml")
                    .unwrap_or_else(|_| todo!("Failed to create ~/.config/qst/backend.yaml file"));
                let f = File {
                    exts: std::collections::HashMap::new(),
                };
                serde_yaml::to_writer(&sf, &f).unwrap_or_else(|_| {
                    todo!("Failed to create default ~/.config/qst/backend.yaml file")
                });
                self.last_update = sf.metadata().unwrap().modified().map_or(None, |t| Some(t));
                self.by_prompt = f
                    .exts
                    .iter()
                    .map(|(k, v)| (v.prompt.clone(), k.clone()))
                    .collect();
                self.file = f;
            }
        }
        // self.last_update = std::time::SystemTime::now();
        // self.by_prompt = f.exts.iter().map(|(k, v)| (v.prompt.clone(), *k)).collect();
        // self.file = f;
    }
    pub fn save(&self) {
        let dir = std::env!("HOME").to_string() + "/.config/qst";
        let f = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(dir.clone() + "/backend.yaml")
            .unwrap_or_else(|_| todo!("Failed to create ~/.config/qst/backend.yaml file"));
        serde_yaml::to_writer(&f, &self.file)
            .unwrap_or_else(|_| todo!("Failed to create default ~/.config/qst/backend.yaml file"));
    }
}
