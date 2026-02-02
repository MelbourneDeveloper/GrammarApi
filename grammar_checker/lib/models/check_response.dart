import 'grammar_match.dart';

class CheckResponse {
  final List<GrammarMatch> matches;
  final int processingTimeMs;

  CheckResponse({
    required this.matches,
    required this.processingTimeMs,
  });

  factory CheckResponse.fromJson(Map<String, dynamic> json) {
    return CheckResponse(
      matches: (json['matches'] as List<dynamic>)
          .map((e) => GrammarMatch.fromJson(e as Map<String, dynamic>))
          .toList(),
      processingTimeMs: json['metrics']['processingTimeMs'] as int,
    );
  }
}
