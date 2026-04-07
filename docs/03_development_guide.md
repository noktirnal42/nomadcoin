# NomadCoin Development Guide: Open-Source Implementation

## Version: 1.1 (Open-Source Based)
## Date: April 2026

---

## Overview
This guide leverages existing, battle-tested open-source blockchain projects to accelerate NomadCoin development. Instead of building from scratch, we fork and customize proven frameworks.

## Recommended Open-Source Foundation

| Project | Use Case | Pros | Cons |
|---------|----------|------|------|
| **Cosmos SDK + Tendermint** | Core blockchain | Most mature, great docs, IBC support | Requires Go expertise |
| **Pactus** | Lightweight node | Runs on Raspberry Pi, simple | Newer project |
| **ARK Ecosystem** | Quick launch | Launcher tool, templates | Less customizable |
| **Substrate** | Full customization | Polkadot ecosystem, Rust | Steeper learning curve |

**Recommendation**: Use **Cosmos SDK** as the primary foundation with **Pactus** as a reference for mobile optimization.

---

## Phase 1: Environment Setup (Week 1-2)

### Step 1.1: Install Development Tools

#### macOS
```bash
# Install Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Go (required for Cosmos SDK)
brew install go@1.21

# Install Git
brew install git

# Install Docker
brew install --cask docker

# Install Rust (for Substrate components)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Cosmos tools
go install github.com/cosmos/gosec/v2/cmd/gosec@latest
go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest

# Verify
go version  # Should be 1.21+
```

#### Linux (Ubuntu/Debian)
```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Go
wget https://go.dev/dl/go1.21.5.linux-amd64.tar.gz
sudo tar -C /usr/local -xzf go1.21.5.linux-amd64.tar.gz
echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.bashrc
source ~/.bashrc

# Install Docker
sudo apt install -y docker.io docker-compose
sudo usermod -aG docker $USER

# Install essential build tools
sudo apt install -y build-essential git curl wget
```

### Step 1.2: Set Up Development Directory
```bash
# Create project directory
mkdir -p ~/dev/nomad_coin
cd ~/dev/nomad_coin

# Initialize git repository
git init
git config user.name "NomadCoin Team"
git config user.email "team@nomadcoin.network"

# Create project structure
mkdir -p docs scripts configs testnet
```

---

## Phase 2: Fork and Customize Blockchain (Week 3-10)

### Step 2.1: Fork Cosmos SDK-Based Chain

We will use **Stargaze** or create a new chain using Cosmos SDK as a template:

```bash
cd ~/dev/nomad_coin

# Clone Cosmos SDK
git clone https://github.com/cosmos/cosmos-sdk.git
cd cosmos-sdk
git checkout v0.50.0
cd ..

# OR: Clone a simple PoS chain template like Perseverance or Neutron
git clone https://github.com/cosmos/gaia.git
cd gaia
git checkout v14.0.0
cd ..
```

### Step 2.2: Create New NomadChain Using Starport

Cosmos provides **Starport** (formerly known as Ignite CLI) to scaffold a new blockchain:

```bash
# Install Starport
curl https://get.starport.com/starport | bash
sudo mv starport /usr/local/bin/

# OR: Install via npm
npm install -g @starport/cli

# Create new blockchain
starport scaffold chain nomadcoin --address-prefix nomad --module nomadcoin

# This creates a new blockchain with:
# - Default Cosmos SDK modules
# - Go-based application
# - CLI and REST API
# - Proof-of-Stake consensus
```

### Step 2.3: Customize Chain Parameters

Edit `app/app.go` to customize:

```go
// In app/app.go - Update chain parameters
package app

const (
    // Chain ID
    AppName     = "NomadCoin"
    ChainID     = "nomadcoin"
    
    // Token details
    BondDenom   = "nomad"      // Staking token
    DisplayDenom = "NOMAD"    // Display token
    
    // Supply
    InitialSupply = sdk.NewInt(100_000_000_000_000)  // 100M NOMAD * 10^12
    
    // Governance
    ProposalDeposit = "100000000nomad"  // 100 NOMAD
    VotingPeriod = 172800               // ~3 days in seconds
    MaxDepositPeriod = 172800          // ~3 days
)
```

### Step 2.4: Add Custom Modules

Create custom module for NomadCoin features:

```bash
# Scaffold new module
starport scaffold module nomad --module-name nomadcoin

# Add custom types
starport scaffold list NomadWallet --set unilateral -y
starport scaffold list NomadTransaction --set unilateral -y
```

### Step 2.5: Modify Proof-of-Stake for Mobile

Edit `x/staking/keeper/validator.go` to add mobile boost:

```go
package keeper

// MobileValidatorWeight returns adjusted weight for mobile validators
func (k Keeper) MobileValidatorWeight(ctx sdk.Context, valAddr sdk.ValAddress) sdk.Dec {
    // Check if validator is running on mobile
    val, found := k.GetValidator(ctx, valAddr)
    if !found {
        return sdk.OneDec()
    }
    
    // Get mobile status from validator metadata
    // Mobile validators get 1.5x weight
    if val.IsMobile {
        return sdk.MustNewDecFromStr("1.5")
    }
    
    return sdk.OneDec()
}
```

---

## Phase 3: Mobile-Optimized Node (Week 11-16)

### Option A: Build Light Client Using Pactus Reference

Pactus is designed for low-resource devices. Reference their implementation:

```bash
# Clone Pactus
git clone https://github.com/pactus/pactus.git
cd pactus

# Study their lightweight node implementation
# Key files to reference:
# - consensus/  - Consensus without heavy computation
# - state/     - State management
# - wallet/    - Lightweight wallet

cd ..
```

### Option B: Build Cosmos SDK Light Client

```bash
# Create light client application
starport scaffold app nomadcoin-light --template=android

# Build for Android (requires Android NDK)
cd nomadcoin-light
go build -o android/libnomadcoin.so ./cmd/nomadcoind
```

### Step 3.1: Create Mobile Miner Service

Reference existing mobile mining implementations:

```bash
# Clone reference implementation - Scala Network Mobile Miner
git clone https://github.com/scala-network/MobileMiner.git
git clone https://github.com/ImL1s/XMRigMiner-Android.git
```

### Step 3.2: Implement Mobile Validation Layer

Create mobile-specific module:

```bash
# Add mobile validation module
starport scaffold module mobile --module-name mobilevalidation
```

Edit `x/mobilevalidation/keeper/validation.go`:

```go
package keeper

import (
    "context"
    "time"
    
    "nomadcoin/x/mobilevalidation/types"
    "github.com/cosmos/cosmos-sdk/store/prefix"
    sdk "github.com/cosmos/cosmos-sdk/types"
)

type Keeper struct {
    // ... existing fields
    
    // Mobile validation tracking
    mobileValidatorStats map[string]MobileValidatorInfo
}

type MobileValidatorInfo struct {
    Address      string
    LastActive   time.Time
    Validations  uint64
    IsOnline     bool
    DeviceType   string  // "android", "ios", "desktop"
}

// RecordValidation - called when mobile device validates a transaction
func (k Keeper) RecordValidation(ctx context.Context, valAddr string, txHash string) error {
    info, found := k.mobileValidatorStats[valAddr]
    if !found {
        info = MobileValidatorInfo{
            Address:     valAddr,
            LastActive:  time.Now(),
            DeviceType:  detectDeviceType(valAddr),
        }
    }
    
    info.Validations++
    info.LastActive = time.Now()
    k.mobileValidatorStats[valAddr] = info
    
    return nil
}

// CalculateMobileReward - calculate rewards for mobile validation
func (k Keeper) CalculateMobileReward(ctx context.Context, valAddr string) sdk.Coin {
    info, found := k.mobileValidatorStats[valAddr]
    if !found {
        return sdk.NewCoin("nomad", sdk.ZeroInt())
    }
    
    // Base reward: 0.01 NOMAD per validation
    rewardAmount := sdk.NewInt(int64(info.Validations)) * sdk.NewInt(100000) // 0.01 NOMAD in micro units
    
    // Mobile boost: 1.5x for mobile devices
    if info.DeviceType == "android" || info.DeviceType == "ios" {
        rewardAmount = rewardAmount.MulRaw(3).QuoRaw(2)
    }
    
    return sdk.NewCoin("nomad", rewardAmount)
}
```

---

## Phase 4: Mesh Networking Implementation (Week 17-22)

### Step 4.1: Integrate Mesh Networking Library

Reference open-source mesh networking projects:

```bash
# Clone Reticulum - mentioned in research as popular with digital nomads
git clone https://github.com/markqvist/Reticulum.git

# Clone NomadNet - mesh networking for nomads
git clone https://github.com/markqvist/NomadNet.git

# Clone FreeFlow - offline-resilient mesh networking
git clone https://github.com/freeflw/FreeFlow.git
```

### Step 4.2: Implement Offline Transaction Protocol

Create `x/offline/keeper/offline_tx.go`:

```go
package keeper

import (
    "crypto/ed25519"
    "encoding/hex"
    
    "nomadcoin/x/offline/types"
    sdk "github.com/cosmos/cosmos-sdk/types"
)

// OfflineTransaction represents a transaction that can be sent offline
type OfflineTransaction struct {
    ID          string          `json:"id"`
    From        string          `json:"from"`
    To          string          `json:"to"`
    Amount      sdk.Int         `json:"amount"`
    Fee         sdk.Int         `json:"fee"`
    Timestamp   int64           `json:"timestamp"`
    Signature   []byte          `json:"signature"`
    ValidAfter  int64           `json:"valid_after"`  // When tx becomes valid
    HopLimit    uint8           `json:"hop_limit"`   // Mesh hop limit
}

// CreateOfflineTransaction - create transaction for mesh network
func (k Keeper) CreateOfflineTransaction(
    ctx sdk.Context,
    from string,
    to string,
    amount sdk.Int,
    privateKey []byte,
) (*types.OfflineTransaction, error) {
    
    timestamp := ctx.BlockTime().Unix()
    
    // Create transaction data
    txData := []byte(from + to + amount.String() + string(rune(timestamp)))
    
    // Sign with Ed25519
    signature := ed25519.Sign(ed25519.PrivateKey(privateKey), txData)
    
    offlineTx := &types.OfflineTransaction{
        Id:          hex.EncodeToString(signature[:16]),
        From:        from,
        To:          to,
        Amount:      amount.String(),
        Fee:         "1000000", // 0.001 NOMAD
        Timestamp:   timestamp,
        Signature:   hex.EncodeToString(signature),
        ValidAfter:  timestamp,
        HopLimit:    5,
    }
    
    // Store locally
    k.SetOfflineTransaction(ctx, *offlineTx)
    
    return offlineTx, nil
}

// ValidateOfflineTransaction - validate transaction received via mesh
func (k Keeper) ValidateOfflineTransaction(
    ctx sdk.Context,
    tx types.OfflineTransaction,
) error {
    
    // Verify signature
    txData := []byte(tx.From + tx.To + tx.Amount + string(rune(tx.Timestamp)))
    signatureBytes, err := hex.DecodeString(tx.Signature)
    if err != nil {
        return err
    }
    
    // Get public key from address
    pubKey, err := k.GetPublicKeyFromAddress(ctx, tx.From)
    if err != nil {
        return err
    }
    
    if !ed25519.Verify(ed25519.PublicKey(pubKey), txData, signatureBytes) {
        return types.ErrInvalidSignature
    }
    
    // Check balance
    balance := k.GetBalance(ctx, tx.From)
    amount, _ := sdk.NewIntFromString(tx.Amount)
    if balance.LT(amount) {
        return types.ErrInsufficientFunds
    }
    
    return nil
}

// SettleOfflineTransaction - settle transaction when online
func (k Keeper) SettleOfflineTransaction(
    ctx sdk.Context,
    txID string,
) error {
    tx, found := k.GetOfflineTransaction(ctx, txID)
    if !found {
        return types.ErrTransactionNotFound
    }
    
    // Execute the transaction
    amount, _ := sdk.NewIntFromString(tx.Amount)
    
    // Deduct from sender
    senderBalance := k.GetBalance(ctx, tx.From)
    k.SetBalance(ctx, tx.From, senderBalance.Sub(amount))
    
    // Add to receiver
    receiverBalance := k.GetBalance(ctx, tx.To)
    k.SetBalance(ctx, tx.To, receiverBalance.Add(amount))
    
    // Mark as settled
    k.RemoveOfflineTransaction(ctx, txID)
    
    return nil
}
```

### Step 4.3: Implement Mesh Communication

Create `x/mesh/keeper/communication.go`:

```go
package keeper

import (
    "encoding/json"
    "time"
    
    "nomadcoin/x/mesh/types"
)

// MeshPeer represents a peer in the mesh network
type MeshPeer struct {
    ID           string    `json:"id"`
    Address      string    `json:"address"`
    Connection   string    `json:"connection"` // "bluetooth", "wifi_direct", "lora"
    LastSeen     time.Time `json:"last_seen"`
    TrustScore   float64   `json:"trust_score"`
}

// BroadcastTransaction - broadcast transaction to mesh network
func (k Keeper) BroadcastTransaction(peer MeshPeer, tx types.OfflineTransaction) error {
    msg := types.MeshMessage{
        Type:      types.MessageTypeTransaction,
        Sender:    k.GetNodeAddress(),
        Receiver:  peer.ID,
        Payload:   mustEncode(tx),
        Timestamp: time.Now().Unix(),
    }
    
    switch peer.Connection {
    case "bluetooth":
        return k.sendViaBluetooth(peer.Address, msg)
    case "wifi_direct":
        return k.sendViaWiFiDirect(peer.Address, msg)
    case "lora":
        return k.sendViaLoRa(peer.Address, msg)
    default:
        return k.sendViaInternet(peer.Address, msg)
    }
}

// PeerDiscovery - discover nearby mesh peers
func (k Keeper) PeerDiscovery(connectionType string) ([]MeshPeer, error) {
    switch connectionType {
    case "bluetooth":
        return k.discoverBluetoothPeers()
    case "wifi_direct":
        return k.discoverWiFiDirectPeers()
    case "lora":
        return k.discoverLoRaPeers()
    default:
        return nil, nil
    }
}
```

---

## Phase 5: Build Wallet and UI (Week 23-28)

### Step 5.1: Use Existing Wallet as Template

```bash
# Clone Keplr Wallet (Cosmos ecosystem)
git clone https://github.com/keplr-wallet/keplr.git

# Clone Cosmos Station
git clone https://github.com/cosmostation/cosmostation-mobile.git

# Clone Frontier Wallet (multi-chain)
git clone https://github.com/stridezone/stride.git
```

### Step 5.2: Create Custom Wallet

```bash
# Scaffold wallet module
starport scaffold module wallet --module-name wallet

# Add balance tracking
starport scaffold list Balance --set unilateral -y
```

### Step 5.3: Build Mobile Wallet App

```bash
# Create Flutter wallet
flutter create nomadcoin_wallet
cd nomadcoin_wallet

# Add dependencies
cat >> pubspec.yaml << 'EOF'
dependencies:
  flutter:
    sdk: flutter
  cosmos_sdk: ^2.0.0
  dio: ^5.4.0
  flutter_secure_storage: ^9.0.0
  qr_code_flutter: ^4.0.0
  intl: ^0.19.0
EOF

# Get dependencies
flutter pub get
```

Create wallet UI in `lib/main.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:cosmos_sdk/cosmos_sdk.dart';

void main() {
  runApp(const NomadCoinWalletApp());
}

class NomadCoinWalletApp extends StatelessWidget {
  const NomadCoinWalletApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'NomadCoin Wallet',
      theme: ThemeData(
        primarySwatch: Colors.blue,
        brightness: Brightness.dark,
      ),
      home: const WalletScreen(),
    );
  }
}

class WalletScreen extends StatefulWidget {
  const WalletScreen({super.key});

  @override
  State<WalletScreen> createState() => _WalletScreenState();
}

class _WalletScreenState extends State<WalletScreen> {
  final _cosmosSDK = CosmosSDK(
    chainId: 'nomadcoin',
    rpcUrl: 'https://rpc.nomadcoin.network',
    grpcUrl: 'https://grpc.nomadcoin.network',
  );
  
  String? _address;
  double _balance = 0.0;
  final _recipientController = TextEditingController();
  final _amountController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _loadWallet();
  }

  Future<void> _loadWallet() async {
    // Load or create wallet
    final wallet = await _cosmosSDK.loadWallet();
    setState(() {
      _address = wallet.address;
      _balance = wallet.balances['nomad'] ?? 0.0;
    });
  }

  Future<void> _sendTransaction() async {
    final tx = await _cosmosSDK.sendToken(
      toAddress: _recipientController.text,
      amount: double.parse(_amountController.text),
      denom: 'nomad',
    );
    
    if (tx.isSuccess) {
      _showSnackBar('Transaction sent!');
      _loadWallet();
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('NomadCoin Wallet'),
        backgroundColor: const Color(0xFF1A237E),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            // Balance Card
            Card(
              color: const Color(0xFF1A237E),
              child: Padding(
                padding: const EdgeInsets.all(24.0),
                child: Column(
                  children: [
                    const Text(
                      'Balance',
                      style: TextStyle(color: Colors.white70),
                    ),
                    const SizedBox(height: 8),
                    Text(
                      '$_balance NOMAD',
                      style: const TextStyle(
                        fontSize: 32,
                        fontWeight: FontWeight.bold,
                        color: Colors.white,
                      ),
                    ),
                    const SizedBox(height: 16),
                    Container(
                      padding: const EdgeInsets.symmetric(
                        horizontal: 12,
                        vertical: 8,
                      ),
                      decoration: BoxDecoration(
                        color: Colors.white12,
                        borderRadius: BorderRadius.circular(8),
                      ),
                      child: Text(
                        _address ?? 'Loading...',
                        style: const TextStyle(
                          color: Colors.white70,
                          fontSize: 12,
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 24),
            // Send Form
            TextField(
              controller: _recipientController,
              decoration: const InputDecoration(
                labelText: 'Recipient Address',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _amountController,
              keyboardType: TextInputType.number,
              decoration: const InputDecoration(
                labelText: 'Amount (NOMAD)',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 24),
            SizedBox(
              width: double.infinity,
              child: ElevatedButton.icon(
                onPressed: _sendTransaction,
                icon: const Icon(Icons.send),
                label: const Text('Send NOMAD'),
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.green,
                  padding: const EdgeInsets.all(16),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
  
  void _showSnackBar(String message) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text(message)),
    );
  }
}
```

---

## Phase 6: Build Mobile Miner App (Week 29-34)

### Step 6.1: Create Miner Application

```bash
# Create Flutter miner app
flutter create nomadcoin_miner
cd nomadcoin_miner

# Add dependencies
cat >> pubspec.yaml << 'EOF'
dependencies:
  flutter:
    sdk: flutter
  dio: ^5.4.0
  shared_preferences: ^2.2.2
  flutter_secure_storage: ^9.0.0
  battery_plus: ^6.0.0
  network_info_plus: ^5.0.0
  connectivity_plus: ^6.0.0
EOF

flutter pub get
```

### Step 6.2: Implement Miner Service

Create `lib/services/miner_service.dart`:

```dart
import 'dart:async';
import 'dart:convert';
import 'package:dio/dio.dart';
import 'package:connectivity_plus/connectivity_plus.dart';

class MinerService {
  final String _nodeUrl;
  bool _isMining = false;
  Timer? _validationTimer;
  int _validationsCount = 0;
  
  // Mobile-specific settings
  bool _isOnBattery = true;
  bool _isLowPowerMode = false;
  String _connectionType = 'unknown';
  
  MinerService([this._nodeUrl = 'https://api.nomadcoin.network']);
  
  bool get isMining => _isMining;
  int get validationsCount => _validationsCount;
  String get connectionType => _connectionType;

  /// Initialize miner service
  Future<void> initialize() async {
    // Check connectivity
    final connectivityResult = await Connectivity().checkConnectivity();
    _updateConnectionType(connectivityResult);
    
    // Listen for connectivity changes
    Connectivity().onConnectivityChanged.listen(_updateConnectionType);
  }
  
  void _updateConnectionType(List<ConnectivityResult> results) {
    if (results.contains(ConnectivityResult.wifi)) {
      _connectionType = 'wifi';
    } else if (results.contains(ConnectivityResult.mobile)) {
      _connectionType = 'mobile';
    } else if (results.contains(ConnectivityResult.none)) {
      _connectionType = 'offline';
    }
  }

  /// Start mining/validation
  Future<void> startMining(String walletAddress, {String? deviceId}) async {
    if (_isMining) return;
    
    _isMining = true;
    _validationsCount = 0;
    
    // Adaptive validation based on device state
    _validationTimer = Timer.periodic(
      Duration(seconds: _getValidationInterval()),
      (_) => _performValidation(walletAddress, deviceId),
    );
    
    // Register with network if online
    if (_connectionType != 'offline') {
      try {
        await Dio().post('$_nodeUrl/miner/register', data: {
          'address': walletAddress,
          'device_id': deviceId,
          'device_type': _getDeviceType(),
          'connection': _connectionType,
        });
      } catch (e) {
        print('Running in offline mode: $e');
      }
    }
  }
  
  int _getValidationInterval() {
    // More frequent if on WiFi, less if on mobile data
    if (_connectionType == 'wifi') return 1;
    if (_connectionType == 'mobile') return 5;
    return 10; // Offline mode
  }
  
  String _getDeviceType() {
    // Detect device type
    return 'mobile'; // Simplified
  }

  /// Stop mining
  void stopMining() {
    _isMining = false;
    _validationTimer?.cancel();
    _validationTimer = null;
  }

  /// Perform validation work
  Future<void> _performValidation(String walletAddress, String? deviceId) async {
    if (!_isMining) return;
    
    _validationsCount++;
    
    // In NomadPOS, mobile validators:
    // 1. Verify transaction signatures
    // 2. Check for double-spends
    // 3. Validate state changes
    
    try {
      // Try to report validation work
      await Dio().post(
        '$_nodeUrl/miner/report',
        data: {
          'address': walletAddress,
          'device_id': deviceId,
          'validations': 1,
          'timestamp': DateTime.now().millisecondsSinceEpoch,
        },
        options: Options(
          sendTimeout: const Duration(seconds: 3),
          receiveTimeout: const Duration(seconds: 3),
        ),
      );
    } catch (e) {
      // Store locally for later sync
      await _storePendingValidation(walletAddress);
    }
  }
  
  Future<void> _storePendingValidation(String walletAddress) async {
    // Store pending validations in local storage for later sync
    // This enables true offline mining
  }

  /// Sync pending validations when back online
  Future<void> syncPendingValidations(String walletAddress) async {
    if (_connectionType == 'offline') return;
    
    try {
      // Get pending validations from local storage
      final pending = await _getPendingValidations();
      
      if (pending.isNotEmpty) {
        await Dio().post('$_nodeUrl/miner/sync', data: {
          'address': walletAddress,
          'pending_validations': pending,
        });
      }
    } catch (e) {
      print('Sync failed: $e');
    }
  }
  
  Future<List<int>> _getPendingValidations() async {
    // Return pending validations count
    return [];
  }

  /// Get estimated earnings
  double calculateEarnings() {
    // Base: 0.01 NOMAD per validation * 1.5x mobile boost
    return _validationsCount * 0.01 * 1.5;
  }
}
```

### Step 6.3: Build Miner UI

Create the main miner screen:

```dart
// lib/main.dart
import 'package:flutter/material.dart';
import 'services/miner_service.dart';

void main() {
  runApp(const NomadCoinMinerApp());
}

class NomadCoinMinerApp extends StatelessWidget {
  const NomadCoinMinerApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'NomadCoin Miner',
      theme: ThemeData.dark().copyWith(
        primaryColor: const Color(0xFF00BCD4),
        scaffoldBackgroundColor: const Color(0xFF121212),
      ),
      home: const MinerScreen(),
    );
  }
}

class MinerScreen extends StatefulWidget {
  const MinerScreen({super.key});

  @override
  State<MinerScreen> createState() => _MinerScreenState();
}

class _MinerScreenState extends State<MinerScreen> {
  final _minerService = MinerService();
  final _addressController = TextEditingController();
  
  bool _isMining = false;
  int _validations = 0;
  String _connectionType = 'unknown';
  double _earnings = 0.0;

  @override
  void initState() {
    super.initState();
    _minerService.initialize().then((_) {
      setState(() {
        _connectionType = _minerService.connectionType;
      });
    });
  }

  @override
  void dispose() {
    _addressController.dispose();
    super.dispose();
  }

  void _toggleMining() {
    if (_isMining) {
      _minerService.stopMining();
    } else {
      _minerService.startMining(_addressController.text);
      
      // Start earnings update timer
      Timer.periodic(const Duration(seconds: 1), (timer) {
        if (!_isMining) {
          timer.cancel();
          return;
        }
        setState(() {
          _validations = _minerService.validationsCount;
          _earnings = _minerService.calculateEarnings();
        });
      });
    }
    
    setState(() {
      _isMining = !_isMining;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('NomadCoin Miner'),
        backgroundColor: const Color(0xFF00BCD4),
        actions: [
          IconButton(
            icon: const Icon(Icons.sync),
            onPressed: () => _minerService.syncPendingValidations(_addressController.text),
            tooltip: 'Sync',
          ),
        ],
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            // Connection Status
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: _connectionType == 'offline' 
                  ? Colors.orange.withOpacity(0.2)
                  : Colors.green.withOpacity(0.2),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Icon(
                    _connectionType == 'offline' ? Icons.cloud_off : Icons.wifi,
                    color: _connectionType == 'offline' ? Colors.orange : Colors.green,
                  ),
                  const SizedBox(width: 8),
                  Text(
                    _connectionType.toUpperCase(),
                    style: TextStyle(
                      color: _connectionType == 'offline' ? Colors.orange : Colors.green,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 24),
            
            // Wallet Input
            TextField(
              controller: _addressController,
              decoration: const InputDecoration(
                labelText: 'Wallet Address',
                hintText: 'nomad1...',
                prefixIcon: Icon(Icons.account_balance_wallet),
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 24),
            
            // Mining Stats
            Card(
              color: const Color(0xFF1E1E1E),
              child: Padding(
                padding: const EdgeInsets.all(24.0),
                child: Column(
                  children: [
                    Icon(
                      _isMining ? Icons.hardware : Icons.phone_android,
                      size: 64,
                      color: _isMining ? Colors.cyan : Colors.grey,
                    ),
                    const SizedBox(height: 16),
                    Text(
                      _isMining ? 'VALIDATING' : 'IDLE',
                      style: const TextStyle(
                        fontSize: 24,
                        fontWeight: FontWeight.bold,
                        letterSpacing: 2,
                      ),
                    ),
                    const SizedBox(height: 24),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                      children: [
                        _StatItem(
                          label: 'Validations',
                          value: '$_validations',
                          icon: Icons.check_circle,
                        ),
                        _StatItem(
                          label: 'Earnings',
                          value: '${_earnings.toStringAsFixed(4)} NOMAD',
                          icon: Icons.monetization_on,
                        ),
                      ],
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 24),
            
            // Mining Button
            SizedBox(
              width: double.infinity,
              height: 56,
              child: ElevatedButton(
                onPressed: _toggleMining,
                style: ElevatedButton.styleFrom(
                  backgroundColor: _isMining ? Colors.red : Colors.cyan,
                ),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(_isMining ? Icons.stop : Icons.play_arrow),
                    const SizedBox(width: 8),
                    Text(
                      _isMining ? 'STOP MINING' : 'START MINING',
                      style: const TextStyle(fontSize: 18),
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 16),
            
            // Info
            const Text(
              'Mobile miners validate transactions and help secure the NomadCoin network.\n'
              'Works offline - sync when back online!',
              textAlign: TextAlign.center,
              style: TextStyle(color: Colors.grey, fontSize: 12),
            ),
          ],
        ),
      ),
    );
  }
}

class _StatItem extends StatelessWidget {
  final String label;
  final String value;
  final IconData icon;
  
  const _StatItem({
    required this.label,
    required this.value,
    required this.icon,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Icon(icon, color: Colors.cyan),
        const SizedBox(height: 8),
        Text(
          value,
          style: const TextStyle(
            fontSize: 20,
            fontWeight: FontWeight.bold,
          ),
        ),
        Text(
          label,
          style: const TextStyle(color: Colors.grey),
        ),
      ],
    );
  }
}
```

### Step 6.4: Build Miner App

```bash
cd ~/dev/nomad_coin/nomadcoin_miner

# Build for Android
flutter build apk --debug

# Build for iOS (requires macOS)
flutter build ios --simulator --no-codesign

# Build for desktop
flutter config --enable-linux-desktop
flutter build linux
```

---

## Phase 7: Testnet Deployment (Week 35-40)

### Step 7.1: Configure Testnet

Create `testnet/config.yaml`:

```yaml
# Testnet Configuration
chain_id: nomadcoin-testnet
moniker: testnet-validator
genesis_time: "2026-04-06T00:00:00Z"

# P2P Configuration
p2p:
  listen_addr: "0.0.0.0:26656"
  seeds: "testseed@rpc.testnet.nomadcoin.network:26656"
  persistent_peers: ""

# RPC Configuration
rpc:
  addr: "0.0.0.0:26657"
  cors_allowed_origins: ["*"]

# State Sync
statesync:
  enable: true
  rpc_servers: "rpc.testnet1.nomadcoin.network,rpc.testnet2.nomadcoin.network"
  trust_height: 1000
  trust_hash: "..."

# Minimum Gas Prices
min_gas_prices: "0.001nomad"
```

### Step 7.2: Launch Testnet

```bash
cd ~/dev/nomad_coin

# Build node binary
cd nomadcoin
make install

# Initialize testnet
nomadcoind init testnet --chain-id nomadcoin-testnet

# Add genesis account (with community tokens)
nomadcoind add-genesis-account $(nomadcoind keys show wallet -a) 100000000nomad

# Create gentxs
nomadcoind gentx validator 10000000nomad --chain-id nomadcoin-testnet

# Collect gentxs
nomadcoind collect-gentxs

# Start testnet
nomadcoind start --chain-id nomadcoin-testnet
```

### Step 7.3: Docker Deployment

Create `Dockerfile`:

```dockerfile
FROM golang:1.21-alpine AS builder

WORKDIR /app
COPY . .
RUN go build -o /nomadcoind ./cmd/nomadcoind

FROM alpine:3.19
RUN apk --no-cache add ca-certificates
WORKDIR /app
COPY --from=builder /nomadcoind /usr/local/bin/
COPY testnet /app/testnet

EXPOSE 26656 26657 26660
ENTRYPOINT ["nomadcoind", "start", "--home=/app/testnet"]
```

Build and run:

```bash
docker build -t nomadcoin/node:latest .
docker run -d -p 26656:26656 -p 26657:26657 nomadcoin/node:latest
```

---

## Phase 8: Mainnet Launch (Week 41-48)

### Step 8.1: Security Audit

```bash
# Run security checks
gosec -fmt=sarif -out=gosec-results.sarif ./...
golangci-lint run
```

### Step 8.2: Launch Mainnet

```bash
# Create mainnet configuration
mkdir -p mainnet
nomadcoind init mainnet --chain-id nomadcoin-mainnet

# Set mainnet parameters
# (Higher staking requirements, longer governance periods, etc.)

# Launch with validator set
nomadcoind start --chain-id nomadcoin-mainnet
```

---

## Development Commands Summary

### Build
```bash
# Build blockchain
make build

# Build with Docker
docker build -t nomadcoin:latest .

# Run tests
make test

# Run integration tests
make test-integration
```

### Deploy
```bash
# Start testnet locally
make localnet-start

# Deploy to Kubernetes (requires kubeconfig)
kubectl apply -f k8s/
```

### Development
```bash
# Lint code
make lint

# Generate protobuf
make proto-gen

# Update Swagger docs
make docs
```

---

## Estimated Timeline & Cost Summary

| Phase | Duration | Estimated Cost |
|-------|----------|----------------|
| Environment Setup | 2 weeks | $0 |
| Blockchain Customization | 8 weeks | $50,000-80,000 |
| Mobile Node Development | 6 weeks | $30,000-50,000 |
| Mesh Networking | 6 weeks | $40,000-60,000 |
| Wallet Development | 6 weeks | $25,000-40,000 |
| Mobile Miner | 6 weeks | $20,000-35,000 |
| Testing & Audit | 6 weeks | $30,000-50,000 |
| Mainnet Launch | 8 weeks | $50,000-100,000 |
| **Total** | **48 weeks** | **$245,000-415,000** |

---

## Next Steps

1. **Select development team** - Hire or contract Go developers with Cosmos SDK experience
2. **Finalize tokenomics** - Adjust supply, rewards, and inflation parameters
3. **Legal consultation** - Consult crypto lawyers for your jurisdiction
4. **Security audit** - Schedule professional security audit (6-8 weeks)
5. **Community building** - Launch marketing before mainnet
6. **Exchange outreach** - Begin exchange listing conversations early