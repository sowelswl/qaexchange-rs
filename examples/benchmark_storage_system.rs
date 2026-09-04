//! 存储系统压力测试
//!
//! 测试场景：
//! 1. 百万级 WAL 写入
//! 2. MemTable 读写性能
//! 3. SSTable 查询性能
//! 4. 零拷贝分发性能
//! 5. 故障恢复性能
//!
//! 运行: cargo run --release --example benchmark_storage_system

// 基础类型（编译需要）
#[allow(unused_imports)]
use std::time::Duration;

// TODO: Phase 1-7 实现后启用这些 import
// use std::time::Instant;
// use std::sync::Arc;
// use std::sync::atomic::{AtomicU64, Ordering};

// TODO: 这些模块将在 Phase 1-7 实现
// use qaexchange::storage::wal::{WalManager, WalRecord};
// use qaexchange::storage::memtable::MemTableManager;
// use qaexchange::storage::sstable::{SSTableBuilder, SSTableReader};
// use qaexchange::distribution::{DistributionPublisher, DistributionSubscriber, DistributionMessage};
// use qaexchange::storage::recovery::WalRecovery;

const TOTAL_OPERATIONS: usize = 1_000_000;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║           存储系统压力测试 - 性能验证                          ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    println!("测试配置:");
    println!("  • 操作数量: {:>12}", format_number(TOTAL_OPERATIONS));
    println!();

    // TODO: Phase 1-7 实现后依次启用

    // Phase 1: WAL 测试
    // benchmark_wal();

    // Phase 2: MemTable 测试
    // benchmark_memtable();

    // Phase 2: SSTable 测试
    // benchmark_sstable();

    // Phase 4: 分发测试
    // benchmark_distribution();

    // Phase 5: 恢复测试
    // benchmark_recovery();

    // 当前占位符测试
    placeholder_benchmark();

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ 所有测试完成");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

// TODO: Phase 1 实现后启用
/*
fn benchmark_wal() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 Phase 1: WAL 性能测试");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let wal = WalManager::new("/home/quantaxis/qaexchange-rs/output//bench_wal");

    // 单条写入测试
    println!("1️⃣ 单条写入测试 ({} 次)", format_number(TOTAL_OPERATIONS));
    let mut latencies = Vec::with_capacity(TOTAL_OPERATIONS);

    let start = Instant::now();

    for i in 0..TOTAL_OPERATIONS {
        let record = WalRecord::OrderInsert {
            order_id: i.to_le_bytes().try_into().unwrap(),
            user_id: [0u8; 40],
            instrument_id: [0u8; 16],
            direction: (i % 2) as u8,
            offset: 0,
            price: 100.0 + (i % 10) as f64,
            volume: 10.0,
            timestamp: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0),
        };

        let op_start = Instant::now();
        wal.append(record).unwrap();
        latencies.push(op_start.elapsed().as_micros() as f64);

        if (i + 1) % 100_000 == 0 {
            println!("  进度: {}/{}", format_number(i + 1), format_number(TOTAL_OPERATIONS));
        }
    }

    let duration = start.elapsed();
    print_latency_stats("WAL 单条写入", &latencies, duration, TOTAL_OPERATIONS);

    // 批量写入测试
    println!("\n2️⃣ 批量写入测试 (100 条/批)");
    let batch_size = 100;
    let num_batches = TOTAL_OPERATIONS / batch_size;

    let start = Instant::now();

    for i in 0..num_batches {
        let mut batch = Vec::with_capacity(batch_size);

        for j in 0..batch_size {
            let idx = i * batch_size + j;
            batch.push(WalRecord::OrderInsert {
                order_id: idx.to_le_bytes().try_into().unwrap(),
                user_id: [0u8; 40],
                instrument_id: [0u8; 16],
                direction: (idx % 2) as u8,
                offset: 0,
                price: 100.0 + (idx % 10) as f64,
                volume: 10.0,
                timestamp: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0),
            });
        }

        wal.append_batch(batch).unwrap();

        if (i + 1) % 1000 == 0 {
            println!("  进度: {}/{} 批", format_number(i + 1), format_number(num_batches));
        }
    }

    let duration = start.elapsed();
    let throughput = TOTAL_OPERATIONS as f64 / duration.as_secs_f64();

    println!("\n结果:");
    println!("  • 总耗时:   {:>10.2} s", duration.as_secs_f64());
    println!("  • 吞吐量:   {:>10} ops/s", format_number(throughput as usize));
    println!("  • 批量加速: {:>10.2}x", throughput / (TOTAL_OPERATIONS as f64 / latencies.len() as f64));

    // 清理
    std::fs::remove_dir_all("/home/quantaxis/qaexchange-rs/output//bench_wal").ok();
}
*/

// TODO: Phase 2 实现后启用
/*
fn benchmark_memtable() {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("💾 Phase 2: MemTable 性能测试");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let memtable = MemTableManager::new(1024 * 1024 * 1024);  // 1GB

    // 插入测试
    println!("1️⃣ 插入测试 ({} 次)", format_number(TOTAL_OPERATIONS));
    let mut insert_latencies = Vec::with_capacity(TOTAL_OPERATIONS);

    let start = Instant::now();

    for i in 0..TOTAL_OPERATIONS {
        let key = format!("order_{:010}", i).into_bytes();
        let value = vec![0u8; 100];  // 100 字节 value

        let op_start = Instant::now();
        memtable.insert(key, value).unwrap();
        insert_latencies.push(op_start.elapsed().as_micros() as f64);

        if (i + 1) % 100_000 == 0 {
            println!("  进度: {}/{}", format_number(i + 1), format_number(TOTAL_OPERATIONS));
        }
    }

    let duration = start.elapsed();
    print_latency_stats("MemTable 插入", &insert_latencies, duration, TOTAL_OPERATIONS);

    // 查询测试
    println!("\n2️⃣ 查询测试 ({} 次)", format_number(TOTAL_OPERATIONS));
    let mut get_latencies = Vec::with_capacity(TOTAL_OPERATIONS);

    let start = Instant::now();

    for i in 0..TOTAL_OPERATIONS {
        let key = format!("order_{:010}", i % TOTAL_OPERATIONS).into_bytes();

        let op_start = Instant::now();
        let _ = memtable.get(&key);
        get_latencies.push(op_start.elapsed().as_micros() as f64);

        if (i + 1) % 100_000 == 0 {
            println!("  进度: {}/{}", format_number(i + 1), format_number(TOTAL_OPERATIONS));
        }
    }

    let duration = start.elapsed();
    print_latency_stats("MemTable 查询", &get_latencies, duration, TOTAL_OPERATIONS);

    // 内存占用
    println!("\n内存占用:");
    println!("  • MemTable 大小: {:>10} MB", memtable.size() / 1024 / 1024);
}
*/

// TODO: Phase 2 实现后启用
/*
fn benchmark_sstable() {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("💿 Phase 2: SSTable 性能测试");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // 构建 SSTable
    println!("1️⃣ 构建 SSTable ({} 条)", format_number(TOTAL_OPERATIONS));

    let start = Instant::now();
    let mut builder = SSTableBuilder::new("/home/quantaxis/qaexchange-rs/output//bench.sst");

    for i in 0..TOTAL_OPERATIONS {
        let key = format!("order_{:010}", i).into_bytes();
        let value = vec![0u8; 100];

        builder.add(key, value).unwrap();

        if (i + 1) % 100_000 == 0 {
            println!("  进度: {}/{}", format_number(i + 1), format_number(TOTAL_OPERATIONS));
        }
    }

    builder.finish().unwrap();
    let build_duration = start.elapsed();

    println!("\n构建完成:");
    println!("  • 耗时:     {:>10.2} s", build_duration.as_secs_f64());
    println!("  • 文件大小: {:>10} MB", std::fs::metadata("/home/quantaxis/qaexchange-rs/output//bench.sst").unwrap().len() / 1024 / 1024);

    // 查询测试
    println!("\n2️⃣ 查询测试 ({} 次)", format_number(TOTAL_OPERATIONS));

    let reader = SSTableReader::open("/home/quantaxis/qaexchange-rs/output//bench.sst").unwrap();
    let mut query_latencies = Vec::with_capacity(TOTAL_OPERATIONS);

    let start = Instant::now();

    for i in 0..TOTAL_OPERATIONS {
        let key = format!("order_{:010}", i % TOTAL_OPERATIONS).into_bytes();

        let op_start = Instant::now();
        let _ = reader.get(&key);
        query_latencies.push(op_start.elapsed().as_micros() as f64);

        if (i + 1) % 100_000 == 0 {
            println!("  进度: {}/{}", format_number(i + 1), format_number(TOTAL_OPERATIONS));
        }
    }

    let duration = start.elapsed();
    print_latency_stats("SSTable 查询", &query_latencies, duration, TOTAL_OPERATIONS);

    // Bloom Filter 效率
    println!("\n3️⃣ Bloom Filter 测试 (不存在的 key)");
    let mut bloom_latencies = Vec::with_capacity(10000);

    for i in 0..10000 {
        let key = format!("nonexistent_{:010}", i).into_bytes();

        let op_start = Instant::now();
        let _ = reader.get(&key);
        bloom_latencies.push(op_start.elapsed().as_micros() as f64);
    }

    let avg_bloom = bloom_latencies.iter().sum::<f64>() / bloom_latencies.len() as f64;
    println!("  • Bloom Filter 过滤延迟: {:.2} μs", avg_bloom);

    // 清理
    std::fs::remove_file("/home/quantaxis/qaexchange-rs/output//bench.sst").ok();
}
*/

// TODO: Phase 4 实现后启用
/*
fn benchmark_distribution() {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📡 Phase 4: 零拷贝分发性能测试");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let publisher = DistributionPublisher::new("bench_topic", "bench_pub").unwrap();

    let received_count = Arc::new(AtomicU64::new(0));
    let received_count_clone = received_count.clone();

    // 启动订阅者
    let subscriber = DistributionSubscriber::new("bench_topic", "bench_sub", move |_msg| {
        received_count_clone.fetch_add(1, Ordering::Relaxed);
    }).unwrap();

    let _sub_handle = subscriber.start();

    // 等待订阅者启动
    std::thread::sleep(Duration::from_millis(100));

    // 发布测试
    println!("1️⃣ 发布测试 ({} 条消息)", format_number(TOTAL_OPERATIONS));
    let mut publish_latencies = Vec::with_capacity(TOTAL_OPERATIONS);

    let start = Instant::now();

    for i in 0..TOTAL_OPERATIONS {
        let msg = DistributionMessage::TradeEvent {
            trade_id: i.to_le_bytes().try_into().unwrap(),
            order_id: [0u8; 40],
            instrument_id: [0u8; 16],
            price: 100.0 + (i % 10) as f64,
            volume: 10.0,
            direction: (i % 2) as u8,
            timestamp: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0),
        };

        let op_start = Instant::now();
        publisher.publish(msg).unwrap();
        publish_latencies.push(op_start.elapsed().as_micros() as f64);

        if (i + 1) % 100_000 == 0 {
            println!("  进度: {}/{}", format_number(i + 1), format_number(TOTAL_OPERATIONS));
        }
    }

    let duration = start.elapsed();
    print_latency_stats("分发 (零拷贝)", &publish_latencies, duration, TOTAL_OPERATIONS);

    // 等待所有消息接收
    std::thread::sleep(Duration::from_secs(1));

    let received = received_count.load(Ordering::Relaxed);
    println!("\n接收统计:");
    println!("  • 接收消息数: {:>10}", format_number(received as usize));
    println!("  • 接收率:     {:>10.2}%", (received as f64 / TOTAL_OPERATIONS as f64) * 100.0);
}
*/

// TODO: Phase 5 实现后启用
/*
fn benchmark_recovery() {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔄 Phase 5: 故障恢复性能测试");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // 准备 WAL 数据
    println!("1️⃣ 准备 WAL 数据 ({} 条)", format_number(TOTAL_OPERATIONS));

    let wal = WalManager::new("/home/quantaxis/qaexchange-rs/output//recovery_bench_wal");

    for i in 0..TOTAL_OPERATIONS {
        let record = WalRecord::OrderInsert {
            order_id: i.to_le_bytes().try_into().unwrap(),
            user_id: [0u8; 40],
            instrument_id: [0u8; 16],
            direction: (i % 2) as u8,
            offset: 0,
            price: 100.0 + (i % 10) as f64,
            volume: 10.0,
            timestamp: i as i64,
        };

        wal.append(record).unwrap();

        if (i + 1) % 100_000 == 0 {
            println!("  进度: {}/{}", format_number(i + 1), format_number(TOTAL_OPERATIONS));
        }
    }

    // WAL 恢复测试
    println!("\n2️⃣ WAL 恢复测试");

    let memtable_mgr = MemTableManager::new(1024 * 1024 * 1024);
    let recovery = WalRecovery::new("/home/quantaxis/qaexchange-rs/output//recovery_bench_wal", "/home/quantaxis/qaexchange-rs/output//checkpoint", memtable_mgr);

    let start = Instant::now();
    let stats = recovery.recover().unwrap();
    let duration = start.elapsed();

    println!("\n恢复完成:");
    println!("  • 耗时:       {:>10.2} s", duration.as_secs_f64());
    println!("  • 回放条目:   {:>10}", format_number(stats.replayed));
    println!("  • 跳过条目:   {:>10}", format_number(stats.skipped));
    println!("  • 恢复速度:   {:>10} entries/s", format_number((stats.replayed as f64 / duration.as_secs_f64()) as usize));

    // 清理
    std::fs::remove_dir_all("/home/quantaxis/qaexchange-rs/output//recovery_bench_wal").ok();
    std::fs::remove_dir_all("/home/quantaxis/qaexchange-rs/output//checkpoint").ok();
}
*/

fn placeholder_benchmark() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 占位符测试 (实际测试将在 Phase 1-7 实现后启用)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("✅ WAL 测试:         待实现 (Phase 1)");
    println!("✅ MemTable 测试:    待实现 (Phase 2)");
    println!("✅ SSTable 测试:     待实现 (Phase 2)");
    println!("✅ 分发测试:         待实现 (Phase 4)");
    println!("✅ 恢复测试:         待实现 (Phase 5)");

    println!("\n参考目标性能指标:");
    println!("  • WAL 写入延迟:     P99 < 10 μs");
    println!("  • MemTable 插入:    P99 < 1 μs");
    println!("  • SSTable 查询:     P99 < 100 μs");
    println!("  • 分发延迟:         P99 < 10 μs");
    println!("  • 恢复速度:         > 1 GB/s");
}

#[allow(dead_code)]
fn print_latency_stats(name: &str, latencies: &[f64], duration: Duration, total_ops: usize) {
    let mut sorted = latencies.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let avg = sorted.iter().sum::<f64>() / sorted.len() as f64;
    let p50 = sorted[sorted.len() / 2];
    let p95 = sorted[(sorted.len() as f64 * 0.95) as usize];
    let p99 = sorted[(sorted.len() as f64 * 0.99) as usize];
    let throughput = total_ops as f64 / duration.as_secs_f64();

    println!("\n{} 结果:", name);
    println!("  • 总耗时:   {:>10.2} s", duration.as_secs_f64());
    println!(
        "  • 吞吐量:   {:>10} ops/s",
        format_number(throughput as usize)
    );
    println!("  • 平均延迟: {:>10.2} μs", avg);
    println!("  • P50 延迟: {:>10.2} μs", p50);
    println!("  • P95 延迟: {:>10.2} μs", p95);
    println!("  • P99 延迟: {:>10.2} μs", p99);

    // 验证目标
    let target = if name.contains("WAL") {
        10.0
    } else if name.contains("MemTable") {
        1.0
    } else if name.contains("SSTable") {
        100.0
    } else if name.contains("分发") {
        10.0
    } else {
        100.0
    };

    if p99 < target {
        println!("  ✅ 达标 (目标: P99 < {} μs)", target);
    } else {
        println!("  ❌ 未达标 (目标: P99 < {} μs)", target);
    }
}

fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}
