// run-rustfix

#![feature(box_patterns)]
#![allow(dead_code)]

fn foo(f: Option<Box<i32>>) {
    match f {
        Some(ref box _i) => {},
        //~^ ERROR switch the order of `ref` and `box`
        //~| ERROR expected one of `)`, `,`, or `|`, found `_i`
        //~| ERROR this pattern has 2 fields, but the corresponding tuple variant has 1 field
        None => {}
    }
}

fn main() { }
