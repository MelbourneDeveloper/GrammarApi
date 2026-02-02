import 'dart:convert';
import 'dart:io';

import 'package:flutter/foundation.dart' show kIsWeb;
import 'package:grammar_checker/models/check_response.dart';
import 'package:http/http.dart' as http;
import 'package:nadz/nadz.dart';

/// Service for communicating with the Grammar API.
class GrammarApiService {
  /// Creates a new [GrammarApiService] instance.
  GrammarApiService({
    String? baseUrl,
    http.Client? client,
  })  : baseUrl = baseUrl ?? _defaultBaseUrl,
        _client = client ?? http.Client();

  /// The base URL for the Grammar API.
  final String baseUrl;

  final http.Client _client;

  static String get _defaultBaseUrl {
    if (kIsWeb) return 'http://localhost:8080';
    if (Platform.isAndroid) return 'http://10.0.2.2:8080';
    return 'http://localhost:8080';
  }

  /// Checks the given [text] for grammar and spelling errors.
  Future<Result<CheckResponse, String>> checkText(String text) async {
    try {
      final uri = Uri.parse('$baseUrl/v1/check');

      final response = await _client.post(
        uri,
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({'text': text}),
      );

      if (response.statusCode == 200) {
        final json = jsonDecode(response.body) as Map<String, dynamic>;
        return Success(CheckResponse.fromJson(json));
      } else if (response.statusCode == 413) {
        return const Error('Text exceeds maximum size (100KB)');
      } else {
        return Error('API error: ${response.statusCode}');
      }
    } on SocketException {
      return const Error('Connection failed. Is the API running?');
    } on Exception catch (e) {
      return Error('Unexpected error: $e');
    }
  }

  /// Disposes of resources used by this service.
  void dispose() {
    _client.close();
  }
}
