// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test that we can do shifts by any integral type.

struct Panolpy {
    char: char,
    str: &'static str,
}

fn foo(p: &Panolpy) {
    22 >> p.char;
    //~^ ERROR right-hand-side of a shift operation must have integral type

    22 >> p.str;
    //~^ ERROR right-hand-side of a shift operation must have integral type

    22 >> p;
    //~^ ERROR right-hand-side of a shift operation must have integral type

    // We could be more accepting in the case of a type not yet inferred, but not
    // known to be an integer, but meh.
    let x;
    22 >> x;
    //~^ ERROR right-hand-side of a shift operation must have integral type

    22 >> 1;
    // Integer literal types are OK

    // Type of the result follows the LHS, not the RHS:
    let _: i32 = 22_i64 >> 1_i32;
    //~^ ERROR mismatched types: expected `i32`, found `i64`
}

fn main() {
}
