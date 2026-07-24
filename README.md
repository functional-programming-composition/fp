# fp

[![skills.sh](https://skills.sh/b/functional-programming-composition/fp)](https://skills.sh/functional-programming-composition/fp)

A functional programming **working contract** for coding agents — not an
introduction. It assumes the decision to use FP is settled and governs *how*.

Language-independent: the same algebra applies in C, Rust, TypeScript, and
others. Examples are in TypeScript; only the naming adapts to host convention.

## Install

```bash
npx skills@latest add functional-programming-composition/fp
```

Or use it without installing:

```bash
npx skills@latest use functional-programming-composition/fp | claude
```

<details>
<summary>More install options</summary>

```bash
# Install globally (available across all projects)
npx skills@latest add functional-programming-composition/fp -g

# Install for a specific agent, non-interactively
npx skills@latest add functional-programming-composition/fp -a claude-code -g -y

# List what the repo provides without installing
npx skills@latest add functional-programming-composition/fp --list
```

Works with Claude Code, OpenCode, Codex, Cursor, and 70+ other agents via the
[`skills`](https://github.com/vercel-labs/skills) CLI.

</details>

## The cores

The same algebra, implemented in five languages. Semantics are identical; only
the naming adapts to host convention. See [NOTICE.md](NOTICE.md) for provenance.

| Language | Directory | Distribution | Notes |
| --- | --- | --- | --- |
| **TypeScript** | [`typescript/`](typescript) | ✅ [npm: `functional-programming-composition`](https://www.npmjs.com/package/functional-programming-composition) | Standalone, zero deps, ESM + CJS + types |
| **Rust** | [`rust/`](rust) | ⏳ crates.io — see [TODO](rust/TODO.md) | `std`-only, 92 fns + `pipe!`/`compose!` macros |
| **GDScript** | [`gdscript/`](gdscript) | ⏳ Asset Library / gd-plug — see [TODO](gdscript/TODO.md) | Godot 4, 85 static fns, tagged dictionaries |
| **Unreal C++** | [`ue-cpp/`](ue-cpp) | ⏳ Fab; vcpkg/Conan/CPM after port — see [TODO](ue-cpp/TODO.md) | C++11, 29 headers, `namespace func`. **Needs Unreal** |
| **JavaScript** | [`javascript/`](javascript) | ⏳ needs ESM port — see [TODO](javascript/TODO.md) | IIFE → `globalThis.functionalCore`; **not** a module |

Each directory has its own README documenting that language's actual API and
constraints, and a TODO with its publishing path.

```bash
# TypeScript
npm install functional-programming-composition

# Rust
cargo add functional-composition
```

```ts
import { just, fmap, match, right, ematch, compose } from 'functional-programming-composition'

match(fmap(just(21), (n) => n * 2), (v) => `Just(${v})`, () => 'Nothing')  // "Just(42)"
```

## What it covers

| Area | Contents |
| --- | --- |
| **Error model** | `Maybe`, `Either`, `Validation` — choosing deliberately, and why collapsing everything into `Nothing` loses information |
| **The ladder** | Functor → Applicative → Monad → Monoid → Traversable, with a decision tree for picking the *weakest* abstraction that solves the problem |
| **Applicative vs Monad** | The most-violated distinction: independent checks accumulate (`ap`), dependent steps short-circuit (`chain`) |
| **Branching** | `match` on shape, dispatch tables on key, predicate combinators on properties — and why a ternary chain is just an `if` that learned to hide |
| **Loops** | Bounded iteration is a fold; unbounded iteration is a trampoline (`Call`/`Done`) |
| **Composition** | `pipe`/`compose`, currying, partial application, effects at the edges |
| **Arity** | Two data parameters maximum — split the function, don't bundle args into an options object |
| **Laws** | Functor, Monad, Monoid, Applicative — every new primitive ships with law tests |

## The non-negotiables

1. **Zero classes** in domain logic — factories returning data plus closures.
2. **At most two data parameters** — over-arity is fixed by splitting, not bundling.
3. **No statement `if`/`for`/`while`/`switch`** in the pure core.
4. **The functional core depends on nothing** — everything may depend on it.
5. **No lazy wrapper nouns** — `Manager`, `Helper`, `Util(s)`, `Bag`.

## Canon

Guidance is judged against three bodies of work, in this order for the question
at hand:

| Source | Governs |
| --- | --- |
| **Philip Wadler** | Laws, algebraic structure, types |
| **Eric Elliott** | Composition, factories, arity |
| **James Sinclair** | Practical monads, effects, optics |

Generic "what is a monad" material and anti-dogma takes are explicitly out of
scope — if a source stops at explaining what a monad *is*, it is not governing
how to build one.

## License

MIT
