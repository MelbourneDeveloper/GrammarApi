import 'package:flutter/foundation.dart';
import 'package:grammar_checker/models/grammar_match.dart';
import 'package:grammar_checker/services/grammar_api_service.dart';
import 'package:nadz/nadz.dart';

/// State of the grammar check operation.
enum CheckState {
  /// No check has been performed yet.
  idle,

  /// A check is in progress.
  loading,

  /// Check completed successfully.
  success,

  /// Check failed with an error.
  error,
}

/// Provider for managing grammar check state.
class GrammarCheckProvider extends ChangeNotifier {
  /// Creates a new [GrammarCheckProvider] instance.
  GrammarCheckProvider({GrammarApiService? apiService})
      : _apiService = apiService ?? GrammarApiService();

  final GrammarApiService _apiService;

  CheckState _state = CheckState.idle;
  String _text = '';
  List<GrammarMatch> _matches = [];
  String? _errorMessage;
  int? _processingTimeMs;

  /// Current state of the check operation.
  CheckState get state => _state;

  /// The text being checked.
  String get text => _text;

  /// List of grammar/spelling errors found.
  List<GrammarMatch> get matches => _matches;

  /// Error message if the check failed.
  String? get errorMessage => _errorMessage;

  /// Processing time in milliseconds.
  int? get processingTimeMs => _processingTimeMs;

  /// Whether any errors were found.
  bool get hasErrors => _matches.isNotEmpty;

  /// Number of spelling errors found.
  int get spellingErrorCount => _matches.where((m) => m.isSpellingError).length;

  /// Number of grammar errors found.
  int get grammarErrorCount => _matches.where((m) => m.isGrammarError).length;

  /// Updates the text to check.
  void updateText(String newText) {
    _text = newText;
    notifyListeners();
  }

  /// Checks the current text for grammar and spelling errors.
  Future<void> checkGrammar() async {
    if (_text.trim().isEmpty) {
      _matches = [];
      _state = CheckState.idle;
      notifyListeners();
      return;
    }

    _state = CheckState.loading;
    _errorMessage = null;
    notifyListeners();

    final result = await _apiService.checkText(_text);

    switch (result) {
      case Success(:final value):
        _matches = value.matches;
        _processingTimeMs = value.processingTimeMs;
        _state = CheckState.success;
      case Error(:final error):
        _errorMessage = error;
        _state = CheckState.error;
    }

    notifyListeners();
  }

  /// Applies a replacement for a specific match.
  void applyReplacement(GrammarMatch match, String replacement) {
    final before = _text.substring(0, match.offset);
    final after = _text.substring(match.offset + match.length);
    _text = before + replacement + after;

    final offsetDiff = replacement.length - match.length;
    _matches = _matches.where((m) => m != match).map((m) {
      if (m.offset > match.offset) {
        return m.copyWith(offset: m.offset + offsetDiff);
      }
      return m;
    }).toList();

    notifyListeners();
  }

  /// Clears all matches and resets to idle state.
  void clearMatches() {
    _matches = [];
    _state = CheckState.idle;
    notifyListeners();
  }

  @override
  void dispose() {
    _apiService.dispose();
    super.dispose();
  }
}
