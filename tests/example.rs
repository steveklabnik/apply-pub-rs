#![feature(phase)]

#[phase(plugin)]
extern crate apply_pub = "apply-pub-rs";

#[apply_pub]
mod foo {
    fn bar() {}
    mod baz {
        fn qux() {}
    }
}

#[test]
fn main() {
    foo::bar();
    foo::baz::qux();
}
