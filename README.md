# handcraft

A function-like proc macro that aids building nested structs by hand, when majority of the fields remain default.

## Example

```rust
let crafted = handcraft!(Foo { a: 0 });
```

The macro basically inserts `..Default::default()` for all nested structs. Thus the code above expands to:

```rust
let crafted = Foo {
  a: 0,
  ..Default::default()
};
```

See more in [tests](tests/handcraft.rs).

## Known Issues

When using `handcraft!`, structs in nested macro calls such as `vec!` can't be handled by the outer macro, hence they'll error with "missing field".

This is illustrated by [this fail-to-compile test](tests/fail/vec_fields.rs).

But specifically for `vec!` cases, it can be trivially avoided by changing `vec![...]` to `[...].into()`. As shown by the `vec_fields_into` test.

## Misc

The idea forms when writing a Kubernetes operator with [`kube-rs`](https://github.com/clux/kube-rs), where I need to build several large, nested structs that maps to kubernetes resources.

The code is much inspired by [`autodefault`](https://github.com/Lucretiel/autodefault) but I wanted a different usage style (attribute vs. function-like macro).

I think `autodefault` makes more sense when a function needs to create many small structs. While `handcraft` is better when only a few structs are created but each are complex and/or deeply nested - like those exposed in [`k8s-openapi`](https://github.com/Arnavion/k8s-openapi).
