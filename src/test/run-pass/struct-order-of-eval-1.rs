// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct S { f0: StrBuf, f1: int }

pub fn main() {
    let s = "Hello, world!".to_strbuf();
    let _s = S {
        f0: s.to_strbuf(),
        ..S {
            f0: s,
            f1: 23
        }
    };
}
