import 'package:flutter/material.dart';
import 'package:grammar_checker/models/grammar_match.dart';
import 'package:grammar_checker/theme/app_colors.dart';
import 'package:grammar_checker/theme/app_theme.dart';

/// A premium text input area with focus animations.
class TextInputArea extends StatefulWidget {
  /// Creates a new [TextInputArea] instance.
  const TextInputArea({
    required this.text,
    required this.matches,
    required this.onTextChanged,
    super.key,
  });

  /// The current text.
  final String text;

  /// List of grammar/spelling errors.
  final List<GrammarMatch> matches;

  /// Callback when text changes.
  final ValueChanged<String> onTextChanged;

  @override
  State<TextInputArea> createState() => _TextInputAreaState();
}

class _TextInputAreaState extends State<TextInputArea> {
  late TextEditingController _controller;
  final FocusNode _focusNode = FocusNode();
  bool _isFocused = false;

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController(text: widget.text);
    _focusNode.addListener(_onFocusChange);
  }

  @override
  void didUpdateWidget(TextInputArea oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.text != _controller.text) {
      _controller.text = widget.text;
    }
  }

  @override
  void dispose() {
    _focusNode
      ..removeListener(_onFocusChange)
      ..dispose();
    _controller.dispose();
    super.dispose();
  }

  void _onFocusChange() {
    setState(() => _isFocused = _focusNode.hasFocus);
  }

  @override
  Widget build(BuildContext context) => AnimatedContainer(
        duration: AppDurations.fast,
        decoration: BoxDecoration(
          color: surface,
          borderRadius: BorderRadius.circular(14),
          border: Border.all(
            color: _isFocused ? coral : mist,
            width: _isFocused ? 2 : 1,
          ),
          boxShadow: _isFocused
              ? const [
                  BoxShadow(
                    color: Color(0x15F97066),
                    blurRadius: 8,
                    spreadRadius: 2,
                  ),
                ]
              : shadow1,
        ),
        child: TextField(
          controller: _controller,
          focusNode: _focusNode,
          onChanged: widget.onTextChanged,
          maxLines: null,
          expands: true,
          textAlignVertical: TextAlignVertical.top,
          cursorColor: coral,
          decoration: const InputDecoration(
            hintText: 'Start writing or paste your text here...',
            hintStyle: TextStyle(color: stone, fontSize: 16),
            border: InputBorder.none,
            contentPadding: EdgeInsets.all(20),
          ),
          style: const TextStyle(
            fontSize: 16,
            height: 1.65,
            color: ink,
          ),
        ),
      );
}
