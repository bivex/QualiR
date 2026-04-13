# QualiRS

**Structural and architectural code smell detector for Rust.**

QualiRS parses your Rust source code via AST analysis and detects 14 types of code smells across 4 categories: Architecture, Design, Implementation, and Unsafe. It is designed to complement `clippy` — where clippy focuses on lint-level correctness and idioms, QualiRS focuses on structural, architectural, and design-level problems.

## Features

- 42 built-in smell detectors across 5 categories: Architecture, Design, Implementation, Concurrency, and Unsafe.
- Parallel analysis via rayon (all CPU cores)
- Configurable thresholds via `qualirs.toml`
- Colored terminal table output with severity levels
- CI-friendly: exits with code 1 on critical smells
- Respects `.gitignore` automatically

## Quick Start

```bash
# Build
cargo build --release

# Analyze current project
cargo run --release -- .

# Analyze a specific path
qualirs ~/projects/my-crate

# List all detectors
qualirs --list-detectors

# Only show warnings and critical
qualirs --min-severity warning .

# Quiet mode (summary only, great for CI)
qualirs --quiet .
```

## CLI Reference

```
qualirs [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to the Rust project or file to analyze [default: .]

Options:
  -c, --config <FILE>          Configuration file path (default: qualirs.toml)
  -m, --min-severity <LEVEL>   Minimum severity to report [default: info]
                               Values: info, warning, critical
  -t, --category <CATEGORY>    Filter by smell category
                               Values: architecture, design, implementation, concurrency, unsafe
  -q, --quiet                  Summary only: file count, smell counts by severity
      --list-detectors         List available detectors and exit
  -h, --help                   Print help
  -V, --version                Print version
```

## Detectors

### Architecture (6)

| Detector | What it detects | Default threshold | Severity |
|---|---|---|---|
| **God Module** | Files with too many lines or too many top-level items | >1000 LOC or >20 items | Warning |
| **Public API Explosion** | Files where >70% of items are `pub` | >70% pub ratio, min 5 items | Info |

### Design (10)

| Detector | What it detects | Default threshold | Severity |
|---|---|---|---|
| **Large Trait** | Traits with too many methods | >15 methods | Warning |
| **Excessive Generics** | Functions/structs/enums with too many generic parameters | >5 type params | Warning |
| **Anemic Struct** | Structs with fields but no `impl` block in the same file | Any | Info |

### Implementation (15)

| Detector | What it detects | Default threshold | Severity |
|---|---|---|---|
| **Long Function** | Functions exceeding a line count | >50 LOC (Critical if >100) | Warning / Critical |
| **Too Many Arguments** | Functions with too many parameters | >6 arguments | Warning |
| **Excessive Unwrap** | Functions with too many `.unwrap()` / `.expect()` calls | >3 calls | Warning |
| **Deep Match Nesting** | Deeply nested `match` expressions | >3 levels deep | Warning |
| **Excessive Clone** | Functions with too many `.clone()` calls | >10 calls | Info |
| **Magic Numbers** | Numeric literals that aren't well-known constants | Any non-whitelisted literal | Info |
| **Large Enum** | Enums with too many variants | >20 variants | Warning |
| **High Cyclomatic Complexity** | Functions with too many branching paths (if/match/loop/&&/\|\|/?/while/for) | >15 | Warning / Critical |
| **Deep If/Else Nesting** | Deeply nested if/else chains | >4 levels deep | Warning |
| **Long Method Chain** | Excessive chained method calls `a.b().c().d().e()` | >=4 chained calls | Info |
| **Unused Result Ignored** | `let _ = expr()` discarding Result/Option values | Any | Warning |
| **Panic in Library** | `panic!`, `todo!`, `unimplemented!` in non-test library code | Any | Warning |
| **Unsafe Block Overuse** | Files with too many unsafe blocks | >5 per file | Warning |
| **Lifetime Explosion** | Functions/structs/enums with too many lifetime parameters | >4 lifetimes | Warning |
| **Copy + Drop Conflict** | Types implementing both Copy and Drop (double-free risk) | Any | Critical |

### Concurrency (6)

| Detector | What it detects | Default threshold | Severity |
|---|---|---|---|
| **Blocking in Async** | Blocking calls (sleep, io) in async fns | Any | Warning |
| **Deadlock Risk** | Nested locking patterns | Any | Critical |
| **Arc Mutex Overuse** | Excessive shared-state primitives in one type | >3 per type | Warning |

### Unsafe (5)

| Detector | What it detects | Default threshold | Severity |
|---|---|---|---|
| **Unsafe Without Comment** | `unsafe` without a `// SAFETY:` comment | Any | Warning |
| **Transmute Usage** | Use of `mem::transmute` | Any | Warning |
| **FFI Without Wrapper** | Naked FFI declarations without safe wrappers | Any | Warning |

### Magic Number Whitelist

The following numbers are **not** flagged as magic: `0`, `1`, `-1`, `2`, `10`, `100`, `1000`, `255`, `256`, `1024`.

## Configuration

Create a `qualirs.toml` in your project root. All fields are optional — missing values use defaults.

```toml
[thresholds]
# Architecture
god_module_loc = 1000
god_module_items = 20
public_api_ratio = 0.7

# Design
large_trait_methods = 15
excessive_generics = 5
deep_trait_bounds = 4

# Implementation
long_function_loc = 50
cyclomatic_complexity = 15
too_many_arguments = 6
deep_match_nesting = 3
deep_if_else = 4
excessive_unwrap = 3
large_enum_variants = 20
long_method_chain = 4
lifetime_explosion = 4
unsafe_block_overuse = 5

# Concurrency
large_future_loc = 100

# Unsafe
unsafe_without_comment = true

[config]
min_severity = "info"

exclude_paths = [
    "target",
    ".git",
    "node_modules",
    "generated",
]
```

## Severity Levels

| Level | Meaning | Exit code impact |
|---|---|---|
| **Info** | Style/convention suggestion, no action required | Exit 0 |
| **Warning** | Structural problem that should be addressed | Exit 0 |
| **Critical** | Serious smell requiring immediate attention | Exit 1 |

Use `--min-severity warning` to hide info-level smells, or `--min-severity critical` to only see the worst.

## CI Integration

```yaml
# GitHub Actions example
- name: Code smell analysis
  run: |
    cargo run --release -- --quiet --min-severity warning .
```

The tool exits with code **1** when any critical smell is detected, making it a natural CI gate.

## Example Output

```
QualiRS — Rust Code Smell Detector
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  → 32 files analyzed, 8 smell(s) detected
    0 critical  2 warning  6 info

┌──────────┬────────────────┬──────────────────────┬──────────────────────┬───────────────────────────────────────────┐
│ Severity │ Category       │ Smell                │ Location             │ Message                                   │
╞══════════╪════════════════╪══════════════════════╪══════════════════════╪═══════════════════════════════════════════╡
│ WARN     │ Implementation │ Long Function        │ src/main.rs:12       │ Function `main` is ~58 lines long         │
│ WARN     │ Implementation │ Long Function        │ src/detectors/...    │ Function `check_generics` is ~53 lines    │
│ INFO     │ Design         │ Anemic Struct        │ src/domain/smell.rs  │ Struct `SourceLocation` has no impl block  │
│ INFO     │ Architecture   │ Public API Explosion │ src/detectors/...    │ 100% of items are pub (7/7)               │
└──────────┴────────────────┴──────────────────────┴──────────────────────┴───────────────────────────────────────────┘
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Found 8 smell(s). Review warnings above.
```

## Architecture

QualiRS follows a clean layered architecture with strict dependency direction:

```
┌─────────────────────────────────────────────────────────┐
│  CLI (clap, colored output)                             │
├─────────────────────────────────────────────────────────┤
│  Analysis Engine (Detector trait, parallel orchestrator) │
├──────────────┬──────────────────────────────────────────┤
│  Detectors   │  Domain (Smell, SourceLocation, Config)  │
│  (42 impls)  │                                         │
├──────────────┴──────────────────────────────────────────┤
│  Infrastructure (file walker, config loader)            │
├─────────────────────────────────────────────────────────┤
│  Source (syn AST, proc_macro2 spans)                    │
└─────────────────────────────────────────────────────────┘

  Dependencies flow inward only.
  No outer layer is referenced by inner layers.
```

### Project Structure

```
src/
├── main.rs                          Entry point, wires everything together
├── domain/                          Core abstractions, zero external deps
│   ├── smell.rs                     Smell, SmellCategory, Severity, SourceLocation
│   ├── config.rs                    Config, Thresholds (TOML loading)
│   └── source.rs                    SourceFile (syn::File + metadata)
├── analysis/                        Analysis framework
│   ├── detector.rs                  Detector trait (the core abstraction)
│   ├── engine.rs                    Engine: registers & runs all detectors
│   └── visitor.rs                   AST visitor utilities
├── detectors/                       All smell detectors, organized by category
│   ├── architecture/
│   │   ├── god_module.rs
│   │   └── public_api_explosion.rs
│   ├── design/
│   │   ├── large_trait.rs
│   │   ├── excessive_generics.rs
│   │   └── anemic_struct.rs
│   ├── implementation/
│   │   ├── long_function.rs
│   │   ├── too_many_arguments.rs
│   │   ├── excessive_unwrap.rs
│   │   ├── deep_match.rs
│   │   ├── excessive_clone.rs
│   │   ├── magic_numbers.rs
│   │   └── large_enum.rs
│   └── unsafe/
│       └── unsafe_without_comment.rs
├── infrastructure/                  IO-bound adapters
│   └── walker.rs                    RustFileWalker (ignore crate)
└── cli/                             Presentation layer
    ├── args.rs                      CLI argument definitions (clap derive)
    └── output.rs                    Colored table report formatting
```

### Writing a Custom Detector

Implement the `Detector` trait:

```rust
use crate::analysis::detector::Detector;
use crate::domain::smell::{Smell, SmellCategory, Severity, SourceLocation};
use crate::domain::source::SourceFile;

pub struct MyCustomDetector;

impl Detector for MyCustomDetector {
    fn name(&self) -> &str {
        "My Custom Smell"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut smells = Vec::new();

        // Inspect file.ast (syn::File) and file.code (raw source)
        for item in &file.ast.items {
            if let syn::Item::Fn(fn_item) = item {
                // Your detection logic here
                if /* condition */ {
                    smells.push(Smell::new(
                        SmellCategory::Implementation,
                        "My Custom Smell",
                        Severity::Warning,
                        SourceLocation {
                            file: file.path.clone(),
                            line_start: fn_item.sig.fn_token.span.start().line,
                            line_end: fn_item.sig.fn_token.span.start().line,
                            column: None,
                        },
                        "Description of the problem".into(),
                        "How to fix it".into(),
                    ));
                }
            }
        }

        smells
    }
}
```

Then register it in `engine.rs`:

```rust
self.register(Box::new(MyCustomDetector));
```

## Roadmap

**42 of 42 detectors implemented.** 100% core coverage.

### Architecture — 6/6 done

| # | Detector | Status | Notes |
|---|---|---|---|
| 1 | God Module | Done | >1000 LOC or >20 top-level items |
| 2 | Public API Explosion | Done | >70% pub ratio |
| 3 | Feature Concentration | Done | >15 external crate deps per module |
| 4 | Cyclic Crate Dependency | Done | Module importing itself or high internal coupling |
| 5 | Unstable Dependency | Done | Dependency on unstable/internal layers |
| 6 | Layer Violation | Done | `domain` depending on `infra` |

### Design — 10/10 done

| # | Detector | Status | Notes |
|---|---|---|---|
| 1 | Large Trait | Done | >15 methods |
| 2 | Excessive Generics | Done | >5 type params, checks deep trait bounds |
| 3 | Anemic Struct | Done | Struct with fields but no impl block |
| 4 | Trait Impl Leakage | Done | 5+ std traits implemented with 0 domain traits |
| 5 | Feature Envy | Done | Fn calls methods on param more than on Self |
| 6 | Wide Hierarchy | Done | 10+ enum variants or struct fields |
| 7 | Broken Constructor | Done | Pub fields + no `new()` constructor |
| 8 | Rebellious Impl | Done | Methods inconsistent with type naming |
| 9 | Deref Abuse | Done | `impl Deref` for non-pointer types |
| 10 | Manual Drop | Done | Manual `Drop` implementation |

### Implementation — 15/15 done

| # | Detector | Status | Notes |
|---|---|---|---|
| 1 | Long Function | Done | >50 LOC, Critical if >100 |
| 2 | Too Many Arguments | Done | >6 parameters |
| 3 | Excessive Unwrap | Done | >3 unwrap/expect calls per fn |
| 4 | Deep Match Nesting | Done | >3 levels of nested match |
| 5 | Excessive Clone | Done | >10 .clone() calls per fn |
| 6 | Magic Numbers | Done | Literals outside whitelist |
| 7 | Large Enum | Done | >20 variants |
| 8 | Cyclomatic Complexity | Done | CC >15; counts if/match/loop/&&/\|\|/? |
| 9 | Deep If/Else | Done | >4 levels of if/else nesting |
| 10 | Long Method Chain | Done | >=4 chained method calls |
| 11 | Unused Result Ignored | Done | `let _ = ...` discarding values |
| 12 | Panic in Library | Done | panic!/todo!/unimplemented! in lib code |
| 13 | Unsafe Block Overuse | Done | >5 unsafe blocks per file |
| 14 | Lifetime Explosion | Done | >4 lifetime params on fn/struct/enum |
| 15 | Copy + Drop Conflict | Done | Types with both Copy and Drop |

### Concurrency — 6/6 done

| # | Detector | Status | Notes |
|---|---|---|---|
| 1 | Blocking in Async | Done | `sleep`, `io::Read`, `fs::*` inside `async fn` |
| 2 | Large Future | Done | async fn >100 LOC |
| 3 | Arc Mutex Overuse | Done | Excessive `Arc<Mutex<T>>` / `RwLock` primitives |
| 4 | Deadlock Risk | Done | Multiple locks acquired in the same scope |
| 5 | Spawn Without Join | Done | Result of `spawn` discarded or assigned to `_` |
| 6 | Missing Send Bound | Done | Spawn used in generic async fn without `Send` |

### Unsafe / Memory — 5/5 done

| # | Detector | Status | Notes |
|---|---|---|---|
| 1 | Unsafe Without Comment | Done | No `// SAFETY:` comment |
| 2 | Transmute Usage | Done | `std::mem::transmute` call detected |
| 3 | Raw Pointer Arithmetic | Done | Pointer `.offset()`, `.add()` etc |
| 4 | Multi Mutable Ref via Unsafe | Done | Aliased &mut from same pointer |
| 5 | FFI Without Wrapper | Done | Extern fn without safe wrapper |

### Infrastructure / Cross-Cutting — 0/5 done

| # | Feature | Status | Notes |
|---|---|---|---|
| 1 | Structural Metrics Export | Todo | Compute LOC, CC, param count, trait count, impl count, pub API size per module; export as structured data |
| 2 | JSON Output | Todo | `--format json` flag for machine-readable output |
| 3 | SARIF Output | Todo | `--format sarif` for GitHub Advanced Security integration |
| 4 | Diff Mode | Todo | `--diff main..HEAD` to only report smells in changed files |
| 5 | Configurable Layer Map | Todo | `layers.toml` mapping module paths to architectural layers for Layer Violation detector |

## QualiRS vs Clippy

| Aspect | Clippy | QualiRS |
|---|---|---|
| Focus | Correctness, idioms, style | Structure, architecture, design |
| Granularity | Expression/statement level | Function/module/crate level |
| Configurability | Lint levels (allow/warn/deny) | Numeric thresholds per smell |
| Unsafe analysis | Basic (`unsafe_removed_from_code`) | SAFETY comment enforcement |
| Structural metrics | None | LOC, CC, item count, pub ratio, nesting depth, method chains, lifetimes |
| Overlap | Minimal | Complementary |

## Tech Stack

| Component | Crate | Purpose |
|---|---|---|
| AST Parsing | `syn` 2.x | Full Rust syntax tree |
| Span Locations | `proc-macro2` | Line/column tracking |
| AST Visitor | `syn::visit` | Recursive tree traversal |
| CLI | `clap` 4.x | Argument parsing |
| Terminal Output | `comfy-table` 7.x | Formatted tables |
| Terminal Colors | `colored` 3.x | ANSI color formatting |
| Config | `serde` + `toml` | TOML deserialization |
| File Discovery | `ignore` 0.4 | .gitignore-aware walking |
| Parallelism | `rayon` 1.x | Parallel file analysis |
| Error Handling | `anyhow` + `thiserror` | CLI + domain errors |

## License

MIT
