import 'package:flutter/material.dart';
import 'package:grammar_checker/models/grammar_match.dart';
import 'package:grammar_checker/widgets/error_card.dart';

/// A list of grammar/spelling errors.
class ErrorList extends StatelessWidget {
  /// Creates a new [ErrorList] instance.
  const ErrorList({
    required this.matches,
    required this.originalText,
    required this.onApplyReplacement,
    super.key,
  });

  /// List of grammar/spelling errors.
  final List<GrammarMatch> matches;

  /// The original text being checked.
  final String originalText;

  /// Callback when a replacement is applied.
  final void Function(GrammarMatch, String) onApplyReplacement;

  @override
  Widget build(BuildContext context) {
    if (matches.isEmpty) {
      return const Center(
        child: Text(
          'No errors found',
          style: TextStyle(color: Colors.grey),
        ),
      );
    }

    return ListView.builder(
      padding: const EdgeInsets.all(16),
      itemCount: matches.length,
      itemBuilder: (context, index) {
        final match = matches[index];
        return ErrorCard(
          match: match,
          errorText: match.getErrorText(originalText),
          onApplyReplacement: (replacement) {
            onApplyReplacement(match, replacement);
          },
        );
      },
    );
  }
}
