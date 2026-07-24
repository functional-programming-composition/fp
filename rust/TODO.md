# TODO — Rust

## Publish to crates.io

The crate is `functional-composition` (name verified available). `cargo check`
passes with zero external dependencies.

- [ ] Create a crates.io account and log in
      ```bash
      cargo login          # paste the token from https://crates.io/settings/tokens
      ```
- [ ] Fill in the remaining manifest fields if desired — `documentation`,
      `rust-version` (MSRV), `exclude` for `target/`
- [ ] Dry-run and inspect the package contents
      ```bash
      cargo package --list     # what ships
      cargo package            # builds the .crate and verifies it compiles clean
      ```
- [ ] Publish
      ```bash
      cargo publish
      ```
- [ ] Verify from a scratch project
      ```bash
      cargo new /tmp/fpcheck && cd /tmp/fpcheck
      cargo add functional-composition
      cargo check
      ```

**Irreversible:** crates.io versions cannot be deleted, only *yanked* (which
hides them from new resolution but keeps existing lockfiles working). The crate
**name is claimed permanently** on first publish.

## Before the first release

- [ ] Add `#![doc = ...]` crate-level docs so docs.rs renders a landing page
- [ ] Add doctests to the public functions — they double as law tests
- [ ] Decide on `no_std` support (the `categories` field currently claims
      `no-std`; either honor it with `#![no_std]` + `alloc`, or drop the claim)
- [ ] Confirm the two exported macros (`pipe!`, `compose!`) are `#[macro_export]`
      and documented
- [ ] Set an MSRV and test it in CI

## Parity

- [ ] Port `Validation` (accumulating applicative) if it is missing here — the
      Unreal core has a dedicated `Validation` module and the TypeScript core
      does not yet
- [ ] Add law tests (Functor, Monad, Monoid, Applicative) mirroring the other cores
