// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub fn main() {
    let yen: char = '¥'; // 0xa5
    let c_cedilla: char = 'ç'; // 0xe7
    let thorn: char = 'þ'; // 0xfe
    let y_diaeresis: char = 'ÿ'; // 0xff
    let pi: char = 'Π'; // 0x3a0

    assert_eq!(yen as int, 0xa5);
    assert_eq!(c_cedilla as int, 0xe7);
    assert_eq!(thorn as int, 0xfe);
    assert_eq!(y_diaeresis as int, 0xff);
    assert_eq!(pi as int, 0x3a0);

    assert_eq!(pi as int, '\u03a0' as int);
    assert_eq!('\x0a' as int, '\n' as int);

    let bhutan: StrBuf = "འབྲུག་ཡུལ།".to_strbuf();
    let japan: StrBuf = "日本".to_strbuf();
    let uzbekistan: StrBuf = "Ўзбекистон".to_strbuf();
    let austria: StrBuf = "Österreich".to_strbuf();

    let bhutan_e: StrBuf =
        "\u0f60\u0f56\u0fb2\u0f74\u0f42\u0f0b\u0f61\u0f74\u0f63\u0f0d".to_strbuf();
    let japan_e: StrBuf = "\u65e5\u672c".to_strbuf();
    let uzbekistan_e: StrBuf =
        "\u040e\u0437\u0431\u0435\u043a\u0438\u0441\u0442\u043e\u043d".to_strbuf();
    let austria_e: StrBuf = "\u00d6sterreich".to_strbuf();

    let oo: char = 'Ö';
    assert_eq!(oo as int, 0xd6);

    fn check_str_eq(a: StrBuf, b: StrBuf) {
        let mut i: int = 0;
        for ab in a.as_slice().bytes() {
            println!("{}", i);
            println!("{}", ab);
            let bb: u8 = b.as_slice()[i as uint];
            println!("{}", bb);
            assert_eq!(ab, bb);
            i += 1;
        }
    }

    check_str_eq(bhutan, bhutan_e);
    check_str_eq(japan, japan_e);
    check_str_eq(uzbekistan, uzbekistan_e);
    check_str_eq(austria, austria_e);
}
