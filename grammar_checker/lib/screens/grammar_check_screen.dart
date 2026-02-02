import 'package:flutter/material.dart';
import 'package:grammar_checker/providers/grammar_check_provider.dart';
import 'package:grammar_checker/theme/app_colors.dart';
import 'package:grammar_checker/widgets/error_list.dart';
import 'package:grammar_checker/widgets/gradient_button.dart';
import 'package:grammar_checker/widgets/score_badge.dart';
import 'package:grammar_checker/widgets/text_input_area.dart';
import 'package:provider/provider.dart';

/// Main screen for checking grammar and spelling.
class GrammarCheckScreen extends StatelessWidget {
  /// Creates a new [GrammarCheckScreen] instance.
  const GrammarCheckScreen({super.key});

  @override
  Widget build(BuildContext context) => Scaffold(
        backgroundColor: paper,
        body: SafeArea(
          child: LayoutBuilder(
            builder: (context, constraints) {
              final isWide = constraints.maxWidth > 900;
              return isWide ? _WideLayout() : _NarrowLayout();
            },
          ),
        ),
      );
}

class _WideLayout extends StatelessWidget {
  @override
  Widget build(BuildContext context) => Row(
        children: [
          // Editor panel (65%)
          Expanded(
            flex: 65,
            child: _EditorPanel(),
          ),
          // Divider
          Container(width: 1, color: mist),
          // Results panel (35%)
          Expanded(
            flex: 35,
            child: _ResultsPanel(),
          ),
        ],
      );
}

class _NarrowLayout extends StatelessWidget {
  @override
  Widget build(BuildContext context) => Column(
        children: [
          // Header
          _Header(),
          // Editor
          Expanded(flex: 2, child: _EditorSection()),
          // Button
          _CheckButton(),
          // Error message
          _ErrorMessage(),
          // Results
          Expanded(child: _ResultsPanel()),
        ],
      );
}

class _EditorPanel extends StatelessWidget {
  @override
  Widget build(BuildContext context) => Column(
        children: [
          _Header(),
          Expanded(child: _EditorSection()),
          _CheckButton(),
          _ErrorMessage(),
          const SizedBox(height: 16),
        ],
      );
}

class _Header extends StatelessWidget {
  @override
  Widget build(BuildContext context) => Container(
        padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
        child: Row(
          children: [
            // Logo/Title
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.all(8),
                  decoration: BoxDecoration(
                    color: coralTint,
                    borderRadius: BorderRadius.circular(10),
                  ),
                  child: const Icon(Icons.edit_note, color: coral, size: 24),
                ),
                const SizedBox(width: 12),
                const Text(
                  'Inkwell',
                  style: TextStyle(
                    fontSize: 22,
                    fontWeight: FontWeight.w700,
                    color: ink,
                    letterSpacing: -0.5,
                  ),
                ),
              ],
            ),
            const Spacer(),
            // Score badge
            Consumer<GrammarCheckProvider>(
              builder: (context, provider, _) {
                if (provider.state == CheckState.success) {
                  return ScoreBadge(score: _calculateScore(provider));
                }
                return const SizedBox.shrink();
              },
            ),
          ],
        ),
      );

  int _calculateScore(GrammarCheckProvider provider) {
    if (!provider.hasErrors) return 100;
    final errorCount =
        provider.spellingErrorCount + provider.grammarErrorCount;
    final textLength = provider.text.length;
    if (textLength == 0) return 100;
    // Score decreases with more errors relative to text length
    final errorRate = errorCount / (textLength / 100);
    return (100 - (errorRate * 10)).clamp(0, 100).round();
  }
}

class _EditorSection extends StatelessWidget {
  @override
  Widget build(BuildContext context) => Padding(
        padding: const EdgeInsets.symmetric(horizontal: 24),
        child: Consumer<GrammarCheckProvider>(
          builder: (context, provider, _) => TextInputArea(
            text: provider.text,
            matches: provider.matches,
            onTextChanged: provider.updateText,
          ),
        ),
      );
}

class _CheckButton extends StatelessWidget {
  @override
  Widget build(BuildContext context) => Padding(
        padding: const EdgeInsets.all(24),
        child: Consumer<GrammarCheckProvider>(
          builder: (context, provider, _) => SizedBox(
            width: double.infinity,
            child: GradientButton(
              onPressed: provider.state == CheckState.loading
                  ? null
                  : provider.checkGrammar,
              isLoading: provider.state == CheckState.loading,
              child: const Text('Check Writing'),
            ),
          ),
        ),
      );
}

class _ErrorMessage extends StatelessWidget {
  @override
  Widget build(BuildContext context) => Consumer<GrammarCheckProvider>(
        builder: (context, provider, _) {
          if (provider.state == CheckState.error) {
            return Container(
              margin: const EdgeInsets.symmetric(horizontal: 24),
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: spellingErrorLight,
                borderRadius: BorderRadius.circular(10),
              ),
              child: Row(
                children: [
                  const Icon(
                    Icons.error_outline,
                    color: spellingError,
                    size: 20,
                  ),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Text(
                      provider.errorMessage ?? 'An error occurred',
                      style: const TextStyle(
                        color: spellingError,
                        fontSize: 14,
                      ),
                    ),
                  ),
                ],
              ),
            );
          }
          return const SizedBox.shrink();
        },
      );
}

class _ResultsPanel extends StatelessWidget {
  @override
  Widget build(BuildContext context) => ColoredBox(
        color: cloud,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Results header
            Padding(
              padding: const EdgeInsets.all(20),
              child: Consumer<GrammarCheckProvider>(
                builder: (context, provider, _) => Row(
                  children: [
                    const Text(
                      'Suggestions',
                      style: TextStyle(
                        fontSize: 16,
                        fontWeight: FontWeight.w600,
                        color: ink,
                      ),
                    ),
                    const SizedBox(width: 8),
                    if (provider.state == CheckState.success &&
                        provider.hasErrors)
                      Container(
                        padding: const EdgeInsets.symmetric(
                          horizontal: 8,
                          vertical: 2,
                        ),
                        decoration: BoxDecoration(
                          color: coral,
                          borderRadius: BorderRadius.circular(12),
                        ),
                        child: Text(
                          '${provider.matches.length}',
                          style: const TextStyle(
                            color: surface,
                            fontSize: 12,
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                      ),
                  ],
                ),
              ),
            ),
            // Error list
            Expanded(
              child: Consumer<GrammarCheckProvider>(
                builder: (context, provider, _) => ErrorList(
                  matches: provider.matches,
                  originalText: provider.text,
                  onApplyReplacement: provider.applyReplacement,
                ),
              ),
            ),
            // Stats footer
            Consumer<GrammarCheckProvider>(
              builder: (context, provider, _) {
                if (provider.state == CheckState.success) {
                  return Container(
                    padding: const EdgeInsets.all(16),
                    decoration: const BoxDecoration(
                      border: Border(top: BorderSide(color: mist)),
                    ),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        _StatItem(
                          icon: Icons.spellcheck,
                          count: provider.spellingErrorCount,
                          label: 'spelling',
                          color: spellingError,
                        ),
                        const SizedBox(width: 24),
                        _StatItem(
                          icon: Icons.rule,
                          count: provider.grammarErrorCount,
                          label: 'grammar',
                          color: grammarError,
                        ),
                      ],
                    ),
                  );
                }
                return const SizedBox.shrink();
              },
            ),
          ],
        ),
      );
}

class _StatItem extends StatelessWidget {
  const _StatItem({
    required this.icon,
    required this.count,
    required this.label,
    required this.color,
  });

  final IconData icon;
  final int count;
  final String label;
  final Color color;

  @override
  Widget build(BuildContext context) => Row(
        children: [
          Icon(icon, size: 16, color: color),
          const SizedBox(width: 4),
          Text(
            '$count $label',
            style: const TextStyle(fontSize: 13, color: slate),
          ),
        ],
      );
}
