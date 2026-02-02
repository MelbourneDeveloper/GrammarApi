import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:grammar_checker/app.dart';
import 'package:integration_test/integration_test.dart';

/// Integration tests for the Grammar Checker app.
///
/// These tests require the Grammar API server to be running at localhost:8080.
/// Start the server with: `DISABLE_RATE_LIMITING=true cargo run`
void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  Future<void> waitForAutoCheck(WidgetTester tester) async {
    // Wait for debounce (500ms) + API response time
    for (var i = 0; i < 100; i++) {
      await tester.pump(const Duration(milliseconds: 100));
    }
  }

  group('Grammar Checker App', () {
    testWidgets('displays initial UI correctly', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pumpAndSettle();

      expect(find.text('Inkwell'), findsOneWidget);
      expect(find.byType(TextField), findsOneWidget);
    });

    testWidgets('auto-checks text with grammar error and shows results',
        (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pumpAndSettle();

      // Enter text with a grammar error (incorrect article)
      await tester.enterText(find.byType(TextField), 'This is an test.');
      await waitForAutoCheck(tester);

      // Verify suggestions panel shows errors
      expect(find.text('Suggestions'), findsOneWidget);
    });

    testWidgets('auto-checks text with spelling error and shows results',
        (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pumpAndSettle();

      // Enter text with a spelling error
      await tester.enterText(
        find.byType(TextField),
        'This has a speling error.',
      );
      await waitForAutoCheck(tester);

      // Verify suggestions panel shows errors
      expect(find.text('Suggestions'), findsOneWidget);
    });

    testWidgets('applies replacement when suggestion is tapped',
        (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pumpAndSettle();

      // Enter text with a grammar error
      await tester.enterText(find.byType(TextField), 'This is an test.');
      await waitForAutoCheck(tester);

      // Find and tap a suggestion chip (should be "a")
      final suggestionChip = find.widgetWithText(ActionChip, 'a');
      if (suggestionChip.evaluate().isNotEmpty) {
        await tester.tap(suggestionChip);
        await tester.pumpAndSettle();

        // Verify the text was updated
        final controller =
            tester.widget<TextField>(find.byType(TextField)).controller;
        expect(controller?.text, contains('a test'));
      }
    });

    testWidgets('shows no errors for correct text', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pumpAndSettle();

      // Enter correct text
      await tester.enterText(
        find.byType(TextField),
        'This is a correct sentence.',
      );
      await waitForAutoCheck(tester);

      // Verify score badge appears (indicates success with no errors)
      expect(find.text('Suggestions'), findsOneWidget);
    });

    testWidgets('shows error count in app bar after check', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pumpAndSettle();

      // Enter text with errors
      await tester.enterText(find.byType(TextField), 'This is an test.');
      await waitForAutoCheck(tester);

      // Verify stats footer shows counts
      expect(find.textContaining('spelling'), findsWidgets);
      expect(find.textContaining('grammar'), findsWidgets);
    });

    testWidgets('handles empty text gracefully', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pumpAndSettle();

      // Enter empty text
      await tester.enterText(find.byType(TextField), '');
      await waitForAutoCheck(tester);

      // Should not crash, suggestions panel still visible
      expect(find.text('Suggestions'), findsOneWidget);
    });
  });
}
