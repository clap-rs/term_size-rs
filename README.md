term_size
====

[![Crates.io](https://img.shields.io/crates/v/term_size.svg)](https://crates.io/crates/term_size) [![Crates.io](https://img.shields.io/crates/d/term_size.svg)](https://crates.io/crates/term_size) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/kbknapp/term_size-rs/blob/master/LICENSE-MIT) [![license](http://img.shields.io/badge/license-Apache2.0-blue.svg)](https://github.com/kbknapp/term_size-rs/blob/master/LICENSE-APACHE) [![Coverage Status](https://coveralls.io/repos/kbknapp/term_size-rs/badge.svg?branch=master&service=github)](https://coveralls.io/github/kbknapp/term_size-rs?branch=master) [![Join the chat at https://gitter.im/kbknapp/term_size-rs](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/kbknapp/term_size-rs?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Linux: [![Build Status](https://travis-ci.org/kbknapp/term_size-rs.svg?branch=master)](https://travis-ci.org/kbknapp/term_size-rs)
Windows: [![Build status](https://ci.appveyor.com/api/projects/status/ejg8c33dn31nhv36/branch/master?svg=true)](https://ci.appveyor.com/project/kbknapp/term_size-rs/branch/master)

A Rust library to enable getting terminal sizes and dimensions

[Documentation](#platforms-and-documentation)

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
term_size = "0.1"
```

Next, add this to your crate root:

```rust
extern crate term_size;
```

## license

## License

Copyright Benjamin Sago, Kevin Knapp, and `term_size` contributors.

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin my-new-feature`
5. Submit a pull request :D

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
