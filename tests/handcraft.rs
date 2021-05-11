use handcraft::handcraft;

#[derive(Debug, Default, PartialEq)]
struct Foo {
  a: u64,
  b: u64,
  extremly_long_name_to_force_line_break: u64,
}

#[test]
fn single_field() {
  let crafted = handcraft!(Foo { a: 0 });
  let expected = Foo {
    a: 0,
    ..Default::default()
  };

  assert_eq!(crafted, expected);
}

#[test]
fn multiple_fields() {
  // also test for trailing comma
  let crafted = handcraft!(Foo {
    a: 0,
    extremly_long_name_to_force_line_break: 1,
  });
  let expected = Foo {
    a: 0,
    extremly_long_name_to_force_line_break: 1,
    ..Default::default()
  };

  assert_eq!(crafted, expected);
}

#[derive(Debug, Default, PartialEq)]
struct Bar {
  x: Foo,
  y: Foo,
}

#[test]
fn nested() {
  let crafted = handcraft!(Bar { x: Foo { a: 0 } });
  let expected = Bar {
    x: Foo {
      a: 0,
      ..Default::default()
    },
    ..Default::default()
  };

  assert_eq!(crafted, expected);
}

fn another_foo() -> Foo {
  Foo {
    a: 1,
    b: 2,
    ..Default::default()
  }
}

#[test]
fn existing_rest() {
  let crafted = handcraft!(Bar {
    x: Foo { a: 0, ..another_foo() }
  });
  let expected = Bar {
    x: Foo { a: 0, ..another_foo() },
    ..Default::default()
  };

  assert_eq!(crafted, expected);
}

#[derive(Debug, Default, PartialEq)]
struct Baz {
  v: Vec<Foo>,
}

#[test]
fn vec_fields_into() {
  let crafted = handcraft!(Baz {
    v: [Foo { a: 0 }].into()
  });
  let expected = Baz {
    v: vec![Foo {
      a: 0,
      ..Default::default()
    }],
  };

  assert_eq!(crafted, expected);
}

#[test]
fn fail() {
  let t = trybuild::TestCases::new();
  t.compile_fail("tests/fail/*.rs");
}
