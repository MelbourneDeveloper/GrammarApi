import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:grammar_checker/app.dart';
import 'package:grammar_checker/widgets/error_card.dart';
import 'package:grammar_checker/widgets/score_badge.dart';
import 'package:grammar_checker/widgets/text_input_area.dart';
import 'package:integration_test/integration_test.dart';

import 'test_fixtures.dart';

/// Integration tests for the Inkwell Grammar Checker app.
///
/// These tests require the Grammar API server to be running at localhost:8080.
/// Start the server with: `DISABLE_RATE_LIMITING=true cargo run`
void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  Future<void> enterTextAndWait(WidgetTester tester, String text) async {
    await tester.enterText(find.byType(TextField), text);
    await tester.pump(const Duration(milliseconds: 550));
    await tester.pumpAndSettle();
  }

  group('Initial UI', () {
    testWidgets('displays branding and input area', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      expect(find.text('Inkwell'), findsOneWidget);
      expect(find.byIcon(Icons.edit_note), findsOneWidget);
      expect(find.byType(TextInputArea), findsOneWidget);
      expect(find.text('Suggestions'), findsOneWidget);
      expect(find.text('No errors found'), findsOneWidget);
    });
  });

  group('Grammar Errors', () {
    testWidgets('detects article error', (tester) async {
      final tc = TestFixtures.getById('grammar_article_an_before_consonant');
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, tc.input);

      expect(find.byType(ErrorCard), findsWidgets);
      expect(find.text('GRAMMAR'), findsWidgets);
      final replacement = tc.expectedErrors.first.replacements!.first;
      expect(find.text(replacement), findsWidgets);
    });
  });

  group('Spelling Errors', () {
    testWidgets('detects misspelling', (tester) async {
      final tc = TestFixtures.getById('spelling_simple_misspelling');
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, tc.input);

      expect(find.byType(ErrorCard), findsWidgets);
      expect(find.text('SPELLING'), findsWidgets);
      final replacement = tc.expectedErrors.first.replacements!.first;
      expect(find.text(replacement), findsWidgets);
    });

    testWidgets('detects multiple errors', (tester) async {
      await tester.binding.setSurfaceSize(const Size(800, 1200));
      addTearDown(() => tester.binding.setSurfaceSize(null));

      final tc = TestFixtures.getById('spelling_multiple_errors');
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, tc.input);

      final count = tc.expectedErrors.first.minCount ?? 1;
      final cards = find.byType(ErrorCard).evaluate().length;
      expect(cards, greaterThanOrEqualTo(count));
    });
  });

  group('Mixed Errors', () {
    testWidgets('detects grammar and spelling', (tester) async {
      await tester.binding.setSurfaceSize(const Size(800, 1200));
      addTearDown(() => tester.binding.setSurfaceSize(null));

      final tc = TestFixtures.getById('mixed_grammar_and_spelling');
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, tc.input);

      expect(find.text('GRAMMAR'), findsWidgets);
      expect(find.text('SPELLING'), findsWidgets);
      expect(find.byType(ErrorCard).evaluate().length, greaterThanOrEqualTo(2));
    });
  });

  group('Correct Text', () {
    testWidgets('shows no errors', (tester) async {
      final tc = TestFixtures.getById('correct_sentence');
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, tc.input);

      expect(find.text('No errors found'), findsOneWidget);
      expect(find.byType(ErrorCard), findsNothing);
      expect(find.byType(ScoreBadge), findsOneWidget);
    });

    testWidgets('shows perfect score', (tester) async {
      final tc = TestFixtures.getById('correct_pangram');
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, tc.input);

      expect(find.text('100'), findsOneWidget);
    });
  });

  group('Replacement', () {
    testWidgets('applies fix when tapped', (tester) async {
      await tester.binding.setSurfaceSize(const Size(1400, 900));
      addTearDown(() => tester.binding.setSurfaceSize(null));

      final tc = TestFixtures.getById('grammar_article_an_before_consonant');
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, tc.input);

      final replacement = tc.expectedErrors.first.replacements!.first;
      await tester.tap(find.text(replacement).first);
      await tester.pumpAndSettle();

      final textField = tester.widget<TextField>(find.byType(TextField));
      expect(textField.controller?.text, contains('$replacement test'));
    });

    testWidgets('removes error card after fix', (tester) async {
      await tester.binding.setSurfaceSize(const Size(1400, 900));
      addTearDown(() => tester.binding.setSurfaceSize(null));

      final tc = TestFixtures.getById('grammar_article_an_before_consonant');
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, tc.input);
      final before = find.byType(ErrorCard).evaluate().length;

      final replacement = tc.expectedErrors.first.replacements!.first;
      await tester.tap(find.text(replacement).first);
      await tester.pumpAndSettle();

      expect(find.byType(ErrorCard).evaluate().length, lessThan(before));
    });
  });

  group('Empty/Whitespace', () {
    testWidgets('empty text shows no errors', (tester) async {
      final tc = TestFixtures.getById('empty_text');
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, tc.input);

      expect(find.text('No errors found'), findsOneWidget);
      expect(find.byType(ScoreBadge), findsNothing);
    });

    testWidgets('whitespace only shows no errors', (tester) async {
      final tc = TestFixtures.getById('whitespace_only');
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, tc.input);

      expect(find.text('No errors found'), findsOneWidget);
    });
  });

  group('UI', () {
    testWidgets('Material theming', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      final app = tester.widget<MaterialApp>(find.byType(MaterialApp));
      expect(app.debugShowCheckedModeBanner, isFalse);
      expect(app.title, equals('Inkwell'));
      expect(find.byType(SafeArea), findsOneWidget);
    });

    testWidgets('multiline input', (tester) async {
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      final textField = tester.widget<TextField>(find.byType(TextField));
      expect(textField.expands, isTrue);
      expect(textField.maxLines, isNull);
    });

    testWidgets('lower score for errors', (tester) async {
      final tc = TestFixtures.getById('spelling_multiple_errors');
      await tester.pumpWidget(const GrammarCheckerApp(loadSample: false));
      await tester.pumpAndSettle();

      await enterTextAndWait(tester, tc.input);

      final badge = tester.widget<ScoreBadge>(find.byType(ScoreBadge));
      expect(badge.score, lessThan(100));
    });
  });

  group('Sample Text', () {
    testWidgets('loads and detects errors', (tester) async {
      await tester.binding.setSurfaceSize(const Size(1400, 900));
      addTearDown(() => tester.binding.setSurfaceSize(null));

      await tester.pumpWidget(const GrammarCheckerApp());
      await tester.pump(const Duration(milliseconds: 550));
      await tester.pumpAndSettle();

      expect(find.textContaining('Inkwell'), findsWidgets);
      expect(find.byType(ErrorCard), findsWidgets);
    });
  });
}
