# fp.js — JavaScript functional core

The **plain JavaScript** implementation of the [fp core](https://github.com/functional-programming-composition/fp).
One file, no build step, no dependencies.

> **Format:** this is an **IIFE that attaches to the global object**, not an ES
> module. It exposes `globalThis.functionalCore`. If you want ESM/CJS with types,
> use the [TypeScript package](../typescript) instead — it is the same algebra
> with a modern module surface. See [TODO.md](TODO.md) for the ESM port.

## Install

Drop it in a page or load it before your code:

```html
<script src="fp.js"></script>
<script>
  const { just, fmap, match } = globalThis.functionalCore;

  const doubled = fmap(just(21), (n) => n * 2);
  console.log(match(doubled, (v) => `Just(${v})`, () => 'Nothing')); // Just(42)
</script>
```

In Node:

```js
require('./fp.js');                       // side-effect: defines globalThis.functionalCore
const { just, fmap } = globalThis.functionalCore;
```

## Method-style carriers

Unlike the TypeScript core's plain tagged unions, this implementation returns
carriers with **methods attached**:

```js
just(21)
  .map((n) => n * 2)
  .chain((n) => just(n + 1))
  .match({ just: (v) => `Just(${v})`, nothing: () => 'Nothing' });
```

Both the standalone functions (`fmap`, `mbind`, `match`) and the methods
(`.map`, `.chain`, `.match`, `.getOrElse`) are available and equivalent.

Because the carriers carry closures, they are **function-valued objects** — keep
them out of anything you serialize (Redux state, actions, `JSON.stringify`,
`postMessage`). Lift plain data into a carrier for a local calculation and
return plain data at the boundary.

## API

`globalThis.functionalCore` exposes 22 members:

### Maybe
`just`, `nothing`, `fmap`, `mbind`, `orElse`, `match`, `isJust`, `isNothing`,
`fromNullable`, `requireJust`

### Either
`left`, `right`, `efmap`, `ebind`, `ematch`, `isLeft`, `isRight`

### Composition and routing
`compose`, `curry`, `createDispatcher`, `multiMatch`, `_`

```js
const { multiMatch, _ } = globalThis.functionalCore;

const tier = multiMatch(order, [
  [(o) => o.total > 500, () => 'priority'],
  [_, () => 'standard'],
]);
```

`createDispatcher` returns a `Maybe`, so an unmatched key is an explicit value
rather than `undefined`.

## When to use this over the TypeScript package

Use `fp.js` when you need a **zero-tooling drop-in** — a static page, a game
prototype, an environment with no bundler. Everywhere else, prefer
[`functional-programming-composition`](../typescript) on npm for types, tree
shaking, and proper module semantics.

## License

MIT — see [NOTICE.md](../NOTICE.md) for provenance.
