# TODO — TypeScript

**Published:** [`functional-programming-composition@0.1.0`](https://www.npmjs.com/package/functional-programming-composition)
on npm. Verified end-to-end with a clean `npm install` + `import` in both ESM
and CJS.

## Release process

```bash
npm version patch          # or minor / major
npm run build              # tsup -> dist/
npm publish --access public
```

`prepublishOnly` runs the build, so the tarball can't ship a stale `dist/`.

- [ ] Set up npm **Trusted Publishing** (OIDC from GitHub Actions) instead of a
      long-lived token. npm is restricting 2FA-bypass tokens — account changes
      from **Aug 2026**, direct publishing from **Jan 2027**
- [ ] Add a release workflow so publishing is reproducible rather than local

## Migrate the tests from the source repo

**The tests were left behind in the extraction.** They exist in the ForbocAI SDK
and cover this exact core — port them over rather than writing new ones:

| Source (`Forboc.AI/sdk/packages/core/src/__tests__/`) | Lines | Covers |
| --- | --- | --- |
| `fpMaybeEither.test.ts` | 246 | `Maybe` / `Either` construction, `fmap`, `mbind`, `match` |
| `fpComposition.test.ts` | 217 | `compose`, `curry`, dispatcher, `multiMatch` |
| `fpNullable.test.ts` | 83 | `fromNullable` / `requireJust` boundary lifts |

- [ ] Copy the three files into `src/__tests__/`
- [ ] Rewrite their imports — they import from the SDK's internal paths, and
      here everything comes from `../index`
- [ ] Add a test runner (vitest) and a `test` script; wire it into
      `prepublishOnly` so a broken core can't be published
- [ ] Check for assertions that depend on the old data-driven carrier tags from
      `data/fp/runtime.json`; those values are inlined now and unchanged, so they
      should pass as-is, but verify rather than assume

## Before 1.0

- [ ] Add **law tests** on top of the migrated suite — Functor identity and
      composition, Monad left/right identity and associativity. The source tests
      cover behavior; the laws are what make the abstraction worth having
- [ ] Add `Validation<E, T>` — accumulating `ap`, deliberately **no** `chain`.
      This is the biggest gap: independent validation currently has to misuse
      `Either`, which reports one error and discards the rest. The Unreal core
      already has a `Validation` module
- [ ] Add `Monoid` (`concat` + `empty`, `foldMap`) and `traverse`/`sequence` as
      first-class exports
- [ ] Consider `trampoline` for parity with the Rust and GDScript cores
- [ ] Document the `multiMatch` return type — it returns a `Maybe`, which
      surprises callers expecting the raw handler result

## Packaging polish

- [ ] Add `provenance: true` to the publish (requires Trusted Publishing) so npm
      shows a verified build attestation
- [ ] Consider `exports` subpaths (`./maybe`, `./either`) if the surface grows
- [ ] Verify tree-shaking actually drops unused modules in a real bundler
      (`sideEffects: false` is set, but untested)

## Consuming from the ForbocAI SDK

The original motivation: depend on this package instead of vendoring the core.

- [ ] Replace `packages/core/src/core/fp/**` with a dependency on
      `functional-programming-composition`
- [ ] Re-point the SDK's `core/fp.ts` aggregate at the package
- [ ] Note the one behavioral difference: the SDK version read carrier tags from
      `data/fp/runtime.json`; this package inlines them. If the SDK relied on
      re-tagging at runtime, that capability is gone and needs a decision
- [ ] Update `check-fp-conformance.sh` — it checks the core's export surface and
      will need to follow the dependency instead of a local file
