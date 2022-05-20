# mem-layout
A crate to help with keeping track of in-memory data structures

## Usage

1. Add it to your `Cargo.toml`, like so:
```toml
[dependencies]
mem-layout = { git = "https://github.com/blu-dev/mem-layout" }
```

2. Import `mem_layout::TypeAssert` and begin creating your data structures with size and offset assertion auto-generated tests:
```rs
use mem_layout::TypeAssert;

#[repr(C)]
#[derive(TypeAssert)]
#[ta(size = 0x20)]
pub struct ExampleType {
  #[ta(off = 0x00)] x: f32,
  #[ta(off = 0x04)] y: f32,
  padding: mem_layout::ByteView<8>,
  #[ta(off = 0x10)] something_else: mem_layout::ByteView<16>
}
```

3. Run `cargo test` to ensure that your structs are valid!

## `ByteView`
`ByteView` is a const-type generic type that functionally serves as an array of bytes, but also provides helper utilities for viewing it
as other types. For example, if you are unsure about what kind of underlying type is inside of a larger type, you could replace it with a `ByteView`
with the appropriate size and then view it as various different types:

```rs
#[repr(C)]
pub struct UnknownType {
  pub underlying: ByteView<64>
}

pub fn some_function(unk: &UnknownType) {
  println!("{:?}", unk.underlying.view_as::<f32>().unwrap());
  println!("{:?}", unk.underlying.view_as::<u32>().unwrap());
}
```

There are two main functions for this: `view_as` and `view_as_unchecked` (with mutable variants). `view_as` performs size checking on the size of
the specified type and alignment checking on the alignment of the type and the address of the `ByteView`. If you know that you are violating
these rules, then you may call `view_as_unchecked` to see the data regardless of alignment.
