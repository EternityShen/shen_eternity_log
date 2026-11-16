# shen_eternity_log

一个轻量级的 Rust 异步日志库，基于 Tokio 实现。提供简单的文件日志写入能力，支持日志级别、按条数阈值触发 flush，并带有文件不可用时的降级输出（回退到标准输出）。

## 特性

- 异步 I/O：基于 `tokio` 的异步文件读写，适合在异步运行时中使用。
- 日志级别：内置 `INFO` / `WARN` / `DEBUG` / `ERROR` 四个级别。
- 批量 flush：可配置每写入 N 条日志后自动 flush 一次，减少磁盘 I/O。
- 优雅降级：当目标文件无法打开或写入失败时，自动尝试恢复，仍失败则回退到标准输出，不影响业务流程。
- 时间戳：每条日志自动附带 `年:月:日 时:分` 格式的本地时间戳。
- 追加写入：以 `append` 模式打开文件，不会覆盖已有日志。

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
shen_eternity_log = "0.1.0"
```

或在本地路径引用（例如 monorepo 场景）：

```toml
[dependencies]
shen_eternity_log = { path = "../shen_eternity_log" }
```

## 快速开始

```rust
use shen_eternity_log::log::handle::LogHandle;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    // 创建日志句柄：指定日志文件路径，每 10 条日志自动 flush 一次
    let mut logger = LogHandle::new(
        PathBuf::from("app.log"),
        10,
    ).await;

    logger.log_info("服务启动".to_string()).await;
    logger.log_debug("加载配置文件".to_string()).await;
    logger.log_warn("检测到配置项缺失，使用默认值".to_string()).await;
    logger.log_error("连接数据库失败".to_string()).await;
}
```

输出到 `app.log` 的日志形如：

```
2026:11:16 14:30 [INFO] 服务启动
2026:11:16 14:30 [DEBUG] 加载配置文件
2026:11:16 14:30 [WARN] 检测到配置项缺失，使用默认值
2026:11:16 14:30 [ERROR] 连接数据库失败
```

## 使用说明

### LogHandle

`LogHandle` 是日志库的核心入口，通过 `LogHandle::new(path, flush_log_num)` 创建：

- `path: PathBuf`：日志文件路径。目录需已存在，文件不存在时会自动创建。
- `flush_log_num: u32`：每累计写入多少条日志后触发一次 flush。

写入接口均为异步方法，需要 `.await`：

| 方法 | 说明 |
| --- | --- |
| `log_info(message: String)` | 写入一条 INFO 级别日志 |
| `log_warn(message: String)` | 写入一条 WARN 级别日志 |
| `log_debug(message: String)` | 写入一条 DEBUG 级别日志 |
| `log_error(message: String)` | 写入一条 ERROR 级别日志 |
| `read_all() -> Option<String>` | 读取日志文件的全部内容，文件不可用时返回 `None` |

### 降级与自动恢复

`FileHandle` 在写入异常时不会直接放弃，而是先尝试自动恢复：

- 单次写入失败：打印错误信息到 stderr，并调用 `fix()` 重新打开文件后重试一次。
- 文件句柄缺失或 `fix()` 仍失败：将该条日志打印到 stdout，保证内容不丢失。
- `fix()`：公开的恢复方法，会根据记录的路径重新以追加模式打开日志文件，成功返回 `true`，失败返回 `false`。

因此即使日志文件临时不可用（例如被移动、句柄失效），库会尝试自动重建连接；在彻底无法写入时也会降级到标准输出，业务流程不会被中断。

## 模块结构

```
src/
├── lib.rs            # 库入口，导出 file 与 log 模块
├── log/
│   ├── mod.rs        # log 模块入口
│   ├── handle.rs     # LogHandle：日志写入入口与 flush 策略
│   └── level.rs      # LogLevel：日志级别定义
└── file/
    ├── mod.rs        # file 模块入口
    └── handle.rs     # FileHandle：底层异步文件读写封装
```

## 依赖

- [tokio](https://crates.io/crates/tokio)：异步运行时与异步文件 I/O。
- [chrono](https://crates.io/crates/chrono)：本地时间获取与格式化。

## License

MIT
