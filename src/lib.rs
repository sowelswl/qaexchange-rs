//! # QAEXCHANGE-RS
//!
//! 高性能量化交易所系统 - 基于 QARS 核心架构
//!
//! ## 核心能力
//!
//! - **账户管理**: 开户/入金/出金/查询 (复用 qars::qaaccount)
//! - **订单系统**: 下单/撤单/订单路由 (复用 qars::qaaccount::QAOrder)
//! - **撮合引擎**: 价格时间优先/集合竞价/连续交易 (复用 qars::qamarket::matchengine)
//! - **成交回报**: 实时成交推送/账户更新
//! - **行情推送**: Level1/Level2/逐笔成交 (复用 qars::qadata::broadcast_hub)
//! - **结算系统**: 日终结算/盯市盈亏/强平处理
//! - **风控系统**: 盘前风控/持仓限额/自成交防范
//! - **对外服务**: WebSocket + HTTP API (基于 Actix-web)
//!
//! ## 架构设计
//!
//! ```text
//! 客户端 (WebSocket/HTTP)
//!     ↓
//! Service Layer (service/)
//!     ↓
//! Exchange Core (exchange/)
//!     ↓
//! Matching Engine (matching/) ← 复用 qars::qamarket::matchengine
//!     ↓
//! Account System (core/) ← 复用 qars::qaaccount
//!     ↓
//! Storage (storage/) ← 复用 qars::qaconnector
//! ```
//!
//! ## 性能目标
//!
//! - 订单吞吐量: > 100K orders/sec
//! - 撮合延迟: P99 < 100μs
//! - 行情推送延迟: P99 < 10μs
//! - 并发账户数: > 10,000
//! - 并发订阅者: > 1,000

#![allow(dead_code)]
#![allow(unused_imports)]

// ============================================================================
// 外部依赖
// ============================================================================

// 复用 QARS 核心模块
pub use qars;

// Web 框架
pub use actix;
pub use actix_web;

// 异步运行时
pub use futures;
pub use tokio;

// 并发工具
pub use crossbeam;
pub use dashmap;
pub use parking_lot;
pub use rayon;

// 序列化
pub use serde;
pub use serde_json;

// 时间
pub use chrono;

// 日志
pub use log;

// 错误处理
pub use anyhow;
pub use thiserror;

// UUID
pub use uuid;

// ============================================================================
// 内部模块
// ============================================================================

/// 核心模块 - 复用 qars 账户/订单/持仓系统
pub mod core;

/// 撮合引擎 - 复用 qars 撮合引擎并扩展
pub mod matching;

/// 交易所核心业务逻辑
pub mod exchange;

/// 账户系统（高性能独立版本）
pub mod account;

/// 风控系统
pub mod risk;

/// 行情推送系统
pub mod market;

/// 用户管理系统
pub mod user;

/// 对外服务层 (WebSocket + HTTP)
pub mod service;

/// 持久化存储
pub mod storage;

/// 协议层 (QIFI/TIFI/MIFI)
pub mod protocol;

/// 工具模块
pub mod utils;

/// 通知消息系统
pub mod notification;

// iceoryx2 零拷贝 IPC
pub mod ipc;

// 主从复制系统
/// ⚠️ **当前未接线** —— 4529 行,生产运行时完全不参与。@yutiansut @quantaxis
///
/// **完全孤立** —— main.rs 0 引用,非自身模块 0 引用
///
/// 实测(生产运行 1 小时、92k 订单):日志里 `factor` / `dsl` / `replication` /
/// `Raft` / `leader` / `follower` 全部 **0 次命中**;`nm -C` 在 release 二进制里
/// 找不到这四块的任何符号 —— Rust 已作为死代码优化掉,**运行时零成本**,
/// 代价只在编译时间与维护负担。
///
/// 保留 `pub mod` 而不删除:这些是有价值的未完成工作(主从复制、因子计算、
/// 查询引擎),删掉会丢失设计。但**不要以为它们在工作** ——
/// 接线前先读 sim/patches/APPLY.md 的「已定位但未修」一节。
pub mod replication;

// 查询引擎系统 (Phase 8)
/// ⚠️ **当前未接线** —— 2908 行,生产运行时完全不参与。@yutiansut @quantaxis
///
/// 1 处外部引用(storage/hybrid/batch_source.rs:18),
/// 而那条分支**永不进入** —— `olap_cutoff_timestamp` 恒为 0、
/// `refresh_olap_files` 零调用
///
/// 实测(生产运行 1 小时、92k 订单):日志里 `factor` / `dsl` / `replication` /
/// `Raft` / `leader` / `follower` 全部 **0 次命中**;`nm -C` 在 release 二进制里
/// 找不到这四块的任何符号 —— Rust 已作为死代码优化掉,**运行时零成本**,
/// 代价只在编译时间与维护负担。
///
/// 保留 `pub mod` 而不删除:这些是有价值的未完成工作(主从复制、因子计算、
/// 查询引擎),删掉会丢失设计。但**不要以为它们在工作** ——
/// 接线前先读 sim/patches/APPLY.md 的「已定位但未修」一节。
pub mod query;

// 因子计算系统 (流批一体化)
/// ⚠️ **当前未接线** —— 6258 行,生产运行时完全不参与。@yutiansut @quantaxis
///
/// 5 处外部引用,但**引用者本身都是死的**:
///   · dsl —— 自身孤立
///   · kline_actor —— `enable_factor_compute` 默认 false(:72),
///     且 set 它的 setter(:92) 生产零调用
///   · unified_recovery —— 生产零调用(审查已确认)
///
/// 实测(生产运行 1 小时、92k 订单):日志里 `factor` / `dsl` / `replication` /
/// `Raft` / `leader` / `follower` 全部 **0 次命中**;`nm -C` 在 release 二进制里
/// 找不到这四块的任何符号 —— Rust 已作为死代码优化掉,**运行时零成本**,
/// 代价只在编译时间与维护负担。
///
/// 保留 `pub mod` 而不删除:这些是有价值的未完成工作(主从复制、因子计算、
/// 查询引擎),删掉会丢失设计。但**不要以为它们在工作** ——
/// 接线前先读 sim/patches/APPLY.md 的「已定位但未修」一节。
pub mod factor;

// 集群管理系统 (一致性哈希)
pub mod cluster;

// DSL 解析系统
/// ⚠️ **当前未接线** —— 1641 行,生产运行时完全不参与。@yutiansut @quantaxis
///
/// **完全孤立** —— 唯一的外部引用是它自己引用 factor
///
/// 实测(生产运行 1 小时、92k 订单):日志里 `factor` / `dsl` / `replication` /
/// `Raft` / `leader` / `follower` 全部 **0 次命中**;`nm -C` 在 release 二进制里
/// 找不到这四块的任何符号 —— Rust 已作为死代码优化掉,**运行时零成本**,
/// 代价只在编译时间与维护负担。
///
/// 保留 `pub mod` 而不删除:这些是有价值的未完成工作(主从复制、因子计算、
/// 查询引擎),删掉会丢失设计。但**不要以为它们在工作** ——
/// 接线前先读 sim/patches/APPLY.md 的「已定位但未修」一节。
pub mod dsl;

// 性能优化模块 (Phase 5.2)
pub mod perf;

// 可观测性模块 (Prometheus)
pub mod observability;

// ============================================================================
// 重导出常用类型
// ============================================================================

// 从 qars 重导出核心类型
pub use qars::qaaccount::account::QA_Account;
pub use qars::qaaccount::order::{QAOrder, QAOrderExt};
pub use qars::qaaccount::position::QA_Position;

// 从 qars 重导出协议类型
pub use qars::qaprotocol::mifi;
pub use qars::qaprotocol::qifi::account::{Account, Order, Position, Trade, QIFI};
pub use qars::qaprotocol::tifi::{ReqCancel, ReqLogin, ReqOrder};

// 从 qars 重导出撮合引擎
pub use qars::qamarket::matchengine::{
    domain::{OrderDirection, OrderType},
    orderbook::{Failed, Orderbook, Success, TradingState},
};

// 从 qars 重导出数据广播
pub use qars::qadata::broadcast_hub::{BroadcastConfig, DataBroadcaster, MarketDataType};

// ============================================================================
// 全局错误类型
// ============================================================================

/// 交易所错误类型
#[derive(Debug, thiserror::Error)]
pub enum ExchangeError {
    #[error("User error: {0}")]
    UserError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Account error: {0}")]
    AccountError(String),

    #[error("Order error: {0}")]
    OrderError(String),

    #[error("Matching error: {0}")]
    MatchingError(String),

    #[error("Risk check failed: {0}")]
    RiskCheckFailed(String),

    #[error("Settlement error: {0}")]
    SettlementError(String),

    #[error("Instrument error: {0}")]
    InstrumentError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Service error: {0}")]
    ServiceError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("IO error: {0}")]
    IOError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Insufficient balance: {0}")]
    InsufficientBalance(String),
}

pub type Result<T> = std::result::Result<T, ExchangeError>;

// ============================================================================
// 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imports() {
        // 验证可以正确导入 qars 模块
        let _account_type = std::any::type_name::<QA_Account>();
        let _order_type = std::any::type_name::<QAOrder>();
    }
}
