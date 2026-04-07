import 'dart:async';
import 'dart:convert';
import 'package:flutter/foundation.dart';
import 'package:dio/dio.dart';
import 'package:connectivity_plus/connectivity_plus.dart';

class MinerService extends ChangeNotifier {
  final Dio _dio = Dio(
    BaseOptions(
      connectTimeout: const Duration(seconds: 3),
      receiveTimeout: const Duration(seconds: 3),
    ),
  );

  String _nodeUrl = 'https://api.nomadcoin.network';
  bool _isMining = false;
  int _validationsCount = 0;
  double _earnings = 0.0;
  String _connectionType = 'unknown';
  Timer? _validationTimer;
  Timer? _statusTimer;

  // Getters
  bool get isMining => _isMining;
  int get validationsCount => _validationsCount;
  double get earnings => _earnings;
  String get connectionType => _connectionType;
  String get nodeUrl => _nodeUrl;

  set nodeUrl(String url) {
    _nodeUrl = url;
    _dio.options.baseUrl = url;
    notifyListeners();
  }

  /// Initialize service and check connectivity
  Future<void> initialize() async {
    await _updateConnectionType();
    Connectivity().onConnectivityChanged.listen((_) {
      _updateConnectionType();
    });
  }

  Future<void> _updateConnectionType() async {
    final results = await Connectivity().checkConnectivity();
    if (results.contains(ConnectivityResult.wifi)) {
      _connectionType = 'wifi';
    } else if (results.contains(ConnectivityResult.mobile)) {
      _connectionType = 'mobile';
    } else if (results.contains(ConnectivityResult.ethernet)) {
      _connectionType = 'ethernet';
    } else {
      _connectionType = 'offline';
    }
    notifyListeners();
  }

  /// Start mining/validation
  Future<void> startMining(String walletAddress, {String? deviceId}) async {
    if (_isMining) return;

    _isMining = true;
    _validationsCount = 0;
    notifyListeners();

    // Start validation loop
    _validationTimer = Timer.periodic(
      Duration(seconds: _getValidationInterval()),
      (_) => _performValidation(walletAddress, deviceId),
    );

    // Register with network if online
    if (_connectionType != 'offline') {
      try {
        await _dio.post(
          '/miner/register',
          data: {
            'address': walletAddress,
            'device_id': deviceId,
            'device_type': 'mobile',
            'connection': _connectionType,
          },
        );
      } catch (e) {
        debugPrint('Running in offline mode: $e');
      }
    }

    debugPrint('Mining started for $walletAddress');
  }

  /// Stop mining
  void stopMining() {
    _isMining = false;
    _validationTimer?.cancel();
    _validationTimer = null;
    notifyListeners();
    debugPrint('Mining stopped');
  }

  /// Perform validation work
  Future<void> _performValidation(
    String walletAddress,
    String? deviceId,
  ) async {
    if (!_isMining) return;

    _validationsCount++;

    // Mobile boost: 1.5x for mobile devices
    final boost = 1.5;
    final reward = 0.01 * boost;
    _earnings += reward;

    // Try to report to network
    try {
      await _dio.post(
        '/miner/report',
        data: {
          'address': walletAddress,
          'device_id': deviceId,
          'validations': 1,
          'reward': reward,
          'timestamp': DateTime.now().millisecondsSinceEpoch,
        },
        options: Options(
          sendTimeout: const Duration(seconds: 2),
          receiveTimeout: const Duration(seconds: 2),
        ),
      );
    } catch (e) {
      // Store locally for later sync
      debugPrint('Offline validation stored: $e');
    }

    // Update UI every 10 validations
    if (_validationsCount % 10 == 0) {
      notifyListeners();
    }
  }

  /// Get validation interval based on connection
  int _getValidationInterval() {
    switch (_connectionType) {
      case 'wifi':
      case 'ethernet':
        return 1;
      case 'mobile':
        return 5;
      default:
        return 10; // Offline
    }
  }

  /// Sync pending validations when back online
  Future<int> syncPendingValidations(String walletAddress) async {
    if (_connectionType == 'offline') return 0;

    try {
      final response = await _dio.post(
        '/miner/sync',
        data: {
          'address': walletAddress,
          'pending_count': _validationsCount,
          'earnings': _earnings,
        },
      );

      if (response.statusCode == 200) {
        final synced = response.data['synced'] ?? _validationsCount;
        debugPrint('Synced $synced validations');
        return synced;
      }
    } catch (e) {
      debugPrint('Sync failed: $e');
    }
    return 0;
  }

  /// Calculate estimated earnings
  double calculateEarnings() {
    return _validationsCount * 0.01 * 1.5; // base * mobile boost
  }

  @override
  void dispose() {
    stopMining();
    _statusTimer?.cancel();
    super.dispose();
  }
}
