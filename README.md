# glam

[![Build Status]][travis-ci] [![Coverage Status]][coveralls.io]
[![Latest Version]][crates.io] [![docs]][docs.rs]

A simple and fast 3D math library for games and graphics.

## Development status

`glam` is in beta stage. Base functionality has been implemented and the look
and feel of the API has solidified.

## Features

* `f32` types
  * vectors: `Vec2`, `Vec3`, `Vec3A` and `Vec4`
  * square matrices: `Mat2`, `Mat3` and `Mat4`
  * a quaternion type: `Quat`
* `f64` types
  * vectors: `DVec2`, `DVec3` and `DVec4`
  * square matrices: `DMat2`, `DMat3` and `DMat4`
  * a quaternion type: `DQuat`
* `i32` types
  * vectors: `IVec2`, `IVec3` and `IVec4`
* `u32` types
  * vectors: `UVec2`, `UVec3` and `UVec4`
* `bool` types
  * vectors: `BVec2`, `BVec3` and `BVec4`

### SIMD

The `Vec3A`, `Vec4` and `Quat` types use SSE2 on x86/x86_64 architectures.
`Mat2`, `Mat3` and `Mat4` also use SSE2 for some functionality. Not everything
has a SIMD implementation yet.

Note that this does result in some wasted space in the case of `Vec3A` as the
SIMD vector type is 16 bytes large and 16 byte aligned.

`glam` outperforms similar Rust libraries for common operations as tested by the
[`mathbench`][mathbench] project.

[mathbench]: https://github.com/bitshifter/mathbench-rs

### `no_std` support

`no_std` support can be enabled by compiling with `--no-default-features` to
disable `std` support and `--features libm` for math functions that are only
defined in `std`. For example:

```toml
[dependencies]
glam = { version = "0.13.0", default-features = false, features = ["libm"] }
```

To support both `std` and `no_std` builds in project, you can use the following
in your `Cargo.toml`:

```toml
[features]
default = ["std"]

std = ["glam/std"]
libm = ["glam/libm"]

[dependencies]
glam = { version = "0.13.0", default-features = false }
```

### Optional features

* [`bytemuck`] - for casting into slices of bytes
* [`libm`] - required to compile with `no_std`
* [`mint`] - for interoperating with other 3D math libraries
* [`num-traits`] - required to compile `no_std`, will be included when enabling
  the `libm` feature
* [`rand`] - implementations of `Distribution` trait for all `glam` types. This
  is primarily used for unit testing
* [`serde`] - implementations of `Serialize` and `Deserialize` for all `glam`
  types. Note that serialization should work between builds of `glam` with and
  without SIMD enabled

[`bytemuck`]: https://docs.rs/bytemuck
[`libm`]: https://github.com/rust-lang/libm
[`mint`]: https://github.com/kvark/mint
[`num-traits`]: https://github.com/rust-num/num-traits
[`rand`]: https://github.com/rust-random/rand
[`serde`]: https://serde.rs

### Feature gates

* `scalar-math` - compiles with SIMD support disabled
* `debug-glam-assert` - adds assertions in debug builds which check the validity
  of parameters passed to `glam` to help catch runtime errors
* `glam-assert` - adds validation assertions to all builds

### Minimum Supported Version of Rust (MSVR)

The minimum supported version of Rust for `glam` is `1.45.0`.

## Conventions

### Column vectors

`glam` interprets vectors as column matrices (also known as "column vectors")
meaning when transforming a vector with a matrix the matrix goes on the left,
e.g. `v' = Mv`.  DirectX uses row vectors, OpenGL uses column vectors. There
are pros and cons to both.

### Column-major order

Matrices are stored in column major format. Each column vector is stored in
contiguous memory.

### Co-ordinate system

`glam` is co-ordinate system agnostic and intends to support both right handed
and left handed conventions.

## Design Philosophy

The design of this library is guided by a desire for simplicity and good
performance.

* No generics and minimal traits in the public API for simplicity of usage
* All dependencies are optional (e.g. `mint`, `rand` and `serde`)
* Follows the [Rust API Guidelines] where possible
* Aiming for 100% test [coverage]
* Common functionality is benchmarked using [Criterion.rs]

[Rust API Guidelines]: https://rust-lang-nursery.github.io/api-guidelines/
[coverage]: coveralls.io
[Criterion.rs]: https://bheisler.github.io/criterion.rs/book/index.html

## Future work

* Experiment with a using a 4x3 matrix as a 3D transform type that can be more
  efficient than `Mat4` for certain operations like inverse and multiplies
* `wasm` support

## Inspirations

There were many inspirations for the interface and internals of glam from the
Rust and C++ worlds. In particular:

* [How to write a maths library in 2016] inspired the initial `Vec3A`
  implementation
* [Realtime Math] - header only C++11 with SSE and NEON SIMD intrinsic support
* [DirectXMath] - header only SIMD C++ linear algebra library for use in games
  and graphics apps
* `glam` is a play on the name of the popular C++ library [GLM]

[How to write a maths library in 2016]: http://www.codersnotes.com/notes/maths-lib-2016/
[Realtime Math]: https://github.com/nfrechette/rtm
[DirectXMath]: https://docs.microsoft.com/en-us/windows/desktop/dxmath/directxmath-portal
[GLM]: https://glm.g-truc.net

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT)
  or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Contributions in any form (issues, pull requests, etc.) to this project must
adhere to Rust's [Code of Conduct].

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Thank you to all of the `glam` [contributors]!

[Code of Conduct]: https://www.rust-lang.org/en-US/conduct.html
[contributors]: https://github.com/bitshifter/glam-rs/graphs/contributors

## Support

If you are interested in contributing or have a request or suggestion
[start a discussion] on github. See [CONTRIBUTING.md] for more information for
contributors.

The [Game Development in Rust Discord] and [Bevy Engine Discord] servers can are
also good places to ask for help with `glam`.

[start a discussion]: https://github.com/bitshifter/glam-rs/discussions
[CONTRIBUTING.md]: CONTRIBUTING.md
[Game Development in Rust Discord]: https://discord.gg/yNtPTb2
[Bevy Engine Discord]: https://discord.gg/gMUk5Ph

## Attribution

`glam` contains code ported from the following C++ libraries:

* [DirectXMath] - MIT License - Copyright (c) 2011-2020 Microsoft Corp
* [Realtime Math] - MIT License - Copyright (c) 2018 Nicholas Frechette
* [GLM] - MIT License - Copyright (c) 2005 - G-Truc Creation

See [ATTRIBUTION.md] for details.

[ATTRIBUTION.md]: ATTRIBUTION.md

[Build Status]: https://travis-ci.org/bitshifter/glam-rs.svg?branch=master
[travis-ci]: https://travis-ci.org/bitshifter/glam-rs
[Coverage Status]: https://coveralls.io/repos/github/bitshifter/glam-rs/badge.svg?branch=master
[coveralls.io]: https://coveralls.io/github/bitshifter/glam-rs?branch=master
[Latest Version]: https://img.shields.io/crates/v/glam.svg
[crates.io]: https://crates.io/crates/glam/
[docs]: https://docs.rs/glam/badge.svg
[docs.rs]: https://docs.rs/glam/
