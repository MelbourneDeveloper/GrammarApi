# Grammar API

Open source spelling and grammar correction API for word processors and text-based UIs.

## Requirements

- **Open source**: No proprietary dependencies
- **Deterministic**: Rule-based, no ML/LLM randomness
- **Fast**: Sub-100ms response times for typical text blocks
- **Language**: English initially, multi-language support planned
- **Deployment**: Containerized API (Docker)
- **Dictionary**: Standard Hunspell en_US (LibreOffice/Firefox)

---

## Technology Stack

### Grammar: Harper
**Language**: Rust | **License**: Apache 2.0

- Millisecond-level linting (50x faster than LanguageTool)
- 1/50th memory footprint of LanguageTool
- Active development by Automattic (WordPress)
- English only (multi-language planned)

### Spelling: Spellbook
**Language**: Rust | **License**: MIT

- Helix editor's Rust rewrite of Nuspell
- Hunspell dictionary compatible (same as LibreOffice, Firefox, Chrome)
- `no_std` compatible, minimal dependencies

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                   Grammar API                       │
│                  (Rust/Axum)                        │
├─────────────────────────────────────────────────────┤
│  POST /v1/check                                     │
│  { "text": "...", "language": "en-US" }            │
│  →                                                  │
│  { "matches": [...], "metrics": {...} }            │
├─────────────────────────────────────────────────────┤
│              Processing Pipeline                    │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐     │
│  │ Spellbook│ →  │  Harper  │ →  │  Merge   │     │
│  │(spelling)│    │(grammar) │    │ Results  │     │
│  └──────────┘    └──────────┘    └──────────┘     │
└─────────────────────────────────────────────────────┘
```

## API

### Check Endpoint

```http
POST /v1/check
Content-Type: application/json

{
  "text": "This is a sentnece with erors.",
  "language": "en-US",
  "options": {
    "spelling": true,
    "grammar": true
  }
}
```

### Response

```json
{
  "matches": [
    {
      "message": "Possible spelling mistake",
      "offset": 10,
      "length": 8,
      "replacements": ["sentence"],
      "rule": { "id": "SPELL", "category": "spelling" }
    },
    {
      "message": "Possible spelling mistake",
      "offset": 24,
      "length": 5,
      "replacements": ["errors"],
      "rule": { "id": "SPELL", "category": "spelling" }
    }
  ],
  "metrics": {
    "processingTimeMs": 12
  }
}
```

---

## Implementation Stack

| Component | Technology | Notes |
|-----------|------------|-------|
| HTTP Server | Axum | Fast async Rust web framework |
| Grammar | harper-core | Native Rust grammar checking |
| Spelling | spellbook | Hunspell-compatible, pure Rust |
| Dictionaries | Hunspell format | LibreOffice/Firefox dictionaries |
| Serialization | serde | Standard Rust JSON handling |
| Deployment | Docker | Single container, minimal footprint |

---

## AI Integration (Future)

Separate service for non-deterministic features:
- Style suggestions
- Rewrite/rephrase
- Tone adjustments
- Context-aware corrections

---

## References

- [Harper](https://github.com/Automattic/harper)
- [Spellbook](https://github.com/helix-editor/spellbook)
- [Hunspell](https://github.com/hunspell/hunspell)
