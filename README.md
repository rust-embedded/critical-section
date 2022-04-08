# critical-section

[![Documentation](https://docs.rs/critical-section/badge.svg)](https://docs.rs/critical-section)

A critical section that works everywhere!

When writing software for embedded systems, it's common to use a "critical section"
as a basic primitive to control concurrency. A critical section is essentially a 
mutex global to the whole process, that can be acquired by only one thread at a time. 
This can be used to protect data behind mutexes, to [emulate atomics](https://github.com/embassy-rs/atomic-polyfill) in 
targets that don't support them, etc.

There's a wide range of possible implementations depending on the execution environment:
- For bare-metal single core, disabling interrupts globally.
- For bare-metal multicore, acquiring a hardware spinlocks and disabling interrupts globally.
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
- It provides some built-in impls for well-known targets, so in many cases it Just Works.
- It provides a way for any crate to supply a "custom impl" that overrides the built-in one. This allows environment-support crates such as RTOS bindings or HALs for multicore chips to supply the correct impl so that all the crates in the dependency tree automatically use it.

## Built-in impls


| Target             | Mechanism                 | Notes |
|--------------------|---------------------------|-------------------|
| thumbv[6-8]        | `cpsid` / `cpsie`.        | Only sound in single-core privileged mode. |
| riscv32*           | set/clear `mstatus.mie`   | Only sound in single-core privileged mode. |
| std targets        | Global `std::sync::Mutex` |  |

## Providing a custom impl

- Enable the Cargo feature `custom-impl` in the `critical-section` crate.
- Define it like the following:

```rust
struct CriticalSection;
critical_section::custom_impl!(CriticalSection);

unsafe impl critical_section::Impl for CriticalSection {
    unsafe fn acquire() -> u8 {
        // TODO
        return token;
    }

    unsafe fn release(token: u8) {
        // TODO
    }
}
```

If you're writing a library crate that provides a custom impl, it is strongly recommended that
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
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
