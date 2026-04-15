// EXPECT: Large Trait
// EXPECT: Excessive Generics
// EXPECT: Anemic Struct
// EXPECT: Wide Hierarchy
// EXPECT: Trait Impl Leakage
// EXPECT: Feature Envy
// EXPECT: Broken Constructor
// EXPECT: Rebellious Impl
// EXPECT: Fat Impl
// EXPECT: Primitive Obsession
// EXPECT: Data Clumps
// EXPECT: Multiple Impl Blocks
// EXPECT: God Struct
// EXPECT: Boolean Flag Argument
// EXPECT: Stringly Typed Domain
// EXPECT: Large Error Enum

#![allow(dead_code, unused_variables)]

trait HugeStore {
    fn m00(&self);
    fn m01(&self);
    fn m02(&self);
    fn m03(&self);
    fn m04(&self);
    fn m05(&self);
    fn m06(&self);
    fn m07(&self);
    fn m08(&self);
    fn m09(&self);
    fn m10(&self);
    fn m11(&self);
    fn m12(&self);
    fn m13(&self);
    fn m14(&self);
    fn m15(&self);
}

fn generic_pipeline<A, B, C, D, E, F>(a: A, b: B, c: C, d: D, e: E, f: F) {}

struct NakedRecord {
    id: String,
    name: String,
}

trait Handler {
    fn handle(&self);
}
struct Handler00;
struct Handler01;
struct Handler02;
struct Handler03;
struct Handler04;
struct Handler05;
struct Handler06;
struct Handler07;
struct Handler08;
struct Handler09;
struct Handler10;
impl Handler for Handler00 { fn handle(&self) {} }
impl Handler for Handler01 { fn handle(&self) {} }
impl Handler for Handler02 { fn handle(&self) {} }
impl Handler for Handler03 { fn handle(&self) {} }
impl Handler for Handler04 { fn handle(&self) {} }
impl Handler for Handler05 { fn handle(&self) {} }
impl Handler for Handler06 { fn handle(&self) {} }
impl Handler for Handler07 { fn handle(&self) {} }
impl Handler for Handler08 { fn handle(&self) {} }
impl Handler for Handler09 { fn handle(&self) {} }
impl Handler for Handler10 { fn handle(&self) {} }

struct User {
    name: String,
}

struct LeakyTraits;

impl std::fmt::Debug for LeakyTraits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LeakyTraits")
    }
}
impl Clone for LeakyTraits {
    fn clone(&self) -> Self {
        Self
    }
}
impl PartialEq for LeakyTraits {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}
impl Eq for LeakyTraits {}
impl std::hash::Hash for LeakyTraits {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {}
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rendered = format!("<user>{}</user>", self.name);
        write!(f, "{rendered}")
    }
}

struct Customer {
    country: String,
    state: String,
    tier: String,
    loyalty_points: u32,
}

struct OrderService;
impl OrderService {
    fn calculate_tax(&self, customer: &Customer) -> u32 {
        customer.country.len() as u32
            + customer.state.len() as u32
            + customer.tier.len() as u32
            + customer.loyalty_points
    }
}

pub fn describe_customer(customer: &Customer) -> String {
    customer.country();
    customer.state();
    customer.tier();
    customer.loyalty_points();
    customer.country();
    customer.state();
    String::new()
}

pub struct OpenAccount {
    pub id: u64,
    pub name: String,
    pub balance: i64,
}

struct Account {
    id: u64,
    name: String,
}
impl Account {
    fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
        }
    }
}

impl User {
    fn render_html(&self) -> String {
        format!("<h1>{}</h1>", self.name)
    }
}

struct UserRepository;
impl UserRepository {
    fn render_html(&self) -> String {
        String::new()
    }
}

struct Client;
impl Client {
    fn m00(&self) {}
    fn m01(&self) {}
    fn m02(&self) {}
    fn m03(&self) {}
    fn m04(&self) {}
    fn m05(&self) {}
    fn m06(&self) {}
    fn m07(&self) {}
    fn m08(&self) {}
    fn m09(&self) {}
    fn m10(&self) {}
    fn m11(&self) {}
    fn m12(&self) {}
    fn m13(&self) {}
    fn m14(&self) {}
    fn m15(&self) {}
    fn m16(&self) {}
    fn m17(&self) {}
    fn m18(&self) {}
    fn m19(&self) {}
    fn m20(&self) {}
}

struct UserPrimitive {
    id: String,
    email: String,
    status: String,
    role: String,
}

fn ship_one(street: String, city: String, zip: String) {}
fn ship_two(street: String, city: String, zip: String) {}
fn ship_three(street: String, city: String, zip: String) {}

struct SplitImpl;
impl SplitImpl { fn a(&self) {} }
impl SplitImpl { fn b(&self) {} }
impl SplitImpl { fn c(&self) {} }
impl SplitImpl { fn d(&self) {} }

struct AppState {
    f00: String,
    f01: String,
    f02: String,
    f03: String,
    f04: String,
    f05: String,
    f06: String,
    f07: String,
    f08: String,
    f09: String,
    f10: String,
    f11: String,
    f12: String,
    f13: String,
    f14: String,
    f15: String,
    f16: String,
    f17: String,
    f18: String,
    f19: String,
    f20: String,
}

fn render(user: &User, is_detailed: bool) {}

struct OrderRecord {
    order_id: String,
    user_id: String,
    status: String,
}

enum BigError {
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
}
