//@ check-pass
#![feature(transmutability)]

mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};

    pub fn is_maybe_transmutable<Src, Dst>()
    where
        Dst: BikeshedIntrinsicFrom<Src, {
            Assume {
                alignment: false,
                lifetimes: false,
                safety: true,
                validity: false,
            }
        }>
    {}
}

fn main() {
    assert::is_maybe_transmutable::<&'static [u16; 0], &'static [u8; 0]>();
}
