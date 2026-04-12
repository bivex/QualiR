mod common;

use std::path::PathBuf;
use qualirs::analysis::detector::Detector;
use qualirs::domain::source::SourceFile;
use common::{assert_clean, assert_smell_count};

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
}

// ─── Implementation ────────────────────────────────────────

mod long_function {
    use super::*;
    use qualirs::detectors::implementation::long_function::LongFunctionDetector;

    static DETECTOR: LongFunctionDetector = LongFunctionDetector;

    #[test]
    fn detects_long_fn() {
        let body: String = (0..55).map(|i| format!("let _ = {i};")).collect::<Vec<_>>().join("\n");
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
        let arms: String = (0..20).map(|i| format!("{i} => (),")).collect::<Vec<_>>().join(" ");
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
        let code = "fn chain(x: Vec<i32>) { x.iter().filter(|&&x| x > 0).map(|&x| x * 2).flatten().collect::<Vec<i32>>(); }";
        assert_smell_count(&DETECTOR, code, "Long Method Chain", 1);
    }

    #[test]
    fn clean_short_chain() {
        let code = "fn short(x: Vec<i32>) { x.iter().count(); }";
        assert_clean(&DETECTOR, code);
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
        assert!(smells.iter().any(|s| s.name == "Panic in Library"), "Should detect panic!: {:?}", smells);
    }

    #[test]
    fn detects_todo() {
        let code = "fn unfinished() { todo!(); }";
        let file = SourceFile::from_source(PathBuf::from("main.rs"), code.to_string()).unwrap();
        let smells = DETECTOR.detect(&file);
        assert!(smells.iter().any(|s| s.name == "Panic in Library"), "Should detect todo!: {:?}", smells);
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
