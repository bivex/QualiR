# QualiRS Implementation TODO

Progress: **22 / 41 detectors** + **0 / 5 infrastructure features**

---

## Architecture Smells (2/6)

- [x] God Module — `src/detectors/architecture/god_module.rs`
- [x] Public API Explosion — `src/detectors/architecture/public_api_explosion.rs`
- [ ] Feature Concentration — parse `Cargo.toml` for crate deps + count `use` statements per module; threshold >15
- [ ] Cyclic Crate Dependency — parse workspace `Cargo.toml`, build dep graph, detect cycles
- [ ] Unstable Dependency — needs layer stability model (stable/unstable/transitional); ratio >0.4
- [ ] Layer Violation — needs configurable layer map (`layers.toml`), check module paths for inward deps

## Design Smells (3/10)

- [x] Large Trait — `src/detectors/design/large_trait.rs`
- [x] Excessive Generics — `src/detectors/design/excessive_generics.rs` (also covers Deep Trait Bounds)
- [x] Anemic Struct — `src/detectors/design/anemic_struct.rs`
- [ ] Trait Impl Leakage — detect trait methods that expose internal types in signatures
- [ ] Feature Envy — count field accesses on `self` vs other types in impl blocks; flag if foreign > own
- [ ] Wide Hierarchy — count `impl Trait for` across crate; flag if >10
- [ ] Broken Constructor Pattern — struct with all `pub` fields and no `fn new()` or builder
- [ ] Rebellious Impl — impl that delegates all methods without adding state
- [ ] Deref Abuse — `impl Deref<Target = X>` used for fake inheritance, not smart pointer
- [ ] Manual Drop — custom `Drop` impl without clear necessity (e.g., no raw pointers or FFI)

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

## Concurrency Smells (0/6)

- [ ] Blocking in Async — detect `std::thread::sleep`, `std::fs::*`, `std::io::Read::read` in `async fn`
- [ ] Large Future — `async fn` body >100 LOC
- [ ] Arc Mutex Overuse — count `Arc<Mutex<T>>`, `Arc<RwLock<T>>` fields per struct; >3
- [ ] Deadlock Risk — detect nested `.lock()` calls on different Mutex/RwLock in same scope
- [ ] Spawn Without Join — `tokio::spawn` / `std::thread::spawn` without storing or awaiting JoinHandle
- [ ] Missing Send Bound — async fn or future passed to `spawn` without `+ Send` in signature

## Unsafe / Memory Smells (1/5)

- [x] Unsafe Without Comment — `src/detectors/unsafe/unsafe_without_comment.rs`
- [ ] Transmute Usage — detect `std::mem::transmute` calls
- [ ] Raw Pointer Arithmetic — detect `*mut T` / `*const T` with `.add()`, `.offset()`, `.sub()`, `ptr::copy`
- [ ] Multi Mutable Ref via Unsafe — multiple `&mut` reborrowed from same raw pointer
- [ ] FFI Without Wrapper — `extern "C"` / `extern "system"` declarations without a safe Rust wrapper fn

## Infrastructure Features (0/5)

- [ ] Structural Metrics Export — compute and export: LOC per fn/mod, CC, param count, trait count, impl count per trait, pub API size, generic param count, enum variant count, crate fan-out/fan-in
- [ ] JSON Output — `--format json` flag
- [ ] SARIF Output — `--format sarif` for GitHub Advanced Security
- [ ] Diff Mode — `--diff <base>..<head>` to only analyze changed files (integrate with `git diff`)
- [ ] Layer Map Config — `layers.toml` defining module path → layer mappings for architecture enforcement

---

## Priority Order

### P0 — High impact, straightforward to implement
1. Broken Constructor Pattern (scan struct fields for all-pub)
2. JSON Output (serialize `AnalysisReport`)

### P1 — High impact, moderate effort
3. Blocking in Async (detect sync calls in async fn)
4. Large Future (like Long Function but for async fn)
5. Transmute Usage (simple method call detection)
6. Feature Concentration (parse Cargo.toml + use stmts)
7. Diff Mode (git integration)

### P2 — Moderate impact, significant effort
8. Feature Envy (needs cross-struct field access tracking)
9. Wide Hierarchy (cross-file impl counting)
10. Deref Abuse (semantic analysis of Deref impl intent)
11. Deadlock Risk (control flow analysis for nested locks)
12. Arc Mutex Overuse (type resolution for Arc<Mutex<T>>)
13. Spawn Without Join (cross-function data flow)
14. Missing Send Bound (type/trait bound analysis)

### P3 — Requires external tooling or deep analysis
15. Cyclic Crate Dependency (workspace Cargo.toml graph)
16. Layer Violation (needs layer config)
17. Unstable Dependency (needs stability model)
18. Trait Impl Leakage (semantic trait API analysis)
19. Rebellious Impl (impl pattern recognition)
20. Manual Drop (intent analysis)
21. Multi Mutable Ref via Unsafe (data flow in unsafe)
22. Raw Pointer Arithmetic (pointer operation tracking)
23. FFI Without Wrapper (cross-reference extern + safe fn)
24. SARIF Output
25. Layer Map Config
26. Structural Metrics Export
