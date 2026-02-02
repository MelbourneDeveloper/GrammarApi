import 'dart:async';

import 'package:flutter/material.dart';
import 'package:grammar_checker/models/grammar_match.dart';
import 'package:grammar_checker/theme/app_colors.dart';
import 'package:grammar_checker/theme/app_theme.dart';
import 'package:grammar_checker/widgets/error_card.dart';

/// A list of grammar/spelling errors with animated transitions.
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
        child: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Icon(Icons.check_circle_outline, size: 48, color: teal),
              SizedBox(height: 16),
              Text(
                'No errors found',
                style: TextStyle(
                  fontSize: 16,
                  fontWeight: FontWeight.w500,
                  color: slate,
                ),
              ),
              SizedBox(height: 4),
              Text(
                'Your writing looks great!',
                style: TextStyle(fontSize: 14, color: stone),
              ),
            ],
          ),
        ),
      );
    }

    return ListView.builder(
      key: ValueKey(matches.length),
      padding: const EdgeInsets.all(16),
      itemCount: matches.length,
      itemBuilder: (context, index) {
        final match = matches[index];
        return _AnimatedErrorCard(
          index: index,
          match: match,
          originalText: originalText,
          onApplyReplacement: onApplyReplacement,
        );
      },
    );
  }
}

class _AnimatedErrorCard extends StatefulWidget {
  const _AnimatedErrorCard({
    required this.index,
    required this.match,
    required this.originalText,
    required this.onApplyReplacement,
  });

  final int index;
  final GrammarMatch match;
  final String originalText;
  final void Function(GrammarMatch, String) onApplyReplacement;

  @override
  State<_AnimatedErrorCard> createState() => _AnimatedErrorCardState();
}

class _AnimatedErrorCardState extends State<_AnimatedErrorCard>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _slideAnimation;
  late Animation<double> _fadeAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: AppDurations.slow,
    );

    final delay = widget.index * 0.1;
    _slideAnimation = Tween<double>(begin: 20, end: 0).animate(
      CurvedAnimation(
        parent: _controller,
        curve: Interval(delay.clamp(0, 0.5), 1, curve: Curves.easeOutCubic),
      ),
    );
    _fadeAnimation = Tween<double>(begin: 0, end: 1).animate(
      CurvedAnimation(
        parent: _controller,
        curve: Interval(delay.clamp(0, 0.5), 1, curve: Curves.easeOut),
      ),
    );

    unawaited(_controller.forward());
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => AnimatedBuilder(
        animation: _controller,
        builder: (context, child) => Transform.translate(
          offset: Offset(0, _slideAnimation.value),
          child: Opacity(
            opacity: _fadeAnimation.value,
            child: child,
          ),
        ),
        child: ErrorCard(
          match: widget.match,
          errorText: widget.match.getErrorText(widget.originalText),
          onApplyReplacement: (replacement) =>
              widget.onApplyReplacement(widget.match, replacement),
        ),
      );
}
