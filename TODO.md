# QualiRS Implementation TODO

Progress: **42 / 42 detectors** + **0 / 5 infrastructure features**

---

## Architecture Smells (6/6)

- [x] God Module — `src/detectors/architecture/god_module.rs`
- [x] Public API Explosion — `src/detectors/architecture/public_api_explosion.rs`
- [x] Feature Concentration — `src/detectors/architecture/feature_concentration.rs`
- [x] Cyclic Crate Dependency — `src/detectors/architecture/cyclic_crate_dependency.rs`
- [x] Unstable Dependency — `src/detectors/architecture/unstable_dependency.rs`
- [x] Layer Violation — `src/detectors/architecture/layer_violation.rs`

## Design Smells (10/10)

- [x] Large Trait — `src/detectors/design/large_trait.rs`
- [x] Excessive Generics — `src/detectors/design/excessive_generics.rs` (also covers Deep Trait Bounds)
- [x] Anemic Struct — `src/detectors/design/anemic_struct.rs`
- [x] Trait Impl Leakage — `src/detectors/design/trait_impl_leakage.rs`
- [x] Feature Envy — `src/detectors/design/feature_envy.rs`
- [x] Wide Hierarchy — `src/detectors/design/wide_hierarchy.rs`
- [x] Broken Constructor Pattern — `src/detectors/design/broken_constructor.rs`
- [x] Rebellious Impl — `src/detectors/design/rebellious_impl.rs`
- [x] Deref Abuse — `src/detectors/design/deref_abuse.rs`
- [x] Manual Drop — `src/detectors/design/manual_drop.rs`

## Implementation Smells (15/15)

- [x] Long Function — `src/detectors/implementation/long_function.rs`
- [x] Too Many Arguments — `src/detectors/implementation/too_many_arguments.rs`
- [x] Excessive Unwrap — `src/detectors/implementation/excessive_unwrap.rs`
- [x] Deep Match Nesting — `src/detectors/implementation/deep_match.rs`
- [x] Excessive Clone — `src/detectors/implementation/excessive_clone.rs`
- [x] Magic Numbers — `src/detectors/implementation/magic_numbers.rs`
- [x] Large Enum — `src/detectors/implementation/large_enum.rs`
- [x] Cyclomatic Complexity — `src/detectors/implementation/cyclomatic_complexity.rs`
- [x] Deep If/Else — `src/detectors/implementation/deep_if_else.rs`
- [x] Long Method Chain — `src/detectors/implementation/long_method_chain.rs`
- [x] Unused Result Ignored — `src/detectors/implementation/unused_result.rs`
- [x] Panic in Library — `src/detectors/implementation/panic_in_library.rs`
- [x] Unsafe Block Overuse — `src/detectors/implementation/unsafe_overuse.rs`
- [x] Lifetime Explosion — `src/detectors/implementation/lifetime_explosion.rs`
- [x] Copy + Drop Conflict — `src/detectors/implementation/copy_drop_conflict.rs`

## Concurrency Smells (6/6)

- [x] Blocking in Async — `src/detectors/concurrency/blocking_in_async.rs`
- [x] Large Future — `src/detectors/concurrency/large_future.rs`
- [x] Arc Mutex Overuse — `src/detectors/concurrency/arc_mutex_overuse.rs`
- [x] Deadlock Risk — `src/detectors/concurrency/deadlock_risk.rs`
- [x] Spawn Without Join — `src/detectors/concurrency/spawn_without_join.rs`
- [x] Missing Send Bound — `src/detectors/concurrency/missing_send_bound.rs`

## Unsafe / Memory Smells (5/5)

- [x] Unsafe Without Comment — `src/detectors/unsafe/unsafe_without_comment.rs`
- [x] Transmute Usage — `src/detectors/unsafe/transmute_usage.rs`
- [x] Raw Pointer Arithmetic — `src/detectors/unsafe/raw_pointer_arithmetic.rs`
- [x] Multi Mutable Ref via Unsafe — `src/detectors/unsafe/multi_mut_ref_unsafe.rs`
- [x] FFI Without Wrapper — `src/detectors/unsafe/ffi_without_wrapper.rs`

---

## Infrastructure Features (0/5)

- [ ] Structural Metrics Export — compute and export: LOC per fn/mod, CC, param count, trait count, impl count per trait, pub API size, generic param count, enum variant count, crate fan-out/fan-in
- [ ] JSON Output — `--format json` flag
- [ ] SARIF Output — `--format sarif` for GitHub Advanced Security
- [ ] Diff Mode — `--diff <base>..<head>` to only analyze changed files (integrate with `git diff`)
- [ ] Layer Map Config — `layers.toml` defining module path → layer mappings for architecture enforcement

---

## Future Improvements

- Cross-file analysis (current detectors work per-file only)
- Incremental analysis / caching
- IDE integration (LSP diagnostics)
- CI GitHub Action
- Configurable per-detector thresholds in `qualirs.toml`
- [x] Test coverage for new detectors (71 integration tests for all 42 detectors)
    - [x] Update README.md to reflect 42 total detectors across 5 categories
