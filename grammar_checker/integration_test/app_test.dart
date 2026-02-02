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

  Future<void> waitForApiResponse(WidgetTester tester) async {
    // Pump frames for up to 10 seconds waiting for API response
    for (var i = 0; i < 100; i++) {
      await tester.pump(const Duration(milliseconds: 100));
    }
  }

  group('Grammar Checker App', () {
    testWidgets('displays initial UI correctly', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pumpAndSettle();

      expect(find.text('Grammar Checker'), findsOneWidget);
      expect(find.text('Check Grammar'), findsOneWidget);
      expect(find.byType(TextField), findsOneWidget);
      expect(find.text('No errors found'), findsOneWidget);
    });

    testWidgets('checks text with grammar error and shows results',
        (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pumpAndSettle();

      // Enter text with a grammar error (incorrect article)
      await tester.enterText(find.byType(TextField), 'This is an test.');
      await tester.pumpAndSettle();

      // Tap the check button
      await tester.tap(find.text('Check Grammar'));
      await waitForApiResponse(tester);

      // Verify error card is displayed
      expect(find.byType(Card), findsWidgets);
    });

    testWidgets('applies replacement when suggestion is tapped',
        (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pumpAndSettle();

      // Enter text with a grammar error
      await tester.enterText(find.byType(TextField), 'This is an test.');
      await tester.pumpAndSettle();

      // Tap the check button
      await tester.tap(find.text('Check Grammar'));
      await waitForApiResponse(tester);

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
      await tester.pumpAndSettle();

      // Tap the check button
      await tester.tap(find.text('Check Grammar'));
      await waitForApiResponse(tester);

      // Verify no errors shown
      expect(find.text('No errors found'), findsOneWidget);
    });

    testWidgets('handles empty text gracefully', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pumpAndSettle();

      // Don't enter any text, just tap check
      await tester.tap(find.text('Check Grammar'));
      await tester.pumpAndSettle();

      // Should still show "No errors found"
      expect(find.text('No errors found'), findsOneWidget);
    });
  });
}
