//@ compile-flags: --error-format=human --color=always
//@ error-pattern: missing lifetime specifier

//@ revisions: windows not-windows
//@ [windows]only-windows
//@ [not-windows]ignore-windows

fn short(foo_bar: &Vec<&i32>) -> &i32 {
    &12
}

fn long(
    foo_bar: &Vec<&i32>,
    something_very_long_so_that_the_line_will_wrap_around__________: i32,
) -> &i32 {
    &12
}

fn long2(
    foo_bar: &Vec<&i32>) -> &i32 {
    &12
}
fn main() {}
