import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../services/wallet_service.dart';

class WalletScreen extends StatefulWidget {
  const WalletScreen({super.key});

  @override
  State<WalletScreen> createState() => _WalletScreenState();
}

class _WalletScreenState extends State<WalletScreen> {
  bool _isLoading = true;

  @override
  void initState() {
    super.initState();
    _loadWalletData();
  }

  Future<void> _loadWalletData() async {
    final walletService = context.read<WalletService>();
    await walletService.loadBalance();
    await walletService.loadTransactions();
    if (mounted) {
      setState(() {
        _isLoading = false;
      });
    }
  }

  void _createNewWallet() {
    final walletService = context.read<WalletService>();
    final address = walletService.generateNewAddress();
    walletService.saveAddress(address);
    _loadWalletData();

    if (mounted) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('New wallet created: $address')));
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('NomadCoin Wallet'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: () {
              setState(() => _isLoading = true);
              _loadWalletData();
            },
          ),
        ],
      ),
      body: _isLoading
          ? const Center(child: CircularProgressIndicator())
          : Consumer<WalletService>(
              builder: (context, walletService, child) {
                return Padding(
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
                                '${walletService.balance.toStringAsFixed(2)} NOMAD',
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
                                  walletService.address ?? 'No wallet created',
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

                      // Action Buttons
                      Row(
                        children: [
                          Expanded(
                            child: ElevatedButton.icon(
                              onPressed: _createNewWallet,
                              icon: const Icon(Icons.add),
                              label: const Text('New Wallet'),
                              style: ElevatedButton.styleFrom(
                                backgroundColor: Colors.cyan,
                                padding: const EdgeInsets.all(12),
                              ),
                            ),
                          ),
                          const SizedBox(width: 12),
                          Expanded(
                            child: OutlinedButton.icon(
                              onPressed: () {},
                              icon: const Icon(Icons.qr_code),
                              label: const Text('Receive'),
                              style: OutlinedButton.styleFrom(
                                foregroundColor: Colors.cyan,
                                side: const BorderSide(color: Colors.cyan),
                                padding: const EdgeInsets.all(12),
                              ),
                            ),
                          ),
                        ],
                      ),
                      const SizedBox(height: 24),

                      // Transaction History
                      const Align(
                        alignment: Alignment.centerLeft,
                        child: Text(
                          'Recent Transactions',
                          style: TextStyle(
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ),
                      const SizedBox(height: 12),
                      Expanded(
                        child: walletService.transactions.isEmpty
                            ? const Center(
                                child: Text(
                                  'No transactions yet',
                                  style: TextStyle(color: Colors.grey),
                                ),
                              )
                            : ListView.builder(
                                itemCount: walletService.transactions.length,
                                itemBuilder: (context, index) {
                                  final tx = walletService.transactions[index];
                                  return _TransactionListItem(
                                    type: tx['type'],
                                    amount: tx['amount'],
                                    address: tx['from'] ?? tx['to'] ?? '',
                                    timestamp: tx['timestamp'],
                                    status: tx['status'],
                                  );
                                },
                              ),
                      ),
                    ],
                  ),
                );
              },
            ),
    );
  }
}

class _TransactionListItem extends StatelessWidget {
  final String type;
  final double amount;
  final String address;
  final String timestamp;
  final String status;

  const _TransactionListItem({
    required this.type,
    required this.amount,
    required this.address,
    required this.timestamp,
    required this.status,
  });

  @override
  Widget build(BuildContext context) {
    final isReceive = type == 'receive';

    return Card(
      margin: const EdgeInsets.only(bottom: 8),
      child: ListTile(
        leading: CircleAvatar(
          backgroundColor: isReceive
              ? Colors.green.withOpacity(0.2)
              : Colors.red.withOpacity(0.2),
          child: Icon(
            isReceive ? Icons.arrow_downward : Icons.arrow_upward,
            color: isReceive ? Colors.green : Colors.red,
          ),
        ),
        title: Text(
          '${isReceive ? '+' : '-'}${amount.toStringAsFixed(2)} NOMAD',
          style: TextStyle(
            color: isReceive ? Colors.green : Colors.red,
            fontWeight: FontWeight.bold,
          ),
        ),
        subtitle: Text(
          '$address\n${DateTime.parse(timestamp).toString().substring(0, 19)}',
        ),
        trailing: Chip(
          label: Text(status, style: const TextStyle(fontSize: 10)),
          backgroundColor: Colors.green.withOpacity(0.2),
        ),
      ),
    );
  }
}
