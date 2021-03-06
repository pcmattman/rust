// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(box_syntax)]

struct Test {
    func: Box<FnMut()+'static>
}

fn main() {
    // FIXME (#22405): Replace `Box::new` with `box` here when/if possible.
    let closure: Box<Fn()+'static> = Box::new(|| ());
    let test = box Test { func: closure }; //~ ERROR mismatched types
}
