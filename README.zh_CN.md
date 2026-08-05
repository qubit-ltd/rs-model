# Qubit Model

[![Rust CI](https://github.com/qubit-ltd/rs-model/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-model/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-model/coverage-badge.json)](https://qubit-ltd.github.io/rs-model/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-model.svg?color=blue)](https://crates.io/crates/qubit-model)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

`qubit-model` 用于存放移植到 Qubit Rust 平台的领域对象。仓库独立于模型元数据运行时和 derive 宏仓库，便于后续领域模型持续演进。

## 适用对象

本仓库面向负责将领域对象迁移到 Rust、或维护 Rust 应用及共享基础设施中领域模型的 Qubit 开发者。

## 安装

当前仓库仍是本地空白骨架，尚未发布到 crates.io。后续添加领域对象后，本地开发可以使用路径依赖：

```toml
[dependencies]
qubit-model = { version = "0.1", path = "../rs-model" }
```

## 当前起点

当前仓库只包含最小 Rust library 结构和标准项目工具，不提供领域类型。

## 计划范围

后续将逐步在这里添加领域对象，并使用 `rs-model-metadata` 与 `rs-model-derive` 提供的共享能力设计模型声明。

## 已知限制

- 当前尚未迁移任何领域对象。
- 当前不提供公共模型 API、持久化映射、校验 API 或生成的 Schema。
- 当前不提供 DAO、Service、随机对象或契约测试实现。

## 测试

```bash
# 使用默认 feature 集运行测试
cargo test

# 使用项目声明的全部 feature 运行测试
cargo test --all-features

# 运行项目 CI 检查
./ci-check.sh

# 检查代码覆盖率
./coverage.sh
```

## 许可证

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

本项目基于 Apache License 2.0 授权。完整许可证文本请参阅
[LICENSE](LICENSE)。

## 贡献

欢迎贡献。请遵循 Rust API 指南，及时更新公共 API 文档与测试，并在提交
Pull Request 前运行 `./align-ci.sh`格式化代码，运行`./ci-check.sh`对齐CI要求。

## 作者

**Haixing Hu** - *Qubit Co. Ltd.*

仓库地址：[https://github.com/qubit-ltd/rs-model](https://github.com/qubit-ltd/rs-model)
