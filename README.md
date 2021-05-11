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

## Misc

The idea forms when writing a Kubernetes operator with [`kube-rs`](https://github.com/clux/kube-rs).

The code is much inspired by [`autodefault`](https://github.com/Lucretiel/autodefault) but I wanted a different usage style (attribute vs. function-like macro).

I think `autodefault` makes more sense when a function needs to create many small structs. While `handcraft` is better when only a few structs are created but each are complex and/or deeply nested - like those exposed in [`k8s-openapi`](https://github.com/Arnavion/k8s-openapi).
