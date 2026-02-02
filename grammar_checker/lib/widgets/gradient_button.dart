import 'dart:async';

import 'package:flutter/material.dart';
import 'package:grammar_checker/theme/app_colors.dart';
import 'package:grammar_checker/theme/app_theme.dart';

/// A premium gradient button with press animation and loading state.
class GradientButton extends StatefulWidget {
  /// Creates a [GradientButton].
  const GradientButton({
    required this.onPressed,
    required this.child,
    this.isLoading = false,
    super.key,
  });

  /// Callback when button is pressed.
  final VoidCallback? onPressed;

  /// Button content.
  final Widget child;

  /// Whether the button is in loading state.
  final bool isLoading;

  @override
  State<GradientButton> createState() => _GradientButtonState();
}

class _GradientButtonState extends State<GradientButton>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _scaleAnimation;
  bool _isPressed = false;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: AppDurations.fast,
    );
    _scaleAnimation = Tween<double>(begin: 1, end: 0.97).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeOut),
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _onTapDown(TapDownDetails details) {
    if (widget.onPressed != null && !widget.isLoading) {
      setState(() => _isPressed = true);
      unawaited(_controller.forward());
    }
  }

  void _onTapUp(TapUpDetails details) {
    if (_isPressed) {
      setState(() => _isPressed = false);
      unawaited(_controller.reverse());
    }
  }

  void _onTapCancel() {
    if (_isPressed) {
      setState(() => _isPressed = false);
      unawaited(_controller.reverse());
    }
  }

  @override
  Widget build(BuildContext context) {
    final isDisabled = widget.onPressed == null || widget.isLoading;

    return GestureDetector(
      onTapDown: _onTapDown,
      onTapUp: _onTapUp,
      onTapCancel: _onTapCancel,
      onTap: isDisabled ? null : widget.onPressed,
      child: AnimatedBuilder(
        animation: _scaleAnimation,
        builder: (context, child) => Transform.scale(
          scale: _scaleAnimation.value,
          child: child,
        ),
        child: AnimatedContainer(
          duration: AppDurations.fast,
          padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 16),
          decoration: BoxDecoration(
            gradient: isDisabled
                ? const LinearGradient(
                    begin: Alignment.topLeft,
                    end: Alignment.bottomRight,
                    colors: [coralLight, coralLight],
                  )
                : const LinearGradient(
                    begin: Alignment.topLeft,
                    end: Alignment.bottomRight,
                    colors: [coral, coralDark],
                  ),
            borderRadius: BorderRadius.circular(10),
            boxShadow: isDisabled || _isPressed ? null : coralGlow,
          ),
          child: AnimatedSwitcher(
            duration: AppDurations.normal,
            child: widget.isLoading
                ? const SizedBox(
                    height: 20,
                    width: 20,
                    child: CircularProgressIndicator(
                      strokeWidth: 2,
                      valueColor: AlwaysStoppedAnimation<Color>(surface),
                    ),
                  )
                : DefaultTextStyle(
                    style: const TextStyle(
                      color: surface,
                      fontSize: 16,
                      fontWeight: FontWeight.w600,
                    ),
                    child: widget.child,
                  ),
          ),
        ),
      ),
    );
  }
}
