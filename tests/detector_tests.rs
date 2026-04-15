mod common;

use common::{assert_clean, assert_smell_count};
use qualirs::analysis::detector::Detector;
use qualirs::domain::source::SourceFile;
use std::path::PathBuf;

// ─── Architecture ──────────────────────────────────────────

mod god_module {
    use super::*;
    use qualirs::detectors::architecture::god_module::GodModuleDetector;

    static DETECTOR: GodModuleDetector = GodModuleDetector;

    #[test]
    fn detects_many_items() {
        // Generate 25 functions to exceed the 20-item threshold
        let fns: String = (0..25)
            .map(|i| format!("fn func_{i}() {{}}"))
            .collect::<Vec<_>>()
            .join("\n");
        assert_smell_count(&DETECTOR, &fns, "God Module (items)", 1);
    }

    #[test]
    fn clean_few_items() {
        let code = "fn main() {}";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_module_registry_file() {
        let mods: String = (0..25)
            .map(|i| format!("pub mod detector_{i};"))
            .collect::<Vec<_>>()
            .join("\n");
        assert_clean(&DETECTOR, &mods);
    }
}

mod public_api_explosion {
    use super::*;
    use qualirs::detectors::architecture::public_api_explosion::PublicApiExplosionDetector;

    static DETECTOR: PublicApiExplosionDetector = PublicApiExplosionDetector;

    #[test]
    fn detects_all_pub() {
        let code = "\
pub fn a() {}
pub fn b() {}
pub fn c() {}
pub fn d() {}
pub fn e() {}
pub fn f() {}
";
        assert_smell_count(&DETECTOR, code, "Public API Explosion", 1);
    }

    #[test]
    fn clean_mostly_private() {
        let code = "\
fn a() {}
fn b() {}
fn c() {}
fn d() {}
pub fn e() {}
";
        assert_clean(&DETECTOR, code);
    }
}

// ─── Design ────────────────────────────────────────────────

mod large_trait {
    use super::*;
    use qualirs::detectors::design::large_trait::LargeTraitDetector;

    static DETECTOR: LargeTraitDetector = LargeTraitDetector;

    #[test]
    fn detects_many_methods() {
        let methods: String = (0..16)
            .map(|i| format!("fn method_{i}(&self);"))
            .collect::<Vec<_>>()
            .join("\n    ");
        let code = format!("trait Huge {{ {methods} }}");
        assert_smell_count(&DETECTOR, &code, "Large Trait", 1);
    }

    #[test]
    fn clean_small_trait() {
        let code = "trait Small { fn do_it(&self); }";
        assert_clean(&DETECTOR, code);
    }
}

mod excessive_generics {
    use super::*;
    use qualirs::detectors::design::excessive_generics::ExcessiveGenericsDetector;

    static DETECTOR: ExcessiveGenericsDetector = ExcessiveGenericsDetector;

    #[test]
    fn detects_many_generics() {
        let code = "fn foo<A, B, C, D, E, F>(a: A, b: B, c: C, d: D, e: E, f: F) {}";
        assert_smell_count(&DETECTOR, code, "Excessive Generics", 1);
    }

    #[test]
    fn clean_few_generics() {
        let code = "fn foo<T>(a: T) where T: Clone {}";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn detects_deep_trait_bounds() {
        let code = "fn foo<T: Clone + Copy + Default + Debug + Send>(a: T) {}";
        assert_smell_count(&DETECTOR, code, "Deep Trait Bounds", 1);
    }
}

mod anemic_struct {
    use super::*;
    use qualirs::detectors::design::anemic_struct::AnemicStructDetector;

    static DETECTOR: AnemicStructDetector = AnemicStructDetector;

    #[test]
    fn detects_struct_without_impl() {
        let code = "struct Point { x: f64, y: f64 }";
        assert_smell_count(&DETECTOR, code, "Anemic Struct", 1);
    }

    #[test]
    fn clean_struct_with_impl() {
        let code = "\
struct Point { x: f64, y: f64 }
impl Point {
    fn distance(&self) -> f64 { self.x }
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_template_struct() {
        let code = r#"
#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    title: String,
    count: usize,
}
"#;
        assert_clean(&DETECTOR, code);
    }
}

// ─── Implementation ────────────────────────────────────────

mod long_function {
    use super::*;
    use qualirs::detectors::implementation::long_function::LongFunctionDetector;

    static DETECTOR: LongFunctionDetector = LongFunctionDetector;

    #[test]
    fn detects_long_fn() {
        let body: String = (0..55)
            .map(|i| format!("let _ = {i};"))
            .collect::<Vec<_>>()
            .join("\n");
        let code = format!("fn long() {{\n{body}\n}}");
        assert_smell_count(&DETECTOR, &code, "Long Function", 1);
    }

    #[test]
    fn clean_short_fn() {
        let code = "fn short() { let x = 1; }";
        assert_clean(&DETECTOR, code);
    }
}

mod too_many_arguments {
    use super::*;
    use qualirs::detectors::implementation::too_many_arguments::TooManyArgumentsDetector;

    static DETECTOR: TooManyArgumentsDetector = TooManyArgumentsDetector;

    #[test]
    fn detects_many_args() {
        let code = "fn foo(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) {}";
        assert_smell_count(&DETECTOR, code, "Too Many Arguments", 1);
    }

    #[test]
    fn clean_few_args() {
        let code = "fn foo(a: i32, b: i32) {}";
        assert_clean(&DETECTOR, code);
    }
}

mod excessive_unwrap {
    use super::*;
    use qualirs::detectors::implementation::excessive_unwrap::ExcessiveUnwrapDetector;

    static DETECTOR: ExcessiveUnwrapDetector = ExcessiveUnwrapDetector;

    #[test]
    fn detects_many_unwrap() {
        let code = "\
fn risky() {
    let a = Some(1).unwrap();
    let b = Some(2).unwrap();
    let c = Some(3).unwrap();
    let d = Some(4).unwrap();
}
";
        assert_smell_count(&DETECTOR, code, "Excessive Unwrap", 1);
    }

    #[test]
    fn clean_single_unwrap() {
        let code = "fn ok() { let x = Some(1).unwrap(); }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_test_file() {
        let code = "\
fn risky() {
    let a = Some(1).unwrap();
    let b = Some(2).unwrap();
    let c = Some(3).unwrap();
    let d = Some(4).unwrap();
}
";
        let file =
            SourceFile::from_source(PathBuf::from("src/tests.rs"), code.to_string()).unwrap();
        assert!(DETECTOR.detect(&file).is_empty());
    }
}

mod deep_match {
    use super::*;
    use qualirs::detectors::implementation::deep_match::DeepMatchDetector;

    static DETECTOR: DeepMatchDetector = DeepMatchDetector;

    #[test]
    fn detects_nested_match() {
        let code = "\
fn deep() {
    match 1 {
        1 => match 2 {
            2 => match 3 {
                3 => match 4 {
                    4 => (),
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        },
        _ => (),
    }
}
";
        assert_smell_count(&DETECTOR, code, "Deep Match Nesting", 1);
    }

    #[test]
    fn clean_flat_match() {
        let code = "\
fn flat(x: i32) {
    match x {
        1 => (),
        2 => (),
        _ => (),
    }
}
";
        assert_clean(&DETECTOR, code);
    }
}

mod excessive_clone {
    use super::*;
    use qualirs::detectors::implementation::excessive_clone::ExcessiveCloneDetector;

    static DETECTOR: ExcessiveCloneDetector = ExcessiveCloneDetector;

    #[test]
    fn detects_many_clones() {
        let clones: String = (0..12)
            .map(|i| format!("let v{i} = x.clone();"))
            .collect::<Vec<_>>()
            .join("\n    ");
        let code = format!("fn clone_happy(x: String) {{\n    {clones}\n}}");
        assert_smell_count(&DETECTOR, &code, "Excessive Clone", 1);
    }

    #[test]
    fn clean_few_clones() {
        let code = "fn ok(x: String) { let y = x.clone(); }";
        assert_clean(&DETECTOR, code);
    }
}

mod magic_numbers {
    use super::*;
    use qualirs::detectors::implementation::magic_numbers::MagicNumbersDetector;

    static DETECTOR: MagicNumbersDetector = MagicNumbersDetector;

    #[test]
    fn detects_magic_number() {
        let code = "fn calc() { let x = 42; let y = 1337; }";
        assert_smell_count(&DETECTOR, code, "Magic Numbers", 1);
    }

    #[test]
    fn clean_whitelisted_numbers() {
        let code = "fn calc() { let x = 0; let y = 1; let z = 100; }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_test_file() {
        let code = "fn calc() { let x = 42; let y = 1337; }";
        let file =
            SourceFile::from_source(PathBuf::from("tests/magic.rs"), code.to_string()).unwrap();
        assert!(DETECTOR.detect(&file).is_empty());
    }
}

mod large_enum {
    use super::*;
    use qualirs::detectors::implementation::large_enum::LargeEnumDetector;

    static DETECTOR: LargeEnumDetector = LargeEnumDetector;

    #[test]
    fn detects_many_variants() {
        let variants: String = (0..22)
            .map(|i| format!("V{i}"))
            .collect::<Vec<_>>()
            .join(", ");
        let code = format!("enum Huge {{ {variants} }}");
        assert_smell_count(&DETECTOR, &code, "Large Enum", 1);
    }

    #[test]
    fn clean_small_enum() {
        let code = "enum Color { Red, Green, Blue }";
        assert_clean(&DETECTOR, code);
    }
}

mod cyclomatic_complexity {
    use super::*;
    use qualirs::detectors::implementation::cyclomatic_complexity::CyclomaticComplexityDetector;

    static DETECTOR: CyclomaticComplexityDetector = CyclomaticComplexityDetector;

    #[test]
    fn detects_complex_fn() {
        // 16 if statements = CC 17 > 15
        let branches: String = (0..16)
            .map(|i| format!("if x > {i} {{"))
            .collect::<Vec<_>>()
            .join("\n");
        let closes = "}".repeat(16);
        let code = format!("fn complex(x: i32) {{\n{branches}\n(){closes}\n}}");
        assert_smell_count(&DETECTOR, &code, "High Cyclomatic Complexity", 1);
    }

    #[test]
    fn clean_simple_fn() {
        let code = "fn simple() { let x = 1; }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn counts_match_arms() {
        let arms: String = (0..20)
            .map(|i| format!("{i} => (),"))
            .collect::<Vec<_>>()
            .join(" ");
        let code = format!("fn matchy(x: i32) {{ match x {{ {arms} _ => () }} }}");
        assert_smell_count(&DETECTOR, &code, "High Cyclomatic Complexity", 1);
    }
}

mod deep_if_else {
    use super::*;
    use qualirs::detectors::implementation::deep_if_else::DeepIfElseDetector;

    static DETECTOR: DeepIfElseDetector = DeepIfElseDetector;

    #[test]
    fn detects_deep_nesting() {
        let code = "\
fn nested(x: i32) {
    if x > 0 {
        if x > 10 {
            if x > 20 {
                if x > 30 {
                    if x > 40 {
                        ()
                    }
                }
            }
        }
    }
}
";
        assert_smell_count(&DETECTOR, code, "Deep If/Else Nesting", 1);
    }

    #[test]
    fn clean_shallow_if() {
        let code = "fn shallow(x: i32) { if x > 0 { () } }";
        assert_clean(&DETECTOR, code);
    }
}

mod long_method_chain {
    use super::*;
    use qualirs::detectors::implementation::long_method_chain::LongMethodChainDetector;

    static DETECTOR: LongMethodChainDetector = LongMethodChainDetector;

    #[test]
    fn detects_long_chain() {
        // 5 chained calls: iter().filter().map().flatten().collect() — depth > 4
        // Must use a non-test file path because the detector skips test files
        let code = "fn chain(x: Vec<i32>) { x.iter().filter(|&&x| x > 0).map(|&x| x * 2).flatten().collect::<Vec<i32>>(); }";
        let file = SourceFile::from_source(PathBuf::from("main.rs"), code.to_string()).unwrap();
        let smells = DETECTOR.detect(&file);
        assert!(
            smells.iter().any(|s| s.name == "Long Method Chain"),
            "Should detect long chain: {:?}",
            smells
        );
    }

    #[test]
    fn clean_short_chain() {
        // Must use a non-test file path because the detector skips test files
        let code = "fn short(x: Vec<i32>) { x.iter().count(); }";
        let file = SourceFile::from_source(PathBuf::from("main.rs"), code.to_string()).unwrap();
        let smells = DETECTOR.detect(&file);
        assert!(
            smells.is_empty(),
            "Expected no smells, but found: {:?}",
            smells
        );
    }
}

mod unused_result {
    use super::*;
    use qualirs::detectors::implementation::unused_result::UnusedResultDetector;

    static DETECTOR: UnusedResultDetector = UnusedResultDetector;

    #[test]
    fn detects_let_underscore() {
        let code = "fn discard() { let _ = std::fs::read_to_string(\"x\"); }";
        assert_smell_count(&DETECTOR, code, "Unused Result Ignored", 1);
    }

    #[test]
    fn clean_used_result() {
        let code = "fn ok() { let _x = std::fs::read_to_string(\"x\"); }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_test_file() {
        let code = "fn discard() { let _ = std::fs::remove_file(\"x\"); }";
        let file =
            SourceFile::from_source(PathBuf::from("src/settings/tests.rs"), code.to_string())
                .unwrap();
        assert!(DETECTOR.detect(&file).is_empty());
    }
}

mod repeated_regex_construction {
    use super::*;
    use qualirs::detectors::implementation::repeated_regex_construction::RepeatedRegexConstructionDetector;

    static DETECTOR: RepeatedRegexConstructionDetector = RepeatedRegexConstructionDetector;

    #[test]
    fn detects_runtime_regex_construction() {
        let code = r#"
use regex::Regex;

fn validate(value: &str) -> bool {
    Regex::new(r"^[a-z]+$").unwrap().is_match(value)
}
"#;
        assert_smell_count(&DETECTOR, code, "Repeated Regex Construction", 1);
    }

    #[test]
    fn clean_lazy_lock_regex_initializer() {
        let code = r#"
use regex::Regex;
use std::sync::LazyLock;

fn validate(value: &str) -> bool {
    static VALID: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^[a-z]+$").expect("valid regex")
    });
    VALID.is_match(value)
}
"#;
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_once_lock_get_or_init_regex_initializer() {
        let code = r#"
use regex::Regex;
use std::sync::OnceLock;

fn validate(value: &str) -> bool {
    static VALID: OnceLock<Regex> = OnceLock::new();
    VALID
        .get_or_init(|| Regex::new(r"^[a-z]+$").expect("valid regex"))
        .is_match(value)
}
"#;
        assert_clean(&DETECTOR, code);
    }
}

mod panic_in_library {
    use super::*;
    use qualirs::detectors::implementation::panic_in_library::PanicInLibraryDetector;

    static DETECTOR: PanicInLibraryDetector = PanicInLibraryDetector;

    #[test]
    fn detects_panic() {
        // File name must NOT contain "test" — detector skips test files
        let code = "fn crash() { panic!(\"oops\"); }";
        let file = SourceFile::from_source(PathBuf::from("main.rs"), code.to_string()).unwrap();
        let smells = DETECTOR.detect(&file);
        assert!(
            smells.iter().any(|s| s.name == "Panic in Library"),
            "Should detect panic!: {:?}",
            smells
        );
    }

    #[test]
    fn detects_todo() {
        let code = "fn unfinished() { todo!(); }";
        let file = SourceFile::from_source(PathBuf::from("main.rs"), code.to_string()).unwrap();
        let smells = DETECTOR.detect(&file);
        assert!(
            smells.iter().any(|s| s.name == "Panic in Library"),
            "Should detect todo!: {:?}",
            smells
        );
    }

    #[test]
    fn clean_result_return() {
        let code = "fn ok() -> Result<(), std::io::Error> { Ok(()) }";
        assert_clean(&DETECTOR, code);
    }
}

mod unsafe_overuse {
    use super::*;
    use qualirs::detectors::implementation::unsafe_overuse::UnsafeOveruseDetector;

    static DETECTOR: UnsafeOveruseDetector = UnsafeOveruseDetector;

    #[test]
    fn detects_many_unsafe_blocks() {
        let blocks: String = (0..6)
            .map(|_| String::from("let _ = unsafe { 1 };"))
            .collect::<Vec<_>>()
            .join("\n");
        let code = format!("fn risky() {{\n{blocks}\n}}");
        assert_smell_count(&DETECTOR, &code, "Unsafe Block Overuse", 1);
    }

    #[test]
    fn clean_single_unsafe() {
        let code = "fn ok() { let x = unsafe { 1 }; }";
        assert_clean(&DETECTOR, code);
    }
}

mod lifetime_explosion {
    use super::*;
    use qualirs::detectors::implementation::lifetime_explosion::LifetimeExplosionDetector;

    static DETECTOR: LifetimeExplosionDetector = LifetimeExplosionDetector;

    #[test]
    fn detects_many_lifetimes() {
        let code = "struct S<'a, 'b, 'c, 'd, 'e> { a: &'a i32, b: &'b i32, c: &'c i32, d: &'d i32, e: &'e i32 }";
        assert_smell_count(&DETECTOR, code, "Lifetime Explosion", 1);
    }

    #[test]
    fn clean_few_lifetimes() {
        let code = "struct S<'a> { x: &'a i32 }";
        assert_clean(&DETECTOR, code);
    }
}

mod copy_drop_conflict {
    use super::*;
    use qualirs::detectors::implementation::copy_drop_conflict::CopyDropConflictDetector;

    static DETECTOR: CopyDropConflictDetector = CopyDropConflictDetector;

    #[test]
    fn detects_copy_and_drop() {
        let code = "\
#[derive(Copy, Clone)]
struct Bad { x: i32 }
impl Drop for Bad {
    fn drop(&mut self) {}
}
";
        assert_smell_count(&DETECTOR, code, "Copy + Drop Conflict", 1);
    }

    #[test]
    fn clean_copy_only() {
        let code = "#[derive(Copy, Clone)] struct Good { x: i32 }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_drop_only() {
        let code = "\
struct Good { x: Box<i32> }
impl Drop for Good {
    fn drop(&mut self) {}
}
";
        assert_clean(&DETECTOR, code);
    }
}

// ─── Unsafe ────────────────────────────────────────────────

mod unsafe_without_comment {
    use super::*;
    use qualirs::detectors::r#unsafe::unsafe_without_comment::UnsafeWithoutCommentDetector;

    static DETECTOR: UnsafeWithoutCommentDetector = UnsafeWithoutCommentDetector;

    #[test]
    fn detects_unsafe_without_comment() {
        let code = "fn risky() { unsafe { let x = 1; } }";
        assert_smell_count(&DETECTOR, code, "Unsafe Without Comment", 1);
    }

    #[test]
    fn clean_unsafe_with_safety_comment() {
        let code = "\
fn ok() {
    // SAFETY: this is safe because reasons
    unsafe { let x = 1; }
}
";
        assert_clean(&DETECTOR, code);
    }
}

// ─── Concurrency ───────────────────────────────────────────

mod blocking_in_async {
    use super::*;
    use qualirs::detectors::concurrency::blocking_in_async::BlockingInAsyncDetector;

    static DETECTOR: BlockingInAsyncDetector = BlockingInAsyncDetector;

    #[test]
    fn detects_std_sleep_in_async() {
        let code = "async fn foo() { std::thread::sleep(std::time::Duration::from_secs(1)); }";
        assert_smell_count(&DETECTOR, code, "Blocking in Async", 1);
    }

    #[test]
    fn detects_fs_read_in_async() {
        let code = "async fn foo() { let _ = std::fs::read_to_string(\"foo.txt\"); }";
        assert_smell_count(&DETECTOR, code, "Blocking in Async", 1);
    }

    #[test]
    fn clean_no_blocking_in_async() {
        let code =
            "async fn foo() { tokio::time::sleep(std::time::Duration::from_secs(1)).await; }";
        assert_clean(&DETECTOR, code);
    }
}

mod deadlock_risk {
    use super::*;
    use qualirs::detectors::concurrency::deadlock_risk::DeadlockRiskDetector;

    static DETECTOR: DeadlockRiskDetector = DeadlockRiskDetector;

    #[test]
    fn detects_multiple_locks() {
        let code = "\
fn foo(m1: &Mutex<i32>, m2: &Mutex<i32>) {
    let _g1 = m1.lock().unwrap();
    let _g2 = m2.lock().unwrap();
}
";
        assert_smell_count(&DETECTOR, code, "Deadlock Risk", 1);
    }

    #[test]
    fn clean_single_lock() {
        let code = "\
fn foo(m1: &Mutex<i32>) {
    let _g1 = m1.lock().unwrap();
}
";
        assert_clean(&DETECTOR, code);
    }
}

mod arc_mutex_overuse {
    use super::*;
    use qualirs::detectors::concurrency::arc_mutex_overuse::ArcMutexOveruseDetector;

    static DETECTOR: ArcMutexOveruseDetector = ArcMutexOveruseDetector;

    #[test]
    fn detects_many_arc_mutex() {
        // Threshold is 3 by default.
        // Our heuristic counts Arc, Mutex, and RwLock segments.
        // Arc<Mutex<i32>> counts twice (once as Arc, once as Mutex via recursive visit).
        let code = "\
struct State {
    a: Arc<Mutex<i32>>,
    b: Arc<Mutex<i32>>,
}
";
        assert_smell_count(&DETECTOR, code, "Arc Mutex Overuse", 1);
    }

    #[test]
    fn clean_few_usages() {
        let code = "\
struct State {
    data: Arc<Mutex<i32>>,
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_no_arc_mutex() {
        let code = "struct State { value: i32 }";
        assert_clean(&DETECTOR, code);
    }
}

mod large_future {
    use super::*;
    use qualirs::detectors::concurrency::large_future::LargeFutureDetector;

    static DETECTOR: LargeFutureDetector = LargeFutureDetector;

    #[test]
    fn detects_long_async_fn() {
        // Threshold is 100 lines by default
        let body: String = (0..120)
            .map(|i| format!("let _ = {i};"))
            .collect::<Vec<_>>()
            .join("\n");
        let code = format!("async fn big() {{\n{body}\n}}");
        assert_smell_count(&DETECTOR, &code, "Large Future", 1);
    }

    #[test]
    fn clean_short_async() {
        let code = "async fn short() { let x = 1; }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_long_sync_function() {
        // Long non-async functions are not flagged by LargeFutureDetector
        let body: String = (0..120)
            .map(|i| format!("let _ = {i};"))
            .collect::<Vec<_>>()
            .join("\n");
        let code = format!("fn big_sync() {{\n{body}\n}}");
        assert_clean(&DETECTOR, &code);
    }
}

// ─── Missing Design ──────────────────────────────────────────

mod manual_drop {
    use super::*;
    use qualirs::detectors::design::manual_drop::ManualDropDetector;

    static DETECTOR: ManualDropDetector = ManualDropDetector;

    #[test]
    fn detects_manual_drop() {
        let code = "\
struct Resource;
impl Drop for Resource {
    fn drop(&mut self) {}
}
";
        assert_smell_count(&DETECTOR, code, "Manual Drop", 1);
    }

    #[test]
    fn clean_no_drop_impl() {
        let code = "struct Resource;";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_trait_impl_not_drop() {
        let code = "\
struct Resource;
impl std::fmt::Debug for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { Ok(()) }
}
";
        assert_clean(&DETECTOR, code);
    }
}

mod deref_abuse {
    use super::*;
    use qualirs::detectors::design::deref_abuse::DerefAbuseDetector;

    static DETECTOR: DerefAbuseDetector = DerefAbuseDetector;

    #[test]
    fn detects_deref_on_non_pointer() {
        let code = "\
struct MyStruct { inner: String }
impl std::ops::Deref for MyStruct {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.inner }
}
";
        assert_smell_count(&DETECTOR, code, "Deref Abuse", 1);
    }

    #[test]
    fn clean_deref_on_pointer_named_type() {
        let code = "\
struct MyPtr<T> { inner: Box<T> }
impl<T> std::ops::Deref for MyPtr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.inner }
}
";
        assert_clean(&DETECTOR, code);
    }
}

// ─── Missing Unsafe ──────────────────────────────────────────

mod transmute_usage {
    use super::*;
    use qualirs::detectors::r#unsafe::transmute_usage::TransmuteUsageDetector;

    static DETECTOR: TransmuteUsageDetector = TransmuteUsageDetector;

    #[test]
    fn detects_transmute() {
        let code = "fn risky() { let x: i32 = unsafe { std::mem::transmute(1.0f32) }; }";
        assert_smell_count(&DETECTOR, code, "Transmute Usage", 1);
    }

    #[test]
    fn clean_no_transmute() {
        let code = "fn safe() { let x = 1 as f32; }";
        assert_clean(&DETECTOR, code);
    }
}

// ─── Extended Architecture ──────────────────────────────

mod feature_concentration {
    use super::*;
    use qualirs::detectors::architecture::feature_concentration::FeatureConcentrationDetector;
    static DETECTOR: FeatureConcentrationDetector = FeatureConcentrationDetector;

    #[test]
    fn detects_high_concentration() {
        let mut code = String::new();
        for i in 0..16 {
            code.push_str(&format!("use crate{}::item;\n", i));
        }
        assert_smell_count(&DETECTOR, &code, "Feature Concentration", 1);
    }

    #[test]
    fn clean_few_external_crates() {
        let code = "\
use serde::Deserialize;
use tokio::runtime::Runtime;
use clap::Parser;
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_only_local_imports() {
        let code = "\
use crate::domain::User;
use super::config;
use self::inner::Helper;
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_repeated_crate_counts_once() {
        // Same crate imported via different paths counts only once
        let code = "\
use serde::Deserialize;
use serde::Serialize;
use serde::de::Deserializer;
";
        assert_clean(&DETECTOR, code);
    }
}

mod layer_violation {
    use super::*;
    use qualirs::detectors::architecture::layer_violation::LayerViolationDetector;
    static DETECTOR: LayerViolationDetector = LayerViolationDetector;

    #[test]
    fn detects_domain_to_infra_violation() {
        let code = "use crate::infrastructure::db::UserRepo;";
        let source =
            SourceFile::from_source("src/domain/user.rs".into(), code.to_string()).unwrap();
        let smells = DETECTOR.detect(&source);
        assert!(!smells.is_empty(), "Expected layer violation smell");
        assert_eq!(smells[0].name, "Layer Violation");
    }

    #[test]
    fn clean_layering() {
        let code = "use crate::domain::model::User;";
        let source =
            SourceFile::from_source("src/infrastructure/db.rs".into(), code.to_string()).unwrap();
        let smells = DETECTOR.detect(&source);
        assert!(smells.is_empty());
    }

    #[test]
    fn clean_io_substring_domain() {
        // 'action' contains 'io', but it's not the 'io' module
        let code = "use crate::domain::action::UserAction;";
        let source =
            SourceFile::from_source("src/domain/user.rs".into(), code.to_string()).unwrap();
        let smells = DETECTOR.detect(&source);
        assert!(
            smells.is_empty(),
            "Should not flag 'action' as 'io' violation. Smells found: {:?}",
            smells
        );
    }

    #[test]
    fn clean_option_domain() {
        // 'option' contains 'io', but it's std::option
        let code = "use std::option::Option;";
        let source =
            SourceFile::from_source("src/domain/user.rs".into(), code.to_string()).unwrap();
        let smells = DETECTOR.detect(&source);
        assert!(
            smells.is_empty(),
            "Should not flag 'option' as 'io' violation. Smells found: {:?}",
            smells
        );
    }

    #[test]
    fn clean_client_domain() {
        // 'client' contains 'cli', but it's not the 'cli' module
        // 'HttpClient' contains 'http', but it's not the 'http' module
        let code = "use crate::app::client::HttpClient;";
        let source =
            SourceFile::from_source("src/domain/user.rs".into(), code.to_string()).unwrap();
        let smells = DETECTOR.detect(&source);
        assert!(
            smells.is_empty(),
            "Should not flag 'client' as 'cli' or 'HttpClient' as 'http'. Smells found: {:?}",
            smells
        );
    }
}

// ─── Extended Design ────────────────────────────────────

mod trait_impl_leakage {
    use super::*;
    use qualirs::detectors::design::trait_impl_leakage::TraitImplLeakageDetector;
    static DETECTOR: TraitImplLeakageDetector = TraitImplLeakageDetector;

    #[test]
    fn detects_std_trait_overload() {
        let code = "\
struct Data;
impl Debug for Data {}
impl Clone for Data {}
impl Copy for Data {}
impl Hash for Data {}
impl Default for Data {}
";
        assert_smell_count(&DETECTOR, code, "Trait Impl Leakage", 1);
    }

    #[test]
    fn clean_with_domain_trait() {
        // Even with 5+ std traits, having one domain trait makes it safe
        let code = "\
struct Data;
impl Debug for Data {}
impl Clone for Data {}
impl Copy for Data {}
impl Hash for Data {}
impl Default for Data {}
impl crate::auth::Authorize for Data {}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_few_std_traits() {
        let code = "\
struct Data;
impl Debug for Data {}
impl Clone for Data {}
";
        assert_clean(&DETECTOR, code);
    }
}

mod feature_envy {
    use super::*;
    use qualirs::detectors::design::feature_envy::FeatureEnvyDetector;
    static DETECTOR: FeatureEnvyDetector = FeatureEnvyDetector;

    #[test]
    fn detects_envy() {
        let code = "\
pub fn process(other: &Other) {
    other.do_a();
    other.do_b();
    other.do_c();
    other.do_d();
    other.do_e();
    other.do_f();
}
";
        assert_smell_count(&DETECTOR, code, "Feature Envy", 1);
    }

    #[test]
    fn clean_private_function() {
        // Private functions are never flagged, even with many calls
        let code = "\
fn process(other: &Other) {
    other.do_a();
    other.do_b();
    other.do_c();
    other.do_d();
    other.do_e();
    other.do_f();
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_within_threshold() {
        let code = "\
pub fn process(other: &Other) {
    other.do_a();
    other.do_b();
    other.do_c();
}
";
        assert_clean(&DETECTOR, code);
    }
}

mod wide_hierarchy {
    use super::*;
    use qualirs::detectors::design::wide_hierarchy::WideHierarchyDetector;
    static DETECTOR: WideHierarchyDetector = WideHierarchyDetector;

    #[test]
    fn detects_wide_enum() {
        let code = "\
enum Huge {
    V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11
}
";
        assert_smell_count(&DETECTOR, code, "Wide Hierarchy", 1);
    }

    #[test]
    fn clean_small_enum() {
        let code = "enum Color { Red, Green, Blue }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_enum_at_threshold() {
        // Exactly 10 variants does NOT trigger (>10 required)
        let code = "enum Status { V1, V2, V3, V4, V5, V6, V7, V8, V9, V10 }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_struct_few_fields() {
        let code = "struct Point { x: f64, y: f64, z: f64 }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_tuple_struct_many_fields() {
        // Tuple structs are not checked regardless of field count
        let code = "pub struct Wrapper(pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32);";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_threshold_config_struct() {
        let code = "\
struct DesignThresholds {
    a: usize, b: usize, c: usize, d: usize, e: usize, f: usize,
    g: usize, h: usize, i: usize, j: usize, k: usize,
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_config_and_template_structs() {
        let code = "\
struct Settings { a:i32,b:i32,c:i32,d:i32,e:i32,f:i32,g:i32,h:i32,i:i32,j:i32,k:i32,l:i32 }
struct DashboardTemplate { a:i32,b:i32,c:i32,d:i32,e:i32,f:i32,g:i32,h:i32,i:i32,j:i32,k:i32,l:i32 }
";
        assert_clean(&DETECTOR, code);
    }
}

mod broken_constructor {
    use super::*;
    use qualirs::detectors::design::broken_constructor::BrokenConstructorDetector;
    static DETECTOR: BrokenConstructorDetector = BrokenConstructorDetector;

    #[test]
    fn detects_missing_new() {
        let code = "\
pub struct State {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}
";
        assert_smell_count(&DETECTOR, code, "Broken Constructor", 1);
    }

    #[test]
    fn clean_with_new_constructor() {
        let code = "\
pub struct State {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}
impl State {
    pub fn new(a: i32, b: i32, c: i32) -> Self { Self { a, b, c } }
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_with_default_derive() {
        let code = "\
#[derive(Default)]
pub struct State {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_private_fields() {
        let code = "\
pub struct State {
    a: i32,
    pub b: i32,
    pub c: i32,
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_few_fields() {
        let code = "\
pub struct Pair {
    pub a: i32,
    pub b: i32,
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_dto_template_and_config_structs() {
        let code = r#"
pub struct CreateUserCommand {
    pub name: String,
    pub email: String,
    pub age: i32,
}

#[derive(Template)]
#[template(path = "user.html")]
pub struct UserTemplate {
    pub name: String,
    pub email: String,
    pub age: i32,
}

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub tls: bool,
}
"#;
        assert_clean(&DETECTOR, code);
    }
}

// ─── Extended Concurrency ───────────────────────────────

mod spawn_without_join {
    use super::*;
    use qualirs::detectors::concurrency::spawn_without_join::SpawnWithoutJoinDetector;
    static DETECTOR: SpawnWithoutJoinDetector = SpawnWithoutJoinDetector;

    #[test]
    fn detects_orphaned_spawn() {
        let code = "fn foo() { std::thread::spawn(|| {}); }";
        assert_smell_count(&DETECTOR, code, "Spawn Without Join", 1);
    }

    #[test]
    fn detects_underscore_spawn() {
        let code = "fn foo() { let _ = tokio::spawn(async {}); }";
        assert_smell_count(&DETECTOR, code, "Spawn Without Join", 1);
    }

    #[test]
    fn clean_no_spawn() {
        let code = "fn foo() { let x = 1 + 2; }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_assigned_spawn() {
        // Spawn assigned to a named variable is safe
        let code = "fn foo() { let handle = std::thread::spawn(|| {}); handle.join().unwrap(); }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_regular_function_with_spawn_in_name() {
        let code = "fn foo() { spawn_detached_mirror_refresh_job(); }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_spawn_handle_returned_from_function() {
        let code = "fn foo() -> Option<JoinHandle> { Some(tokio::spawn(async {})) }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_rayon_spawn_without_join_handle() {
        let code = "fn foo() { rayon::spawn(|| {}); }";
        assert_clean(&DETECTOR, code);
    }
}

mod holding_lock_across_await {
    use super::*;
    use qualirs::detectors::concurrency::holding_lock_across_await::HoldingLockAcrossAwaitDetector;

    static DETECTOR: HoldingLockAcrossAwaitDetector = HoldingLockAcrossAwaitDetector;

    #[test]
    fn detects_bound_guard_held_across_later_await() {
        let code = r#"
async fn foo(lock: &tokio::sync::Mutex<i32>) {
    let guard = lock.lock().await;
    do_work().await;
    drop(guard);
}
"#;
        assert_smell_count(&DETECTOR, code, "Holding Lock Across Await", 1);
    }

    #[test]
    fn clean_lock_temporary_used_in_single_statement_before_await() {
        let code = r#"
async fn logout(state: State) {
    let removed = state.sessions.write().await.remove("session-id");
    if removed.is_some() {
        record_audit_event().await;
    }
}
"#;
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_bound_guard_without_later_await() {
        let code = r#"
async fn require_session(state: State) {
    let mut sessions = state.sessions.write().await;
    sessions.remove("session-id");
}
"#;
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_explicit_drop_before_await() {
        let code = r#"
async fn foo(lock: &tokio::sync::Mutex<i32>) {
    let guard = lock.lock().await;
    drop(guard);
    do_work().await;
}
"#;
        assert_clean(&DETECTOR, code);
    }
}

mod blocking_channel_in_async {
    use super::*;
    use qualirs::detectors::concurrency::blocking_channel_in_async::BlockingChannelInAsyncDetector;

    static DETECTOR: BlockingChannelInAsyncDetector = BlockingChannelInAsyncDetector;

    #[test]
    fn detects_blocking_recv_in_async() {
        let code = r#"
async fn wait(receiver: std::sync::mpsc::Receiver<i32>) {
    let _ = receiver.recv();
}
"#;
        assert_smell_count(&DETECTOR, code, "Blocking Channel in Async", 1);
    }

    #[test]
    fn clean_awaited_async_recv() {
        let code = r#"
async fn wait(mut signal: tokio::sync::watch::Receiver<bool>) {
    signal.recv().await;
}
"#;
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_send_inside_non_async_worker_closure() {
        let code = r#"
async fn inspect() {
    let (sender, receiver) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let _ = sender.send(1);
    });
    let _ = receiver.await;
}
"#;
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_nonblocking_send_in_async() {
        let code = r#"
async fn notify(sender: tokio::sync::watch::Sender<bool>) {
    sender.send(true).expect("send shutdown");
}
"#;
        assert_clean(&DETECTOR, code);
    }
}

mod missing_send_bound {
    use super::*;
    use qualirs::detectors::concurrency::missing_send_bound::MissingSendBoundDetector;
    static DETECTOR: MissingSendBoundDetector = MissingSendBoundDetector;

    #[test]
    fn detects_missing_send() {
        let code = "fn run<T>(data: T) { spawn(move || { let _ = data; }); }";
        assert_smell_count(&DETECTOR, code, "Missing Send Bound", 1);
    }

    #[test]
    fn clean_with_send_bound() {
        let code = "fn run<T: Send>(data: T) { spawn(move || { let _ = data; }); }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_non_generic_fn() {
        // Non-generic functions with spawn are not flagged
        let code = "fn run() { spawn(|| {}); }";
        assert_clean(&DETECTOR, code);
    }
}

// ─── Extended Unsafe ────────────────────────────────────

mod raw_pointer_arithmetic {
    use super::*;
    use qualirs::detectors::r#unsafe::raw_pointer_arithmetic::RawPointerArithmeticDetector;
    static DETECTOR: RawPointerArithmeticDetector = RawPointerArithmeticDetector;

    #[test]
    fn detects_pointer_arith() {
        let code = "fn foo(raw_ptr: *const i32) { unsafe { let _ = raw_ptr.offset(1); } }";
        assert_smell_count(&DETECTOR, code, "Raw Pointer Arithmetic", 1);
    }

    #[test]
    fn clean_non_pointer_method() {
        // .add() on a Vec receiver does not trigger — receiver must contain "ptr" or "raw"
        let code = "fn foo(v: Vec<i32>) { let _ = v.len(); }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_no_pointer_methods() {
        let code = "fn foo(p: *const i32) { let _ = p; }";
        assert_clean(&DETECTOR, code);
    }
}

mod ffi_without_wrapper {
    use super::*;
    use qualirs::detectors::r#unsafe::ffi_without_wrapper::FfiWithoutWrapperDetector;
    static DETECTOR: FfiWithoutWrapperDetector = FfiWithoutWrapperDetector;

    #[test]
    fn detects_naked_ffi() {
        let code = "extern \"C\" { fn unsafe_c_api(); }";
        assert_smell_count(&DETECTOR, code, "FFI Without Wrapper", 1);
    }

    #[test]
    fn clean_with_wrapper() {
        let code = "\
extern \"C\" { fn some_api(); }
pub fn some_api_wrapper() { unsafe { some_api(); } }
";
        assert_clean(&DETECTOR, code);
    }
}

mod ffi_type_not_repr_c {
    use super::*;
    use qualirs::detectors::r#unsafe::ffi_type_not_repr_c::FfiTypeNotReprCDetector;
    static DETECTOR: FfiTypeNotReprCDetector = FfiTypeNotReprCDetector;

    #[test]
    fn detects_ffi_type_without_repr_c() {
        let code = r#"
extern "C" { fn use_config(config: *const CConfig); }
pub struct CConfig {
    value: i32,
}
"#;
        assert_smell_count(&DETECTOR, code, "FFI Type Not repr(C)", 1);
    }

    #[test]
    fn clean_repr_c_ffi_type() {
        let code = r#"
extern "C" { fn use_config(config: *const CConfig); }
#[repr(C)]
pub struct CConfig {
    value: i32,
}
"#;
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_public_c_named_rust_types_outside_ffi_context() {
        let code = r#"
pub struct Config;
pub struct CategorySmells;
pub struct CloneOnCopyDetector;
pub struct ConcurrencyThresholds;
"#;
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_detector_names_outside_ffi_context() {
        let code = r#"
pub struct FfiTypeNotReprCDetector;
pub struct FfiWithoutWrapperDetector;
"#;
        assert_clean(&DETECTOR, code);
    }
}

mod duplicate_match_arms {
    use super::*;
    use qualirs::detectors::implementation::duplicate_match_arms::DuplicateMatchArmsDetector;
    static DETECTOR: DuplicateMatchArmsDetector = DuplicateMatchArmsDetector;

    #[test]
    fn detects_duplicate_match_arm_bodies() {
        let code = r#"
fn classify(value: i32) -> i32 {
    match value {
        1 => score(),
        2 => score(),
        _ => 0,
    }
}
"#;
        assert_smell_count(&DETECTOR, code, "Duplicate Match Arms", 1);
    }

    #[test]
    fn clean_same_field_projection_from_different_variant_payloads() {
        let code = r#"
enum Item {
    Const(ConstItem),
    Enum(EnumItem),
}

struct ConstItem {
    vis: bool,
}

struct EnumItem {
    vis: bool,
}

fn is_pub(item: Item) -> bool {
    let vis = match item {
        Item::Const(i) => &i.vis,
        Item::Enum(i) => &i.vis,
    };
    *vis
}
"#;
        assert_clean(&DETECTOR, code);
    }
}

mod inline_assembly {
    use super::*;
    use qualirs::detectors::r#unsafe::inline_assembly::InlineAssemblyDetector;
    static DETECTOR: InlineAssemblyDetector = InlineAssemblyDetector;

    #[test]
    fn detects_asm_macro() {
        let code = "fn foo() { unsafe { std::arch::asm!(\"nop\"); } }";
        assert_smell_count(&DETECTOR, code, "Inline Assembly Usage", 1);
    }

    #[test]
    fn detects_global_asm_macro() {
        let code = "core::arch::global_asm!(\".global _start\");";
        assert_smell_count(&DETECTOR, code, "Inline Assembly Usage", 1);
    }

    #[test]
    fn clean_other_macros() {
        let code = "fn foo() { println!(\"hello\"); vec![1, 2, 3]; }";
        assert_clean(&DETECTOR, code);
    }
}

// ─── False Positive Tests: Architecture ──────────────────────

mod hidden_global_state {
    use super::*;
    use qualirs::detectors::architecture::hidden_global_state::HiddenGlobalStateDetector;
    static DETECTOR: HiddenGlobalStateDetector = HiddenGlobalStateDetector;

    #[test]
    fn detects_many_statics() {
        let code = "\
static A: i32 = 1;
static B: i32 = 2;
static C: i32 = 3;
static D: i32 = 4;
";
        assert_smell_count(&DETECTOR, code, "Hidden Global State", 1);
    }

    #[test]
    fn clean_few_statics() {
        let code = "\
static COUNTER: i32 = 0;
static VERSION: &str = \"1.0\";
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_const_not_static() {
        // const items should not be counted as global state
        let code = "\
const MAX: i32 = 100;
const MIN: i32 = 0;
const NAME: &str = \"app\";
const VER: &str = \"1.0\";
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_no_globals() {
        let code = "fn main() { let x = 1; }";
        assert_clean(&DETECTOR, code);
    }
}

mod leaky_error {
    use super::*;
    use qualirs::detectors::architecture::leaky_error::LeakyErrorAbstractionDetector;
    static DETECTOR: LeakyErrorAbstractionDetector = LeakyErrorAbstractionDetector;

    #[test]
    fn detects_leaky_error() {
        // One smell per leaking variant
        let code = "\
pub enum AppError {
    Db(sqlx::Error),
    Http(reqwest::Error),
}
";
        assert_smell_count(&DETECTOR, code, "Leaky Error Abstraction", 2);
    }

    #[test]
    fn clean_private_error_enum() {
        // Non-public enums are safe
        let code = "\
enum AppError {
    Db(sqlx::Error),
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_non_error_name() {
        // Enum name not ending in "Error" is safe
        let code = "\
pub enum AppResult {
    Db(sqlx::Error),
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_named_field_variant() {
        // Named-field variants are safe (only tuple variants checked)
        let code = "\
pub enum AppError {
    Io { inner: std::io::Error },
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_non_flagged_crate() {
        // Crates not in the hardcoded list are safe
        let code = "\
pub enum AppError {
    Other(diesel::result::Error),
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_boxed_error() {
        // Box<reqwest::Error> is safe — first segment is "Box", not "reqwest"
        let code = "\
pub enum AppError {
    Http(Box<reqwest::Error>),
}
";
        assert_clean(&DETECTOR, code);
    }
}

// ─── False Positive Tests: Design ────────────────────────────

mod fat_impl {
    use super::*;
    use qualirs::detectors::design::fat_impl::FatImplDetector;
    static DETECTOR: FatImplDetector = FatImplDetector;

    #[test]
    fn detects_fat_impl() {
        let methods: String = (0..21)
            .map(|i| format!("fn method_{i}(&self) {{}}"))
            .collect::<Vec<_>>()
            .join("\n    ");
        let code = format!("struct Big;\nimpl Big {{\n    {methods}\n}}");
        assert_smell_count(&DETECTOR, &code, "Fat Impl (God Object)", 1);
    }

    #[test]
    fn clean_reasonable_impl() {
        let code = "\
struct Service;
impl Service {
    fn new() -> Self { Service }
    fn run(&self) {}
    fn stop(&self) {}
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_trait_impl_not_counted() {
        // Trait impls are not counted toward the fat impl limit
        let methods: String = (0..21)
            .map(|i| format!("fn method_{i}(&self);"))
            .collect::<Vec<_>>()
            .join("\n    ");
        let code = format!("trait Big {{\n    {methods}\n}}");
        assert_clean(&DETECTOR, &code);
    }

    #[test]
    fn clean_at_threshold() {
        // Exactly 20 methods does NOT trigger
        let methods: String = (0..20)
            .map(|i| format!("fn method_{i}(&self) {{}}"))
            .collect::<Vec<_>>()
            .join("\n    ");
        let code = format!("struct Big;\nimpl Big {{\n    {methods}\n}}");
        assert_clean(&DETECTOR, &code);
    }
}

mod primitive_obsession {
    use super::*;
    use qualirs::detectors::design::primitive_obsession::PrimitiveObsessionDetector;
    static DETECTOR: PrimitiveObsessionDetector = PrimitiveObsessionDetector;

    #[test]
    fn detects_primitive_obsession() {
        let code = "struct Data { a: i32, b: i32, c: i32, d: i32, e: i32 }";
        assert_smell_count(&DETECTOR, code, "Primitive Obsession", 1);
    }

    #[test]
    fn clean_mixed_types() {
        // A struct with non-primitive fields is safe
        let code = "struct Data { a: i32, b: i32, c: i32, d: i32, e: Vec<String> }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_few_fields() {
        let code = "struct Point { x: f64, y: f64, z: f64 }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_thresholds_suffix() {
        // Structs ending in "Thresholds" are excluded
        let code = "struct MyThresholds { a: i32, b: i32, c: i32, d: i32, e: i32 }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_config_and_view_carriers() {
        let code = "\
struct RateLimitConfig { a: i32, b: i32, c: i32, d: i32, e: i32 }
struct DashboardView { a: String, b: String, c: String, d: String, e: String }
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_option_not_primitive() {
        // Option<i32> is NOT considered primitive
        let code = "struct Data { a: i32, b: i32, c: i32, d: i32, e: Option<i32> }";
        assert_clean(&DETECTOR, code);
    }
}

mod data_clumps {
    use super::*;
    use qualirs::detectors::design::data_clumps::DataClumpsDetector;
    static DETECTOR: DataClumpsDetector = DataClumpsDetector;

    #[test]
    fn detects_data_clumps() {
        let code = "\
fn build_user(name: String, age: i32, email: String) {}
fn update_user(name: String, age: i32, email: String) {}
fn migrate_user(name: String, age: i32, email: String) {}
";
        assert_smell_count(&DETECTOR, code, "Data Clumps", 1);
    }

    #[test]
    fn clean_few_params() {
        // Functions with <3 params are never grouped
        let code = "\
fn build_user(name: String, age: i32) {}
fn update_user(name: String, age: i32) {}
fn migrate_user(name: String, age: i32) {}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_destructured_arguments_do_not_create_one_param_clumps() {
        let code = "\
fn simple_index(State(state): State, Path(tenant): Path<String>, headers: HeaderMap) {}
fn simple_project(State(state): State, Path(tenant): Path<String>, headers: HeaderMap) {}
fn download_artifact(State(state): State, Path(tenant): Path<String>, headers: HeaderMap) {}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_unique_signatures() {
        // Different parameter names break the match
        let code = "\
fn build_user(name: String, age: i32, email: String) {}
fn update_user(user_name: String, user_age: i32, user_email: String) {}
fn migrate_user(full_name: String, years: i32, mail: String) {}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_only_two_occurrences() {
        // Need 3+ occurrences of the same signature
        let code = "\
fn build_user(name: String, age: i32, email: String) {}
fn update_user(name: String, age: i32, email: String) {}
";
        assert_clean(&DETECTOR, code);
    }
}

mod multiple_impl_blocks {
    use super::*;
    use qualirs::detectors::design::multiple_impl_blocks::MultipleImplBlocksDetector;
    static DETECTOR: MultipleImplBlocksDetector = MultipleImplBlocksDetector;

    #[test]
    fn detects_scattered_impl() {
        let code = "\
struct S;
impl S { fn a(&self) {} }
impl S { fn b(&self) {} }
";
        assert_smell_count(&DETECTOR, code, "Scattered Implementation", 1);
    }

    #[test]
    fn clean_single_impl() {
        let code = "\
struct S;
impl S { fn a(&self) {} fn b(&self) {} }
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_trait_impls_only() {
        // Only inherent impls count; trait impls are excluded
        let code = "\
struct S;
impl S { fn a(&self) {} }
impl Debug for S { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { Ok(()) } }
impl Clone for S { fn clone(&self) -> Self { S } }
";
        assert_clean(&DETECTOR, code);
    }
}

mod rebellious_impl {
    use super::*;
    use qualirs::detectors::design::rebellious_impl::RebelliousImplDetector;
    static DETECTOR: RebelliousImplDetector = RebelliousImplDetector;

    #[test]
    fn detects_rebellious_repo() {
        let code = "\
struct UserRepository;
impl UserRepository {
    fn find_by_id(&self) {}
    fn print_report(&self) {}
}
";
        assert_smell_count(&DETECTOR, code, "Rebellious Impl", 1);
    }

    #[test]
    fn clean_unrelated_type() {
        // Type name doesn't match any keyword pattern
        let code = "\
struct UserService;
impl UserService {
    fn print_report(&self) {}
    fn format_output(&self) {}
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_well_behaved_repo() {
        // Repo with only data-access methods is safe
        let code = "\
struct UserRepository;
impl UserRepository {
    fn find_by_id(&self) {}
    fn save(&self) {}
    fn delete(&self) {}
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_well_behaved_validator() {
        // Validator with only validation methods is safe
        let code = "\
struct InputValidator;
impl InputValidator {
    fn is_valid(&self) -> bool { true }
    fn check_length(&self) -> bool { true }
}
";
        assert_clean(&DETECTOR, code);
    }
}

// ─── False Positive Tests: Implementation ────────────────────

mod deeply_nested_type {
    use super::*;
    use qualirs::detectors::implementation::deeply_nested_type::DeeplyNestedTypeDetector;
    static DETECTOR: DeeplyNestedTypeDetector = DeeplyNestedTypeDetector;

    #[test]
    fn detects_deep_nesting() {
        let code = "struct S { data: Arc<Mutex<HashMap<String, Vec<i32>>>> }";
        assert_smell_count(&DETECTOR, code, "Type Alias Explosion (Deep Nesting)", 1);
    }

    #[test]
    fn clean_shallow_nesting() {
        let code = "struct S { data: HashMap<String, Vec<i32>> }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_no_generics() {
        let code = "struct S { value: i32 }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_option_result() {
        let code = "struct S { data: Option<Result<String, std::io::Error>> }";
        assert_clean(&DETECTOR, code);
    }
}

mod interior_mutability_abuse {
    use super::*;
    use qualirs::detectors::implementation::interior_mutability_abuse::InteriorMutabilityAbuseDetector;
    static DETECTOR: InteriorMutabilityAbuseDetector = InteriorMutabilityAbuseDetector;

    #[test]
    fn detects_many_refcell() {
        let fields: String = (0..6)
            .map(|i| format!("f{i}: RefCell<i32>"))
            .collect::<Vec<_>>()
            .join(", ");
        let code = format!("struct S {{ {fields} }}");
        assert_smell_count(&DETECTOR, &code, "Interior Mutability Abuse", 1);
    }

    #[test]
    fn clean_few_usages() {
        let code = "struct S { a: RefCell<i32>, b: Cell<bool> }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_no_interior_mutability() {
        let code = "struct S { value: i32 }";
        assert_clean(&DETECTOR, code);
    }
}

// ─── False Positive Tests: Concurrency ───────────────────────

mod sync_drop_blocking {
    use super::*;
    use qualirs::detectors::concurrency::sync_drop_blocking::SyncDropBlockingDetector;
    static DETECTOR: SyncDropBlockingDetector = SyncDropBlockingDetector;

    #[test]
    fn detects_blocking_in_drop() {
        let code = "\
struct Conn;
impl Drop for Conn {
    fn drop(&mut self) {
        self.flush();
    }
}
";
        assert_smell_count(&DETECTOR, code, "Sync Drop Blocking (Async Hazard)", 1);
    }

    #[test]
    fn clean_safe_drop() {
        let code = "\
struct Handle;
impl Drop for Handle {
    fn drop(&mut self) {
        println!(\"dropped\");
    }
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_no_drop_impl() {
        let code = "struct Handle;";
        assert_clean(&DETECTOR, code);
    }
}

mod async_trait_overhead {
    use super::*;
    use qualirs::detectors::concurrency::async_trait_overhead::AsyncTraitOverheadDetector;
    static DETECTOR: AsyncTraitOverheadDetector = AsyncTraitOverheadDetector;

    #[test]
    fn detects_async_trait() {
        let code = "\
#[async_trait]
pub trait Handler {
    async fn handle(&self);
}
";
        assert_smell_count(&DETECTOR, code, "Async Trait Overhead", 1);
    }

    #[test]
    fn clean_no_async_trait() {
        let code = "\
pub trait Handler {
    fn handle(&self);
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_other_attributes() {
        let code = "\
#[derive(Clone)]
pub struct Handler;
";
        assert_clean(&DETECTOR, code);
    }
}

mod unnecessary_allocation_in_loop {
    use super::*;
    use qualirs::detectors::implementation::unnecessary_allocation_in_loop::UnnecessaryAllocationInLoopDetector;
    static DETECTOR: UnnecessaryAllocationInLoopDetector = UnnecessaryAllocationInLoopDetector;

    #[test]
    fn detects_clear_loop_allocations() {
        let code = r#"
fn build(items: &[&str]) {
    for item in items {
        let a = String::from(*item);
        let b = item.to_owned();
        let c = format!("item: {item}");
    }
}
"#;
        assert_smell_count(&DETECTOR, code, "Unnecessary Allocation in Loop", 3);
    }

    #[test]
    fn clean_owned_reporting_and_grouping_work() {
        let code = r#"
fn report(items: &[i32]) {
    for item in items {
        let label = item.to_string();
        let selected: Vec<_> = [1, 2, 3].iter().filter(|value| **value > 1).collect();
        println!("{label}: {}", selected.len());
    }
}
"#;
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_borrowed_format_argument() {
        let code = r#"
fn report(items: &[i32]) {
    for item in items {
        consume(&format!("item: {item}"));
    }
}
"#;
        assert_clean(&DETECTOR, code);
    }
}

mod manual_default_constructor {
    use super::*;
    use qualirs::detectors::implementation::manual_default_constructor::ManualDefaultConstructorDetector;
    static DETECTOR: ManualDefaultConstructorDetector = ManualDefaultConstructorDetector;

    #[test]
    fn detects_no_arg_defaultish_new() {
        let code = "\
struct Bag { items: Vec<String> }
impl Bag {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
}
";
        assert_smell_count(&DETECTOR, code, "Manual Default Constructor", 1);
    }

    #[test]
    fn clean_constructor_uses_parameter() {
        let code = "\
struct Engine { detectors: Vec<String>, config: Config }
struct Config;
impl Engine {
    fn new(config: Config) -> Self {
        Self { detectors: Vec::new(), config }
    }
}
";
        assert_clean(&DETECTOR, code);
    }
}

mod manual_find_loop {
    use super::*;
    use qualirs::detectors::implementation::manual_find_loop::ManualFindLoopDetector;
    static DETECTOR: ManualFindLoopDetector = ManualFindLoopDetector;

    #[test]
    fn detects_direct_conditional_return_in_loop() {
        let code = "\
fn has_even(values: &[i32]) -> bool {
    for value in values {
        if *value % 2 == 0 {
            return true;
        }
    }
    false
}
";
        assert_smell_count(&DETECTOR, code, "Manual Find/Any Loop", 1);
    }

    #[test]
    fn clean_return_inside_nested_closure() {
        let code = "\
pub fn process(items: &[i32]) {
    for item in items {
        let selected = [1, 2, 3].iter().filter_map(|value| {
            if *value == *item {
                return Some(value);
            }
            None
        });
        let _ = selected.count();
    }
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_loop_returning_constructed_value() {
        let code = "\
fn find_path(paths: &[&str]) -> Option<String> {
    for path in paths {
        if path.ends_with(\".rs\") {
            return Some(path.to_string());
        }
    }
    None
}
";
        assert_clean(&DETECTOR, code);
    }
}

mod derivable_impl {
    use super::*;
    use qualirs::detectors::implementation::derivable_impl::DerivableImplDetector;
    static DETECTOR: DerivableImplDetector = DerivableImplDetector;

    #[test]
    fn clean_custom_default_values() {
        let code = "\
struct Thresholds { limit: usize }
impl Default for Thresholds {
    fn default() -> Self {
        Self { limit: 50 }
    }
}
";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn detects_mechanical_default_impl() {
        let code = "\
struct State { name: String, items: Vec<String> }
impl Default for State {
    fn default() -> Self {
        Self { name: String::new(), items: Vec::new() }
    }
}
";
        assert_smell_count(&DETECTOR, code, "Derivable Impl", 1);
    }
}

// ─── False Positive Tests: Unsafe ────────────────────────────

mod multi_mut_ref_unsafe {
    use super::*;
    use qualirs::detectors::r#unsafe::multi_mut_ref_unsafe::MultiMutRefUnsafeDetector;
    static DETECTOR: MultiMutRefUnsafeDetector = MultiMutRefUnsafeDetector;

    #[test]
    fn detects_multiple_mut_ref() {
        // Reports one smell per instance when threshold (>=2) is met
        let code = "\
fn foo(a: &mut i32, b: &mut i32) {
    let x = &mut *a;
    let y = &mut *b;
}
";
        assert_smell_count(&DETECTOR, code, "Multi Mut Ref Unsafe", 2);
    }

    #[test]
    fn clean_single_instance() {
        // A single &mut *expr is safe (< 2 threshold)
        let code = "fn foo(a: &mut i32) { let x = &mut *a; }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_immut_deref() {
        // Immutable deref is safe
        let code = "fn foo(a: &i32) { let x = &*a; let y = &*a; }";
        assert_clean(&DETECTOR, code);
    }

    #[test]
    fn clean_normal_refs() {
        let code = "fn foo(a: &mut i32) { *a = 42; }";
        assert_clean(&DETECTOR, code);
    }
}
