import 'package:flutter/material.dart';
import 'package:grammar_checker/models/grammar_match.dart';

/// A card displaying a grammar or spelling error with suggestions.
class ErrorCard extends StatelessWidget {
  /// Creates a new [ErrorCard] instance.
  const ErrorCard({
    required this.match,
    required this.errorText,
    required this.onApplyReplacement,
    super.key,
  });

  /// The grammar match to display.
  final GrammarMatch match;

  /// The error text extracted from the original.
  final String errorText;

  /// Callback when a replacement is selected.
  final ValueChanged<String> onApplyReplacement;

  @override
  Widget build(BuildContext context) {
    final isSpelling = match.isSpellingError;
    final color = isSpelling ? Colors.red : Colors.orange;

    return Card(
      margin: const EdgeInsets.only(bottom: 8),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(
                  isSpelling ? Icons.spellcheck : Icons.rule,
                  size: 16,
                  color: color,
                ),
                const SizedBox(width: 8),
                DecoratedBox(
                  decoration: BoxDecoration(
                    color: color.withValues(alpha: 0.1),
                    borderRadius: BorderRadius.circular(4),
                  ),
                  child: Padding(
                    padding: const EdgeInsets.symmetric(
                      horizontal: 8,
                      vertical: 2,
                    ),
                    child: Text(
                      isSpelling ? 'Spelling' : 'Grammar',
                      style: TextStyle(
                        fontSize: 12,
                        color: color,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
                ),
                const Spacer(),
                Text(
                  '"$errorText"',
                  style: TextStyle(color: color, fontWeight: FontWeight.bold),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Text(match.message),
            if (match.replacements.isNotEmpty) ...[
              const SizedBox(height: 8),
              Wrap(
                spacing: 8,
                runSpacing: 4,
                children: match.replacements
                    .take(5)
                    .map(
                      (replacement) => ActionChip(
                        label: Text(replacement),
                        onPressed: () => onApplyReplacement(replacement),
                      ),
                    )
                    .toList(),
              ),
            ],
          ],
        ),
      ),
    );
  }
}
