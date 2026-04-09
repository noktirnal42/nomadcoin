use crate::crypto::generate_address;
use crate::types::{Transaction, TxOutput};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Load test configuration
#[derive(Debug, Clone)]
pub struct LoadTestConfig {
    /// Target TPS (transactions per second)
    pub target_tps: u32,
    /// Test duration in seconds
    pub duration_secs: u64,
    /// Number of concurrent accounts
    pub num_accounts: u32,
    /// Transaction fee
    pub fee: f64,
    /// Transaction amount
    pub amount: f64,
}

impl Default for LoadTestConfig {
    fn default() -> Self {
        LoadTestConfig {
            target_tps: 10,
            duration_secs: 3600, // 1 hour
            num_accounts: 100,
            fee: 0.001,
            amount: 1.0,
        }
    }
}

/// Load test metrics
#[derive(Debug, Clone)]
pub struct LoadTestMetrics {
    /// Total transactions submitted
    pub total_submitted: u64,
    /// Total transactions confirmed
    pub total_confirmed: u64,
    /// Total transactions failed
    pub total_failed: u64,
    /// Peak TPS observed
    pub peak_tps: f64,
    /// Average TPS
    pub average_tps: f64,
    /// Min block time
    pub min_block_time_ms: u64,
    /// Max block time
    pub max_block_time_ms: u64,
    /// Average block time
    pub avg_block_time_ms: u64,
    /// Peak memory usage (MB)
    pub peak_memory_mb: u64,
    /// Final block height
    pub final_block_height: u64,
    /// Test duration (seconds)
    pub test_duration_secs: u64,
}

/// Load test runner
pub struct LoadTestRunner {
    config: LoadTestConfig,
    start_time: Option<Instant>,
    submitted: Arc<AtomicU64>,
    confirmed: Arc<AtomicU64>,
    failed: Arc<AtomicU64>,
    peak_tps: Arc<std::sync::Mutex<f64>>,
    block_times: Arc<std::sync::Mutex<Vec<u64>>>,
    initial_height: u64,
    current_height: Arc<AtomicU64>,
}

impl LoadTestRunner {
    /// Create new load test runner
    pub fn new(config: LoadTestConfig) -> Self {
        LoadTestRunner {
            config,
            start_time: None,
            submitted: Arc::new(AtomicU64::new(0)),
            confirmed: Arc::new(AtomicU64::new(0)),
            failed: Arc::new(AtomicU64::new(0)),
            peak_tps: Arc::new(std::sync::Mutex::new(0.0)),
            block_times: Arc::new(std::sync::Mutex::new(Vec::new())),
            initial_height: 0,
            current_height: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Start the load test
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        tracing::info!(
            "🔥 Starting load test: {} TPS for {} seconds",
            self.config.target_tps,
            self.config.duration_secs
        );
    }

    /// Record a submitted transaction
    pub fn record_submitted(&self) {
        self.submitted.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a confirmed transaction
    pub fn record_confirmed(&self) {
        self.confirmed.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a failed transaction
    pub fn record_failed(&self) {
        self.failed.fetch_add(1, Ordering::Relaxed);
    }

    /// Record block time in milliseconds
    pub fn record_block_time(&self, ms: u64) {
        if let Ok(mut times) = self.block_times.lock() {
            times.push(ms);
        }
    }

    /// Update current block height
    pub fn update_block_height(&self, height: u64) {
        self.current_height.store(height, Ordering::Relaxed);
    }

    /// Check if test duration exceeded
    pub fn is_duration_exceeded(&self) -> bool {
        match self.start_time {
            Some(start) => start.elapsed().as_secs() >= self.config.duration_secs,
            None => false,
        }
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        match self.start_time {
            Some(start) => start.elapsed(),
            None => Duration::ZERO,
        }
    }

    /// Get current TPS
    pub fn current_tps(&self) -> f64 {
        let elapsed = self.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.submitted.load(Ordering::Relaxed) as f64 / elapsed
        } else {
            0.0
        }
    }

    /// Calculate final metrics
    pub fn calculate_metrics(&self) -> LoadTestMetrics {
        let submitted = self.submitted.load(Ordering::Relaxed);
        let confirmed = self.confirmed.load(Ordering::Relaxed);
        let failed = self.failed.load(Ordering::Relaxed);
        let elapsed = self.elapsed().as_secs();

        let avg_tps = if elapsed > 0 {
            submitted as f64 / elapsed as f64
        } else {
            0.0
        };

        let peak_tps = self.peak_tps.lock().map(|g| *g).unwrap_or(0.0);

        let block_times = self.block_times.lock().map(|g| g.clone()).unwrap_or_default();
        let (min_block_time, max_block_time, avg_block_time) = if !block_times.is_empty() {
            let min = block_times.iter().min().copied().unwrap_or(0);
            let max = block_times.iter().max().copied().unwrap_or(0);
            let sum: u64 = block_times.iter().sum();
            let avg = sum / block_times.len() as u64;
            (min, max, avg)
        } else {
            (0, 0, 0)
        };

        let final_height = self.current_height.load(Ordering::Relaxed);
        let _new_blocks = final_height.saturating_sub(self.initial_height);

        LoadTestMetrics {
            total_submitted: submitted,
            total_confirmed: confirmed,
            total_failed: failed,
            peak_tps,
            average_tps: avg_tps,
            min_block_time_ms: min_block_time,
            max_block_time_ms: max_block_time,
            avg_block_time_ms: avg_block_time,
            peak_memory_mb: 0, // Would be populated with system metrics
            final_block_height: final_height,
            test_duration_secs: elapsed,
        }
    }

    /// Generate a test transaction
    pub fn generate_transaction(&self, from_index: u32) -> Transaction {
        let _from = format!("nomad{:0>39}", from_index);
        let to_index = (from_index + 1) % self.config.num_accounts;
        let to = format!("nomad{:0>39}", to_index);

        Transaction {
            version: 1,
            txid: format!("test-tx-{}", from_index),
            inputs: vec![],
            outputs: vec![TxOutput {
                address: to,
                amount: self.config.amount,
                stealth: false,
            }],
            fee: self.config.fee,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            memo: Some(format!("Load test tx from account {}", from_index)),
            nonce: from_index as u64,
            chain_id: "nomadcoin".to_string(),
            sequence_number: 0,
        }
    }

    /// Print test progress
    pub fn print_progress(&self) {
        let submitted = self.submitted.load(Ordering::Relaxed);
        let confirmed = self.confirmed.load(Ordering::Relaxed);
        let failed = self.failed.load(Ordering::Relaxed);
        let elapsed = self.elapsed().as_secs();
        let current_tps = self.current_tps();

        println!(
            "⏱  Progress: {}s | TPS: {:.2} (target: {}) | Submitted: {} | Confirmed: {} | Failed: {}",
            elapsed, current_tps, self.config.target_tps, submitted, confirmed, failed
        );
    }

    /// Print final report
    pub fn print_report(&self) {
        let metrics = self.calculate_metrics();

        println!("\n{:=<70}", "");
        println!("📊 LOAD TEST REPORT");
        println!("{:=<70}", "");

        println!("\nTransaction Statistics:");
        println!("  Total Submitted:    {}", metrics.total_submitted);
        println!("  Total Confirmed:    {}", metrics.total_confirmed);
        println!("  Total Failed:       {}", metrics.total_failed);
        println!(
            "  Confirmation Rate:  {:.2}%",
            if metrics.total_submitted > 0 {
                (metrics.total_confirmed as f64 / metrics.total_submitted as f64) * 100.0
            } else {
                0.0
            }
        );

        println!("\nThroughput:");
        println!("  Average TPS:        {:.2}", metrics.average_tps);
        println!("  Peak TPS:           {:.2}", metrics.peak_tps);

        println!("\nBlock Metrics:");
        println!("  Final Height:       {}", metrics.final_block_height);
        println!("  Min Block Time:     {} ms", metrics.min_block_time_ms);
        println!("  Max Block Time:     {} ms", metrics.max_block_time_ms);
        println!("  Avg Block Time:     {} ms", metrics.avg_block_time_ms);

        println!("\nTest Duration:");
        println!("  Total Time:         {} seconds", metrics.test_duration_secs);

        println!("{:=<70}\n", "");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_test_config_defaults() {
        let config = LoadTestConfig::default();
        assert_eq!(config.target_tps, 10);
        assert_eq!(config.duration_secs, 3600);
    }

    #[test]
    fn test_load_test_runner() {
        let mut runner = LoadTestRunner::new(LoadTestConfig::default());
        runner.start();

        runner.record_submitted();
        runner.record_confirmed();
        runner.record_block_time(5000);

        assert!(runner.current_tps() > 0.0);
    }
}
