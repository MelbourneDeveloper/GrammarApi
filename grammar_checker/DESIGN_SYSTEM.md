# Inkwell Design System

**A premium writing assistant with soul.**

---

## Brand Identity

**Name**: Inkwell
**Tagline**: "Write with confidence"
**Personality**: Warm, intelligent, elegant, trustworthy

Unlike Grammarly's clinical green, Inkwell uses a **warm coral and deep teal** palette that feels approachable yet sophisticated - like a wise writing mentor.

---

## Color Palette

### Primary - Warm Coral
The heart of Inkwell. Coral is energetic yet refined - it draws attention without demanding it.

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| Coral | `#F97066` | 249, 112, 102 | Primary actions, brand |
| Coral Dark | `#DC4A3D` | 220, 74, 61 | Hover, pressed states |
| Coral Light | `#FEB8B3` | 254, 184, 179 | Highlights, badges |
| Coral Tint | `#FEF1F0` | 254, 241, 240 | Subtle backgrounds |

### Secondary - Deep Teal
Balances the warmth with sophistication. Used for text and secondary actions.

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| Teal | `#0D9488` | 13, 148, 136 | Links, secondary buttons |
| Teal Dark | `#0F766E` | 15, 118, 110 | Text accents |
| Teal Light | `#5EEAD4` | 94, 234, 212 | Highlights |
| Teal Tint | `#F0FDFA` | 240, 253, 250 | Success backgrounds |

### Error States
| Name | Hex | Usage |
|------|-----|-------|
| Spelling | `#E11D48` | Rose 600 - Spelling errors |
| Spelling Light | `#FFE4E6` | Rose 100 - Spelling background |
| Grammar | `#D97706` | Amber 600 - Grammar issues |
| Grammar Light | `#FEF3C7` | Amber 100 - Grammar background |
| Style | `#7C3AED` | Violet 600 - Style suggestions |
| Style Light | `#EDE9FE` | Violet 100 - Style background |

### Neutrals - Warm Grays
Slightly warm grays that complement the coral, not cold clinical grays.

| Name | Hex | Usage |
|------|-----|-------|
| Ink | `#1F2937` | Primary text |
| Slate | `#475569` | Secondary text |
| Stone | `#78716C` | Tertiary text |
| Mist | `#D6D3D1` | Borders |
| Cloud | `#F5F5F4` | Backgrounds |
| Paper | `#FAFAF9` | Page background |
| White | `#FFFFFF` | Cards, surfaces |

---

## Typography

### Font Stack
```css
font-family: 'Source Serif 4', 'Georgia', serif; /* Headings - literary feel */
font-family: 'Inter', system-ui, sans-serif;     /* Body - clean readability */
font-family: 'JetBrains Mono', monospace;        /* Code snippets */
```

### Type Scale
| Style | Font | Weight | Size | Line Height | Letter Spacing |
|-------|------|--------|------|-------------|----------------|
| Display | Source Serif 4 | 600 | 32px | 1.2 | -0.02em |
| Headline | Source Serif 4 | 600 | 24px | 1.3 | -0.01em |
| Title | Inter | 600 | 18px | 1.4 | 0 |
| Subtitle | Inter | 500 | 14px | 1.4 | 0.01em |
| Body | Inter | 400 | 16px | 1.65 | 0 |
| Caption | Inter | 400 | 13px | 1.4 | 0.01em |
| Overline | Inter | 600 | 11px | 1.4 | 0.08em |

---

## Spacing System

### Base Unit: 4px

| Token | Value | Usage |
|-------|-------|-------|
| space-1 | 4px | Tight spacing |
| space-2 | 8px | Icon gaps |
| space-3 | 12px | Compact padding |
| space-4 | 16px | Standard padding |
| space-5 | 20px | Card padding |
| space-6 | 24px | Section gaps |
| space-8 | 32px | Large gaps |
| space-10 | 40px | Section spacing |
| space-12 | 48px | Page margins |

---

## Border Radius

| Token | Value | Usage |
|-------|-------|-------|
| radius-sm | 6px | Chips, small elements |
| radius-md | 10px | Buttons, inputs |
| radius-lg | 14px | Cards |
| radius-xl | 20px | Panels, modals |
| radius-full | 9999px | Pills, avatars |

---

## Elevation & Shadows

### Soft Shadow System
Inkwell uses soft, diffused shadows that feel natural and warm.

```dart
// Level 1 - Cards at rest
shadow1: BoxShadow(
  color: Color(0x08000000),
  blurRadius: 8,
  offset: Offset(0, 2),
)

// Level 2 - Cards on hover
shadow2: BoxShadow(
  color: Color(0x0F000000),
  blurRadius: 16,
  offset: Offset(0, 4),
)

// Level 3 - Floating elements
shadow3: BoxShadow(
  color: Color(0x18000000),
  blurRadius: 24,
  offset: Offset(0, 8),
)

// Coral glow - For primary buttons
coralGlow: BoxShadow(
  color: Color(0x30F97066),
  blurRadius: 20,
  offset: Offset(0, 8),
)
```

---

## Motion Design

### Principles
1. **Purposeful** - Animation guides attention, not distracts
2. **Swift** - Fast enough to feel responsive
3. **Natural** - Follows natural motion curves

### Timing
| Name | Duration | Usage |
|------|----------|-------|
| instant | 50ms | Color changes |
| fast | 150ms | Button states |
| normal | 250ms | Transitions |
| slow | 400ms | Entrances |
| stagger | 60ms | List item delay |

### Curves
```dart
// Standard ease - most transitions
easeOut: Curves.easeOutCubic

// Bouncy - playful elements
spring: Curves.elasticOut

// Smooth - scroll-linked animations
smooth: Curves.easeInOutCubic
```

### Signature Animations

**Card Entrance** (Inkwell's unique "unfold" effect)
```dart
// Cards slide up and fade in with a slight rotation
SlideTransition(
  position: Tween(
    begin: Offset(0, 0.1),
    end: Offset.zero,
  ).animate(curve: Curves.easeOutCubic),
)
FadeTransition(opacity: animation)
```

**Suggestion Chips** (Gentle pulse on hover)
```dart
// Chips gently pulse and lift on hover
Transform.scale(scale: 1.02)
BoxShadow with animated blur
```

**Success Celebration** (Subtle confetti burst)
```dart
// When all errors are fixed, tiny particles burst from the score
ParticleAnimation with coral/teal particles
```

---

## Components

### Writing Score (Unique to Inkwell)

A circular score badge with animated progress ring.

```
     ╭──────────╮
     │    87    │
     │  ━━━━━━  │
     │   Good   │
     ╰──────────╯
```

| Score | Color | Label | Ring |
|-------|-------|-------|------|
| 90-100 | Teal | Excellent | Full ring, pulse glow |
| 75-89 | Coral | Good | 3/4 ring |
| 50-74 | Grammar (Amber) | Fair | 1/2 ring |
| 0-49 | Spelling (Rose) | Needs Work | 1/4 ring |

### Error Cards

Left accent bar indicates error type. Clean hierarchy.

```
┌────────────────────────────────────────────────┐
│█                                               │
│█  SPELLING                            "teh"   │
│█                                               │
│█  Did you mean "the"?                          │
│█                                               │
│█  ┌──────┐  ┌───────┐  ┌────────┐            │
│█  │  the │  │ they  │  │ their  │            │
│█  └──────┘  └───────┘  └────────┘            │
└────────────────────────────────────────────────┘
```

### Primary Button

Gradient coral with glow effect.

```dart
Container(
  decoration: BoxDecoration(
    gradient: LinearGradient(
      colors: [coral, coralDark],
      begin: Alignment.topLeft,
      end: Alignment.bottomRight,
    ),
    borderRadius: BorderRadius.circular(10),
    boxShadow: coralGlow,
  ),
  child: Text('Check Writing', style: white/600/16px),
)
```

### Text Input Area

Clean with focus glow.

```dart
// Unfocused
Border: 1px Mist
Background: White
Shadow: shadow1

// Focused
Border: 2px Coral
Shadow: 0 0 0 4px Coral/10%
```

---

## Layout

### Desktop (>1024px)
```
┌─────────────────────────────────────────────────────┐
│  🖋 Inkwell                              Score: 87  │
├─────────────────────────────────────────────────────┤
│                              │                      │
│                              │  ┌────────────────┐  │
│     Writing Area             │  │  Error Card 1  │  │
│     (Editor)                 │  └────────────────┘  │
│                              │  ┌────────────────┐  │
│     65%                      │  │  Error Card 2  │  │
│                              │  └────────────────┘  │
│                              │                      │
│                              │       35%            │
├──────────────────────────────┴──────────────────────┤
│  [Check Writing]                     12ms • 847 words│
└─────────────────────────────────────────────────────┘
```

### Mobile (<768px)
```
┌─────────────────────┐
│ 🖋 Inkwell    87    │
├─────────────────────┤
│                     │
│   Writing Area      │
│   (Collapsed)       │
│                     │
├─────────────────────┤
│ [Check Writing]     │
├─────────────────────┤
│ ┌─────────────────┐ │
│ │  Error Card 1   │ │
│ └─────────────────┘ │
│ ┌─────────────────┐ │
│ │  Error Card 2   │ │
│ └─────────────────┘ │
└─────────────────────┘
```

---

## Accessibility

### Color Contrast
All text meets WCAG AA standards:
- Ink on Paper: 14.7:1 (AAA)
- Slate on Paper: 7.2:1 (AAA)
- Coral on White: 4.8:1 (AA Large Text)
- White on Coral: 4.8:1 (AA Large Text)

### Focus States
- Visible focus ring: 3px Coral with 4px offset
- All interactive elements keyboard accessible
- Screen reader announcements for errors

---

## Differentiators from Grammarly

| Aspect | Grammarly | Inkwell |
|--------|-----------|---------|
| Primary Color | Clinical green | Warm coral |
| Feel | Corporate tool | Writing companion |
| Typography | Sans-serif only | Serif headings (literary) |
| Shadows | Sharp | Soft, warm |
| Animations | Minimal | Signature "unfold" effect |
| Score | Plain number | Animated ring with glow |
| Identity | Generic | "Inkwell" brand with pen motif |
