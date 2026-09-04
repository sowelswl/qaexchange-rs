//! WAL恢复管理器
//!
//! **职责**:
//! 1. 从WAL文件恢复账户状态
//! 2. 重放AccountOpen、AccountUpdate事件
//! 3. 支持增量恢复（从最后一个checkpoint开始）
//!
//! **恢复流程**:
//! ```text
//! 启动时
//!   ↓
//! RecoveryManager::recover()
//!   ↓
//! 1. 读取所有WAL文件
//! 2. 按sequence排序
//! 3. 重放WalRecord:
//!    - AccountOpen → 创建账户
//!    - AccountUpdate → 更新账户余额
//!    - OrderInsert → 恢复订单（可选）
//!    - TradeExecuted → 恢复成交（可选）
//!   ↓
//! 4. 返回恢复的账户列表
//! ```

use crate::core::account_ext::{AccountType, OpenAccountRequest};
use crate::exchange::account_mgr::AccountManager;
use crate::storage::wal::manager::WalManager;
use crate::storage::wal::record::WalRecord;
use crate::ExchangeError;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

/// WAL恢复管理器
pub struct RecoveryManager {
    /// 存储根目录（**不是** WAL 目录）@yutiansut @quantaxis
    ///
    /// 账户 WAL 的真实位置是 `{storage_base}/__ACCOUNT__/wal`，由
    /// `OltpHybridStorage::create` 写出（`hybrid/oltp.rs:112` join instrument_id,
    /// `:117` join "wal"）。此前这里叫 `wal_dir` 且由 main.rs 传入
    /// `{storage_base}/wal`，拼出来是 `{storage_base}/wal/__ACCOUNT__` ——
    /// 两段顺序颠倒，该目录从不存在，账户 WAL 恢复永久空转。
    storage_base: String,
}

impl RecoveryManager {
    /// 创建恢复管理器
    ///
    /// `storage_base` 是存储根目录，例如 `.../output/qaexchange/storage`。
    pub fn new(storage_base: impl Into<String>) -> Self {
        Self {
            storage_base: storage_base.into(),
        }
    }

    /// 从WAL恢复所有账户
    ///
    /// # 参数
    /// - `account_mgr`: 账户管理器
    ///
    /// # 返回
    /// - `Ok(count)`: 恢复的账户数量
    /// - `Err(e)`: 恢复失败
    pub fn recover(&self, account_mgr: &AccountManager) -> Result<usize, ExchangeError> {
        let account_wal_dir = format!("{}/__ACCOUNT__/wal", self.storage_base);
        let wal_path = Path::new(&account_wal_dir);

        if !wal_path.exists() {
            log::info!(
                "No WAL directory found at {}, skipping WAL recovery",
                account_wal_dir
            );
            return Ok(0);
        }

        log::info!("Starting WAL recovery from {}", account_wal_dir);

        // 账户状态缓存
        let mut account_states: HashMap<String, AccountState> = HashMap::new();

        // 使用WalManager的replay方法重放所有WAL记录
        let wal_manager = WalManager::new(&account_wal_dir);

        wal_manager
            .replay(|entry| {
                if let Err(e) = self.apply_record(entry.sequence, entry.record, &mut account_states)
                {
                    log::error!("Failed to apply WAL record {}: {}", entry.sequence, e);
                }
                Ok(())
            })
            .map_err(|e| ExchangeError::StorageError(format!("WAL replay failed: {}", e)))?;

        // 从账户状态恢复到AccountManager
        let recovered_count = self.restore_accounts(account_mgr, account_states)?;

        log::info!(
            "✅ WAL recovery completed: {} accounts recovered",
            recovered_count
        );
        Ok(recovered_count)
    }

    /// 应用单条WAL记录
    fn apply_record(
        &self,
        sequence: u64,
        record: WalRecord,
        account_states: &mut HashMap<String, AccountState>,
    ) -> Result<(), ExchangeError> {
        match record {
            WalRecord::AccountOpen {
                account_id,
                user_id,
                account_name,
                init_cash,
                account_type,
                timestamp,
            } => {
                let account_id_str = String::from_utf8_lossy(&account_id)
                    .trim_end_matches('\0')
                    .to_string();
                let user_id_str = String::from_utf8_lossy(&user_id)
                    .trim_end_matches('\0')
                    .to_string();
                let account_name_str = String::from_utf8_lossy(&account_name)
                    .trim_end_matches('\0')
                    .to_string();

                log::debug!(
                    "Replaying AccountOpen: account_id={}, user_id={}, name={}, init_cash={}, sequence={}",
                    account_id_str,
                    user_id_str,
                    account_name_str,
                    init_cash,
                    sequence
                );

                account_states.insert(
                    account_id_str.clone(),
                    AccountState {
                        account_id: account_id_str,
                        user_id: user_id_str,
                        account_name: account_name_str,
                        init_cash,
                        account_type: Self::u8_to_account_type(account_type),
                        created_at: timestamp, // 从WAL恢复创建时间
                        balance: init_cash,
                        available: init_cash,
                        frozen: 0.0,
                        deposit: 0.0,  // 初始入金为0
                        withdraw: 0.0, // 初始出金为0
                        margin: 0.0,
                        last_sequence: sequence,
                    },
                );
            }

            WalRecord::AccountUpdate {
                user_id,
                balance,
                available,
                frozen,
                margin,
                timestamp: _,
            } => {
                let user_id_str = String::from_utf8_lossy(&user_id)
                    .trim_end_matches('\0')
                    .to_string();

                log::debug!(
                    "Replaying AccountUpdate: user_id={}, balance={}, sequence={}",
                    user_id_str,
                    balance,
                    sequence
                );

                if let Some(state) = account_states.get_mut(&user_id_str) {
                    // 只应用比当前sequence更新的记录
                    if sequence > state.last_sequence {
                        state.balance = balance;
                        state.available = available;
                        state.frozen = frozen;
                        state.margin = margin;
                        state.last_sequence = sequence;
                    }
                } else {
                    log::warn!(
                        "AccountUpdate for unknown user: {} (sequence={}), skipping",
                        user_id_str,
                        sequence
                    );
                }
            }

            WalRecord::OrderInsert { .. } | WalRecord::TradeExecuted { .. } => {
                // 订单和成交记录暂不处理（仅用于审计）
                // 如果需要恢复订单历史，可以在这里实现
            }

            WalRecord::Checkpoint { .. } => {
                // Checkpoint记录用于优化恢复性能（未来实现）
            }

            // 行情记录（恢复时跳过，行情数据无需恢复到内存）
            WalRecord::TickData { .. }
            | WalRecord::OrderBookSnapshot { .. }
            | WalRecord::OrderBookDelta { .. }
            | WalRecord::KLineFinished { .. } => {
                // 行情数据和K线数据不需要恢复到账户状态，仅存档用于历史查询
            }

            // 用户记录（恢复时跳过，用户数据由 UserManager 独立恢复）
            WalRecord::UserRegister { .. } | WalRecord::AccountBind { .. } | WalRecord::UserRoleUpdate { .. } => {
                // 用户数据不需要恢复到账户状态，由 UserManager 独立管理
            }

            // 交易所内部记录（恢复时跳过，仅用于审计和查询）Phase 5
            WalRecord::ExchangeOrderRecord { .. }
            | WalRecord::ExchangeTradeRecord { .. }
            | WalRecord::ExchangeResponseRecord { .. } => {
                // 交易所内部记录不需要恢复到账户状态，仅存档用于历史查询和审计
            }

            // 因子记录（恢复时跳过，因子状态由 FactorEngine 独立恢复）
            WalRecord::FactorUpdate { .. } | WalRecord::FactorSnapshot { .. } => {
                // 因子数据不需要恢复到账户状态，因子计算状态由 FactorEngine 独立管理
                // 可通过 factor/state.rs 的 StateRecovery::load_checkpoint 恢复
            }

            // Phase 14: 订单生命周期恢复记录
            // 这些记录由 unified_recovery.rs 处理，此处跳过
            WalRecord::OrderStatusUpdate { .. }
            | WalRecord::PositionSnapshot { .. }
            | WalRecord::AccountSnapshot { .. } => {
                // 订单状态、持仓快照、账户快照由统一恢复管理器 (UnifiedRecoveryManager) 处理
                // 此 RecoveryManager 仅处理账户基础状态恢复
            }
        }

        Ok(())
    }

    /// 从账户状态恢复到AccountManager
    fn restore_accounts(
        &self,
        account_mgr: &AccountManager,
        account_states: HashMap<String, AccountState>,
    ) -> Result<usize, ExchangeError> {
        let mut restored_count = 0;
        let mut skipped_existing = 0usize;   // 已由快照恢复、WAL 状态按设计跳过的账户数

        for (account_id, state) in account_states {
            // 创建开户请求（使用原始的 account_id）
            let open_req = OpenAccountRequest {
                user_id: state.user_id.clone(),
                account_id: Some(account_id.clone()), // 使用原始 account_id
                account_name: state.account_name.clone(),
                init_cash: state.init_cash,
                account_type: state.account_type,
            };

            // 开户
            match account_mgr.open_account(open_req) {
                Ok(_) => {
                    log::debug!(
                        "Restored account: {} (user={}, balance={})",
                        account_id,
                        state.user_id,
                        state.balance
                    );

                    // 更新账户余额到恢复时的状态
                    if let Err(e) = account_mgr.update_balance_for_recovery(
                        &account_id,
                        state.balance,
                        state.available,
                        state.deposit,
                        state.withdraw,
                    ) {
                        log::error!("Failed to update balance for account {}: {}", account_id, e);
                        continue;
                    }

                    // 更新账户元数据（account_type 和 created_at）
                    if let Err(e) = account_mgr.update_metadata_for_recovery(
                        &account_id,
                        state.account_type,
                        state.created_at,
                    ) {
                        log::error!(
                            "Failed to update metadata for account {}: {}",
                            account_id,
                            e
                        );
                        continue;
                    }

                    restored_count += 1;
                }
                Err(e) => {
                    // ✨ 区分「账户已存在」与真正的失败 @yutiansut @quantaxis
                    //
                    // 启动顺序是:`recover_from_snapshots()`(main.rs:1091)先跑,
                    // 把账户连同**持仓 + 冻结**一起从快照恢复出来;
                    // 随后 `recover_from_wal()`(:1108)走到这里,`open_account()`
                    // 必然返回「已存在」。这是**预期路径**,不是失败。
                    //
                    // 原先一律 `log::error!("Failed to restore account ...")`,
                    // 于是每次启动刷 14 条 ERROR。危害有三:
                    //   ① 读起来像故障,实际是有意的空操作;
                    //   ② 淹没真正的失败(告警疲劳 —— 2026-09-04 排查时
                    //      我自己忽略了它数小时);
                    //   ③ 掩盖了真实语义:「有快照时 WAL 账户恢复是 no-op」。
                    //
                    // ⚠️ 保留 no-op 语义是**有意的**,不要改成「覆盖」:
                    //   WAL 的 `AccountUpdate` 只有 balance/available/frozen/margin,
                    //   **没有持仓**;而快照是自洽的(余额+持仓+冻结同一时点)。
                    //   用较新的 WAL 余额覆盖较旧的快照余额,会把新余额嫁接到旧持仓上。
                    //   快照每 60 秒写一次(main.rs:1077),丢失窗口上限 60 秒,
                    //   两害相权取自洽。
                    //   若要真正修好,正确做法是让快照携带 WAL 序号,
                    //   恢复时从该序号之后重放 —— 属设计变更,需单独评估。
                    if e.to_string().contains("Account already exists") {
                        log::debug!(
                            "Account {} already restored from snapshot, skipping WAL state \
                             (WAL-derived balance={} discarded by design)",
                            account_id,
                            state.balance
                        );
                        skipped_existing += 1;
                    } else {
                        log::error!("Failed to restore account {}: {}", account_id, e);
                    }
                }
            }
        }

        if skipped_existing > 0 {
            log::info!(
                "WAL account restore: {} 个账户已由快照恢复,其 WAL 状态按设计跳过(见 Err 分支注释);\
                 {} 个从 WAL 新建",
                skipped_existing,
                restored_count
            );
        }

        Ok(restored_count)
    }

    /// 将u8转换为AccountType
    fn u8_to_account_type(value: u8) -> AccountType {
        match value {
            0 => AccountType::Individual,
            1 => AccountType::Institutional,
            _ => {
                log::warn!(
                    "Unknown account_type value: {}, defaulting to Individual",
                    value
                );
                AccountType::Individual
            }
        }
    }
}

/// 账户状态（恢复过程中的临时状态）
#[derive(Debug, Clone)]
struct AccountState {
    account_id: String,
    user_id: String,
    account_name: String,
    init_cash: f64,
    account_type: AccountType,
    created_at: i64, // 添加创建时间字段
    balance: f64,
    available: f64,
    frozen: f64,
    deposit: f64,  // 累计入金
    withdraw: f64, // 累计出金
    margin: f64,
    last_sequence: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recovery_manager_creation() {
        let recovery = RecoveryManager::new("/tmp/wal_test");
        assert_eq!(recovery.storage_base, "/tmp/wal_test");
    }
}
