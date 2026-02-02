class GrammarMatch {
  final String message;
  final int offset;
  final int length;
  final List<String> replacements;
  final Rule rule;
  final MatchContext context;

  GrammarMatch({
    required this.message,
    required this.offset,
    required this.length,
    required this.replacements,
    required this.rule,
    required this.context,
  });

  factory GrammarMatch.fromJson(Map<String, dynamic> json) {
    return GrammarMatch(
      message: json['message'] as String,
      offset: json['offset'] as int,
      length: json['length'] as int,
      replacements: (json['replacements'] as List<dynamic>)
          .map((e) => e as String)
          .toList(),
      rule: Rule.fromJson(json['rule'] as Map<String, dynamic>),
      context: MatchContext.fromJson(json['context'] as Map<String, dynamic>),
    );
  }

  String getErrorText(String originalText) {
    if (offset >= originalText.length) return '';
    final end = (offset + length).clamp(0, originalText.length);
    return originalText.substring(offset, end);
  }

  bool get isSpellingError => rule.category == 'spelling';
  bool get isGrammarError => rule.category == 'grammar';

  GrammarMatch copyWith({int? offset}) {
    return GrammarMatch(
      message: message,
      offset: offset ?? this.offset,
      length: length,
      replacements: replacements,
      rule: rule,
      context: context,
    );
  }
}

class Rule {
  final String id;
  final String category;

  Rule({required this.id, required this.category});

  factory Rule.fromJson(Map<String, dynamic> json) {
    return Rule(
      id: json['id'] as String,
      category: json['category'] as String,
    );
  }
}

class MatchContext {
  final String text;
  final int offset;
  final int length;

  MatchContext({
    required this.text,
    required this.offset,
    required this.length,
  });

  factory MatchContext.fromJson(Map<String, dynamic> json) {
    return MatchContext(
      text: json['text'] as String,
      offset: json['offset'] as int,
      length: json['length'] as int,
    );
  }
}
