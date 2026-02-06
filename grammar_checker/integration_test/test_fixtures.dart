import 'dart:convert';

const _fixturesJson = '''
{
  "version": "1.0",
  "cases": [
    {
      "id": "grammar_article_an_before_consonant",
      "input": "This is an test.",
      "expectedErrors": [{"category": "grammar", "replacements": ["a"]}]
    },
    {
      "id": "spelling_simple_misspelling",
      "input": "This has a speling mistake.",
      "expectedErrors": [{"category": "spelling", "replacements": ["spelling"]}]
    },
    {
      "id": "spelling_multiple_errors",
      "input": "The quik brwon fox jumps ovar the lazzy dog.",
      "expectedErrors": [{"category": "spelling", "minCount": 3}]
    },
    {
      "id": "mixed_grammar_and_spelling",
      "input": "This is an test with speling errors.",
      "expectedErrors": [
        {"category": "grammar", "replacements": ["a"]},
        {"category": "spelling", "replacements": ["spelling"]}
      ]
    },
    {
      "id": "correct_sentence",
      "input": "This is a correct sentence.",
      "expectedErrors": []
    },
    {
      "id": "correct_pangram",
      "input": "The quick brown fox jumps over the lazy dog.",
      "expectedErrors": []
    },
    {
      "id": "empty_text",
      "input": "",
      "expectedErrors": []
    },
    {
      "id": "whitespace_only",
      "input": "   ",
      "expectedErrors": []
    }
  ]
}
''';

/// Shared test fixtures.
class TestFixtures {
  TestFixtures._({required this.cases});

  factory TestFixtures.load() {
    if (_loaded) return _instance;
    final json = jsonDecode(_fixturesJson) as Map<String, dynamic>;
    _instance = TestFixtures._(
      cases: (json['cases'] as List<dynamic>)
          .map((e) => TestCase.fromJson(e as Map<String, dynamic>))
          .toList(),
    );
    _loaded = true;
    return _instance;
  }

  static late TestFixtures _instance;
  static bool _loaded = false;
  final List<TestCase> cases;

  static TestCase getById(String id) {
    final fixtures = TestFixtures.load();
    return fixtures.cases.firstWhere(
      (c) => c.id == id,
      orElse: () => throw StateError('Missing fixture: $id'),
    );
  }
}

/// A single test case.
class TestCase {
  TestCase({
    required this.id,
    required this.input,
    required this.expectedErrors,
  });

  factory TestCase.fromJson(Map<String, dynamic> json) => TestCase(
        id: json['id'] as String,
        input: json['input'] as String,
        expectedErrors: (json['expectedErrors'] as List<dynamic>)
            .map((e) => ExpectedError.fromJson(e as Map<String, dynamic>))
            .toList(),
      );

  final String id;
  final String input;
  final List<ExpectedError> expectedErrors;
}

/// Expected error specification.
class ExpectedError {
  ExpectedError({required this.category, this.replacements, this.minCount});

  factory ExpectedError.fromJson(Map<String, dynamic> json) => ExpectedError(
        category: json['category'] as String,
        replacements: (json['replacements'] as List<dynamic>?)
            ?.map((e) => e as String)
            .toList(),
        minCount: json['minCount'] as int?,
      );

  final String category;
  final List<String>? replacements;
  final int? minCount;
}
