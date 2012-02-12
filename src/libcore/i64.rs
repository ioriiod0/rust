#[doc = "Operations and constants constants for `i64`"];

const min_value: i64 = -1i64 << 63i64;
const max_value: i64 = (-1i64 << 63i64) - 1i64;

pure fn add(x: i64, y: i64) -> i64 { x + y }
pure fn sub(x: i64, y: i64) -> i64 { x - y }
pure fn mul(x: i64, y: i64) -> i64 { x * y }
pure fn div(x: i64, y: i64) -> i64 { x / y }
pure fn rem(x: i64, y: i64) -> i64 { x % y }

pure fn lt(x: i64, y: i64) -> bool { x < y }
pure fn le(x: i64, y: i64) -> bool { x <= y }
pure fn eq(x: i64, y: i64) -> bool { x == y }
pure fn ne(x: i64, y: i64) -> bool { x != y }
pure fn ge(x: i64, y: i64) -> bool { x >= y }
pure fn gt(x: i64, y: i64) -> bool { x > y }

pure fn positive(x: i64) -> bool { x > 0i64 }
pure fn negative(x: i64) -> bool { x < 0i64 }
pure fn nonpositive(x: i64) -> bool { x <= 0i64 }
pure fn nonnegative(x: i64) -> bool { x >= 0i64 }

#[doc = "Iterate over the range [`lo`..`hi`)"]
fn range(lo: i64, hi: i64, it: fn(i64)) {
    let i = lo;
    while i < hi { it(i); i += 1i64; }
}