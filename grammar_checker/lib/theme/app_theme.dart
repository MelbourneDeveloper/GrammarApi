import 'package:flutter/material.dart';
import 'package:grammar_checker/theme/app_colors.dart';

/// Inkwell Material 3 theme configuration.
final inkwellTheme = ThemeData(
  useMaterial3: true,
  colorScheme: inkwellColorScheme,
  scaffoldBackgroundColor: paper,
  appBarTheme: const AppBarTheme(
    backgroundColor: surface,
    foregroundColor: ink,
    elevation: 0,
    centerTitle: false,
    titleTextStyle: TextStyle(
      color: ink,
      fontSize: 20,
      fontWeight: FontWeight.w600,
      letterSpacing: -0.5,
    ),
  ),
  cardTheme: CardThemeData(
    color: surface,
    elevation: 0,
    shape: RoundedRectangleBorder(
      borderRadius: BorderRadius.circular(14),
    ),
    margin: EdgeInsets.zero,
  ),
  elevatedButtonTheme: ElevatedButtonThemeData(
    style: ElevatedButton.styleFrom(
      backgroundColor: coral,
      foregroundColor: surface,
      elevation: 0,
      padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(10),
      ),
      textStyle: const TextStyle(
        fontSize: 16,
        fontWeight: FontWeight.w600,
      ),
    ),
  ),
  chipTheme: ChipThemeData(
    backgroundColor: coralTint,
    labelStyle: const TextStyle(
      color: coralDark,
      fontSize: 14,
      fontWeight: FontWeight.w500,
    ),
    shape: RoundedRectangleBorder(
      borderRadius: BorderRadius.circular(20),
    ),
    padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
  ),
  inputDecorationTheme: InputDecorationTheme(
    filled: true,
    fillColor: surface,
    border: OutlineInputBorder(
      borderRadius: BorderRadius.circular(12),
      borderSide: const BorderSide(color: mist),
    ),
    enabledBorder: OutlineInputBorder(
      borderRadius: BorderRadius.circular(12),
      borderSide: const BorderSide(color: mist),
    ),
    focusedBorder: OutlineInputBorder(
      borderRadius: BorderRadius.circular(12),
      borderSide: const BorderSide(color: coral, width: 2),
    ),
    contentPadding: const EdgeInsets.all(16),
    hintStyle: const TextStyle(color: stone),
  ),
  textTheme: const TextTheme(
    displayLarge: TextStyle(
      color: ink,
      fontSize: 32,
      fontWeight: FontWeight.w600,
      letterSpacing: -0.5,
    ),
    headlineMedium: TextStyle(
      color: ink,
      fontSize: 24,
      fontWeight: FontWeight.w600,
      letterSpacing: -0.3,
    ),
    titleLarge: TextStyle(
      color: ink,
      fontSize: 18,
      fontWeight: FontWeight.w600,
    ),
    titleMedium: TextStyle(
      color: ink,
      fontSize: 16,
      fontWeight: FontWeight.w500,
    ),
    bodyLarge: TextStyle(
      color: ink,
      fontSize: 16,
      height: 1.6,
    ),
    bodyMedium: TextStyle(
      color: slate,
      fontSize: 14,
      height: 1.5,
    ),
    labelLarge: TextStyle(
      color: ink,
      fontSize: 14,
      fontWeight: FontWeight.w600,
    ),
    bodySmall: TextStyle(
      color: stone,
      fontSize: 13,
    ),
  ),
  dividerTheme: const DividerThemeData(
    color: mist,
    thickness: 1,
  ),
);

/// Animation durations for the app.
abstract final class AppDurations {
  /// Instant transitions.
  static const instant = Duration(milliseconds: 50);

  /// Fast transitions (button press).
  static const fast = Duration(milliseconds: 150);

  /// Normal transitions.
  static const normal = Duration(milliseconds: 250);

  /// Slow transitions (page changes).
  static const slow = Duration(milliseconds: 400);

  /// Stagger delay for list items.
  static const stagger = Duration(milliseconds: 60);
}
