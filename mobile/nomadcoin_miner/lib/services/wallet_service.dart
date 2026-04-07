import 'package:flutter/foundation.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:crypto/crypto.dart';
import 'dart:convert';
import 'dart:math';

class WalletService extends ChangeNotifier {
  String? _address;
  double _balance = 0.0;
  List<Map<String, dynamic>> _transactions = [];
  final Random _random = Random.secure();

  // Getters
  String? get address => _address;
  double get balance => _balance;
  List<Map<String, dynamic>> get transactions => _transactions;
  String get deviceId => _generateDeviceId();

  /// Load saved wallet address
  Future<String?> getSavedAddress() async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.getString('wallet_address');
  }

  /// Save wallet address
  Future<void> saveAddress(String address) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('wallet_address', address);
    _address = address;
    notifyListeners();
  }

  /// Generate a new wallet address (simplified)
  String generateNewAddress() {
    final bytes = List<int>.generate(32, (_) => _random.nextInt(256));
    final hash = sha256.convert(bytes).toString();
    _address = 'nomad1${hash.substring(0, 38)}';
    notifyListeners();
    return _address!;
  }

  /// Load wallet balance (simulated)
  Future<void> loadBalance() async {
    // In production, fetch from node API
    await Future.delayed(const Duration(milliseconds: 500));
    _balance = 100.0; // Simulated balance
    notifyListeners();
  }

  /// Load transaction history (simulated)
  Future<void> loadTransactions() async {
    // In production, fetch from node API
    await Future.delayed(const Duration(milliseconds: 500));
    _transactions = [
      {
        'type': 'receive',
        'amount': 50.0,
        'from': 'nomad1sender...',
        'timestamp': DateTime.now()
            .subtract(const Duration(hours: 2))
            .toIso8601String(),
        'status': 'confirmed',
      },
      {
        'type': 'send',
        'amount': 10.0,
        'to': 'nomad1recipient...',
        'timestamp': DateTime.now()
            .subtract(const Duration(days: 1))
            .toIso8601String(),
        'status': 'confirmed',
      },
    ];
    notifyListeners();
  }

  /// Generate device ID
  String _generateDeviceId() {
    final bytes = List<int>.generate(16, (_) => _random.nextInt(256));
    return base64
        .encode(bytes)
        .replaceAll(RegExp(r'[^a-zA-Z0-9]'), '')
        .substring(0, 16);
  }

  @override
  void dispose() {
    super.dispose();
  }
}
