# wp-infras

![CI](https://github.com/wp-labs/wp-infras/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/wp-labs/wp-infras/graph/badge.svg?token=6SVCXBHB6B)](https://codecov.io/gh/wp-labs/wp-infras)
![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)
![Rust](https://img.shields.io/badge/rust-stable%2Bbeta-orange.svg)

## 目录

| Crate | 说明 |
| --- | --- |
| `wp-conf-base` | 抽象出的配置对象操作、标签解析、序列化辅助工具。提供 `ConfStdOperation`、`Validate`、布尔反序列化等通用 trait。|
| `wp-data-fmt` | 数据格式化适配层，输出 JSON/CSV/KV/Raw/ProtoText/SQL 等不同文本格式。包含快照测试覆盖 Nginx 等典型日志样例。|
| `wp-error` | 领域统一的错误类型、系统错误码（SysErrorCode）、HTTP 映射与错误响应生成工具。|
| `wp-log` | 控制台/文件/滚动日志的初始化封装、结构化日志配置及校验。|
| `wp-specs` | Warp Parse 规格、配置样例与集成测试集合。|

## 开发指南

1. **准备环境**：需要 Rust stable toolchain、`cargo`、以及 git LFS（如果要拉取数据文件）。
2. **编译/测试**：在 workspace 根目录执行 `cargo test` 会构建全部 crate；可通过 `-p <crate>` 指定子项目。
3. **格式化**：遵循 rustfmt；如需要额外 lint，可开启 `cargo clippy`。
4. **提交流程**：
   - 变更前同步主分支：`git pull origin main`；
   - 为每个 feature/new fix 创建分支，提交前确保 `cargo fmt`、`cargo test -p <crate>` 均通过；
   - 更新对应 README/CODES 文档，保持契约一致性；
   - 通过 PR 发起代码审查。

## 贡献说明

欢迎 issue/PR。有疑问可先查阅各 crate 的 README；如需新增错误码、格式或配置 trait，记得同步相关文档（`CODES.md`、`README.md`）并添加测试覆盖。
