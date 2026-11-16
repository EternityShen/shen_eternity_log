use crate::file::handle::FileHandle;
use crate::log::level::LogLevel;
use std::path::PathBuf;

pub fn get_now_time() -> String {
    let now = chrono::Local::now();
    now.format("%Y:%m:%d %H:%M").to_string()
}

pub struct LogHandle {
    file_handle: FileHandle,
    log_num: u32,
    max_num: u32,
}

impl LogHandle {
    pub async fn new(path: PathBuf, flush_log_num: u32) -> Self {
        let handle = FileHandle::new(path).await;
        Self {
            file_handle: handle,
            log_num: 0,
            max_num: flush_log_num,
        }
    }

    pub async fn read_all(&mut self) -> Option<String> {
        self.file_handle.read_all().await
    }

    async fn log(&mut self, level: LogLevel, message: String) {
        self.log_num += 1;
        let text = format!("{} [{}] {}\n", get_now_time(), level.to_string(), message);
        self.file_handle.write(&text).await;
        if self.log_num >= self.max_num {
            self.log_num = 0;
            self.file_handle.flush().await;
        }
    }

    pub async fn log_info(&mut self, message: String) {
        self.log(LogLevel::INFO, message).await;
    }

    pub async fn log_error(&mut self, message: String) {
        self.log(LogLevel::ERROR, message).await;
    }

    pub async fn log_debug(&mut self, message: String) {
        self.log(LogLevel::DEBUG, message).await;
    }

    pub async fn log_warn(&mut self, message: String) {
        self.log(LogLevel::WARN, message).await;
    }
}
