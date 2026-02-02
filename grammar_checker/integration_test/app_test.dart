import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:grammar_checker/app.dart';
import 'package:integration_test/integration_test.dart';

/// Integration tests for the Grammar Checker app.
///
/// These tests require the Grammar API server to be running at localhost:8080.
/// Start the server with: `cargo run` in the GrammarApi directory.
void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

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
      final textField = find.byType(TextField);
      await tester.enterText(textField, 'This is an test.');
      await tester.pumpAndSettle();

      // Tap the check button
      final checkButton = find.text('Check Grammar');
      await tester.tap(checkButton);

      // Wait for the API response
      await tester.pumpAndSettle(const Duration(seconds: 5));

      // Verify error is displayed
      expect(find.text('Grammar'), findsOneWidget);
      expect(find.byType(Card), findsWidgets);
    });

    testWidgets('checks text with spelling error and shows results',
        (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pumpAndSettle();

      // Enter text with a spelling error
      final textField = find.byType(TextField);
      await tester.enterText(textField, 'This is a tset.');
      await tester.pumpAndSettle();

      // Tap the check button
      final checkButton = find.text('Check Grammar');
      await tester.tap(checkButton);

      // Wait for the API response
      await tester.pumpAndSettle(const Duration(seconds: 5));

      // Verify spelling error is displayed
      expect(find.text('Spelling'), findsOneWidget);
    });

    testWidgets('applies replacement when suggestion is tapped',
        (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pumpAndSettle();

      // Enter text with a grammar error
      final textField = find.byType(TextField);
      await tester.enterText(textField, 'This is an test.');
      await tester.pumpAndSettle();

      // Tap the check button
      final checkButton = find.text('Check Grammar');
      await tester.tap(checkButton);

      // Wait for the API response
      await tester.pumpAndSettle(const Duration(seconds: 5));

      // Find and tap a suggestion chip (should be "a")
      final suggestionChip = find.widgetWithText(ActionChip, 'a');
      if (suggestionChip.evaluate().isNotEmpty) {
        await tester.tap(suggestionChip);
        await tester.pumpAndSettle();

        // Verify the text was updated
        final updatedTextField = find.byType(TextField);
        final controller = tester
            .widget<TextField>(updatedTextField)
            .controller;
        expect(controller?.text, contains('a test'));
      }
    });

    testWidgets('shows no errors for correct text', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pumpAndSettle();

      // Enter correct text
      final textField = find.byType(TextField);
      await tester.enterText(textField, 'This is a correct sentence.');
      await tester.pumpAndSettle();

      // Tap the check button
      final checkButton = find.text('Check Grammar');
      await tester.tap(checkButton);

      // Wait for the API response
      await tester.pumpAndSettle(const Duration(seconds: 5));

      // Verify no errors shown
      expect(find.text('No errors found'), findsOneWidget);
    });

    testWidgets('shows error count in app bar after check', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pumpAndSettle();

      // Enter text with errors
      final textField = find.byType(TextField);
      await tester.enterText(textField, 'This is an tset.');
      await tester.pumpAndSettle();

      // Tap the check button
      final checkButton = find.text('Check Grammar');
      await tester.tap(checkButton);

      // Wait for the API response
      await tester.pumpAndSettle(const Duration(seconds: 5));

      // Verify the chip with counts is displayed
      expect(find.byType(Chip), findsOneWidget);
    });

    testWidgets('handles empty text gracefully', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pumpAndSettle();

      // Don't enter any text, just tap check
      final checkButton = find.text('Check Grammar');
      await tester.tap(checkButton);
      await tester.pumpAndSettle();

      // Should still show "No errors found"
      expect(find.text('No errors found'), findsOneWidget);
    });
  });
}
