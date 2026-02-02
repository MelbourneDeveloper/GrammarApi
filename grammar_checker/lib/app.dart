import 'package:flutter/material.dart';
import 'package:grammar_checker/providers/grammar_check_provider.dart';
import 'package:grammar_checker/screens/grammar_check_screen.dart';
import 'package:grammar_checker/theme/app_theme.dart';
import 'package:provider/provider.dart';

/// The main application widget.
class GrammarCheckerApp extends StatelessWidget {
  /// Creates a new [GrammarCheckerApp] instance.
  const GrammarCheckerApp({super.key, this.loadSample = true});

  /// Whether to load sample text on startup.
  final bool loadSample;

  @override
  Widget build(BuildContext context) => ChangeNotifierProvider(
        create: (_) => GrammarCheckProvider(loadSample: loadSample),
        child: MaterialApp(
          title: 'Inkwell',
          debugShowCheckedModeBanner: false,
          theme: inkwellTheme,
          home: const GrammarCheckScreen(),
        ),
      );
}
