# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.0.0-alpha.2 - 2022-07-28

- Change name of the `extern fn`s to avoid clash with critical-section 0.2.

## 1.0.0-alpha.1 - 2022-07-28

Breaking changes:

- Removed all builtin impls. These are going to be provided by platform-support crates now.
- Renamed `custom_impl!` to `set_impl!`.
- RestoreState is now an opaque struct for the user, and a transparent `RawRestoreState` type alias for impl writers.
- RestoreState type is now configurable with Cargo features. Default is `()`. (previously it was fixed to `u8`.)
- Added own `CriticalSection` and `Mutex` types, instead of reexporting them from `bare_metal`.

## 0.2.7 - 2022-04-08

- Add support for AVR targets.

## 0.2.6 - 2022-04-02

- Improved docs.

## 0.2.5 - 2021-11-02

- Fix `std` implementation to allow reentrant (nested) critical sections. This would previously deadlock.

## 0.2.4 - 2021-09-24

- Add support for 32bit RISC-V targets.

## 0.2.3 - 2021-09-13

- Use correct `#[cfg]` for `wasm` targets.

## 0.2.2 - 2021-09-13

- Added support for `wasm` targets.

## 0.2.1 - 2021-05-11

- Added critical section implementation for `std`, based on a global Mutex.

## 0.2.0 - 2021-05-10

- Breaking change: use `CriticalSection<'_>` instead of `&CriticalSection<'_>`

## 0.1.0 - 2021-05-10

- First release
