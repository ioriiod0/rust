// exec-env:RUST_POISON_ON_FREE=1

fn borrow(x: &int, f: fn(x: &int)) {
    let before = *x;
    f(x);
    let after = *x;
    assert before == after;
}

fn main() {
    let mut x = @{f: ~3};
    do borrow((*x).f) |b_x| {
        assert *b_x == 3;
        assert ptr::addr_of(*x.f) == ptr::addr_of(*b_x);
        x = @{f: ~4};

        debug!("ptr::addr_of(*b_x) = %x", ptr::addr_of(*b_x) as uint);
        assert *b_x == 3;
        assert ptr::addr_of(*x.f) != ptr::addr_of(*b_x);
    }
}
