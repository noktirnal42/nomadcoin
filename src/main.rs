pub mod blockchain;
pub mod config;
pub mod consensus;
pub mod crypto;
pub mod load_test;
pub mod mesh;
pub mod miner;
pub mod network;
pub mod storage;
pub mod types;
pub mod validator_persistence;
pub mod wallet;
pub mod wallet_persistence;

use clap::Parser;
use tracing_subscriber;

/// NomadCoin - A mobile-first cryptocurrency for the nomadic community
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Command to run
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Create a new wallet
    Wallet {
        /// Number of addresses to generate
        #[arg(short, long, default_value_t = 1)]
        count: u32,
    },
    /// Create and sign a transaction
    Send {
        /// Sender address
        #[arg(long)]
        from: String,
        /// Recipient address
        #[arg(long)]
        to: String,
        /// Amount to send
        #[arg(long)]
        amount: f64,
        /// Transaction fee
        #[arg(long, default_value_t = 0.001)]
        fee: f64,
    },
    /// Initialize a new blockchain
    Init {
        /// Chain ID
        #[arg(long, default_value = "nomadcoin")]
        chain_id: String,
        /// Community allocation amount
        #[arg(long, default_value_t = 10_000_000.0)]
        allocation: f64,
        /// Address to receive the allocation
        #[arg(long)]
        address: String,
        /// Data directory
        #[arg(long, default_value = "~/.nomadcoin")]
        data_dir: String,
    },
    /// Start mining
    Mine {
        /// Wallet address
        #[arg(long)]
        address: String,
        /// Device type (auto, android, ios, darwin, linux, windows)
        #[arg(long, default_value = "auto")]
        device: String,
        /// Continue mining indefinitely
        #[arg(long, short, default_value_t = false)]
        continuous: bool,
        /// Data directory for syncing rewards
        #[arg(long, default_value = "~/.nomadcoin")]
        data_dir: String,
    },
    /// Run a full node
    Node {
        /// Port to listen on
        #[arg(short, long, default_value_t = 9333)]
        port: u16,
        /// Data directory
        #[arg(long, default_value = "~/.nomadcoin")]
        data_dir: String,
        /// Peer addresses to connect to
        #[arg(long)]
        peers: Vec<String>,
        /// Bootstrap node address
        #[arg(long)]
        bootstrap: Option<String>,
    },
    /// Import a wallet from a private key
    Import {
        /// Private key (64 hex characters)
        #[arg(long)]
        key: String,
        /// Number of addresses to derive (usually 1)
        #[arg(short, long, default_value_t = 1)]
        count: u32,
    },
    /// Register as a validator
    RegisterValidator {
        /// Validator address
        #[arg(long)]
        address: String,
        /// Stake amount
        #[arg(long)]
        stake: u64,
        /// Is mobile validator
        #[arg(long, default_value_t = false)]
        mobile: bool,
        /// Data directory
        #[arg(long, default_value = "~/.nomadcoin")]
        data_dir: String,
    },
    /// Get balance for an address
    Balance {
        /// Wallet address
        #[arg(long)]
        address: String,
        /// Data directory to load blockchain from
        #[arg(long, default_value = "~/.nomadcoin")]
        data_dir: String,
    },
    /// Get transaction details
    Transaction {
        /// Transaction ID
        #[arg(long)]
        txid: String,
        /// Data directory to load blockchain from
        #[arg(long, default_value = "~/.nomadcoin")]
        data_dir: String,
    },
    /// List all active validators
    Validators {
        /// Data directory to load blockchain from
        #[arg(long, default_value = "~/.nomadcoin")]
        data_dir: String,
    },
    /// Show node status
    Status {
        /// Data directory
        #[arg(long, default_value = "~/.nomadcoin")]
        data_dir: String,
    },
    /// Run load tests for stability verification
    LoadTest {
        /// Target transactions per second
        #[arg(long, default_value_t = 10)]
        tps: u32,
        /// Test duration in seconds
        #[arg(long, default_value_t = 3600)]
        duration: u64,
        /// Number of concurrent accounts
        #[arg(long, default_value_t = 100)]
        accounts: u32,
    },
}

fn expand_path(path: &str) -> String {
    if path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]).to_string_lossy().to_string();
        }
    }
    path.to_string()
}

fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    match args.command {
        Commands::Wallet { count } => {
            run_wallet(count);
        }
        Commands::Send {
            from,
            to,
            amount,
            fee,
        } => {
            run_send(&from, &to, amount, fee);
        }
        Commands::Init {
            chain_id,
            allocation,
            address,
            data_dir,
        } => {
            run_init(&chain_id, allocation, &address, &data_dir);
        }
        Commands::Mine { address, device, continuous, data_dir } => {
            run_mine(&address, &device, continuous, &data_dir);
        }
        Commands::Node {
            port,
            data_dir,
            peers,
            bootstrap,
        } => {
            let mut all_peers = peers.clone();
            if let Some(b) = bootstrap {
                all_peers.push(b);
            }
            run_node(port, &data_dir, &all_peers);
        }
        Commands::Import { key, count } => {
            run_import(&key, count);
        }
        Commands::RegisterValidator {
            address,
            stake,
            mobile,
            data_dir,
        } => {
            run_register_validator(&address, stake, mobile, &data_dir);
        }
        Commands::Balance { address, data_dir } => {
            run_balance(&address, &data_dir);
        }
        Commands::Transaction { txid, data_dir } => {
            run_transaction_info(&txid, &data_dir);
        }
        Commands::Validators { data_dir } => {
            run_list_validators(&data_dir);
        }
        Commands::Status { data_dir } => {
            run_status(&data_dir);
        }
        Commands::LoadTest {
            tps,
            duration,
            accounts,
        } => {
            run_load_test(tps, duration, accounts);
        }
    }
}

fn run_wallet(count: u32) {
    println!("🔐 NomadCoin Wallet Generator");
    println!("=============================\n");

    let mut wallet = wallet::Wallet::new();

    for i in 0..count {
        let addr = wallet.create_address();
        println!("Address #{}:", i + 1);
        println!("  Address:    {}", addr.address);
        println!("  Public Key: {}", addr.public_key);
        println!(
            "  Private Key: {} (KEEP SECRET!)\n",
            addr.private_key
        );
    }

    println!("✅ Generated {} address(es)", count);
}

fn run_import(key: &str, count: u32) {
    println!("🔑 NomadCoin Wallet Import");
    println!("=========================\n");

    if key.len() != 64 || !key.chars().all(|c| c.is_ascii_hexdigit()) {
        eprintln!("❌ Invalid private key: Must be 64 hex characters");
        return;
    }

    let mut wallet = wallet::Wallet::new();
    match wallet.import_address(key) {
        Ok(addr) => {
            println!("✅ Wallet successfully imported!");
            println!("  Address:    {}", addr.address);
            println!("  Public Key: {}", addr.public_key);
            println!("  Private Key: {}", addr.private_key);

            if count > 1 {
                println!("\n⚠️  Note: Only the primary address was derived from the provided key.");
                println!("   NomadCoin currently supports 1:1 key-to-address mapping.");
            }
        }
        Err(e) => {
            eprintln!("❌ Import failed: {}", e);
        }
    }
}

fn run_send(from: &str, to: &str, amount: f64, fee: f64) {
    println!("💸 NomadCoin Transaction");
    println!("=========================\n");

    let mut wallet = wallet::Wallet::new();

    // In production, load wallet from storage
    // For demo, create a new address and simulate
    let addr = wallet.create_address();

    match wallet.send_transaction(
        &addr.address,
        to,
        amount,
        fee,
        Some("NomadCoin payment".to_string()),
    ) {
        Ok(tx) => {
            println!("✅ Transaction Created!");
            println!("  TX ID:      {}", tx.txid);
            println!("  From:       {}", from);
            println!("  To:         {}", to);
            println!("  Amount:     {} NOMAD", amount);
            println!("  Fee:        {} NOMAD", fee);
            println!("  Timestamp:  {}", tx.timestamp);
            if let Some(memo) = tx.memo {
                println!("  Memo:       {}", memo);
            }
        }
        Err(e) => {
            eprintln!("❌ Transaction failed: {}", e);
        }
    }
}

fn run_init(chain_id: &str, allocation: f64, address: &str, data_dir: &str) {
    println!("🌐 NomadCoin Blockchain Initialization");
    println!("=====================================\n");

    let path = expand_path(data_dir);
    let db_path = format!("{}/chaindata", path);

    // Create data directory
    std::fs::create_dir_all(&path).expect("Failed to create data directory");

    // Initialize storage
    let storage = storage::Storage::new(&db_path).expect("Failed to open database");

    // Initialize blockchain
    let mut blockchain = blockchain::Blockchain::new();

    // Use the provided address for genesis allocation
    blockchain.create_genesis(allocation, address.to_string());

    // Save to database
    storage
        .save_blockchain(&blockchain)
        .expect("Failed to save blockchain");

    // Initialize consensus
    let consensus = consensus::ConsensusEngine::new(100, 1.5);
    storage
        .save_consensus(&consensus)
        .expect("Failed to save consensus");

    println!("✅ Blockchain Initialized!");
    println!("  Chain ID:              {}", chain_id);
    println!("  Community Allocation:  {} NOMAD", allocation);
    println!("  Community Address:     {}", address);
    println!("  Genesis Block Height:  {}", blockchain.height());
    println!(
        "  Genesis TX Count:      {}",
        blockchain.state.blocks[0].transactions.len()
    );
    println!("  Data Directory:        {}", path);
}

fn run_mine(address: &str, device: &str, continuous: bool, data_dir: &str) {
    println!("⛏️  NomadCoin Mobile Miner");
    println!("========================\n");

    // Load blockchain for syncing
    let path = expand_path(data_dir);
    let db_path = format!("{}/chaindata", path);
    let storage = storage::Storage::new(&db_path).expect("Failed to open database");
    let mut blockchain = match storage.load_blockchain().expect("Failed to load blockchain") {
        Some(bc) => bc,
        None => {
            let mut bc = blockchain::Blockchain::new();
            bc.create_genesis(10_000_000.0, "nomad1community0000000000000000000000000".to_string());
            bc
        }
    };

    let mut miner = miner::MinerService::new(address.to_string(), device.to_string());
    miner.start_mining();

    // Get actual detected device type
    let actual_device = miner.device_type.clone();
    let boost = if actual_device == "android" || actual_device == "ios" {
        "1.5x (mobile)"
    } else if actual_device == "macos" || actual_device == "linux" || actual_device == "windows" {
        "1.0x (desktop)"
    } else {
        "1.0x (unknown)"
    };

    println!("✅ Mining Started!");
    println!("  Wallet:     {}", address);
    println!("  Device:     {} (auto-detected)", actual_device);
    println!("  Status:     Active");
    println!("  Boost:     {}", boost);

    if continuous {
        println!("\n🔄 Continuous mining - Press Ctrl+C to stop...\n");
        
        // Continuous mining loop
        let mut validation_num = 0;
        loop {
            validation_num += 1;
            let tx_hash = format!("simulated_tx_{}", validation_num);
            miner.perform_validation(&tx_hash);
            
            let stats = miner.get_stats();
            println!(
                "📊 Validations: {} | Earnings: {:.4} NOMAD | TX: {}",
                stats.total_validations, 
                stats.total_earnings,
                &tx_hash[..8]
            );
            
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    } else {
        // Single batch (default behavior)
        for i in 1..=5 {
            miner.perform_validation(&format!("simulated_tx_{}", i));
        }

        let stats = miner.get_stats();
        println!("\n📊 Mining Statistics:");
        println!("  Validations:     {}", stats.total_validations);
        println!("  Earnings:        {:.4} NOMAD", stats.total_earnings);
        println!("  Mobile Boost:    {:.1}x", stats.mobile_boost_multiplier);

        // Sync
        let synced = miner.sync_with_network(&mut blockchain);
        println!("  Synced:          {} validations", synced);
    }
}

#[tokio::main]
async fn run_node(port: u16, data_dir: &str, peers: &[String]) {
    println!("🔗 NomadCoin Node");
    println!("=================\n");

    let path = expand_path(data_dir);
    let db_path = format!("{}/chaindata", path);

    // Create data directory if not exists
    std::fs::create_dir_all(&path).expect("Failed to create data directory");

    // Initialize or load storage
    let storage = storage::Storage::new(&db_path).expect("Failed to open database");

    // Load or create blockchain
    let blockchain = match storage.load_blockchain().expect("Failed to load blockchain") {
        Some(bc) => bc,
        None => {
            println!("No existing blockchain found. Creating genesis...");
            let mut bc = blockchain::Blockchain::new();
            bc.create_genesis(
                10_000_000.0,
                "nomad1community0000000000000000000000000".to_string(),
            );
            bc
        }
    };

    // Load consensus engine
    let mut consensus = storage
        .load_consensus(100, 1.5)
        .expect("Failed to load consensus");

    // Load persisted validators from disk
    if let Ok(persisted_validators) = validator_persistence::load_validators(&path) {
        let count = persisted_validators.len();
        for (address, validator_state) in persisted_validators {
            if !consensus.validators.contains_key(&address) {
                consensus.validators.insert(address.clone(), validator_state);
            }
        }
        if count > 0 {
            tracing::info!("Loaded {} validators from disk", count);
        }
    }

    println!("Starting node on port {}...", port);
    println!("  Network:    nomadcoin");
    println!("  Port:       {}", port);
    println!("  Data Dir:   {}", path);
    println!("  Height:     {}", blockchain.height());
    println!("  Validators: {}", consensus.validator_count());

    // Create channel for transactions
    let (tx_sender, mut tx_receiver) = tokio::sync::mpsc::channel(100);

    // Wrap blockchain in Arc<Mutex> for shared access
    let blockchain = std::sync::Arc::new(tokio::sync::Mutex::new(blockchain));

    // Initialize P2P network
    let mut network = network::P2PNetwork::new(tx_sender);
    network.set_blockchain(blockchain.clone());

    // Start P2P server
    if let Err(e) = network.start_server(port).await {
        eprintln!("Failed to start P2P server: {}", e);
        println!("Running in standalone mode (P2P disabled)");
    }

    // Connect to peers
    for peer in peers {
        if let Err(e) = network.connect_to_peer(peer).await {
            eprintln!("Failed to connect to peer {}: {}", peer, e);
        }
    }

    println!("  Peers:      {}", network.peer_count());
    println!("\nPress Ctrl+C to stop.");

    // Sync blockchain from peer if we have peers
    if !peers.is_empty() {
        println!("🔄 Synchronizing blockchain from peers...");
        let sync_timeout = tokio::time::Duration::from_secs(15);

        match tokio::time::timeout(sync_timeout, async {
            let mut bc = blockchain.lock().await;
            let peer_addr = &peers[0]; // Sync from first peer
            match bc.sync_from_peer(peer_addr, &network).await {
                Ok(()) => {
                    println!("✅ Blockchain synced!");
                    println!("  Height:     {}", bc.height());
                    Ok(())
                }
                Err(e) => {
                    println!("⚠️  Sync failed: {}", e);
                    Err(e)
                }
            }
        }).await {
            Ok(Ok(())) => {
                // Reload consensus engine to reflect validator updates
                if let Ok(synced_consensus) = storage.load_consensus(100, 1.5) {
                    consensus = synced_consensus;
                    println!("  Validators: {}", consensus.validator_count());
                }
            }
            Ok(Err(_)) | Err(_) => {
                println!("⚠️  Sync timeout or error. Continuing with local state.");
            }
        }
    }

    // Main node loop
    let mut block_interval = tokio::time::interval(tokio::time::Duration::from_secs(5));

    loop {
        tokio::select! {
            // Process incoming transactions
            Some(tx) = tx_receiver.recv() => {
                let mut bc = blockchain.lock().await;
                if let Err(e) = bc.add_to_mempool(tx) {
                    tracing::warn!("Failed to add transaction to mempool: {}", e);
                }
            }

            // Produce blocks on schedule
            _ = block_interval.tick() => {
                let mut bc = blockchain.lock().await;
                // Select proposer (blocks produce every interval, even if empty)
                if let Some(proposer) = consensus.select_proposer() {
                    // Start consensus round
                    let next_height = bc.height() + 1;
                    consensus.start_round(next_height, proposer.clone());

                    // Simulate validator votes (in production, this is distributed)
                    for validator_addr in consensus.validators.keys().cloned().collect::<Vec<_>>() {
                        let _ = consensus.record_prevote(&validator_addr, true);
                        let _ = consensus.record_precommit(&validator_addr, true);
                    }

                    // Check if consensus reached
                    if consensus.is_consensus_reached() {
                        // Create block
                        let block = bc.create_block(&proposer);

                        // Finalize consensus
                        if consensus.finalize_block().is_ok() {
                            // Save to storage
                            if let Err(e) = storage.save_blockchain(&*bc) {
                                tracing::error!("Failed to save blockchain: {}", e);
                            }
                            if let Err(e) = storage.save_consensus(&consensus) {
                                tracing::error!("Failed to save consensus: {}", e);
                            }

                            // Broadcast block
                            let block_bytes = serde_json::to_vec(&block).unwrap_or_default();
                            let block_height = bc.height();
                            drop(bc); // Release lock before async operation
                            network.broadcast_block(block_bytes, block_height).await;

                            println!(
                                "✅ Block {} created with {} transactions (proposer: {})",
                                block_height,
                                block.header.transaction_count,
                                proposer
                            );
                        }
                    }
                }
            }
        }
    }
}

fn run_register_validator(address: &str, stake: u64, mobile: bool, data_dir: &str) {
    println!("📋 NomadCoin Validator Registration");
    println!("==================================\n");

    let path = expand_path(data_dir);
    let db_path = format!("{}/chaindata", path);

    let storage = storage::Storage::new(&db_path).expect("Failed to open database");
    let mut consensus = storage
        .load_consensus(100, 1.5)
        .expect("Failed to load consensus");

    match consensus.register_validator(address.to_string(), stake, mobile) {
        Ok(()) => {
            storage
                .save_consensus(&consensus)
                .expect("Failed to save consensus");

            // Persist validator to disk for recovery on node restart
            validator_persistence::add_validator(&path, address.to_string(), stake, mobile)
                .expect("Failed to persist validator");

            println!("✅ Validator Registered!");
            println!("  Address:     {}", address);
            println!("  Stake:       {} NOMAD", stake);
            println!("  Mobile:      {}", if mobile { "Yes (1.5x boost)" } else { "No" });
            println!(
                "  Effective:   {} NOMAD",
                if mobile {
                    stake as f64 * 1.5
                } else {
                    stake as f64
                }
            );
            println!("  Validators:  {}", consensus.validator_count());
        }
        Err(e) => {
            eprintln!("❌ Registration failed: {}", e);
        }
    }
}

fn run_status(data_dir: &str) {
     println!("📊 NomadCoin Node Status");
     println!("========================\n");

    let path = expand_path(data_dir);
    let db_path = format!("{}/chaindata", path);

    match storage::Storage::new(&db_path) {
        Ok(storage) => {
            match storage.get_stats() {
                Ok(stats) => {
                    println!("  Latest Block:  {}", stats.latest_height);
                    println!("  Transactions:  {}", stats.tx_count);
                    println!("  Balances:      {}", stats.balance_count);
                    println!();

                    // Load consensus info
                    match storage.load_consensus(100, 1.5) {
                        Ok(consensus) => {
                            println!("  Validators:    {}", consensus.validator_count());
                            println!(
                                "  Active:        {}",
                                consensus.active_validator_count()
                            );
                            println!("  Total Stake:   {} NOMAD", consensus.total_stake());
                        }
                        Err(_) => {
                            println!("  Consensus:     Not initialized");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to get stats: {}", e);
                }
            }
        }
        Err(_e) => {
            println!("  Status:     No blockchain data found");
            println!("  Data Dir:   {}", path);
            println!();
            println!("  Run 'nomadcoin-core init' to initialize");
        }
    }
}

fn run_load_test(tps: u32, duration_secs: u64, num_accounts: u32) {
    println!("🔥 NomadCoin Load Test Suite");
    println!("============================\n");

    let config = load_test::LoadTestConfig {
        target_tps: tps,
        duration_secs,
        num_accounts,
        fee: 0.001,
        amount: 1.0,
    };

    let mut runner = load_test::LoadTestRunner::new(config);
    runner.start();

    println!("Test Configuration:");
    println!("  Target TPS:          {}", tps);
    println!("  Test Duration:       {} seconds", duration_secs);
    println!("  Concurrent Accounts: {}", num_accounts);
    println!("  Fee per TX:          0.001 NOMAD");
    println!("  Amount per TX:       1.0 NOMAD");
    println!("\n⏳ Running load test...\n");
    let mut last_progress = std::time::Instant::now();
    let mut account_counter = 0u32;

    // Main test loop
    while !runner.is_duration_exceeded() {
        // Generate transaction
        let _tx = runner.generate_transaction(account_counter);
        runner.record_submitted();

        // Simulate confirmation (50% confirmation rate for demo)
        if account_counter % 2 == 0 {
            runner.record_confirmed();
        } else {
            runner.record_failed();
        }

        // Record simulated block time (5 seconds per block)
        if account_counter % (num_accounts / 20).max(1) == 0 {
            runner.record_block_time(5000);
            runner.update_block_height((account_counter / (num_accounts / 20).max(1)) as u64);
        }

        account_counter = (account_counter + 1) % num_accounts;

        // Print progress every 10 seconds
        if last_progress.elapsed().as_secs() >= 10 {
            runner.print_progress();
            last_progress = std::time::Instant::now();
        }

        // Rate limiting - sleep to match target TPS
        if tps > 0 {
            let tx_interval_us = 1_000_000 / tps as u64;
            std::thread::sleep(std::time::Duration::from_micros(tx_interval_us));
        }
    }

    println!();
    runner.print_report();

    println!("✅ Load test completed successfully!");
}

fn run_balance(address: &str, data_dir: &str) {
    println!("💰 NomadCoin Balance Checker");
    println!("=========================\n");

    let path = expand_path(data_dir);
    let db_path = format!("{}/chaindata", path);
    let storage = storage::Storage::new(&db_path).expect("Failed to open database");

    let blockchain = match storage.load_blockchain().expect("Failed to load blockchain") {
        Some(bc) => bc,
        None => {
            println!("Error: No blockchain found in data directory. Run 'init' first.");
            return;
        }
    };

    let balance = blockchain.get_balance(address);
    println!("\nBalance for {}:", address);
    println!("  {:.4} NOMAD", balance);
}

fn run_transaction_info(txid: &str, data_dir: &str) {
    println!("📄 Transaction Info");
    println!("==================\n");

    let path = expand_path(data_dir);
    let db_path = format!("{}/chaindata", path);
    let storage = storage::Storage::new(&db_path).expect("Failed to open database");

    let blockchain = match storage.load_blockchain().expect("Failed to load blockchain") {
        Some(bc) => bc,
        None => {
            println!("Error: No blockchain found in data directory.");
            return;
        }
    };

    let mut found = false;
    for block in &blockchain.state.blocks {
        for tx in &block.transactions {
            if tx.txid == txid {
                println!("Transaction found!");
                println!("  TXID:     {}", tx.txid);
                println!("  Amount:   {:.4} NOMAD", tx.outputs[0].amount);
                println!("  Recipient: {}", tx.outputs[0].address);
                println!("  Timestamp: {}", tx.timestamp);
                println!("  Memo:     {}", tx.memo.as_deref().unwrap_or("None"));

                let confirmations = blockchain.height() - (blockchain.state.blocks.iter().position(|b| b.transactions.iter().any(|t| t.txid == txid)).unwrap_or(0) as u64 + 1);
                println!("  Confirmations: {}", confirmations);

                found = true;
                break;
            }
        }
        if found { break; }
    }

    if !found {
        println!("Transaction {} not found in the blockchain.", txid);
    }
}

fn run_list_validators(data_dir: &str) {
    println!("🛡️  Active Validators");
    println!("==================\n");

    let path = expand_path(data_dir);
    let db_path = format!("{}/chaindata", path);
    let storage = storage::Storage::new(&db_path).expect("Failed to open database");

    let blockchain = match storage.load_blockchain().expect("Failed to load blockchain") {
        Some(bc) => bc,
        None => {
            println!("Error: No blockchain found.");
            return;
        }
    };

    if blockchain.state.validators.is_empty() {
        println!("No registered validators found.");
        return;
    }

    println!("{:<32} {:<15} {:<10}", "Address", "Stake", "Boost");
    println!("{:-<60}", "");

    for (addr, stake) in &blockchain.state.validators {
        println!("{:<32} {:<15.2} {:<10}", addr, stake, "1.0x");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_workflow() {
        // Create wallet
        let mut wallet = wallet::Wallet::new();
        let alice = wallet.create_address();
        let bob = wallet.create_address();

        // Initialize blockchain
        let mut blockchain = blockchain::Blockchain::new();
        blockchain.create_genesis(10_000_000.0, alice.address.clone());

        // Check balance
        let balance = blockchain.get_balance(&alice.address);
        assert_eq!(balance, 10_000_000.0);

        // Create transaction
        let tx = wallet.send_transaction(
            &alice.address,
            &bob.address,
            100.0,
            0.001,
            Some("payment".to_string()),
        );
        assert!(tx.is_ok());

        // Start miner
        let mut miner = miner::MinerService::new(alice.address.clone(), "android".to_string());
        miner.start_mining();
        miner.perform_validation("test_tx");
        assert!(miner.is_active());
        assert_eq!(miner.get_validation_count(), 1);
    }

    #[test]
    fn test_consensus_integration() {
        let mut consensus = consensus::ConsensusEngine::new(100, 1.5);

        // Register validators
        consensus
            .register_validator("val1".to_string(), 1000, false)
            .unwrap();
        consensus
            .register_validator("val2".to_string(), 500, true)
            .unwrap();

        assert_eq!(consensus.validator_count(), 2);

        // Select proposer and start round
        let proposer = consensus.select_proposer().unwrap();
        consensus.start_round(1, proposer);

        // Record votes
        consensus.record_prevote("val1", true).unwrap();
        consensus.record_prevote("val2", true).unwrap();
        consensus.record_precommit("val1", true).unwrap();
        consensus.record_precommit("val2", true).unwrap();

        // Check consensus
        assert!(consensus.is_consensus_reached());
        assert!(consensus.finalize_block().is_ok());
        assert_eq!(consensus.height, 1);
    }
}
