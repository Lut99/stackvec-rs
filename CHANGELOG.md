# CHANGELOG
This file keeps track of changes to the `stackvec` Rust crate.

This project uses [semantic versioning](https://semver.org). As such, breaking changes are indicated with **\[breaking\]**.


## v0.2.0 (2024-04-16)
### Added 
- A `Deref<Target = [u8]>` implementation for the `StackVec`.
    - This should give it a great deal of implementations for convenient slice operations.
    - **Note**: Some functions have now moved to the slice implementations. Should not be breaking, but be aware.
- A `From<StackVec>` implementation for `Vec`s.


## v0.1.0 (2024-04-05)
Initial release!

### Added
- The `StackVec`, a `Vec`-like data structure that lives on the stack.
