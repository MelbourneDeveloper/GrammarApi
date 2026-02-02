import 'package:grammar_checker/models/grammar_match.dart';

/// Response from the Grammar API check endpoint.
class CheckResponse {
  /// Creates a new [CheckResponse] instance.
  CheckResponse({
    required this.matches,
    required this.processingTimeMs,
  });

  /// Creates a [CheckResponse] from JSON data.
  factory CheckResponse.fromJson(Map<String, dynamic> json) {
    final metrics = json['metrics'] as Map<String, dynamic>;
    return CheckResponse(
      matches: (json['matches'] as List<dynamic>)
          .map((e) => GrammarMatch.fromJson(e as Map<String, dynamic>))
          .toList(),
      processingTimeMs: metrics['processingTimeMs'] as int,
    );
  }

  /// List of grammar/spelling errors found.
  final List<GrammarMatch> matches;

  /// Time taken to process the request in milliseconds.
  final int processingTimeMs;
}
