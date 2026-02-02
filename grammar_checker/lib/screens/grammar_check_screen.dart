import 'package:flutter/material.dart';
import 'package:grammar_checker/providers/grammar_check_provider.dart';
import 'package:grammar_checker/widgets/error_list.dart';
import 'package:grammar_checker/widgets/text_input_area.dart';
import 'package:provider/provider.dart';

/// Main screen for checking grammar and spelling.
class GrammarCheckScreen extends StatelessWidget {
  /// Creates a new [GrammarCheckScreen] instance.
  const GrammarCheckScreen({super.key});

  @override
  Widget build(BuildContext context) => Scaffold(
        appBar: AppBar(
          title: const Text('Grammar Checker'),
          actions: [
            Consumer<GrammarCheckProvider>(
              builder: (context, provider, _) {
                if (provider.state == CheckState.success) {
                  return Padding(
                    padding: const EdgeInsets.all(8),
                    child: Chip(
                      label: Text(
                        '${provider.spellingErrorCount} spelling, '
                        '${provider.grammarErrorCount} grammar',
                      ),
                      backgroundColor: provider.hasErrors
                          ? Colors.orange.shade100
                          : Colors.green.shade100,
                    ),
                  );
                }
                return const SizedBox.shrink();
              },
            ),
          ],
        ),
        body: Column(
          children: [
            Expanded(
              flex: 2,
              child: Padding(
                padding: const EdgeInsets.all(16),
                child: Consumer<GrammarCheckProvider>(
                  builder: (context, provider, _) => TextInputArea(
                    text: provider.text,
                    matches: provider.matches,
                    onTextChanged: provider.updateText,
                  ),
                ),
              ),
            ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              child: Consumer<GrammarCheckProvider>(
                builder: (context, provider, _) => ElevatedButton(
                  onPressed: provider.state == CheckState.loading
                      ? null
                      : provider.checkGrammar,
                  child: provider.state == CheckState.loading
                      ? const SizedBox(
                          height: 20,
                          width: 20,
                          child: CircularProgressIndicator(strokeWidth: 2),
                        )
                      : const Text('Check Grammar'),
                ),
              ),
            ),
            const SizedBox(height: 8),
            Consumer<GrammarCheckProvider>(
              builder: (context, provider, _) {
                if (provider.state == CheckState.error) {
                  return Padding(
                    padding: const EdgeInsets.all(16),
                    child: Text(
                      provider.errorMessage ?? 'Unknown error',
                      style: const TextStyle(color: Colors.red),
                    ),
                  );
                }
                return const SizedBox.shrink();
              },
            ),
            Expanded(
              child: Consumer<GrammarCheckProvider>(
                builder: (context, provider, _) => ErrorList(
                  matches: provider.matches,
                  originalText: provider.text,
                  onApplyReplacement: provider.applyReplacement,
                ),
              ),
            ),
          ],
        ),
      );
}
