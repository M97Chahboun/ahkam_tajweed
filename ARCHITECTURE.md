# Module Documentation

This document provides an overview of the library's module structure and how they work together.

## Module Hierarchy

```
tajweed_warsh_rules (crate root)
├── processor      (Main processor module)
├── types          (Core types and structures)
├── utils          (Utility functions)
└── main           (CLI application - binary only)
```

## Module Details

### `types` Module

**File**: `src/types.rs`
**Purpose**: Defines all core data structures and enumerations

**Key Types**:
- `RecitationStyle` enum - Hafs or Warsh recitation
- `TajweedRuleType` enum - 30+ rule variants
- `TajweedRule` struct - Rule metadata with descriptions
- `RuleMatch` struct - Detection results
- `MaddContext` enum - Context information for Madd rules

**Responsibilities**:
- Type definitions
- Enum variants for all supported rules
- Rule descriptions in Arabic and English
- Type-specific metadata

**No Dependencies**: This module is self-contained and has no dependencies on other project modules.

### `utils` Module

**File**: `src/utils.rs`
**Purpose**: Provides helper functions for Arabic character and diacritic handling

**Key Functions**:
- `is_tajweed_ignorable()` - Check if character is diacritic
- `is_sukun()` - Detect Sukun marker
- `is_tanwin()` - Detect Tanwin markers
- `is_shadda()` - Detect Shadda (doubling)
- `is_vowel()` - Detect vowel marks
- `is_hamza()` - Detect Hamza in any form
- `get_preceding_vowel()` - Get vowel before/after character
- `get_context()` - Extract surrounding characters
- `is_following_hamza()` - Check for hamza after position
- `is_following_shadda()` - Check for shadda after position
- `is_word_end()` - Determine if at word boundary

**Test Coverage**:
- `test_is_vowel()` - Vowel detection
- `test_is_hamza()` - Hamza detection
- `test_is_tajweed_ignorable()` - Diacritic handling

**Dependencies**: Only std library

### `processor` Module

**File**: `src/processor.rs`
**Purpose**: Main Tajweed rule detection and processing

**Key Types**:
- `TajweedProcessor` struct - Main processor with internal maps

**Public Methods**:
- `new(style)` - Create processor for a style
- `process_verse()` - Analyze verse and detect rules
- `get_style()` - Get processor's recitation style

**Private Methods** (Internal implementation):
- `determine_rule_for_noon()` - Noon/Tanwin rules
- `determine_rule_for_mim()` - Mim Sakinah rules
- `determine_rule_for_lam_al()` - Lam Al-Ta'rif rules
- `detect_madd()` - Madd rule detection
- `detect_soft_madd()` - Waaw/Ya madd variants
- `detect_tafkhim_ra()` - Ra emphasis
- `detect_tarqeeq_ra()` - Ra thinning (Warsh)
- `detect_tafkhim_lafuljalala()` - Allah name emphasis
- `detect_qalqalah()` - Qalqalah detection
- Helper methods for first pass through verse

**Processing Pipeline**:
1. Noon/Mim Sakinah detection
2. Madd rules detection
3. Qalqalah detection
4. Ra emphasis detection
5. Allah name emphasis detection

**Test Coverage**:
- `test_processor_creation()` - Processor instantiation
- `test_basic_rule_detection()` - Basic functionality

**Dependencies**: Uses `types`, `utils`, and `std::collections::HashMap`

### `lib.rs` Module (Root)

**File**: `src/lib.rs`
**Purpose**: Library root, public API definition, and module organization

**Public Exports**:
```rust
pub use processor::TajweedProcessor;
pub use types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType};
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
```

**Module Declarations**:
- `pub mod processor`
- `pub mod types`
- `pub mod utils`

**Documentation**: Comprehensive library documentation with feature list and examples.

### `main.rs` (CLI Application)

**File**: `src/main.rs`
**Purpose**: Interactive command-line interface for verse analysis

**Features**:
- Interactive prompt for verse input
- Style switching (Warsh/Hafs/Both)
- Formatted output with rule details
- Context display around detected rules

**Uses**: Public API from library via `tajweed_warsh_rules` crate import

## Data Flow

```
User Input (Verse)
    ↓
TajweedProcessor::process_verse()
    ↓
[Detection Passes]
  1. Noon/Mim detection → RuleMatch
  2. Madd detection → RuleMatch
  3. Qalqalah detection → RuleMatch
  4. Ra detection → RuleMatch
  5. Allah name detection → RuleMatch
    ↓
Vec<RuleMatch>
    ↓
CLI Output / User Application
```

## Module Dependencies

```
main.rs
  └─ tajweed_warsh_rules (public crate API)
       ├── processor
       │    ├── types
       │    └── utils
       ├── types
       └── utils
```

## Type Flow

```
Input: String (Quranic verse)
    ↓
Converted to Vec<char>
    ↓
Process with utility functions (utils)
    ↓
Match against rule types (types)
    ↓
Create RuleMatch (types)
    ↓
Output: Vec<RuleMatch>
```

## Extension Points

### Adding New Rules

1. Add variant to `TajweedRuleType` enum in `types.rs`
2. Add rule metadata in `TajweedRule::from_type()` in `types.rs`
3. Implement detection in `processor.rs`
4. Add to appropriate pass in `process_verse()`
5. Add tests in `processor::tests`

### Adding New Utilities

1. Add function to `utils.rs`
2. Add unit tests in `utils::tests`
3. Update documentation
4. Export from `processor.rs` if needed

### Adding Recitation Styles

1. Add variant to `RecitationStyle` enum in `types.rs`
2. Update conditional logic in `processor.rs`
3. Add rule variants as needed
4. Test with both styles

## Documentation

- **Module Level**: Each module has comprehensive doc comments
- **Function Level**: All public functions documented
- **Examples**: Functional examples in lib.rs documentation
- **README**: User-facing documentation in README.md

## Testing Strategy

**Unit Tests**:
- Located in respective modules
- Test individual functions
- Cover happy paths and edge cases

**Integration Tests**:
- Can be added to `tests/` directory
- Test complete workflows
- Currently tested via CLI

**CLI Testing**:
- Manual testing via interactive interface
- Sample verses in test_verses.txt

## Performance Characteristics

- **Time Complexity**: O(n) where n = verse length
- **Space Complexity**: O(m) where m = number of detected rules
- **Single Pass**: Multiple detection passes through verse
- **Efficient Lookups**: HashMap-based character lookups

## Compatibility

- **Rust Edition**: 2021
- **MSRV**: 1.56+ (estimated)
- **Dependencies**: None (uses only std library)
- **Platforms**: Cross-platform (works on Windows, macOS, Linux)
