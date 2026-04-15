// EXPECT: Long Function
// EXPECT: Too Many Arguments
// EXPECT: Deep Match Nesting
// EXPECT: Magic Numbers
// EXPECT: Large Enum
// EXPECT: High Cyclomatic Complexity
// EXPECT: Deep If/Else Nesting
// EXPECT: Long Method Chain
// EXPECT: Unsafe Block Overuse
// EXPECT: Lifetime Explosion
// EXPECT: Deeply Nested Type
// EXPECT: Duplicate Match Arms
// EXPECT: Long Closure
// EXPECT: Deep Closure Nesting

#![allow(dead_code, unused_variables, unused_unsafe)]

fn long_function(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) {
    let magic = 1337;
    let _ = magic;
    if a > 0 {
        if b > 0 {
            if c > 0 {
                if d > 0 {
                    if e > 0 {
                        println!("deep");
                    }
                }
            }
        }
    }
    if a == 0 {}
    if a == 1 {}
    if a == 2 {}
    if a == 3 {}
    if a == 4 {}
    if a == 5 {}
    if a == 6 {}
    if a == 7 {}
    if a == 8 {}
    if a == 9 {}
    if a == 10 {}
    if a == 11 {}
    if a == 12 {}
    if a == 13 {}
    if a == 14 {}
    if a == 15 {}
    unsafe { let _ = a; }
    unsafe { let _ = b; }
    unsafe { let _ = c; }
    unsafe { let _ = d; }
    unsafe { let _ = e; }
    unsafe { let _ = f; }
    let _ = a + b + c + d + e + f + g;
    let _ = 40;
    let _ = 41;
    let _ = 42;
    let _ = 43;
    let _ = 44;
    let _ = 45;
    let _ = 46;
    let _ = 47;
    let _ = 48;
    let _ = 49;
    let _ = 50;
    let _ = 51;
    let _ = 52;
    let _ = 53;
    let _ = 54;
    let _ = 55;
    let _ = 56;
    let _ = 57;
    let _ = 58;
    let _ = 59;
    let _ = 60;
}

fn nested_match(value: Option<Option<Option<Option<i32>>>>) {
    match value {
        Some(a) => match a {
            Some(b) => match b {
                Some(c) => match c {
                    Some(_) => {}
                    None => {}
                },
                None => {}
            },
            None => {}
        },
        None => {}
    }
}

enum BigEvent {
    E00,
    E01,
    E02,
    E03,
    E04,
    E05,
    E06,
    E07,
    E08,
    E09,
    E10,
    E11,
    E12,
    E13,
    E14,
    E15,
    E16,
    E17,
    E18,
    E19,
    E20,
}

fn method_chain(items: Vec<String>) -> Vec<String> {
    items
        .iter()
        .filter(|item| !item.is_empty())
        .map(|item| item.trim())
        .filter(|item| item.len() > 3)
        .map(|item| item.to_string())
        .collect()
}

fn lifetime_heavy<'a, 'b, 'c, 'd, 'e>(
    a: &'a str,
    b: &'b str,
    c: &'c str,
    d: &'d str,
    e: &'e str,
) -> (&'a str, &'b str, &'c str, &'d str, &'e str) {
    (a, b, c, d, e)
}

type Nested = std::collections::HashMap<String, Vec<Box<std::sync::Arc<String>>>>;

fn duplicate_match_arms(value: i32) -> &'static str {
    match value {
        1 => "same",
        2 => "same",
        3 => "different",
        _ => "other",
    }
}

fn long_and_nested_closures(values: Vec<i32>) {
    let _ = values.iter().map(|a| {
        let _ = Some(a).map(|b| {
            let _ = Some(b).map(|c| {
                let _ = Some(c).map(|d| {
                    let _ = Some(d).map(|e| *e + 1);
                });
            });
        });
        let mut total = 0;
        total += 1;
        total += 2;
        total += 3;
        total += 4;
        total += 5;
        total += 6;
        total += 7;
        total += 8;
        total += 9;
        total += 10;
        total += 11;
        total += 12;
        total += 13;
        total += 14;
        total += 15;
        total += 16;
        total += 17;
        total += 18;
        total += 19;
        total += 20;
        total += 21;
        total += 22;
        total += 23;
        total += 24;
        total += 25;
        total
    });
}
