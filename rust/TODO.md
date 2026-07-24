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

## Tests

Unlike the other cores, the Rust tests **came across with the extraction** —
`functional.rs` carried its `#[cfg(test)] mod tests` inline, so `src/lib.rs`
already has **35 tests and they all pass**:

```bash
cargo test    # 35 passed; 0 failed
```

Still outstanding from the source repo:

- [ ] Check `magik-lux/core/tests/bdd.rs` and `core/tests/steps/` for
      integration coverage that exercises the functional core from outside the
      crate — that lives in a separate `tests/` directory and did **not** come
      across with `functional.rs`
- [ ] Port anything there that is genuinely about the core (as opposed to
      magik-lux game logic) into a `rust/tests/` directory here

## Before the first release

- [ ] Add `#![doc = ...]` crate-level docs so docs.rs renders a landing page
- [ ] Add doctests to the public functions — `cargo test` currently reports
      **0 doc-tests**, so every example in the README is unverified
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
