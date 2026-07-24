# TODO — JavaScript

This core is currently an **IIFE that assigns `globalThis.functionalCore`**. It
has no module exports, so it cannot be `import`ed, bundled, or tree-shaken, and
npm would be the wrong home for it as-is.

## Decide: port or retire

The [TypeScript package](../typescript) is the same algebra with a modern module
surface, types, and an npm release. Two honest options:

- **Port** — convert this to ES modules and keep it as the no-build-step,
  no-TypeScript option for static pages and prototypes.
- **Retire** — delete it and point users at the TypeScript package's `dist/`
  bundle, which already works in a browser.

- [ ] Make the call before doing any of the work below

## If porting to ESM

- [ ] Replace the IIFE wrapper with named exports
      ```js
      export const just = (value) => ({ /* ... */ });
      ```
- [ ] Keep a UMD/IIFE build alongside it so existing `<script>` users don't break
- [ ] Decide whether to keep the **method-style carriers** (`.map`, `.chain`,
      `.getOrElse`, `.match`). They are ergonomic but make carriers
      function-valued, which is why they must never enter serializable state.
      The TypeScript core deliberately uses plain tagged data instead
- [ ] Publish under a distinct npm name (e.g. `functional-composition-js`) or
      as a subpath export of the TypeScript package — **not** a second package
      with the same name

## Parity gaps vs the other cores

This is the smallest core at 232 lines. Missing relative to Rust/GDScript/UE:

- [ ] `trampoline` (`Call`/`Done`) — unbounded iteration without `while`
- [ ] Predicate combinators — `both`, `allPass`, `anyPass`, `complement`
- [ ] `fold` / `filter` / `traverse` collection operations
- [ ] `pipe` (only `compose` exists)
- [ ] `tap` for explicit effect boundaries
- [ ] `memoize` / `lazy`
- [ ] `Validation` (accumulating applicative)

## Tests

- [ ] There are none. Add law tests at minimum — the method-style carriers make
      it easy to accidentally break the monad laws when editing
