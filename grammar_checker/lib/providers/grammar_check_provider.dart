import 'dart:async';

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

/// Sample text with intentional grammar and spelling errors for demo.
const _sampleText = '''
Welcome to Inkwell, an powerful grammar and spelling checker. This tool will help you write more clearly and catch common mistakes before your readers do.

Writing well is an skill that takes practise. Whether your working on an email, a blog post, or an important document, having a second pair of eyes can make all the diffrence. Thats exactly what this application provides.

The quick brown fox jumps over the lazy dog. This sentence is perfectly correct and contains every letter of the alphabet. However, the next sentence has a few issues that our checker will catch.

Their are many reasons to use a grammar checker. First, it helps you avoid embarassing mistakes in professional comunication. Second, it can teach you common patterns so you make less errors over time. Third, its simply faster then reading through everything yourself.

Some common mistakes include using "an" before words that start with consonant sounds, misspelling words like "recieve" instead of "receive", and confusing homophones like "there", "their", and "they're".

We hope you enjoy using Inkwell. Feel free to edit this text or paste your own content to see the checker in action!''';

/// Provider for managing grammar check state.
class GrammarCheckProvider extends ChangeNotifier {
  /// Creates a new [GrammarCheckProvider] instance.
  GrammarCheckProvider({GrammarApiService? apiService, bool loadSample = true})
      : _apiService = apiService ?? GrammarApiService(),
        _text = loadSample ? _sampleText : '' {
    if (loadSample && _text.isNotEmpty) {
      _scheduleCheck();
    }
  }

  final GrammarApiService _apiService;
  Timer? _debounceTimer;
  static const _debounceDuration = Duration(milliseconds: 500);

  CheckState _state = CheckState.idle;
  String _text;
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

  /// Updates the text and triggers auto-check with debounce.
  void updateText(String newText) {
    _text = newText;
    notifyListeners();
    _scheduleCheck();
  }

  void _scheduleCheck() {
    _debounceTimer?.cancel();
    _debounceTimer = Timer(_debounceDuration, checkGrammar);
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

  @override
  void dispose() {
    _debounceTimer?.cancel();
    _apiService.dispose();
    super.dispose();
  }
}
