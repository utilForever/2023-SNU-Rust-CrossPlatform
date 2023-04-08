pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

pub fn sub(a: i64, b: i64) -> i64 {
    a - b
}

pub fn mul(a: i64, b: i64) -> i64 {
    a * b
}

pub fn div(a: i64, b: i64) -> i64 {
    a / b
}

uniffi_macros::include_scaffolding!("calc");
