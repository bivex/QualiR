use colored::*;

struct DetectorGroup {
    category: &'static str,
    names: &'static [&'static str],
}

impl DetectorGroup {
    const fn new(category: &'static str, names: &'static [&'static str]) -> Self {
        Self { category, names }
    }
}

const DETECTOR_GROUPS: &[DetectorGroup] = &[
    DetectorGroup::new(
        "Architecture",
        &[
            "God Module",
            "Public API Explosion",
            "Feature Concentration",
            "Cyclic Crate Dependency",
            "Layer Violation",
            "Unstable Dependency",
            "Leaky Error Abstraction",
            "Hidden Global State",
            "Public API Leak",
            "Test-only Dependency in Production",
            "Duplicate Dependency Versions",
            "Feature Flag Sprawl",
            "Circular Module Dependency",
        ],
    ),
    DetectorGroup::new(
        "Design",
        &[
            "Large Trait",
            "Excessive Generics",
            "Anemic Struct",
            "Wide Hierarchy",
            "Trait Impl Leakage",
            "Feature Envy",
            "Broken Constructor",
            "Rebellious Impl",
            "Fat Impl",
            "Primitive Obsession",
            "Data Clumps",
            "Multiple Impl Blocks",
            "God Struct",
            "Boolean Flag Argument",
            "Stringly Typed Domain",
            "Large Error Enum",
        ],
    ),
    DetectorGroup::new(
        "Implementation",
        &[
            "Long Function",
            "Too Many Arguments",
            "Deep Match Nesting",
            "Magic Numbers",
            "Large Enum",
            "High Cyclomatic Complexity",
            "Deep If/Else Nesting",
            "Long Method Chain",
            "Unsafe Block Overuse",
            "Lifetime Explosion",
            "Deeply Nested Type",
            "Duplicate Match Arms",
            "Long Closure",
            "Deep Closure Nesting",
        ],
    ),
    DetectorGroup::new(
        "Performance",
        &[
            "Excessive Clone",
            "Arc Mutex Overuse",
            "Large Future",
            "Async Trait Overhead",
            "Interior Mutability Abuse",
            "Unnecessary Allocation in Loop",
            "Collect Then Iterate",
            "Repeated Regex Construction",
            "Clone on Copy",
            "Large Value Passed By Value",
        ],
    ),
    DetectorGroup::new(
        "Idiomaticity",
        &[
            "Excessive Unwrap",
            "Unused Result Ignored",
            "Panic in Library",
            "Copy + Drop Conflict",
            "Deref Abuse",
            "Manual Drop",
            "Manual Default Constructor",
            "Manual Option/Result Mapping",
            "Manual Find/Any Loop",
            "Needless Explicit Lifetime",
            "Derivable Impl",
        ],
    ),
    DetectorGroup::new(
        "Concurrency",
        &[
            "Blocking in Async",
            "Deadlock Risk",
            "Spawn Without Join",
            "Missing Send Bound",
            "Sync Drop Blocking",
            "Std Mutex in Async",
            "Blocking Channel in Async",
            "Holding Lock Across Await",
            "Dropped JoinHandle",
        ],
    ),
    DetectorGroup::new(
        "Unsafe",
        &[
            "Unsafe Without Comment",
            "Transmute Usage",
            "Raw Pointer Arithmetic",
            "Multi Mut Ref Unsafe",
            "FFI Without Wrapper",
            "Inline Assembly",
            "Unsafe Fn Missing Safety Docs",
            "Unsafe Impl Missing Safety Docs",
            "Large Unsafe Block",
            "FFI Type Not repr(C)",
        ],
    ),
];

/// Print the list of available detectors.
pub fn print_detector_list() {
    println!();
    println!("{}", "Available detectors:".bright_cyan().bold());
    println!("{}", "━".repeat(40).dimmed());
    print_groups(DETECTOR_GROUPS);
    println!();
}

fn print_groups(groups: &[DetectorGroup]) {
    for group in groups {
        print_group(group);
    }
}

fn print_group(group: &DetectorGroup) {
    println!();
    println!("  {} {}", "▸".bright_magenta(), group.category.bold());
    for name in group.names {
        println!("    • {name}");
    }
}
