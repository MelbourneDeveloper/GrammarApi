import 'package:flutter/material.dart';
import 'package:grammar_checker/models/grammar_match.dart';
import 'package:grammar_checker/theme/app_colors.dart';

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

  Color get _accentColor =>
      match.isSpellingError ? spellingError : grammarError;

  Color get _lightColor =>
      match.isSpellingError ? spellingErrorLight : grammarErrorLight;

  String get _typeLabel => match.isSpellingError ? 'SPELLING' : 'GRAMMAR';

  @override
  Widget build(BuildContext context) => Container(
        margin: const EdgeInsets.only(bottom: 12),
        decoration: BoxDecoration(
          color: surface,
          borderRadius: BorderRadius.circular(14),
          boxShadow: shadow1,
        ),
        child: ClipRRect(
          borderRadius: BorderRadius.circular(14),
          child: IntrinsicHeight(
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Container(width: 4, color: _accentColor),
                Expanded(
                  child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          children: [
                            Container(
                              padding: const EdgeInsets.symmetric(
                                horizontal: 8,
                                vertical: 4,
                              ),
                              decoration: BoxDecoration(
                                color: _lightColor,
                                borderRadius: BorderRadius.circular(6),
                              ),
                              child: Text(
                                _typeLabel,
                                style: TextStyle(
                                  fontSize: 11,
                                  fontWeight: FontWeight.w700,
                                  color: _accentColor,
                                  letterSpacing: 0.5,
                                ),
                              ),
                            ),
                            const Spacer(),
                            Text(
                              '"$errorText"',
                              style: TextStyle(
                                fontSize: 14,
                                fontWeight: FontWeight.w600,
                                color: _accentColor,
                              ),
                            ),
                          ],
                        ),
                        const SizedBox(height: 12),
                        Text(
                          match.message,
                          style: const TextStyle(
                            fontSize: 14,
                            color: slate,
                            height: 1.4,
                          ),
                        ),
                        if (match.replacements.isNotEmpty) ...[
                          const SizedBox(height: 16),
                          Wrap(
                            spacing: 8,
                            runSpacing: 8,
                            children: match.replacements
                                .take(5)
                                .map(
                                  (r) => _SuggestionChip(
                                    label: r,
                                    onPressed: () => onApplyReplacement(r),
                                  ),
                                )
                                .toList(),
                          ),
                        ],
                      ],
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
      );
}

class _SuggestionChip extends StatelessWidget {
  const _SuggestionChip({
    required this.label,
    required this.onPressed,
  });

  final String label;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) => GestureDetector(
        onTap: onPressed,
        child: Container(
          padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 8),
          decoration: BoxDecoration(
            color: tealTint,
            borderRadius: BorderRadius.circular(20),
            border: Border.all(color: teal.withValues(alpha: 0.3)),
          ),
          child: Text(
            label,
            style: const TextStyle(
              fontSize: 14,
              fontWeight: FontWeight.w500,
              color: tealDark,
            ),
          ),
        ),
      );
}
