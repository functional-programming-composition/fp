# fp.gd — GDScript functional core

The **GDScript** implementation of the [fp core](https://github.com/functional-programming-composition/fp),
for **Godot 4**. A single file, 85 static functions, no dependencies.

## Install

Copy `fp.gd` into your project and call through the class:

```gdscript
const FP = preload("res://fp.gd")

var doubled = FP.maybe_map(FP.just(21), func(n): return n * 2)
print(FP.maybe_match(doubled, func(v): return "Just(%d)" % v, func(): return "Nothing"))
# Just(42)
```

Every function is `static`, so there is nothing to instantiate and no node to
add to the tree.

## Tagged dictionaries

GDScript has no sum types, so carriers are tagged `Dictionary` values:

```gdscript
{ "_tag": "Just", "value": 42 }
{ "_tag": "Nothing" }
{ "_tag": "Left",  "error": "..." }
{ "_tag": "Right", "value": ... }
```

Because the host type system is dynamic, the shape is a runtime contract rather
than a compile-time one — an arbitrary dictionary can violate it. Construct
carriers only through `just`, `nothing`, `left`, and `right`, and read them only
through the `*_match` helpers, so the tag stays trustworthy.

## API by area

### Maybe / Either

`just`, `nothing`, `is_just`, `is_nothing`, `maybe_map`, `maybe_chain`,
`maybe_filter`, `maybe_match`, `maybe_or_else`, `maybe_or_else_lazy`,
`maybe_to_nullable`, `from_nullable`, `from_nullable_value`, `require_just`
· `left`, `right`, `is_left`, `is_right`, `either_map`, `either_map_left`,
`either_chain`, `either_match`, `either_or_else`, `either_pred`

### Composition and arity

`pipe`, `pipe3`, `pipe4`, `compose`, `compose_many`, `create_pipeline`,
`pipeline`, `curry2`, `curry3`, `partial_apply`, `partial_apply2`, `identity`,
`constant`, `flip`, `juxt2`, `converge2`

GDScript exposes fixed `curry2`/`curry3` rather than a variadic curry —
runtime arity reflection would weaken the contract.

### Predicate combinators

`both`, `all_pass`, `any_pass`, `complement`, `equals`, `bool_match`

```gdscript
var is_renderable = FP.both(is_alive, is_on_screen)
```

### Routing

`create_dispatcher`, `create_dispatcher_from`, `create_dispatcher_with_table`,
`match_case`, `match_case_accepts`, `multi_match`, `multi_match_maybe`,
`wildcard`

`multi_match` evaluates ordered predicate/handler pairs and falls through to a
wildcard. Note the evaluation contract: this implementation selects the first
result **after filtering**, so every predicate runs — keep predicates pure and
cheap.

### Trampoline — iteration without `while`

`bounce_call`, `bounce_done`, `trampoline`

```gdscript
func sum_to(n, acc):
    return FP.bounce_done(acc) if n == 0 else FP.bounce_call(func(): return sum_to(n - 1, acc + n))

var total = FP.trampoline(sum_to(1000, 0))
```

### Collections

`fmap_array`, `filter_array`, `fold_array`, `merge_dict`

Use these instead of `for` loops over arrays.

### Effects, laziness, memoization

`tap`, `tap_mut`, `lazy`, `eval_lazy`, `memoize`, `memoize_last`

Memoization here uses **deep value comparison** for keys, which differs from the
reference-identity caches in the TypeScript core — the same-looking code can
produce different hit rates. Decide deliberately.

### Validation, config, async, HTTP, test helpers

`validation_pipeline` · `config_builder` · `create_async_result`,
`async_chain_then` · `http_success`, `http_failure` · `test_success`,
`test_failure`, `test_get_value`, `test_try_get_value`, `test_with_detail`

## Godot notes

- Callables are passed with `func(x): return ...` lambda syntax (Godot 4).
- Keep `Callable` values out of anything you serialize to disk or send over the
  wire — they are host-only.

See [TODO.md](TODO.md) for the Asset Library submission path.

## License

MIT — this core was already MIT at its source. See [NOTICE.md](../NOTICE.md).
