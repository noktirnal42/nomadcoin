import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../services/miner_service.dart';
import '../services/wallet_service.dart';

class MinerScreen extends StatefulWidget {
  const MinerScreen({super.key});

  @override
  State<MinerScreen> createState() => _MinerScreenState();
}

class _MinerScreenState extends State<MinerScreen> {
  final _addressController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _loadSavedAddress();
  }

  @override
  void dispose() {
    _addressController.dispose();
    super.dispose();
  }

  Future<void> _loadSavedAddress() async {
    final walletService = context.read<WalletService>();
    final savedAddress = await walletService.getSavedAddress();
    if (savedAddress != null && mounted) {
      setState(() {
        _addressController.text = savedAddress;
      });
    }
  }

  void _toggleMining() {
    final minerService = context.read<MinerService>();
    final walletService = context.read<WalletService>();

    if (minerService.isMining) {
      minerService.stopMining();
    } else {
      minerService.startMining(
        _addressController.text,
        deviceId: walletService.deviceId,
      );
      walletService.saveAddress(_addressController.text);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('NomadCoin Miner'),
        actions: [
          Consumer<MinerService>(
            builder: (context, service, child) {
              if (service.connectionType != 'offline') {
                return IconButton(
                  icon: const Icon(Icons.sync),
                  onPressed: () =>
                      service.syncPendingValidations(_addressController.text),
                  tooltip: 'Sync with Network',
                );
              }
              return const SizedBox.shrink();
            },
          ),
        ],
      ),
      body: Consumer<MinerService>(
        builder: (context, minerService, child) {
          return Padding(
            padding: const EdgeInsets.all(16.0),
            child: Column(
              children: [
                // Connection Status Banner
                _ConnectionBanner(connectionType: minerService.connectionType),
                const SizedBox(height: 24),

                // Wallet Address Input
                TextField(
                  controller: _addressController,
                  style: const TextStyle(color: Colors.white),
                  decoration: InputDecoration(
                    labelText: 'Wallet Address',
                    hintText: 'nomad1...',
                    hintStyle: const TextStyle(color: Colors.grey),
                    prefixIcon: const Icon(
                      Icons.account_balance_wallet,
                      color: Colors.cyan,
                    ),
                    border: const OutlineInputBorder(),
                    enabledBorder: const OutlineInputBorder(
                      borderSide: BorderSide(color: Colors.grey),
                    ),
                    focusedBorder: const OutlineInputBorder(
                      borderSide: BorderSide(color: Colors.cyan),
                    ),
                  ),
                ),
                const SizedBox(height: 24),

                // Mining Stats Card
                _MiningStatsCard(
                  isMining: minerService.isMining,
                  validations: minerService.validationsCount,
                  earnings: minerService.earnings,
                ),
                const SizedBox(height: 24),

                // Mining Button
                SizedBox(
                  width: double.infinity,
                  height: 56,
                  child: ElevatedButton(
                    onPressed: _toggleMining,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: minerService.isMining
                          ? Colors.red
                          : Colors.cyan,
                      foregroundColor: Colors.white,
                    ),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(
                          minerService.isMining ? Icons.stop : Icons.play_arrow,
                        ),
                        const SizedBox(width: 8),
                        Text(
                          minerService.isMining
                              ? 'STOP MINING'
                              : 'START MINING',
                          style: const TextStyle(
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 16),

                // Info Text
                const Text(
                  'Mobile miners validate transactions and help secure the NomadCoin network.\n'
                  'Works offline — sync when back online!\n'
                  'Mobile devices get 1.5x reward boost.',
                  textAlign: TextAlign.center,
                  style: TextStyle(color: Colors.grey, fontSize: 12),
                ),
              ],
            ),
          );
        },
      ),
    );
  }
}

class _ConnectionBanner extends StatelessWidget {
  final String connectionType;

  const _ConnectionBanner({required this.connectionType});

  @override
  Widget build(BuildContext context) {
    final isOnline = connectionType != 'offline';

    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: isOnline
            ? Colors.green.withOpacity(0.15)
            : Colors.orange.withOpacity(0.15),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(
          color: isOnline
              ? Colors.green.withOpacity(0.3)
              : Colors.orange.withOpacity(0.3),
        ),
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(
            isOnline ? Icons.wifi : Icons.cloud_off,
            color: isOnline ? Colors.green : Colors.orange,
            size: 20,
          ),
          const SizedBox(width: 8),
          Text(
            connectionType.toUpperCase(),
            style: TextStyle(
              color: isOnline ? Colors.green : Colors.orange,
              fontWeight: FontWeight.bold,
              fontSize: 14,
            ),
          ),
        ],
      ),
    );
  }
}

class _MiningStatsCard extends StatelessWidget {
  final bool isMining;
  final int validations;
  final double earnings;

  const _MiningStatsCard({
    required this.isMining,
    required this.validations,
    required this.earnings,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      color: const Color(0xFF1E1E1E),
      child: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          children: [
            Icon(
              isMining ? Icons.hardware : Icons.phone_android,
              size: 64,
              color: isMining ? Colors.cyan : Colors.grey,
            ),
            const SizedBox(height: 16),
            Text(
              isMining ? 'VALIDATING' : 'IDLE',
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
                  value: '$validations',
                  icon: Icons.check_circle,
                ),
                _StatItem(
                  label: 'Earnings',
                  value: '${earnings.toStringAsFixed(4)} NOMAD',
                  icon: Icons.monetization_on,
                ),
              ],
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
        Icon(icon, color: Colors.cyan, size: 28),
        const SizedBox(height: 8),
        Text(
          value,
          style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
        ),
        Text(label, style: const TextStyle(color: Colors.grey, fontSize: 12)),
      ],
    );
  }
}
