# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

No unreleased changes yet

## 0.2.2 - 2021-09-13

- Added support for `wasm` targets.

## 0.2.1 - 2021-05-11

- Added critical section implementation for `std`, based on a global Mutex.

## 0.2.0 - 2021-05-10

- Breaking change: use `CriticalSection<'_>` instead of `&CriticalSection<'_>`

## 0.1.0 - 2021-05-10

- First release
