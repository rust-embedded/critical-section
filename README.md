# critical-section hello
[![crates.io](https://img.shields.io/crates/d/critical-section.svg)](https://crates.io/crates/critical-section)
[![crates.io](https://img.shields.io/crates/v/critical-section.svg)](https://crates.io/crates/critical-section)
[![Documentation](https://docs.rs/critical-section/badge.svg)](https://docs.rs/critical-section)

This project is developed and maintained by the [HAL team][team].

A critical section that works everywhere!

When writing software for embedded systems, it's common to use a "critical section"
as a basic primitive to control concurrency. A critical section is essentially a 
mutex global to the whole process, that can be acquired by only one thread at a time. 
This can be used to protect data behind mutexes, to [emulate atomics](https://github.com/embassy-rs/atomic-polyfill) in 
targets that don't support them, etc.

There's a wide range of possible implementations depending on the execution environment:
- For bare-metal single core, disabling interrupts globally.
- For bare-metal multicore, acquiring a hardware spinlock and disabling interrupts globally.
- For bare-metal using a RTOS, it usually provides library functions for acquiring a critical section, often named "scheduler lock" or "kernel lock".
- For bare-metal running in non-privileged mode, usually some system call is needed.
- For `std` targets, acquiring a global `std::sync::Mutex`.

Libraries often need to use critical sections, but there's no universal API for this in `core`. This leads
library authors to hardcode them for their target, or at best add some `cfg`s to support a few targets.
This doesn't scale since there are many targets out there, and in the general case it's impossible to know
which critical section impl is needed from the Rust target alone. For example, the `thumbv7em-none-eabi` target
could be cases 1-4 from the above list.

This crate solves the problem by providing this missing universal API.

- It provides functions `acquire`, `release` and `free` that libraries can directly use.
- It provides a way for any crate to supply an implementation. This allows "target support" crates such as architecture crates (`cortex-m`, `riscv`), RTOS bindings, or HALs for multicore chips to supply the correct impl so that all the crates in the dependency tree automatically use it.

## Providing an implementation

```rust
use critical_section::RawRestoreState;

struct MyCriticalSection;
critical_section::set_impl!(MyCriticalSection);

unsafe impl critical_section::Impl for MyCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        // TODO
    }

    unsafe fn release(token: RawRestoreState) {
        // TODO
    }
}
```

If you're writing a library crate that provides an impl, it is strongly recommended that
you only provide it if explicitly enabled by the user via a Cargo feature `critical-section-impl`.
This allows the user to opt out from your impl to supply their own. 

## Why not generics?

An alternative solution would be to use a `CriticalSection` trait, and make all
code that needs acquiring the critical section generic over it. This has a few problems:

- It would require passing it as a generic param to a very big amount of code, which
would be quite unergonomic.
- It's common to put `Mutex`es in `static` variables, and `static`s can't 
be generic.
- The user can mix different critical section implementations in the same program,
which would be unsound.

## License

This work is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [Cortex-M team][team], promises
to intervene to uphold that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-hal-team