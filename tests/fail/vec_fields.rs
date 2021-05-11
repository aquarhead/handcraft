use handcraft::handcraft;

#[derive(Debug, Default, PartialEq)]
struct Foo {
  a: u64,
  b: u64,
}

#[derive(Debug, Default, PartialEq)]
struct Baz {
  v: Vec<Foo>,
}

fn main() {
  let crafted = handcraft!(Baz { v: vec![Foo { a: 0 }] });
}
