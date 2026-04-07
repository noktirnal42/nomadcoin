import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../services/miner_service.dart';

class SettingsScreen extends StatefulWidget {
  const SettingsScreen({super.key});

  @override
  State<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends State<SettingsScreen> {
  final _nodeUrlController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _loadSettings();
  }

  @override
  void dispose() {
    _nodeUrlController.dispose();
    super.dispose();
  }

  void _loadSettings() {
    final minerService = context.read<MinerService>();
    _nodeUrlController.text = minerService.nodeUrl;
  }

  void _saveSettings() {
    final minerService = context.read<MinerService>();
    minerService.nodeUrl = _nodeUrlController.text;

    if (mounted) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(const SnackBar(content: Text('Settings saved!')));
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Settings')),
      body: ListView(
        padding: const EdgeInsets.all(16.0),
        children: [
          // Node Configuration
          const Text(
            'Node Configuration',
            style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 12),
          TextField(
            controller: _nodeUrlController,
            style: const TextStyle(color: Colors.white),
            decoration: const InputDecoration(
              labelText: 'Node URL',
              hintText: 'https://api.nomadcoin.network',
              border: OutlineInputBorder(),
              prefixIcon: Icon(Icons.dns, color: Colors.cyan),
            ),
          ),
          const SizedBox(height: 12),
          SizedBox(
            width: double.infinity,
            child: ElevatedButton.icon(
              onPressed: _saveSettings,
              icon: const Icon(Icons.save),
              label: const Text('Save Settings'),
              style: ElevatedButton.styleFrom(
                backgroundColor: Colors.cyan,
                padding: const EdgeInsets.all(12),
              ),
            ),
          ),
          const SizedBox(height: 32),

          // About
          const Text(
            'About',
            style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 12),
          const ListTile(
            leading: Icon(Icons.info, color: Colors.cyan),
            title: Text('NomadCoin'),
            subtitle: Text('Version 1.0.0'),
          ),
          const ListTile(
            leading: Icon(Icons.description, color: Colors.cyan),
            title: Text('Documentation'),
            subtitle: Text('docs.nomadcoin.network'),
            onTap: null, // Add URL launcher
          ),
          const ListTile(
            leading: Icon(Icons.code, color: Colors.cyan),
            title: Text('Source Code'),
            subtitle: Text('github.com/nomadcoin'),
            onTap: null, // Add URL launcher
          ),
          const SizedBox(height: 32),

          // Mining Info
          const Text(
            'Mining Information',
            style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 12),
          const Card(
            color: Color(0xFF1E1E1E),
            child: Padding(
              padding: EdgeInsets.all(16.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'How Mining Works',
                    style: TextStyle(fontWeight: FontWeight.bold, fontSize: 16),
                  ),
                  SizedBox(height: 8),
                  Text(
                    'NomadCoin uses Proof-of-Stake with mobile validation.\n\n'
                    '• Your device validates transactions\n'
                    '• Mobile devices get 1.5x reward boost\n'
                    '• Works offline - sync when back online\n'
                    '• Battery friendly - no heavy computation\n'
                    '• Earn ~0.015 NOMAD per validation (mobile)',
                    style: TextStyle(color: Colors.grey),
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
