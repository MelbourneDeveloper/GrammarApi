import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:grammar_checker/theme/app_colors.dart';
import 'package:grammar_checker/theme/app_theme.dart';

/// An animated circular score badge with progress ring.
class ScoreBadge extends StatelessWidget {
  /// Creates a [ScoreBadge].
  const ScoreBadge({
    required this.score,
    this.size = 56,
    super.key,
  });

  /// The score value (0-100).
  final int score;

  /// Size of the badge.
  final double size;

  @override
  Widget build(BuildContext context) {
    final color = _getColorForScore(score);
    final label = _getLabelForScore(score);

    return TweenAnimationBuilder<double>(
      tween: Tween(begin: 0, end: score / 100),
      duration: AppDurations.slow,
      curve: Curves.easeOutCubic,
      builder: (context, value, child) => SizedBox(
        width: size,
        height: size,
        child: Stack(
          alignment: Alignment.center,
          children: [
            // Background ring
            CustomPaint(
              size: Size(size, size),
              painter: _RingPainter(
                progress: 1,
                color: mist.withValues(alpha: 0.3),
                strokeWidth: 4,
              ),
            ),
            // Progress ring
            CustomPaint(
              size: Size(size, size),
              painter: _RingPainter(
                progress: value,
                color: color,
                strokeWidth: 4,
              ),
            ),
            // Score text
            Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                TweenAnimationBuilder<int>(
                  tween: IntTween(begin: 0, end: score),
                  duration: AppDurations.slow,
                  builder: (context, value, child) => Text(
                    '$value',
                    style: TextStyle(
                      fontSize: size * 0.32,
                      fontWeight: FontWeight.w700,
                      color: ink,
                      height: 1,
                    ),
                  ),
                ),
                Text(
                  label,
                  style: TextStyle(
                    fontSize: size * 0.14,
                    fontWeight: FontWeight.w500,
                    color: color,
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Color _getColorForScore(int score) {
    if (score >= 90) return teal;
    if (score >= 75) return coral;
    if (score >= 50) return grammarError;
    return spellingError;
  }

  String _getLabelForScore(int score) {
    if (score >= 90) return 'Excellent';
    if (score >= 75) return 'Good';
    if (score >= 50) return 'Fair';
    return 'Poor';
  }
}

class _RingPainter extends CustomPainter {
  _RingPainter({
    required this.progress,
    required this.color,
    required this.strokeWidth,
  });

  final double progress;
  final Color color;
  final double strokeWidth;

  @override
  void paint(Canvas canvas, Size size) {
    final center = Offset(size.width / 2, size.height / 2);
    final radius = (size.width - strokeWidth) / 2;

    final paint = Paint()
      ..color = color
      ..strokeWidth = strokeWidth
      ..style = PaintingStyle.stroke
      ..strokeCap = StrokeCap.round;

    canvas.drawArc(
      Rect.fromCircle(center: center, radius: radius),
      -math.pi / 2,
      2 * math.pi * progress,
      false,
      paint,
    );
  }

  @override
  bool shouldRepaint(_RingPainter oldDelegate) =>
      progress != oldDelegate.progress ||
      color != oldDelegate.color ||
      strokeWidth != oldDelegate.strokeWidth;
}
