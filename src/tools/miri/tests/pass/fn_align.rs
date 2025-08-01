//@compile-flags: -Zmin-function-alignment=8
#![feature(fn_align)]

// When a function uses `align(N)`, the function address should be a multiple of `N`.

#[align(256)]
fn foo() {}

#[align(16)]
fn bar() {}

#[align(4)]
fn baz() {}

fn main() {
    assert!((foo as usize).is_multiple_of(256));
    assert!((bar as usize).is_multiple_of(16));

    // The maximum of `align(N)` and `-Zmin-function-alignment=N` is used.
    assert!((baz as usize).is_multiple_of(8));
}
