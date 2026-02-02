import 'package:flutter/material.dart';
import 'package:grammar_checker/models/grammar_match.dart';
import 'package:grammar_checker/theme/app_colors.dart';
import 'package:grammar_checker/theme/app_theme.dart';

/// A premium card displaying a grammar or spelling error with suggestions.
class ErrorCard extends StatefulWidget {
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
  State<ErrorCard> createState() => _ErrorCardState();
}

class _ErrorCardState extends State<ErrorCard> {
  bool _isHovered = false;

  Color get _accentColor =>
      widget.match.isSpellingError ? spellingError : grammarError;

  Color get _lightColor =>
      widget.match.isSpellingError ? spellingErrorLight : grammarErrorLight;

  String get _typeLabel =>
      widget.match.isSpellingError ? 'SPELLING' : 'GRAMMAR';

  @override
  Widget build(BuildContext context) => MouseRegion(
        onEnter: (_) => setState(() => _isHovered = true),
        onExit: (_) => setState(() => _isHovered = false),
        child: AnimatedContainer(
          duration: AppDurations.fast,
          margin: const EdgeInsets.only(bottom: 12),
          decoration: BoxDecoration(
            color: surface,
            borderRadius: BorderRadius.circular(14),
            boxShadow: _isHovered ? shadow2 : shadow1,
          ),
          child: ClipRRect(
            borderRadius: BorderRadius.circular(14),
            child: IntrinsicHeight(
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  // Left accent bar
                  Container(width: 4, color: _accentColor),
                  // Content
                  Expanded(
                    child: Padding(
                      padding: const EdgeInsets.all(16),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          // Header row
                          Row(
                            children: [
                              // Type badge
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
                              // Error text
                              Text(
                                '"${widget.errorText}"',
                                style: TextStyle(
                                  fontSize: 14,
                                  fontWeight: FontWeight.w600,
                                  color: _accentColor,
                                ),
                              ),
                            ],
                          ),
                          const SizedBox(height: 12),
                          // Message
                          Text(
                            widget.match.message,
                            style: const TextStyle(
                              fontSize: 14,
                              color: slate,
                              height: 1.4,
                            ),
                          ),
                          // Suggestions
                          if (widget.match.replacements.isNotEmpty) ...[
                            const SizedBox(height: 16),
                            Wrap(
                              spacing: 8,
                              runSpacing: 8,
                              children: widget.match.replacements
                                  .take(5)
                                  .map(_buildSuggestionChip)
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
        ),
      );

  Widget _buildSuggestionChip(String replacement) => _SuggestionChip(
        label: replacement,
        onPressed: () => widget.onApplyReplacement(replacement),
      );
}

class _SuggestionChip extends StatefulWidget {
  const _SuggestionChip({
    required this.label,
    required this.onPressed,
  });

  final String label;
  final VoidCallback onPressed;

  @override
  State<_SuggestionChip> createState() => _SuggestionChipState();
}

class _SuggestionChipState extends State<_SuggestionChip> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) => MouseRegion(
        onEnter: (_) => setState(() => _isHovered = true),
        onExit: (_) => setState(() => _isHovered = false),
        child: GestureDetector(
          onTap: widget.onPressed,
          child: AnimatedContainer(
            duration: AppDurations.fast,
            padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 8),
            decoration: BoxDecoration(
              color: _isHovered ? coralTint : tealTint,
              borderRadius: BorderRadius.circular(20),
              border: Border.all(
                color: _isHovered ? coral : teal.withValues(alpha: 0.3),
              ),
            ),
            child: Text(
              widget.label,
              style: TextStyle(
                fontSize: 14,
                fontWeight: FontWeight.w500,
                color: _isHovered ? coralDark : tealDark,
              ),
            ),
          ),
        ),
      );
}
