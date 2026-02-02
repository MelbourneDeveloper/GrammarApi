import 'package:flutter/material.dart';

// Primary - Warm Coral
/// Main brand color.
const coral = Color(0xFFF97066);

/// Darker shade for pressed states.
const coralDark = Color(0xFFDC4A3D);

/// Lighter shade for highlights.
const coralLight = Color(0xFFFEB8B3);

/// Subtle background tint.
const coralTint = Color(0xFFFEF1F0);

// Secondary - Deep Teal
/// Secondary accent color.
const teal = Color(0xFF0D9488);

/// Dark teal for accents.
const tealDark = Color(0xFF0F766E);

/// Light teal for highlights.
const tealLight = Color(0xFF5EEAD4);

/// Subtle teal background.
const tealTint = Color(0xFFF0FDFA);

// Error Indicators
/// Spelling error color.
const spellingError = Color(0xFFE11D48);

/// Spelling error background.
const spellingErrorLight = Color(0xFFFFE4E6);

/// Grammar error color.
const grammarError = Color(0xFFD97706);

/// Grammar error background.
const grammarErrorLight = Color(0xFFFEF3C7);

// Neutrals - Warm Grays
/// Primary text color.
const ink = Color(0xFF1F2937);

/// Secondary text color.
const slate = Color(0xFF475569);

/// Tertiary text color.
const stone = Color(0xFF78716C);

/// Border color.
const mist = Color(0xFFD6D3D1);

/// Background color.
const cloud = Color(0xFFF5F5F4);

/// Page background.
const paper = Color(0xFFFAFAF9);

/// Card/surface color.
const surface = Color(0xFFFFFFFF);

/// Material 3 color scheme from coral seed.
final inkwellColorScheme = ColorScheme.fromSeed(
  seedColor: coral,
  primary: coral,
  secondary: teal,
  surface: surface,
  error: spellingError,
);

/// Card shadow level 1.
const shadow1 = [
  BoxShadow(
    color: Color(0x08000000),
    blurRadius: 8,
    offset: Offset(0, 2),
  ),
];

/// Card shadow level 2 (hover).
const shadow2 = [
  BoxShadow(
    color: Color(0x0F000000),
    blurRadius: 16,
    offset: Offset(0, 4),
  ),
];

/// Card shadow level 3 (floating).
const shadow3 = [
  BoxShadow(
    color: Color(0x18000000),
    blurRadius: 24,
    offset: Offset(0, 8),
  ),
];

/// Coral glow for primary buttons.
const coralGlow = [
  BoxShadow(
    color: Color(0x30F97066),
    blurRadius: 20,
    offset: Offset(0, 8),
  ),
];
