# functional-composition

The **Rust** implementation of the [fp core](https://github.com/functional-programming-composition/fp).
`std`-only — **no external crates**. 92 public functions, 2 macros, three carriers.

```toml
[dependencies]
functional-composition = "0.1"
```

```rust
use functional_composition::{just, maybe_map, maybe_match, right, either_match, pipe};

let doubled = maybe_map(&just(21), |n| n * 2);
maybe_match(&doubled, |v| format!("Just({v})"), || "Nothing".to_string()); // "Just(42)"
```

## Native carriers, shared vocabulary

`Maybe<T>` and `Either<E, T>` alias Rust's own `Option<T>` and `Result<T, E>`,
so there is **zero conversion cost** at the boundary and everything in `std`
still works. The named helpers exist so the same vocabulary reads identically
across the TypeScript, GDScript, and Unreal cores.

```rust
pub type Maybe<T> = Option<T>;
```

## API by area

### Maybe / Either

`just`, `nothing`, `is_just`, `is_nothing`, `maybe_map`, `maybe_chain`,
`maybe_filter`, `maybe_match`, `maybe_or_else`, `from_nullable`, `mbind`,
`maybe_fmap`, `or_else`, `require_just`, `maybe_from_option`, `maybe_to_option`
· `left`, `right`, `make_left`, `make_right`, `either_map`, `either_map_left`,
`either_chain`, `either_match`, `either_or_else`, `ebind`, `either_fmap`,
`ematch`, `is_left`, `is_right`

### Composition and arity

`pipe`, `compose`, `pipe!`, `compose!` (macros), `curry2`, `curry3`,
`partial_apply`, `partial_apply2`, `identity`, `constant`, `flip`, `juxt2`,
`converge2`

The `pipe!` and `compose!` macros avoid the nested-call plumbing that hand-rolled
composition produces:

```rust
use functional_composition::pipe;
let result = pipe!(value, parse, validate, transform);
```

### Predicate combinators

`both`, `either_pred`, `all_pass`, `any_pass`, `complement`, `equals`

Compose named predicates instead of inlining boolean logic into a branch:

```rust
let admissible = both(is_alive, is_on_screen);
```

### Routing

`create_dispatcher`, `create_optional_dispatcher`, `dispatcher_register`,
`dispatcher_dispatch`, `dispatch`, `match_case`, `multi_match`

`Dispatcher<K, A, R>` is a typed key → handler table; `multi_match` evaluates
ordered predicate/handler pairs with a wildcard fallthrough. Together they
replace `match` blocks that route on runtime values.

### Trampoline — iteration without `loop`

`Bounce<T>` (`Bounce::Call` / `Bounce::Done`) plus `trampoline` express
unbounded iteration as data, keeping it stack-safe without a `while`:

```rust
use functional_composition::{trampoline, Bounce};

fn sum_to(n: u64, acc: u64) -> Bounce<u64> {
    match n { 0 => Bounce::Done(acc), _ => Bounce::Call(Box::new(move || sum_to(n - 1, acc + n))) }
}
let total = trampoline(sum_to(1_000_000, 0));
```

Rust does not guarantee tail-call elimination, so this is the portable way to
run unbounded recursion.

### Effects, laziness, memoization

`tap`, `tap_mut`, `lazy`, `eval`, `memoize`, `memoize_last`

`tap`/`tap_mut` mark an effect inside a pipeline *explicitly* — a declaration
that something impure happens here, not a loophole.

### Validation, config, async, HTTP

`add_validation`, `validation_pipeline` · `config_builder`, `build_config` ·
`create_async_result`, `async_execute`, `async_then`, `async_catch` ·
`http_success`, `http_failure`

## Build and test

```bash
cargo check
cargo test    # 35 passing
```

This is the only core in the repository that arrived **with its tests** — they
were inline in the source `functional.rs` as a `#[cfg(test)] mod tests`, so they
came across with the extraction and cover the carriers, composition, predicate
combinators, the trampoline, and the validation pipeline. The other languages'
suites still need migrating; see each directory's `TODO.md`.

Doc-tests are currently empty, so the examples above and in `TODO.md` are
unverified — that is the next gap to close.

See [TODO.md](TODO.md) for the crates.io release path.

## License

MIT — see [NOTICE.md](../NOTICE.md) for provenance.
