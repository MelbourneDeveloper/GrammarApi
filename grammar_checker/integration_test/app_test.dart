import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:grammar_checker/app.dart';
import 'package:grammar_checker/widgets/error_card.dart';
import 'package:grammar_checker/widgets/score_badge.dart';
import 'package:grammar_checker/widgets/text_input_area.dart';
import 'package:integration_test/integration_test.dart';

/// Integration tests for the Inkwell Grammar Checker app.
///
/// These tests require the Grammar API server to be running at localhost:8080.
/// Start the server with: `DISABLE_RATE_LIMITING=true cargo run`
void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  /// Waits for debounce then settles when API completes.
  Future<void> waitForAutoCheck(WidgetTester tester) async {
    await tester.pump(const Duration(milliseconds: 550));
    await tester.pumpAndSettle();
  }

  /// Enters text and waits for auto-check to complete.
  Future<void> enterTextAndWait(WidgetTester tester, String text) async {
    await tester.enterText(find.byType(TextField), text);
    await tester.pump(const Duration(milliseconds: 550));
    await tester.pumpAndSettle();
  }

  group('Initial UI', () {
    testWidgets('displays Inkwell branding', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      expect(find.text('Inkwell'), findsOneWidget);
      expect(find.byIcon(Icons.edit_note), findsOneWidget);
    });

    testWidgets('displays text input area with placeholder', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      expect(find.byType(TextInputArea), findsOneWidget);
      expect(find.byType(TextField), findsOneWidget);
      expect(
        find.text('Start writing or paste your text here...'),
        findsOneWidget,
      );
    });

    testWidgets('displays Suggestions header', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      expect(find.text('Suggestions'), findsOneWidget);
    });

    testWidgets('displays No errors found initially', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      expect(find.text('No errors found'), findsOneWidget);
      expect(find.text('Your writing looks great!'), findsOneWidget);
      expect(find.byIcon(Icons.check_circle_outline), findsOneWidget);
    });
  });

  group('Grammar Error Detection', () {
    testWidgets('detects article error "an test"', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This is an test.');

      expect(find.byType(ErrorCard), findsWidgets);
      expect(find.text('GRAMMAR'), findsWidgets);
    });

    testWidgets('shows error message for detected issues', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This is an test.');

      final errorCards = find.byType(ErrorCard);
      expect(errorCards.evaluate().isNotEmpty, isTrue);
    });

    testWidgets('shows replacement suggestions', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This is an test.');

      expect(find.text('a'), findsWidgets);
    });

    testWidgets('displays quoted error text in card', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This is an test.');

      expect(find.textContaining('"'), findsWidgets);
    });
  });

  group('Spelling Error Detection', () {
    testWidgets('detects simple misspelling', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This has a speling mistake.');

      expect(find.byType(ErrorCard), findsWidgets);
      expect(find.text('SPELLING'), findsWidgets);
    });

    testWidgets('suggests correct spelling', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This has a speling mistake.');

      expect(find.text('spelling'), findsWidgets);
    });

    testWidgets('detects multiple spelling errors', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This has speling and writting errors.');

      final errorCards = find.byType(ErrorCard);
      expect(errorCards.evaluate().length, greaterThanOrEqualTo(2));
    });
  });

  group('Replacement Application', () {
    testWidgets('applies replacement when suggestion tapped', (tester) async {
      await tester.binding.setSurfaceSize(const Size(1400, 900));
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();
      addTearDown(() => tester.binding.setSurfaceSize(null));

      await enterTextAndWait(tester, 'This is an test.');

      final suggestionFinder = find.text('a');
      expect(suggestionFinder, findsWidgets);
      await tester.tap(suggestionFinder.first);
      await tester.pumpAndSettle();

      final textField = tester.widget<TextField>(find.byType(TextField));
      expect(textField.controller?.text, contains('a test'));
    });

    testWidgets('removes error card after applying fix', (tester) async {
      await tester.binding.setSurfaceSize(const Size(1400, 900));
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();
      addTearDown(() => tester.binding.setSurfaceSize(null));

      await enterTextAndWait(tester, 'This is an test.');

      final initialErrorCount = find.byType(ErrorCard).evaluate().length;
      expect(initialErrorCount, greaterThan(0));

      final suggestionFinder = find.text('a');
      expect(suggestionFinder, findsWidgets);
      await tester.tap(suggestionFinder.first);
      await tester.pumpAndSettle();

      final newErrorCount = find.byType(ErrorCard).evaluate().length;
      expect(newErrorCount, lessThan(initialErrorCount));
    });
  });

  group('Correct Text Handling', () {
    testWidgets('shows no errors for correct sentence', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This is a correct sentence.');

      expect(find.text('No errors found'), findsOneWidget);
      expect(find.text('Your writing looks great!'), findsOneWidget);
      expect(find.byType(ErrorCard), findsNothing);
    });

    testWidgets('displays score badge for checked text', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This is a correct sentence.');

      expect(find.byType(ScoreBadge), findsOneWidget);
    });

    testWidgets('shows perfect score for error-free text', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This is perfectly written text.');

      expect(find.text('100'), findsOneWidget);
    });
  });

  group('Empty Text Handling', () {
    testWidgets('shows no errors for empty input', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await waitForAutoCheck(tester);

      expect(find.text('No errors found'), findsOneWidget);
      expect(find.byType(ErrorCard), findsNothing);
    });

    testWidgets('shows no errors for whitespace only', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, '   ');

      expect(find.text('No errors found'), findsOneWidget);
    });

    testWidgets('no score badge for empty text', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      expect(find.byType(ScoreBadge), findsNothing);
    });
  });

  group('Loading State', () {
    testWidgets('shows loading indicator during check', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await tester.enterText(find.byType(TextField), 'This is an test.');
      await tester.pump(const Duration(milliseconds: 510));
      await tester.pump();

      expect(find.byType(LinearProgressIndicator), findsOneWidget);
    });

    testWidgets('hides loading after check completes', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This is an test.');

      expect(find.byType(LinearProgressIndicator), findsNothing);
    });
  });

  group('Error Statistics', () {
    testWidgets('shows error count badge', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This is an test with speling errors.');

      final errorCountText = find.ancestor(
        of: find.textContaining(RegExp(r'^\d+$')),
        matching: find.byType(Container),
      );
      expect(errorCountText, findsWidgets);
    });

    testWidgets('shows spelling error count in footer', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This has speling erors.');

      expect(find.textContaining('spelling'), findsWidgets);
      expect(find.byIcon(Icons.spellcheck), findsOneWidget);
    });

    testWidgets('shows grammar error count in footer', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This is an test.');

      expect(find.textContaining('grammar'), findsWidgets);
      expect(find.byIcon(Icons.rule), findsOneWidget);
    });
  });

  group('Text Input Behavior', () {
    testWidgets('text field accepts multiline input', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      const multilineText = 'First line.\nSecond line.\nThird line.';
      await tester.enterText(find.byType(TextField), multilineText);
      await tester.pumpAndSettle();

      final textField = tester.widget<TextField>(find.byType(TextField));
      expect(textField.controller?.text, equals(multilineText));
    });

    testWidgets('text field expands vertically', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      final textField = tester.widget<TextField>(find.byType(TextField));
      expect(textField.expands, isTrue);
      expect(textField.maxLines, isNull);
    });

    testWidgets('debounces rapid text changes', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await tester.enterText(find.byType(TextField), 'T');
      await tester.pump(const Duration(milliseconds: 100));
      await tester.enterText(find.byType(TextField), 'Th');
      await tester.pump(const Duration(milliseconds: 100));
      await tester.enterText(find.byType(TextField), 'Thi');
      await tester.pump(const Duration(milliseconds: 100));

      expect(find.byType(LinearProgressIndicator), findsNothing);
    });
  });

  group('Mixed Error Types', () {
    testWidgets('detects both grammar and spelling errors', (tester) async {
      await tester.binding.setSurfaceSize(const Size(800, 1200));
      addTearDown(() => tester.binding.setSurfaceSize(null));

      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This is an test with speling errors.');

      expect(find.text('GRAMMAR'), findsWidgets);
      expect(find.text('SPELLING'), findsWidgets);
    });

    testWidgets('shows multiple error cards', (tester) async {
      await tester.binding.setSurfaceSize(const Size(800, 1200));
      addTearDown(() => tester.binding.setSurfaceSize(null));

      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This is an test with speling errors.');

      final errorCards = tester.widgetList<ErrorCard>(find.byType(ErrorCard));
      expect(errorCards.length, greaterThanOrEqualTo(2));
    });
  });

  group('UI Elements', () {
    testWidgets('has proper Material Design theming', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      final materialApp = tester.widget<MaterialApp>(find.byType(MaterialApp));
      expect(materialApp.debugShowCheckedModeBanner, isFalse);
      expect(materialApp.title, equals('Inkwell'));
    });

    testWidgets('uses safe area padding', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      expect(find.byType(SafeArea), findsOneWidget);
    });

    testWidgets('has divider in wide layout', (tester) async {
      await tester.binding.setSurfaceSize(const Size(1200, 800));
      addTearDown(() => tester.binding.setSurfaceSize(null));

      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      expect(find.byType(Row), findsWidgets);
    });
  });

  group('Error Card Details', () {
    testWidgets('displays error message text', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This is an test.');

      final errorCard = tester.widget<ErrorCard>(find.byType(ErrorCard).first);
      expect(errorCard.match.message.isNotEmpty, isTrue);
    });

    testWidgets('error card has match data', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This is a tset.');

      final errorCard = find.byType(ErrorCard).first;
      expect(errorCard, findsOneWidget);
    });

    testWidgets('error card displays error text', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'This is an test.');

      expect(find.byType(ErrorCard), findsWidgets);
    });
  });

  group('Score Badge', () {
    testWidgets('shows lower score for more errors', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, 'Ths is an tst wth mny erors.');

      final scoreBadge = tester.widget<ScoreBadge>(find.byType(ScoreBadge));
      expect(scoreBadge.score, lessThan(100));
    });

    testWidgets('does not show score before check', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      expect(find.byType(ScoreBadge), findsNothing);
    });
  });
}
