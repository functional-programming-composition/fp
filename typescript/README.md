# functional-programming-composition

[![npm](https://img.shields.io/npm/v/functional-programming-composition)](https://www.npmjs.com/package/functional-programming-composition)

The **TypeScript** implementation of the [fp core](https://github.com/functional-programming-composition/fp).
Standalone, **zero runtime dependencies**, ships ESM + CJS + type declarations.

```bash
npm install functional-programming-composition
```

```ts
import { just, fmap, match, right, ematch, compose } from 'functional-programming-composition'

match(fmap(just(21), (n) => n * 2), (v) => `Just(${v})`, () => 'Nothing') // "Just(42)"
ematch(right(7), (e) => `Left(${e})`, (v) => `Right(${v})`)               // "Right(7)"
compose((n: number) => n + 1, (n: number) => n * 2)(5)                    // 11
```

## Carriers are plain tagged data

`Maybe` and `Either` are discriminated unions carrying a `_tag`, so they stay
serializable and narrowable. Keep them out of serializable state (a Redux slice
holds plain data) and lift at the selector or reducer edge with `fromNullable`:

```ts
import { fromNullable, mbind, match } from 'functional-programming-composition'

const discountFor = (user: User): number =>
  match(
    mbind(fromNullable(user.discountCode), lookupDiscount),
    (rate) => rate,
    () => 0,
  )
```

That single pipeline replaces a nested `if (x != null)` ladder.

## API

### Maybe

| Function | Signature |
| --- | --- |
| `just` | `<T>(value: T) => Maybe<T>` |
| `nothing` | `<T>() => Maybe<T>` |
| `fmap` | `<A, B>(m: Maybe<A>, f: (a: A) => B) => Maybe<B>` |
| `mbind` | `<A, B>(m: Maybe<A>, f: (a: A) => Maybe<B>) => Maybe<B>` |
| `match` | `<T, R>(m, onJust: (v: T) => R, onNothing: () => R) => R` |
| `orElse` | `<T>(m: Maybe<T>, fallback: T) => T` |
| `isJust` / `isNothing` | `<T>(m: Maybe<T>) => boolean` |
| `fromNullable` | `<T>(v: T \| null \| undefined) => Maybe<T>` |
| `requireJust` | `<T>(m, message) => T` — throws; boundary assertion only |

### Either

| Function | Signature |
| --- | --- |
| `left` / `right` | `(v) => Either<E, T>` |
| `efmap` | `<E, A, B>(e, f: (a: A) => B) => Either<E, B>` |
| `ebind` | `<E, A, B>(e, f: (a: A) => Either<E, B>) => Either<E, B>` |
| `ematch` | `<E, T, R>(e, onLeft: (e: E) => R, onRight: (t: T) => R) => R` |
| `isLeft` / `isRight` | `<E, T>(e) => boolean` |

### Composition, collections, routing

| Function | Purpose |
| --- | --- |
| `compose` | Right-to-left function composition |
| `curry` | Fix arity; capture stable inputs |
| `fold` / `filter` / `traverse` | Collection operations — use these instead of `for` |
| `createDispatcher` | Key → handler table; returns `Maybe` so a miss is explicit |
| `multiMatch` | Ordered predicate/handler cases with a wildcard |
| `_` | Wildcard sentinel for `multiMatch` |

Exported types: `Maybe<T>`, `Either<E, T>`, `Dispatcher`, `Predicate`.

## Branching without `if`

Route by **shape** with `match`, by **key** with `createDispatcher`, by
**predicate** with `multiMatch`:

```ts
import { multiMatch, _ } from 'functional-programming-composition'

const tier = multiMatch(order, [
  [(o: Order) => o.total > 500, () => 'priority'],
  [_, () => 'standard'],
])
```

`createDispatcher` and `multiMatch` both return a `Maybe`, so an unmatched key
is a value you handle rather than an exception or `undefined`.

## Standalone note

Extracted from the ForbocAI SDK, where it loaded carrier tags and
match/composition constants from a host `data/fp/runtime.json`. Those values are
inlined in `src/runtime/runtimeAdapters.ts` so this package stands alone — shape
and values unchanged, so code written against the data-driven version still
type-checks here.

## Build

```bash
npm install
npm run typecheck   # tsc --noEmit
npm run build       # tsup -> dist/{index.js, index.cjs, index.d.ts}
```

Targets ES2020 with `moduleResolution: Bundler` and `strict: true`, matching the
source SDK's compiler settings.

See [TODO.md](TODO.md) for release follow-ups.

## License

MIT — see [NOTICE.md](../NOTICE.md) for provenance.
