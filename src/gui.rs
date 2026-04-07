//! NomadCoin GUI - Simplified version for desktop/mobile
//! Run with: cargo run --features gui --bin nomadcoin-gui

pub mod blockchain;
pub mod consensus;
pub mod crypto;
pub mod mesh;
pub mod miner;
pub mod network;
pub mod storage;
pub mod types;
pub mod wallet;

use eframe::egui;
use chrono::{DateTime, Utc};

// Re-export types for convenience
use types::WalletAddress;

struct NomadCoinApp {
    // Wallet
    wallet: wallet::Wallet,
    addresses: Vec<WalletAddress>,
    selected_address: usize,
    balance: f64,
    
    // Miner
    miner_active: bool,
    miner_device: String,
    earnings: f64,
    validations: u64,
    
    // Send form
    send_to: String,
    send_amount: String,
    send_memo: String,
    
    // Community
    offline_mode: bool,
    peer_count: usize,
    
    // Tabs
    current_tab: usize,
    
    last_update: DateTime<Utc>,
}

impl Default for NomadCoinApp {
    fn default() -> Self {
        let mut wallet = wallet::Wallet::new();
        let addr1 = wallet.create_address();
        let addr2 = wallet.create_address();
        
        let mut blockchain = blockchain::Blockchain::new();
        blockchain.create_genesis(10_000_000.0, addr1.address.clone());
        
        let device = Self::detect_device();
        
        NomadCoinApp {
            wallet,
            addresses: vec![addr1, addr2],
            selected_address: 0,
            balance: 10_000_000.0,
            miner_active: false,
            miner_device: device.clone(),
            earnings: 0.0,
            validations: 0,
            send_to: String::new(),
            send_amount: String::new(),
            send_memo: String::new(),
            offline_mode: false,
            peer_count: 0,
            current_tab: 0,
            last_update: Utc::now(),
        }
    }
}

impl NomadCoinApp {
    fn detect_device() -> String {
        #[cfg(target_os = "android")] return "android".to_string();
        #[cfg(target_os = "ios")] return "ios".to_string();
        #[cfg(target_os = "macos")] return "macos".to_string();
        #[cfg(target_os = "linux")] return "linux".to_string();
        #[cfg(target_os = "windows")] return "windows".to_string();
        #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos", target_os = "linux", target_os = "windows")))] return "unknown".to_string();
    }
    
    fn get_boost(device: &str) -> f64 {
        match device {
            "android" | "ios" => 1.5,
            _ => 1.0,
        }
    }
}

impl eframe::App for NomadCoinApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
        
        let device = Self::detect_device();
        let boost = Self::get_boost(&device);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.horizontal(|ui| {
                ui.heading("⛓️ NomadCoin");
                ui.separator();
                ui.label(format!("{} ({:.1}x)", device, boost));
                ui.separator();
                if self.miner_active {
                    ui.colored_label(egui::Color32::GREEN, "Mining");
                } else if self.offline_mode {
                    ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "Offline");
                } else {
                    ui.label("Online");
                }
            });
            
            ui.separator();
            
            // Tab buttons
            ui.horizontal(|ui| {
                if ui.button("💳 Wallet").clicked() { self.current_tab = 0; }
                if ui.button("⛏️ Miner").clicked() { self.current_tab = 1; }
                if ui.button("📤 Send").clicked() { self.current_tab = 2; }
                if ui.button("🏕️ Community").clicked() { self.current_tab = 3; }
            });
            
            ui.separator();
            
            // Tab content
            match self.current_tab {
                0 => self.wallet_tab(ui),
                1 => self.miner_tab(ui),
                2 => self.send_tab(ui),
                3 => self.community_tab(ui),
                _ => {}
            }
            
            ui.separator();
            ui.label(format!("Last: {}", self.last_update.format("%H:%M:%S")));
        });
    }
}

impl NomadCoinApp {
    fn wallet_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("💳 Wallet");
        
        ui.horizontal(|ui| {
            ui.label("Balance:");
            ui.heading(format!("{:.4} NOMAD", self.balance));
        });
        
        ui.label("Your Addresses:");
        for (i, addr) in self.addresses.iter().enumerate() {
            ui.horizontal(|ui| {
                let label = if self.selected_address == i {
                    format!("✓ #{}", i + 1)
                } else {
                    format!("#{}", i + 1)
                };
                if ui.button(label).clicked() {
                    self.selected_address = i;
                }
                ui.label(truncate(&addr.address, 24));
                
                if ui.button("📋").clicked() {
                    ui.output_mut(|o| o.copied_text = addr.address.clone());
                }
            });
            
            // Show QR-style visualization (simplified)
            if self.selected_address == i {
                ui.separator();
                ui.label("📱 Receive:");
                ui.label(truncate(&addr.address, 32));
                
                // Generate pseudo-QR visual from address
                let chars: Vec<char> = addr.address.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
                let rows = chars.chunks(8).take(21).collect::<Vec<_>>();
                
                ui.horizontal(|ui| {
                    ui.label("█".repeat(8));
                });
                for row in &rows {
                    let mut pattern = String::new();
                    for c in *row {
                        // Simple visual encoding
                        let v = (*c as u8).wrapping_sub(48);
                        if v % 2 == 0 { pattern.push_str("██"); } 
                        else { pattern.push_str("  "); }
                    }
                    ui.label(pattern);
                }
                ui.horizontal(|ui| {
                    ui.label("█".repeat(8));
                });
            }
        }
        
        if ui.button("+ New Address").clicked() {
            let new_addr = self.wallet.create_address();
            self.addresses.push(new_addr);
        }
    }
    
    fn miner_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("⛏️ Miner");
        
        let device = Self::detect_device();
        let boost = Self::get_boost(&device);
        
        ui.horizontal(|ui| {
            ui.label("Status:");
            ui.colored_label(
                if self.miner_active { egui::Color32::GREEN } else { egui::Color32::GRAY },
                if self.miner_active { "Active" } else { "Idle" }
            );
        });
        
        ui.label(format!("Device: {} (auto)", device));
        
        ui.horizontal(|ui| {
            ui.label("Boost:");
            if boost > 1.0 {
                ui.colored_label(egui::Color32::GREEN, format!("{:.1}x Mobile!", boost));
            } else {
                ui.colored_label(egui::Color32::GRAY, format!("{:.1}x", boost));
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Earnings:");
            ui.heading(format!("{:.4} NOMAD", self.earnings));
        });
        
        ui.horizontal(|ui| {
            ui.label("Validations:");
            ui.label(format!("{}", self.validations));
        });
        
        if ui.button(if self.miner_active { "⏹ Stop" } else { "▶ Start" }).clicked() {
            self.miner_active = !self.miner_active;
            if self.miner_active {
                self.earnings += 0.01 * boost;
                self.validations += 1;
            }
        }
        
        ui.separator();
        ui.label("💡 Mobile hotspot = 1.5x bonus!");
    }
    
    fn send_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("📤 Send NOMAD");
        
        ui.horizontal(|ui| {
            ui.label("From:");
            ui.label(truncate(&self.addresses[self.selected_address].address, 20));
        });
        
        ui.horizontal(|ui| {
            ui.label("To:");
            ui.text_edit_singleline(&mut self.send_to);
        });
        
        ui.horizontal(|ui| {
            ui.label("Amount:");
            ui.text_edit_singleline(&mut self.send_amount);
            ui.label("NOMAD");
        });
        
        ui.horizontal(|ui| {
            ui.label("Memo:");
            ui.text_edit_singleline(&mut self.send_memo);
        });
        
        if ui.button("📤 Send").clicked() {
            let to = self.send_to.clone();
            let amount: f64 = self.send_amount.parse().unwrap_or(0.0);
            
            if !to.is_empty() && amount > 0.0 {
                let from = &self.addresses[self.selected_address].address;
                let memo = if self.send_memo.is_empty() { None } else { Some(self.send_memo.clone()) };
                
                match self.wallet.send_transaction(from, &to, amount, 0.001, memo) {
                    Ok(_tx) => {
                        self.balance -= amount;
                        self.send_to.clear();
                        self.send_amount.clear();
                        self.send_memo.clear();
                    }
                    Err(e) => {
                        ui.colored_label(egui::Color32::RED, format!("Error: {}", e));
                    }
                }
            }
        }
    }
    
    fn community_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("🏕️ Community");
        
        ui.horizontal(|ui| {
            ui.label("Mode:");
            ui.colored_label(
                if self.offline_mode { egui::Color32::from_rgb(255, 165, 0) } else { egui::Color32::GREEN },
                if self.offline_mode { "📴 Offline" } else { "🟢 Online" }
            );
        });
        
        if ui.button(if self.offline_mode { "🟢 Go Online" } else { "📴 Go Offline" }).clicked() {
            self.offline_mode = !self.offline_mode;
        }
        
        ui.horizontal(|ui| {
            ui.label("Peer Network:");
            ui.label(format!("{} peers", self.peer_count));
        });
        
        ui.separator();
        ui.label("💡 Nomad Tips:");
        ui.label("• Use offline mode when traveling");
        ui.label("• Share coins with peer-to-peer mesh");
        ui.label("• Mobile hotspot gives 1.5x mining bonus");
    }
}

fn truncate(s: &str, len: usize) -> String {
    if s.len() > len {
        format!("{}...", &s[..len])
    } else {
        s.to_string()
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([380.0, 600.0])
            .with_min_inner_size([300.0, 400.0])
            .with_title("NomadCoin"),
        ..Default::default()
    };
    
    eframe::run_native(
        "NomadCoin",
        options,
        Box::new(|_cc| Ok(Box::new(NomadCoinApp::default()))),
    )
}