//@ aux-build:block-on.rs
//@ edition:2021
//@ run-pass

#![feature(async_closure)]

extern crate block_on;

fn main() {
    block_on::block_on(async {
        let x = async || {};

        async fn needs_async_fn_mut(mut x: impl async FnMut()) {
            x().await;
        }
        needs_async_fn_mut(x).await;
    });
}
