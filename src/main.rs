use hello_macro_derive::HelloMacro;
use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Foo;

fn main() {
    Foo::hello_macro();
}
