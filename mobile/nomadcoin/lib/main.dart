import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'dart:math';
import 'dart:async';
import 'package:qr_flutter/qr_flutter.dart';

void main() {
  runApp(const NomadCoinApp());
}

class NomadCoinApp extends StatelessWidget {
  const NomadCoinApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'NomadCoin',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        useMaterial3: true,
        colorScheme: ColorScheme.dark(
          primary: const Color(0xFF00BCD4),
          secondary: const Color(0xFF4CAF50),
          surface: const Color(0xFF1E1E1E),
        ),
        scaffoldBackgroundColor: const Color(0xFF121212),
        appBarTheme: const AppBarTheme(
          backgroundColor: Color(0xFF1A237E),
          elevation: 0,
          centerTitle: true,
        ),
      ),
      home: const MainScreen(),
    );
  }
}

class MainScreen extends StatefulWidget {
  const MainScreen({super.key});

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  int _selectedIndex = 0;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Row(
          mainAxisSize: MainAxisSize.min,
          children: [const Text('⛓️ '), Text(_titles[_selectedIndex])],
        ),
        actions: [
          IconButton(
            icon: const Icon(Icons.info_outline),
            onPressed: _showAbout,
          ),
        ],
      ),
      body: IndexedStack(
        index: _selectedIndex,
        children: const [
          MinerScreen(),
          WalletScreen(),
          SendScreen(),
          CommunityScreen(),
        ],
      ),
      bottomNavigationBar: NavigationBar(
        selectedIndex: _selectedIndex,
        onDestinationSelected: (i) => setState(() => _selectedIndex = i),
        destinations: const [
          NavigationDestination(icon: Icon(Icons.hardware), label: 'Mine'),
          NavigationDestination(
            icon: Icon(Icons.account_balance_wallet),
            label: 'Wallet',
          ),
          NavigationDestination(icon: Icon(Icons.send), label: 'Send'),
          NavigationDestination(icon: Icon(Icons.people), label: 'Community'),
        ],
      ),
    );
  }

  final _titles = ['Miner', 'Wallet', 'Send', 'Community'];

  void _showAbout() {
    showDialog(
      context: context,
      builder: (_) => AlertDialog(
        title: const Text('⛓️ NomadCoin'),
        content: const Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Version 1.0.0'),
            SizedBox(height: 8),
            Text('Mobile-first crypto for the nomadic community'),
            SizedBox(height: 8),
            Text('📱 Mobile = 1.5x mining bonus!'),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('OK'),
          ),
        ],
      ),
    );
  }
}

// ============ MINER SCREEN ============
class MinerScreen extends StatefulWidget {
  const MinerScreen({super.key});

  @override
  State<MinerScreen> createState() => _MinerScreenState();
}

class _MinerScreenState extends State<MinerScreen> {
  bool _isMining = false;
  double _earnings = 0.0;
  int _validations = 0;
  Timer? _timer;

  String get _deviceType => 'iOS';
  double get _boost => 1.5;

  @override
  void dispose() {
    _timer?.cancel();
    super.dispose();
  }

  void _toggleMining() {
    setState(() {
      _isMining = !_isMining;
      if (_isMining) {
        _timer = Timer.periodic(const Duration(seconds: 2), (_) {
          _earnings += 0.01 * _boost;
          _validations++;
        });
      } else {
        _timer?.cancel();
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                children: [
                  Icon(
                    _isMining ? Icons.hardware : Icons.hardware_outlined,
                    size: 64,
                    color: _isMining ? Colors.green : Colors.grey,
                  ),
                  const SizedBox(height: 8),
                  Text(
                    _isMining ? 'MINING' : 'Idle',
                    style: TextStyle(
                      fontSize: 24,
                      fontWeight: FontWeight.bold,
                      color: _isMining ? Colors.green : Colors.grey,
                    ),
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 16),
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [const Text('Device:'), Text(_deviceType)],
                  ),
                  const SizedBox(height: 8),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      const Text('Boost:'),
                      Text(
                        '${_boost}x 🏃',
                        style: const TextStyle(
                          color: Colors.green,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 16),
          SizedBox(
            width: double.infinity,
            child: FilledButton.icon(
              onPressed: _toggleMining,
              icon: Icon(_isMining ? Icons.stop : Icons.play_arrow),
              label: Text(_isMining ? 'Stop Mining' : 'Start Mining'),
              style: FilledButton.styleFrom(
                backgroundColor: _isMining ? Colors.red : Colors.green,
                padding: const EdgeInsets.all(16),
              ),
            ),
          ),
          const SizedBox(height: 16),
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      const Text('Earnings:'),
                      Text(
                        '${_earnings.toStringAsFixed(4)} NOMAD',
                        style: const TextStyle(
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 8),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      const Text('Validations:'),
                      Text('$_validations'),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

// ============ WALLET SCREEN ============
class WalletScreen extends StatefulWidget {
  const WalletScreen({super.key});

  @override
  State<WalletScreen> createState() => _WalletScreenState();
}

class _WalletScreenState extends State<WalletScreen> {
  String _address = '';
  double _balance = 10000.0;

  @override
  void initState() {
    super.initState();
    _address = 'nomad1${_generateAddress()}';
  }

  String _generateAddress() {
    const chars = 'abcdefghijklmnopqrstuvwxyz0123456789';
    return List.generate(
      24,
      (_) => chars[Random().nextInt(chars.length)],
    ).join();
  }

  void _copyAddress() {
    Clipboard.setData(ClipboardData(text: _address));
    ScaffoldMessenger.of(
      context,
    ).showSnackBar(const SnackBar(content: Text('Address copied!')));
  }

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        children: [
          Card(
            color: const Color(0xFF1A237E),
            child: Padding(
              padding: const EdgeInsets.all(24),
              child: Column(
                children: [
                  const Text(
                    'Balance',
                    style: TextStyle(color: Colors.white70),
                  ),
                  const SizedBox(height: 8),
                  Text(
                    '${_balance.toStringAsFixed(4)} NOMAD',
                    style: const TextStyle(
                      fontSize: 32,
                      fontWeight: FontWeight.bold,
                      color: Colors.white,
                    ),
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 16),
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                children: [
                  const Text(
                    'Your Address:',
                    style: TextStyle(fontWeight: FontWeight.bold),
                  ),
                  const SizedBox(height: 8),
                  Center(
                    child: QrImageView(
                      data: _address,
                      size: 150,
                      backgroundColor: Colors.white,
                    ),
                  ),
                  const SizedBox(height: 16),
                  Container(
                    padding: const EdgeInsets.all(8),
                    decoration: BoxDecoration(
                      color: Colors.grey[900],
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: SelectableText(
                      _address,
                      style: const TextStyle(
                        fontFamily: 'monospace',
                        fontSize: 12,
                      ),
                    ),
                  ),
                  const SizedBox(height: 8),
                  SizedBox(
                    width: double.infinity,
                    child: FilledButton.icon(
                      onPressed: _copyAddress,
                      icon: const Icon(Icons.copy),
                      label: const Text('Copy Address'),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

// ============ SEND SCREEN ============
class SendScreen extends StatefulWidget {
  const SendScreen({super.key});

  @override
  State<SendScreen> createState() => _SendScreenState();
}

class _SendScreenState extends State<SendScreen> {
  final _toController = TextEditingController();
  final _amountController = TextEditingController();
  final _memoController = TextEditingController();

  @override
  void dispose() {
    _toController.dispose();
    _amountController.dispose();
    _memoController.dispose();
    super.dispose();
  }

  void _send() {
    if (_toController.text.isEmpty || _amountController.text.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Please fill in recipient and amount')),
      );
      return;
    }

    showDialog(
      context: context,
      builder: (_) => AlertDialog(
        title: const Text('Confirm Transaction'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('To: ${_toController.text.substring(0, 16)}...'),
            Text('Amount: ${_amountController.text} NOMAD'),
            if (_memoController.text.isNotEmpty)
              Text('Memo: ${_memoController.text}'),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Cancel'),
          ),
          FilledButton(
            onPressed: () {
              Navigator.pop(context);
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(content: Text('Transaction sent!')),
              );
              _toController.clear();
              _amountController.clear();
              _memoController.clear();
            },
            child: const Text('Send'),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            'Send NOMAD',
            style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 16),
          TextField(
            controller: _toController,
            decoration: const InputDecoration(
              labelText: 'To Address',
              hintText: 'nomad1...',
              prefixIcon: Icon(Icons.person),
            ),
          ),
          const SizedBox(height: 16),
          TextField(
            controller: _amountController,
            keyboardType: TextInputType.number,
            decoration: const InputDecoration(
              labelText: 'Amount',
              hintText: '0.0',
              prefixIcon: Icon(Icons.numbers),
              suffixText: 'NOMAD',
            ),
          ),
          const SizedBox(height: 16),
          TextField(
            controller: _memoController,
            decoration: const InputDecoration(
              labelText: 'Memo (optional)',
              hintText: 'Payment for...',
              prefixIcon: Icon(Icons.note),
            ),
          ),
          const SizedBox(height: 24),
          SizedBox(
            width: double.infinity,
            child: FilledButton.icon(
              onPressed: _send,
              icon: const Icon(Icons.send),
              label: const Text('Send Transaction'),
              style: FilledButton.styleFrom(padding: EdgeInsets.all(16)),
            ),
          ),
        ],
      ),
    );
  }
}

// ============ COMMUNITY SCREEN ============
class CommunityScreen extends StatefulWidget {
  const CommunityScreen({super.key});

  @override
  State<CommunityScreen> createState() => _CommunityScreenState();
}

class _CommunityScreenState extends State<CommunityScreen> {
  bool _offlineMode = false;
  int _peers = 0;
  bool _hotspot = false;

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        children: [
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  const Text('Connection:'),
                  Text(
                    _offlineMode ? '📴 Offline' : '🟢 Online',
                    style: TextStyle(
                      color: _offlineMode ? Colors.orange : Colors.green,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 16),
          Card(
            child: SwitchListTile(
              title: const Text('Offline Mode'),
              subtitle: const Text('For travel without internet'),
              secondary: Icon(
                Icons.wifi_off,
                color: _offlineMode ? Colors.orange : Colors.grey,
              ),
              value: _offlineMode,
              onChanged: (v) => setState(() => _offlineMode = v),
            ),
          ),
          const SizedBox(height: 16),
          Card(
            child: SwitchListTile(
              title: const Text('Mobile Hotspot'),
              subtitle: const Text('Connect via hotspot for 1.5x bonus'),
              secondary: Icon(
                Icons.wifi_tethering,
                color: _hotspot ? Colors.green : Colors.grey,
              ),
              value: _hotspot,
              onChanged: (v) => setState(() => _hotspot = v),
            ),
          ),
          const SizedBox(height: 16),
          Card(
            child: ListTile(
              leading: const Icon(Icons.people),
              title: const Text('Peer Network'),
              subtitle: Text('$_peers peers connected'),
              trailing: IconButton(
                icon: const Icon(Icons.refresh),
                onPressed: () => setState(() => _peers = Random().nextInt(10)),
              ),
            ),
          ),
          const SizedBox(height: 16),
          const Card(
            child: Padding(
              padding: EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    '💡 Nomad Tips:',
                    style: TextStyle(fontWeight: FontWeight.bold),
                  ),
                  SizedBox(height: 8),
                  Text('• Go offline when traveling'),
                  Text('• Use mesh network to sync'),
                  Text('• Mobile hotspot = 1.5x bonus'),
                  Text('• Share with fellow nomads'),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
