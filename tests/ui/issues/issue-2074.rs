//@ run-pass
//@ pretty-expanded FIXME #23616

#![allow(non_camel_case_types)]

pub fn main() {
    let one = || {
        enum r { a }
        r::a as usize
    };
    let two = || {
        enum r { a }
        r::a as usize
    };
    one(); two();
}
