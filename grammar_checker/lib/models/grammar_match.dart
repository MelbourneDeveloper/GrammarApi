/// Represents a grammar or spelling error found in the text.
class GrammarMatch {
  /// Creates a new [GrammarMatch] instance.
  GrammarMatch({
    required this.message,
    required this.offset,
    required this.length,
    required this.replacements,
    required this.rule,
  });

  /// Creates a [GrammarMatch] from JSON data.
  factory GrammarMatch.fromJson(Map<String, dynamic> json) => GrammarMatch(
        message: json['message'] as String,
        offset: json['offset'] as int,
        length: json['length'] as int,
        replacements: (json['replacements'] as List<dynamic>)
            .map((e) => e as String)
            .toList(),
        rule: Rule.fromJson(json['rule'] as Map<String, dynamic>),
      );

  /// Human-readable description of the error.
  final String message;

  /// Character offset where the error starts.
  final int offset;

  /// Length of the problematic text.
  final int length;

  /// Suggested replacements for the error.
  final List<String> replacements;

  /// Rule that triggered this error.
  final Rule rule;

  /// Extracts the error text from the original input.
  String getErrorText(String originalText) {
    if (offset >= originalText.length) return '';
    final end = (offset + length).clamp(0, originalText.length);
    return originalText.substring(offset, end);
  }

  /// Whether this is a spelling error.
  bool get isSpellingError => rule.category == 'spelling';

  /// Whether this is a grammar error.
  bool get isGrammarError => rule.category == 'grammar';

  /// Creates a copy with an updated offset.
  GrammarMatch copyWith({int? offset}) => GrammarMatch(
        message: message,
        offset: offset ?? this.offset,
        length: length,
        replacements: replacements,
        rule: rule,
      );
}

/// Represents the rule that triggered an error.
class Rule {
  /// Creates a new [Rule] instance.
  Rule({required this.id, required this.category});

  /// Creates a [Rule] from JSON data.
  factory Rule.fromJson(Map<String, dynamic> json) => Rule(
        id: json['id'] as String,
        category: json['category'] as String,
      );

  /// Unique identifier for the rule.
  final String id;

  /// Category of the rule (spelling or grammar).
  final String category;
}
