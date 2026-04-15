# Detector Reference

This document explains every built-in QualiRS detector. Each entry gives the intent of the rule plus a small bad and good example. The examples are intentionally short; real findings usually involve larger code around the highlighted pattern.

## Architecture

Architecture smells point to module, crate, dependency, and public API structure that will make a project harder to evolve.

| Code | Item | What it catches | Bad example | Good example |
|---|---|---|---|---|
| Q0001 | God Module | One module owns too many lines or items. | `mod app { fn parse() {} fn save() {} fn render() {} /* many more */ }` | `mod parser; mod storage; mod ui;` |
| Q0002 | Public API Explosion | Too much of a crate is public compared with its internal surface. | `pub fn helper() {}` for every internal helper | `pub(crate) fn helper() {}` and expose a small `pub` facade |
| Q0003 | Feature Concentration | One feature area is concentrated in too many files or items. | `payment_*` types spread across unrelated modules | `mod payment { mod gateway; mod invoice; }` |
| Q0004 | Cyclic Crate Dependency | Crates depend on each other in a cycle. | `api -> core -> api` | `api -> core` and `core -> shared_types` |
| Q0005 | Layer Violation | A lower or inner layer imports a higher layer. | `domain` imports `crate::web::Request` | `web` converts `Request` into domain types |
| Q0006 | Unstable Dependency | Production code depends on unstable or risky dependency versions. | `serde = "1.0.0-alpha"` | `serde = "1"` or a pinned stable version |
| Q0007 | Leaky Error Abstraction | Public errors expose implementation details from lower layers. | `pub enum Error { Sqlx(sqlx::Error) }` | `pub enum Error { StoreUnavailable }` with source kept internal |
| Q0008 | Hidden Global State | Global mutable state makes behavior order-dependent. | `static CACHE: Mutex<Vec<Item>> = ...;` used everywhere | Pass a `Cache` through constructors or request context |
| Q0009 | Public API Leak | Public signatures expose private or dependency-specific types. | `pub fn find() -> sqlx::Row` | `pub fn find() -> UserRecord` |
| Q0010 | Test-only Dependency in Production | Production dependencies include libraries meant only for tests. | `[dependencies] pretty_assertions = "1"` | `[dev-dependencies] pretty_assertions = "1"` |
| Q0011 | Duplicate Dependency Versions | The dependency graph contains multiple versions of one crate. | `foo` uses `rand 0.8`, `bar` uses `rand 0.9` | Align both crates on one compatible `rand` version |
| Q0012 | Feature Flag Sprawl | Too many feature flags make combinations hard to reason about. | `features = ["s3", "gcs", "azure", "fast", "legacy", ...]` | Group related options, for example `cloud-storage = ["s3", "gcs"]` |
| Q0013 | Circular Module Dependency | Modules in one crate refer to each other in a cycle. | `parser` uses `resolver`, `resolver` uses `parser` | Move shared types to `syntax` and depend one way |

## Design

Design smells identify type and API shapes that make code harder to understand, test, or extend.

| Code | Item | What it catches | Bad example | Good example |
|---|---|---|---|---|
| Q0014 | Large Trait | A trait has too many required methods. | `trait Store { fn get(); fn put(); fn delete(); fn migrate(); ... }` | Split into `ReadableStore`, `WritableStore`, and `MigratesStore` |
| Q0015 | Excessive Generics | Types or functions carry too many generic parameters or deep bounds. | `fn run<A, B, C, D, E, F>(...) where A: X, B: Y, ...` | Introduce a config type or trait alias-like helper trait |
| Q0016 | Anemic Struct | A struct mostly exposes data with little behavior. | `pub struct Invoice { pub total: Money, pub paid: bool }` | `impl Invoice { pub fn mark_paid(&mut self) { ... } }` |
| Q0017 | Wide Hierarchy | Many types implement the same trait or pattern without clear grouping. | Dozens of `impl Handler for TypeN` in one area | Group handlers by domain or use an enum for closed sets |
| Q0018 | Trait Impl Leakage | Trait implementations expose unrelated domain responsibilities. | `impl Display for User` performs database lookup | `Display` formats existing fields only |
| Q0019 | Feature Envy | A method works mostly with another type's data. | `OrderService::tax(&Customer)` reads many `Customer` fields | Move tax logic to `Customer` or a focused tax policy |
| Q0020 | Broken Constructor | Constructor leaves required fields invalid or returns partially initialized values. | `fn new() -> Self { Self { id: 0, name: "" } }` | `fn new(id: Id, name: NonEmptyString) -> Self` |
| Q0021 | Rebellious Impl | An impl block contains methods unrelated to the type's responsibility. | `impl User { fn render_html(&self) -> String { ... } }` | Put rendering in `UserView` or presentation code |
| Q0022 | Fat Impl | One impl block contains too many methods. | `impl Client { fn connect(); fn query(); fn parse(); fn retry(); ... }` | Split protocol, retry, and parsing behavior into focused impls/types |
| Q0023 | Primitive Obsession | Domain concepts are modeled as raw primitives. | `struct User { id: String, email: String, age: u8 }` | `struct User { id: UserId, email: Email, age: Age }` |
| Q0024 | Data Clumps | The same group of parameters travels together repeatedly. | `fn bill(street: String, city: String, zip: String)` | `fn bill(address: Address)` |
| Q0025 | Multiple Impl Blocks | A type has many scattered inherent impl blocks. | `impl User { fn a() {} }` repeated across many files | Keep cohesive methods together or split the type |
| Q0026 | God Struct | One struct owns too many fields or responsibilities. | `struct AppState { db, cache, config, metrics, mailer, ... }` | Compose `AppState { infra: Infra, services: Services }` |
| Q0027 | Boolean Flag Argument | Boolean parameters hide which behavior is requested. | `render(user, true)` | `render(user, RenderMode::Detailed)` |
| Q0028 | Stringly Typed Domain | Domain states or identifiers are represented as unvalidated strings. | `order.status = "shippped".to_string()` | `order.status = OrderStatus::Shipped` |
| Q0029 | Large Error Enum | A single error enum has too many variants. | `enum Error { Parse, Db, Http, Auth, Config, ... }` | Split into `ParseError`, `StoreError`, and top-level conversions |

## Implementation

Implementation smells flag local complexity and maintainability issues inside functions, expressions, and type signatures.

| Code | Item | What it catches | Bad example | Good example |
|---|---|---|---|---|
| Q0030 | Long Function | A function exceeds the configured line threshold. | `fn process() { validate(); parse(); enrich(); save(); notify(); ... }` | Extract `validate_input`, `build_record`, and `persist_record` |
| Q0031 | Too Many Arguments | A function takes too many parameters. | `fn connect(host, port, tls, timeout, retries, proxy, log)` | `fn connect(options: ConnectOptions)` |
| Q0032 | Deep Match Nesting | `match` expressions nest too deeply. | `match a { Some(x) => match x.kind { ... } }` | Use early returns, helper functions, or flattened pattern matching |
| Q0033 | Magic Numbers | Unnamed numeric literals obscure intent. | `if payload.len() > 8192 { ... }` | `const MAX_PAYLOAD_BYTES: usize = 8192;` |
| Q0034 | Large Enum | An enum has too many variants. | `enum Event { Created, Updated, Deleted, Retried, ... }` | Split by domain or wrap related variants in nested enums |
| Q0035 | High Cyclomatic Complexity | A function has too many branches and decision paths. | One function with many `if`, `match`, and loop exits | Delegate each decision to focused helper functions |
| Q0036 | Deep If/Else Nesting | Conditional logic nests beyond the threshold. | `if ok { if auth { if paid { ... } } }` | Use guard clauses: `if !ok { return Err(...) }` |
| Q0037 | Long Method Chain | A chain of method calls becomes hard to inspect or debug. | `items.iter().filter(...).map(...).flat_map(...).collect()` | Name intermediate steps or extract a pipeline function |
| Q0038 | Unsafe Block Overuse | A file or function contains too many unsafe blocks. | Many small `unsafe { ... }` regions mixed through logic | Isolate unsafe code in one audited abstraction |
| Q0039 | Lifetime Explosion | Signatures carry many explicit lifetimes. | `fn merge<'a, 'b, 'c, 'd>(...) -> ...` | Use owned types, structs, or elided lifetimes where possible |
| Q0040 | Deeply Nested Type | Type signatures are nested enough to be hard to read. | `Option<Result<Vec<Box<dyn Fn()>>, Error>>` | Introduce `type HandlerList = Vec<Box<dyn Fn()>>;` |
| Q0041 | Duplicate Match Arms | Multiple match arms perform the same work. | `A => handle(), B => handle(), C => other()` | `A \| B => handle(), C => other()` |
| Q0042 | Long Closure | A closure contains too much logic. | `iter.map(\|item\| { validate; transform; log; persist; item })` | Move the body into `fn transform_item(item: Item) -> Item` |
| Q0043 | Deep Closure Nesting | Closures are nested inside closures repeatedly. | `a.map(\|x\| b.map(\|y\| c.map(\|z\| ...)))` | Use named functions or straightforward loops |

## Performance

Performance smells highlight allocation, copying, locking, and iteration patterns that can become expensive in hot paths.

| Code | Item | What it catches | Bad example | Good example |
|---|---|---|---|---|
| Q0044 | Excessive Clone | Repeated cloning where borrowing or moving would work. | `let a = value.clone(); let b = value.clone();` | Borrow with `&value` or move ownership once |
| Q0045 | Arc Mutex Overuse | Shared mutable state relies heavily on `Arc<Mutex<_>>`. | `Arc<Mutex<State>>` passed through every service | Use ownership, channels, `RwLock`, atomics, or narrower locks |
| Q0046 | Large Future | An async function's state machine becomes large. | `async fn run() { large buffers live across await; ... }` | Drop large locals before `await` or split the async workflow |
| Q0047 | Async Trait Overhead | Async trait methods may allocate or dispatch unnecessarily. | `#[async_trait] trait Repo { async fn get(&self); }` in hot paths | Use native async traits where available or concrete async functions |
| Q0048 | Interior Mutability Abuse | `RefCell`, `Cell`, or mutex-like wrappers appear where ownership would suffice. | `struct Counter { value: RefCell<u64> }` | `struct Counter { value: u64 }` with `&mut self` methods |
| Q0049 | Unnecessary Allocation in Loop | A loop allocates new owned values each iteration unnecessarily. | `for x in xs { let s = x.to_string(); use_it(&s); }` | Reuse a buffer or pass borrowed data |
| Q0050 | Collect Then Iterate | Code collects into a temporary collection only to iterate once. | `items.map(f).collect::<Vec<_>>().iter().for_each(g)` | `items.map(f).for_each(g)` |
| Q0051 | Repeated Regex Construction | Regexes are compiled repeatedly. | `for s in lines { Regex::new(PAT).unwrap().is_match(s); }` | `static RE: LazyLock<Regex> = ...;` |
| Q0052 | Missing Collection Preallocation | Code pushes many items without reserving capacity. | `let mut out = Vec::new(); for x in xs { out.push(f(x)); }` | `let mut out = Vec::with_capacity(xs.len());` |
| Q0053 | Repeated String Conversion in Hot Path | Strings are converted repeatedly in loops or chains. | `for id in ids { lookup(id.to_string()); }` | Accept `&str` or convert once outside the hot path |
| Q0054 | Needless Intermediate String Formatting | Formatting creates a temporary string only to pass it on. | `log(format!("id={id}").as_str())` | `write!(buf, "id={id}")` or pass format args directly |
| Q0055 | Vec Contains in Loop | Linear `Vec::contains` lookup repeats inside a loop. | `for id in ids { if allowed.contains(id) { ... } }` | `let allowed: HashSet<_> = allowed.into_iter().collect();` |
| Q0056 | Sort Before Min or Max | A full sort is used to get only min or max. | `xs.sort(); xs.first()` | `xs.iter().min()` |
| Q0057 | Full Sort for Single Element | A full sort is used to get one ranked element. | `xs.sort_by_key(score); xs[0]` | `xs.iter().min_by_key(score)` or `select_nth_unstable_by_key` |
| Q0058 | Clone Before Move Into Collection | A value is cloned immediately before insertion where a move would work. | `items.push(value.clone()); drop(value);` | `items.push(value);` |
| Q0059 | Inefficient Iterator Step | Iterator adapters use an indirect form for one step. | `iter.nth(0)` or `iter.skip(n).next()` | `iter.next()` or `iter.nth(n)` |
| Q0060 | Chars Count Length Check | Character counting is used where byte length is enough. | `if s.chars().count() == 0 { ... }` | `if s.is_empty() { ... }` |
| Q0061 | Repeated Expensive Construction in Loop | Expensive objects are rebuilt on every iteration. | `for item in items { let client = Client::new(); ... }` | Construct `client` once before the loop |
| Q0062 | Needless Dynamic Dispatch | `dyn Trait` is used where static dispatch would be simpler or faster. | `fn run(job: Box<dyn Job>)` for one concrete type | `fn run<J: Job>(job: J)` or accept the concrete type |
| Q0063 | Local Lock in Single-Threaded Scope | A lock protects data that is only used locally. | `let value = Mutex::new(0); *value.lock().unwrap() += 1;` | `let mut value = 0; value += 1;` |
| Q0064 | Clone on Copy | Calling `.clone()` on a `Copy` value is redundant. | `let n = count.clone();` | `let n = count;` |
| Q0065 | Large Value Passed By Value | Large values are passed by value when borrowing would avoid copies or moves. | `fn analyze(report: BigReport)` | `fn analyze(report: &BigReport)` |
| Q0066 | Inline Candidate | Tiny wrappers or single-use functions add call overhead and indirection. | `fn is_empty(s: &str) -> bool { s.is_empty() }` used once | Inline the expression or mark a widely used tiny function appropriately |

## Idiomaticity

Idiomaticity smells find code that works but fights common Rust patterns or makes intent less clear.

| Code | Item | What it catches | Bad example | Good example |
|---|---|---|---|---|
| Q0067 | Excessive Unwrap | Frequent `unwrap` or `expect` calls in production paths. | `read_config().unwrap().parse().unwrap()` | `let cfg = read_config()?; let parsed = cfg.parse()?;` |
| Q0068 | Unused Result Ignored | A `Result` is ignored instead of handled. | `file.write_all(bytes);` | `file.write_all(bytes)?;` |
| Q0069 | Panic in Library | Library code calls panic-like macros for recoverable errors. | `panic!("invalid input")` in a public library function | `return Err(Error::InvalidInput)` |
| Q0070 | Copy + Drop Conflict | A type combines `Copy` semantics with custom destruction expectations. | `#[derive(Copy, Clone)] struct Handle(RawFd); impl Drop for Handle { ... }` | Remove `Copy`; use move-only ownership for resources |
| Q0071 | Deref Abuse | `Deref` is implemented for domain behavior instead of pointer-like access. | `impl Deref<Target = Config> for App` | Add explicit `app.config()` accessors |
| Q0072 | Manual Drop | Code calls `drop` manually where scope control would be clearer. | `drop(lock); do_work();` hidden in long function | Use a smaller block: `{ let lock = mutex.lock()?; } do_work();` |
| Q0073 | Manual Default Constructor | A no-argument `new` duplicates `Default`. | `impl Settings { fn new() -> Self { Self { retries: 3 } } }` | `impl Default for Settings { fn default() -> Self { ... } }` |
| Q0074 | Manual Option/Result Mapping | Manual `match` repeats combinator behavior. | `match opt { Some(x) => Some(f(x)), None => None }` | `opt.map(f)` |
| Q0075 | Manual Find/Any Loop | Loops manually implement iterator search predicates. | `for x in xs { if pred(x) { return Some(x); } }` | `xs.into_iter().find(pred)` |
| Q0076 | Needless Explicit Lifetime | Explicit lifetimes are written where elision is clear. | `fn name<'a>(u: &'a User) -> &'a str` | `fn name(u: &User) -> &str` |
| Q0077 | Derivable Impl | Manual impl duplicates a standard derive. | `impl Default for Mode { fn default() -> Self { Mode::Fast } }` | `#[derive(Default)] enum Mode { #[default] Fast }` |

## Concurrency

Concurrency smells flag async, locking, spawning, and thread-safety patterns that can stall tasks or create race-prone behavior.

| Code | Item | What it catches | Bad example | Good example |
|---|---|---|---|---|
| Q0078 | Blocking in Async | Blocking calls are made inside async functions. | `async fn load() { std::fs::read(path).unwrap(); }` | `tokio::fs::read(path).await?` or `spawn_blocking` |
| Q0079 | Deadlock Risk | Locks are acquired in inconsistent orders. | `lock(a); lock(b);` in one path and `lock(b); lock(a);` in another | Use one lock order or combine state under one lock |
| Q0080 | Spawn Without Join | A task is spawned and its handle is not tracked. | `tokio::spawn(work());` | `let handle = tokio::spawn(work()); handle.await?;` |
| Q0081 | Missing Send Bound | Async or spawned generic work lacks a `Send` bound. | `fn spawn_task<T: Job>(job: T) { tokio::spawn(async move { job.run() }) }` | `fn spawn_task<T: Job + Send + 'static>(job: T) { ... }` |
| Q0082 | Sync Drop Blocking | `Drop` performs blocking work. | `impl Drop for Client { fn drop(&mut self) { self.flush_blocking(); } }` | Provide explicit async or fallible shutdown before drop |
| Q0083 | Std Mutex in Async | `std::sync::Mutex` is used in async code where it may block an executor. | `let guard = state.lock().unwrap(); do_async().await;` | Use `tokio::sync::Mutex` or avoid holding locks across await |
| Q0084 | Blocking Channel in Async | Blocking channel receive/send is used inside async code. | `rx.recv().unwrap()` in an async function | `tokio::sync::mpsc` with `rx.recv().await` |
| Q0085 | Holding Lock Across Await | A lock guard lives across an `.await`. | `let guard = lock.lock().await; fetch().await;` | Finish locked work, drop guard, then await |
| Q0086 | Dropped JoinHandle | A join handle is explicitly discarded. | `let _ = tokio::spawn(work());` | Store the handle, await it, or document detached task ownership |

## Unsafe

Unsafe smells focus on auditability and FFI boundaries. They do not mean unsafe code is always wrong; they mark code that deserves a narrow, documented safety argument.

| Code | Item | What it catches | Bad example | Good example |
|---|---|---|---|---|
| Q0087 | Unsafe Without Comment | Unsafe blocks or impls lack a nearby safety explanation. | `unsafe { ptr.read() }` | `// SAFETY: ptr is non-null and aligned. unsafe { ptr.read() }` |
| Q0088 | Transmute Usage | `std::mem::transmute` bypasses normal type checks. | `let y: U = unsafe { transmute(x) };` | Use `from_ne_bytes`, pointer casts with checks, or explicit conversions |
| Q0089 | Raw Pointer Arithmetic | Raw pointer offset math is used directly. | `unsafe { ptr.add(i).read() }` without bounds proof | Use slices or document bounds before pointer arithmetic |
| Q0090 | Multi Mut Ref Unsafe | Unsafe code creates multiple mutable references to the same data. | `let a = &mut *p; let b = &mut *p;` | Split with `slice.split_at_mut()` or prove disjoint pointers |
| Q0091 | FFI Without Wrapper | Raw extern functions are called directly from broad application code. | `unsafe { c_library_call(arg) }` in handlers | Wrap FFI in a small safe Rust API that validates inputs |
| Q0092 | Inline Assembly | Inline assembly appears and needs focused review. | `unsafe { asm!("nop") }` | Prefer compiler intrinsics or isolate assembly behind a documented function |
| Q0093 | Unsafe Fn Missing Safety Docs | An `unsafe fn` lacks a `# Safety` contract. | `pub unsafe fn from_raw(p: *mut T) -> Self` with no docs | Document caller obligations under `# Safety` |
| Q0094 | Unsafe Impl Missing Safety Docs | An unsafe trait impl lacks justification. | `unsafe impl Send for Buffer {}` with no comment | Add why sharing or sending is sound for all fields |
| Q0095 | Large Unsafe Block | An unsafe block contains too much code to audit easily. | `unsafe { allocate(); cast(); loop { ... } release(); }` | Keep unsafe blocks to the exact operations requiring unsafe |
| Q0096 | FFI Type Not repr(C) | Types used across FFI lack a stable C layout. | `struct Header { len: u32, tag: u16 }` in extern API | `#[repr(C)] struct Header { len: u32, tag: u16 }` |
