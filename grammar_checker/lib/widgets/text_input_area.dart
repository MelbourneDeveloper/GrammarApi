import 'package:flutter/material.dart';
import 'package:grammar_checker/models/grammar_match.dart';

/// A text input area for entering text to check.
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

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController(text: widget.text);
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
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => DecoratedBox(
        decoration: BoxDecoration(
          border: Border.all(color: Colors.grey.shade300),
          borderRadius: BorderRadius.circular(8),
        ),
        child: TextField(
          controller: _controller,
          onChanged: widget.onTextChanged,
          maxLines: null,
          expands: true,
          textAlignVertical: TextAlignVertical.top,
          decoration: const InputDecoration(
            hintText: 'Enter text to check...',
            border: InputBorder.none,
            contentPadding: EdgeInsets.all(16),
          ),
          style: const TextStyle(fontSize: 16, height: 1.5),
        ),
      );
}
