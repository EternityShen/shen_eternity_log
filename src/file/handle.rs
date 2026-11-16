use std::path::PathBuf;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct FileHandle {
    pub path: Option<PathBuf>,
    pub handle: Option<File>,
}

impl FileHandle {
    pub async fn new(path: PathBuf) -> Self {
        match Self::open(&path).await {
            Some(file) => Self {
                path: Some(path),
                handle: Some(file),
            },
            None => Self {
                path: Some(path),
                handle: None,
            },
        }
    }

    /// 打开日志文件（不存在则创建，追加写入）。
    /// 供 `new` 和 `fix` 复用，失败时打印错误并返回 None。
    async fn open(path: &PathBuf) -> Option<File> {
        match OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(path)
            .await
        {
            Ok(file) => Some(file),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }

    pub async fn fix(&mut self) -> bool {
        let Some(path) = self.path.as_ref() else {
            return false;
        };
        match Self::open(path).await {
            Some(file) => {
                self.handle = Some(file);
                true
            }
            None => {
                self.handle = None;
                false
            }
        }
    }

    pub async fn write(&mut self, text: &str) {
        match self.handle.as_mut() {
            Some(handle) => match handle.write_all(text.as_bytes()).await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                    self.recover(text).await;
                }
            },
            None => {
                self.recover(text).await;
            }
        }
    }

    async fn recover(&mut self, text: &str) {
        if self.fix().await {
            if let Some(handle) = self.handle.as_mut() {
                if let Err(e) = handle.write_all(text.as_bytes()).await {
                    eprintln!("{}", e);
                    println!("{}", text);
                }
                return;
            }
        }
        println!("{}", text);
    }

    pub async fn flush(&mut self) {
        if let Some(handle) = self.handle.as_mut() {
            let result = handle.flush().await;

            match result {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
    }

    pub async fn read_all(&mut self) -> Option<String> {
        match self.handle.as_mut() {
            Some(handle) => {
                let mut buffer = String::new();
                let _ = handle.read_to_string(&mut buffer);
                Some(buffer)
            }
            None => None,
        }
    }
}
