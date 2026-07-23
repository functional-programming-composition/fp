## Functional Core — Pure FP primitives for GDScript
##
## User Stories:
##   - As a feature author, I need Maybe and Either so optional values and
##     recoverable failures compose without sentinel state.
##   - As a reducer, selector, or system author, I need pipe, compose, curry,
##     memoization, pattern matching, validation, and boundary helpers so
##     domain logic stays explicit and testable.
##   - As a maintainer, I need FP to remain the dependency root so RTK and ECS
##     can share primitives without circular architecture.
##
## Complete port of the FP substrate shared across the Forboc ecosystem:
##   - magik-lux/core/src/functional.rs      (Rust)
##   - sdk/packages/core/src/functional_core.ts (TypeScript)
##   - sdk-ue-5/.../functional_core.hpp  (C++11)
##
## ZERO CLASSES. Every construct uses factory functions returning Dictionaries
## with closures. This guarantees no OOP inheritance, no `new()`, no `self`.
## Every function here is referentially transparent.
##
## Design Principles:
##   - Prefer Dictionaries and plain data for domain state.
##   - Prefer factory functions for construction.
##   - Keep domain behavior in free (static) functions.
##   - Value semantics throughout.
##   - NO classes. Factories only.
##
## Contents:
##   0.  bool_match       — Boolean dispatch without inline if/else branches
##   1.  Maybe            — Optional monad (absence without sentinel nulls)
##   2.  Either           — Result/Error monad (recoverable failure)
##   3.  compose / pipe   — Function composition (right-to-left / left-to-right)
##   4.  curry            — Automatic function currying (curry2, curry3)
##   5.  Lazy             — Memoized deferred evaluation (factory)
##   6.  MemoizedLast     — Last-input memoization for derived values (factory)
##   7.  Pipeline         — Value transformation chains (factory)
##   8.  fmap             — Functor map (Maybe, Either, Array)
##   9.  Dispatcher       — Dictionary-based typed dispatch (factory)
##  10.  multi_match      — Multi-case value-based pattern matching
##  11.  ValidationPipeline — Functional validation chain (factory)
##  12.  ConfigBuilder    — Functional configuration builder (factory)
##  13.  TestResult       — Functional testing result (factory)
##  14.  AsyncResult      — Functional async result handling (factory)
##  15.  HttpResult       — Functional HTTP result wrapper (factory)
##  16.  AsyncChain       — AsyncResult chaining helpers

# ---------------------------------------------------------------------------
# 0. Boolean Dispatch (functional branch selection)
# ---------------------------------------------------------------------------

## Dispatches to one of two thunks based on a boolean condition.
##
## Signature: bool_match(condition: bool, on_true: Callable, on_false: Callable) -> Variant
##
## User Stories:
##   - As a functional-core author, I want boolean decisions represented as data-driven thunk dispatch so higher-level helpers do not need inline branch syntax.
##   - As an RTK or ECS author, I want a neutral primitive for guard/fallback logic that keeps reducers and systems declarative.
static func bool_match(condition: bool, on_true: Callable, on_false: Callable) -> Variant:
	var dispatch_table = {
		true: on_true,
		false: on_false
	}
	return dispatch_table[condition].call()

# ---------------------------------------------------------------------------
# 1. Maybe Monad (absence without sentinel nulls)
# ---------------------------------------------------------------------------
# Rust:  Maybe::Just(T) / Maybe::Nothing
# TS:    { _tag: 'Just', value } / { _tag: 'Nothing' }
# C++:   Maybe<T> { hasValue, value }
# GD:    { "_tag": "Just", "value": val } / { "_tag": "Nothing" }
# ---------------------------------------------------------------------------

## Creates a Just value that carries a present payload through Maybe chains.
##
## Signature: just(val: Variant) -> Dictionary
##
## User Stories:
##   - As a developer, I want to wrap an existing value in a Maybe so I can pass it into a Maybe pipeline.
static func just(val: Variant) -> Dictionary:
	return { "_tag": "Just", "value": val }

## Creates a Nothing value that short-circuits Maybe chains without a payload.
##
## Signature: nothing() -> Dictionary
##
## User Stories:
##   - As a developer, I want to represent the absence of a value without returning null, avoiding null pointer exceptions.
static func nothing() -> Dictionary:
	return { "_tag": "Nothing" }

## Narrows a Maybe to Just when a payload is present.
##
## Signature: is_just(m: Dictionary) -> bool
##
## User Stories:
##   - As a developer, I want to check if a Maybe contains a value before attempting to extract it manually.
static func is_just(m: Dictionary) -> bool:
	return m.get("_tag") == "Just"

## Narrows a Maybe to Nothing when no payload is present.
##
## Signature: is_nothing(m: Dictionary) -> bool
##
## User Stories:
##   - As a developer, I want to check if a Maybe is empty so I can handle the failure or fallback case.
static func is_nothing(m: Dictionary) -> bool:
	return m.get("_tag") == "Nothing"

## Maps a present Maybe value and preserves Nothing unchanged.
## Rust: maybe_map | TS: fmap | C++: fmap(Maybe, f)
##
## Signature: maybe_map(m: Dictionary, f: Callable) -> Dictionary
##
## User Stories:
##   - As a developer, I want to transform the value inside a Maybe without worrying about null checks; if it's Nothing, the mapping is safely ignored.
static func maybe_map(m: Dictionary, f: Callable) -> Dictionary:
	return bool_match(
		is_just(m),
		func(): return just(f.call(m.value)),
		func(): return nothing()
	)

## Chains a Maybe-producing function over a Maybe value (flat map / bind).
## Rust: maybe_chain | TS: mbind | C++: mbind
##
## Signature: maybe_chain(m: Dictionary, f: Callable) -> Dictionary
##
## User Stories:
##   - As a developer, I want to chain multiple operations that might each fail, so that the entire chain short-circuits to Nothing on the first failure.
static func maybe_chain(m: Dictionary, f: Callable) -> Dictionary:
	return bool_match(
		is_just(m),
		func(): return f.call(m.value),
		func(): return nothing()
	)

## Extracts a Maybe payload or falls back to the provided default value.
## Rust: maybe_or_else | TS: orElse | C++: or_else
##
## Signature: maybe_or_else(m: Dictionary, default_val: Variant) -> Variant
##
## User Stories:
##   - As a developer, I want to safely unwrap a Maybe by providing a fallback value in case the Maybe is Nothing.
static func maybe_or_else(m: Dictionary, default_val: Variant) -> Variant:
	return bool_match(
		is_just(m),
		func(): return m.value,
		func(): return default_val
	)

## Extracts a Maybe payload or evaluates a lazy default thunk.
## Rust: maybe_or_else (closure) — lazy variant avoids computing default when Just.
##
## Signature: maybe_or_else_lazy(m: Dictionary, default_thunk: Callable) -> Variant
##
## User Stories:
##   - As a developer, I want to unwrap a Maybe with a fallback, but the fallback is expensive to compute, so I only want to evaluate it if the Maybe is actually Nothing.
static func maybe_or_else_lazy(m: Dictionary, default_thunk: Callable) -> Variant:
	return bool_match(
		is_just(m),
		func(): return m.value,
		func(): return default_thunk.call()
	)

## Pattern-matches a Maybe into a single return value.
## Rust: maybe_match | TS: match | C++: match(Maybe, onJust, onNothing)
##
## Signature: maybe_match(m: Dictionary, on_just: Callable, on_nothing: Callable) -> Variant
##
## User Stories:
##   - As a developer, I want to branch my logic cleanly based on whether a Maybe is Just or Nothing, returning a cohesive result either way.
static func maybe_match(m: Dictionary, on_just: Callable, on_nothing: Callable) -> Variant:
	return bool_match(
		is_just(m),
		func(): return on_just.call(m.value),
		func(): return on_nothing.call()
	)

## Filters a Just value through a predicate; Nothing if predicate fails.
##
## Signature: maybe_filter(m: Dictionary, predicate: Callable) -> Dictionary
##
## User Stories:
##   - As a developer, I want to convert a Just into a Nothing if its value doesn't meet a specific condition, allowing conditional validation in a pipeline.
static func maybe_filter(m: Dictionary, predicate: Callable) -> Dictionary:
	return bool_match(
		is_just(m),
		func():
			return bool_match(
				predicate.call(m.value),
				func(): return m,
				func(): return nothing()
			),
		func(): return nothing()
	)

## Lift a nullable value into Maybe — null -> Nothing, otherwise Just.
## Rust: maybe_from_option | TS: fromNullable | C++: from_nullable
##
## Signature: from_nullable(val: Variant) -> Dictionary
##
## User Stories:
##   - As a developer, I want to interface with existing Godot APIs that return null, safely converting their results into the Maybe monad.
static func from_nullable(val: Variant) -> Dictionary:
	return bool_match(
		val != null,
		func(): return just(val),
		func(): return nothing()
	)

## Lift a value into Maybe when the caller marks it as valid.
## C++: from_nullable_value(value, valid)
##
## Signature: from_nullable_value(val: Variant, valid: bool) -> Dictionary
##
## User Stories:
##   - As a developer, I want to conditionally wrap a value in a Maybe based on an external validation flag.
static func from_nullable_value(val: Variant, valid: bool) -> Dictionary:
	return bool_match(
		valid,
		func(): return just(val),
		func(): return nothing()
	)

## Extract from Maybe or fail — boundary helper for code that must have a value.
## Rust: (panics) | TS: requireJust | C++: require_just
##
## Signature: require_just(m: Dictionary, error_msg: String = "Expected Just, got Nothing") -> Variant
##
## User Stories:
##   - As a developer, I want to assert that a Maybe must contain a value at an architectural boundary, failing loudly if it doesn't.
static func require_just(m: Dictionary, error_msg: String = "Expected Just, got Nothing") -> Variant:
	return bool_match(
		is_just(m),
		func(): return m.value,
		func():
			push_error(error_msg)
			assert(false, error_msg)
			return null
	)

## Convert Maybe to Godot's null-based Option semantics.
## Rust: maybe_to_option
##
## Signature: maybe_to_nullable(m: Dictionary) -> Variant
##
## User Stories:
##   - As a developer, I want to extract a value from a Maybe to pass to a standard Godot API that expects null for missing data.
static func maybe_to_nullable(m: Dictionary) -> Variant:
	return bool_match(
		is_just(m),
		func(): return m.value,
		func(): return null
	)

# ---------------------------------------------------------------------------
# 2. Either Monad (recoverable failure)
# ---------------------------------------------------------------------------
# Rust:  Either::Left(E) / Either::Right(T)
# TS:    { _tag: 'Left', error } / { _tag: 'Right', value }
# C++:   Either<E,T> { isLeft, left, right }
# GD:    { "_tag": "Left", "error": e } / { "_tag": "Right", "value": v }
# ---------------------------------------------------------------------------

## Creates a failing Either branch that carries an error payload.
##
## Signature: left(err: Variant) -> Dictionary
##
## User Stories:
##   - As a developer, I want to return a recoverable error state so the caller can choose how to handle the failure without throwing exceptions.
static func left(err: Variant) -> Dictionary:
	return { "_tag": "Left", "error": err }

## Creates a successful Either branch that carries a value payload.
##
## Signature: right(val: Variant) -> Dictionary
##
## User Stories:
##   - As a developer, I want to return a successful result in a standardized Either wrapper so it composes easily with other functional operations.
static func right(val: Variant) -> Dictionary:
	return { "_tag": "Right", "value": val }

## Narrows an Either to its error branch.
##
## Signature: is_left(e: Dictionary) -> bool
##
## User Stories:
##   - As a developer, I want to explicitly check if an operation failed before attempting to read its error payload.
static func is_left(e: Dictionary) -> bool:
	return e.get("_tag") == "Left"

## Narrows an Either to its success branch.
##
## Signature: is_right(e: Dictionary) -> bool
##
## User Stories:
##   - As a developer, I want to explicitly check if an operation succeeded before extracting its payload.
static func is_right(e: Dictionary) -> bool:
	return e.get("_tag") == "Right"

## Maps the success branch of an Either and preserves failures unchanged.
## Rust: either_map | TS: efmap | C++: fmap(Either, f)
##
## Signature: either_map(e: Dictionary, f: Callable) -> Dictionary
##
## User Stories:
##   - As a developer, I want to transform the success payload of an operation, automatically skipping the transformation if the previous operation yielded an error.
static func either_map(e: Dictionary, f: Callable) -> Dictionary:
	return bool_match(
		is_right(e),
		func(): return right(f.call(e.value)),
		func(): return e
	)

## Chains an Either-producing function onto a successful Either value.
## Rust: either_chain | TS: ebind | C++: ebind
##
## Signature: either_chain(e: Dictionary, f: Callable) -> Dictionary
##
## User Stories:
##   - As a developer, I want to sequence multiple operations that can fail, letting the first encountered error short-circuit the entire pipeline.
static func either_chain(e: Dictionary, f: Callable) -> Dictionary:
	return bool_match(
		is_right(e),
		func(): return f.call(e.value),
		func(): return e
	)

## Pattern-matches an Either into a single return value.
## Rust: either_match | TS: ematch | C++: ematch
##
## Signature: either_match(e: Dictionary, on_left: Callable, on_right: Callable) -> Variant
##
## User Stories:
##   - As a developer, I want to handle both success and error cases simultaneously, collapsing them into a final uniform result.
static func either_match(e: Dictionary, on_left: Callable, on_right: Callable) -> Variant:
	return bool_match(
		is_left(e),
		func(): return on_left.call(e.error),
		func(): return on_right.call(e.value)
	)

## Extracts an Either success value or falls back to the provided default.
## C++: (via match) — convenience for extracting Right or using a default.
##
## Signature: either_or_else(e: Dictionary, default_val: Variant) -> Variant
##
## User Stories:
##   - As a developer, I want to extract a value from an Either, substituting a safe default if the Either represents an error state.
static func either_or_else(e: Dictionary, default_val: Variant) -> Variant:
	return bool_match(
		is_right(e),
		func(): return e.value,
		func(): return default_val
	)

## Maps the error branch of an Either while preserving the success branch.
##
## Signature: either_map_left(e: Dictionary, f: Callable) -> Dictionary
##
## User Stories:
##   - As a developer, I want to transform or enrich an error payload (e.g., adding context) before passing it up the chain.
static func either_map_left(e: Dictionary, f: Callable) -> Dictionary:
	return bool_match(
		is_left(e),
		func(): return left(f.call(e.error)),
		func(): return e
	)

# ---------------------------------------------------------------------------
# 3. Composition (compose / pipe)
# ---------------------------------------------------------------------------
# Rust: pipe(f, g) left-to-right, compose(g, f) right-to-left
# TS:   compose(...fns) right-to-left
# C++:  Composed<F,G> / compose(f, g) right-to-left, Pipeline<T> / pipe(v) left-to-right
# ---------------------------------------------------------------------------

## Composes two functions from right to left: compose(f, g)(x) == f(g(x))
## Rust: compose | C++: compose(f, g)
##
## Signature: compose(f: Callable, g: Callable) -> Callable
##
## User Stories:
##   - As a developer, I want to combine two functions mathematically so that the output of the second feeds cleanly into the first.
static func compose(f: Callable, g: Callable) -> Callable:
	return func(x: Variant) -> Variant:
		return f.call(g.call(x))

## Composes an array of functions right-to-left.
## TS: compose(...fns) with reduceRight
##
## Signature: compose_many(fns: Array[Callable]) -> Callable
##
## User Stories:
##   - As a developer, I want to combine a large array of operations into a single cohesive function, evaluated from right to left.
static func compose_many(fns: Array[Callable]) -> Callable:
	return func(x: Variant) -> Variant:
		var reversed_fns = fns.duplicate()
		reversed_fns.reverse()
		return reversed_fns.reduce(func(acc, f): return f.call(acc), x)

## Pipes a value through a sequence of functions left-to-right.
## Rust: pipe(f, g) | C++: pipe(v) | <val> | f1 | f2
##
## Signature: pipe(val: Variant, fns: Array[Callable]) -> Variant
##
## User Stories:
##   - As a developer, I want to read my transformations in top-down or left-to-right execution order, improving the readability of multi-step transformations.
static func pipe(val: Variant, fns: Array[Callable]) -> Variant:
	return fns.reduce(func(acc, f): return f.call(acc), val)

## Creates a Pipeline Dictionary for fluent chaining via apply().
## C++: Pipeline<T> / pipe(v) | f1 | f2 — GDScript lacks operator| so we use apply().
## FACTORY FUNCTION — returns a Dictionary with closures, not a class instance.
##
## Signature: create_pipeline(initial_value: Variant) -> Dictionary
##
## User Stories:
##   - As a developer, I want to fluidly chain functional methods together on a base value without needing intermediate variables.
static func create_pipeline(initial_value: Variant) -> Dictionary:
	var _val: Variant = initial_value
	return {
		"val": _val,
		"apply": func(f: Callable) -> Dictionary:
			return create_pipeline(f.call(_val)),
		"result": func() -> Variant:
			return _val
	}

# Convenience alias matching old API
##
## Signature: pipeline(initial_value: Variant) -> Dictionary
##
## User Stories:
##   - As a developer, I want an abbreviated helper to quickly start a fluent data transformation pipeline.
static func pipeline(initial_value: Variant) -> Dictionary:
	return create_pipeline(initial_value)

# ---------------------------------------------------------------------------
# 4. Currying
# ---------------------------------------------------------------------------
# Rust: curry2 / curry3  |  TS: curry(fn)  |  C++: Curried<Arity, Func> / curry<N>(f)
# ---------------------------------------------------------------------------

## Curries a 2-argument function into a chain of single-argument applications.
##
## Signature: curry2(f: Callable) -> Callable
##
## User Stories:
##   - As a developer, I want to partially apply arguments to a 2-argument function, enabling easier composition within map or chain operations.
static func curry2(f: Callable) -> Callable:
	return func(a: Variant) -> Callable:
		return func(b: Variant) -> Variant:
			return f.call(a, b)

## Curries a 3-argument function into a chain of single-argument applications.
##
## Signature: curry3(f: Callable) -> Callable
##
## User Stories:
##   - As a developer, I want to partially apply up to two arguments to a 3-argument function to create customized unary functions.
static func curry3(f: Callable) -> Callable:
	return func(a: Variant) -> Callable:
		return func(b: Variant) -> Callable:
			return func(c: Variant) -> Variant:
				return f.call(a, b, c)


# ---------------------------------------------------------------------------
# 4b. Unary-Composition Cookbook (factory returns impl Fn pattern)
# ---------------------------------------------------------------------------
# Therapy 12 reference: src/features/* uses factories that capture configuration
# and return one-argument functions. These helpers keep that shape explicit in
# GDScript so reducers, selectors, ECS systems, and UI transforms can share the
# same neutral arrows without loops or setter-style control flow.
# ---------------------------------------------------------------------------

## Returns its input unchanged, the neutral element of composition.
##
## Signature: identity(value: Variant) -> Variant
##
## User Stories:
##   - As a pipeline author, I need a named no-op arrow for default match arms.
##   - As a cross-project maintainer, I need the same primitive available in GDScript, Rust, TypeScript, and C++.
static func identity(value: Variant) -> Variant:
	return value

## Builds a unary function that ignores its input and returns a fixed value.
##
## Signature: constant(value: Variant) -> Callable
##
## User Stories:
##   - As a selector author, I want an "always this" arrow that composes with dispatch tables and match cases.
static func constant(value: Variant) -> Callable:
	return func(_ignored: Variant) -> Variant:
		return value

## Swaps the argument order of a binary function.
##
## Signature: flip(f: Callable) -> Callable
##
## User Stories:
##   - As a partial-application author, I need to bind a right-hand dependency while preserving unary pipeline style.
static func flip(f: Callable) -> Callable:
	return func(b: Variant, a: Variant) -> Variant:
		return f.call(a, b)

## Negates a unary predicate.
##
## Signature: complement(predicate: Callable) -> Callable
##
## User Stories:
##   - As a validation author, I derive reject logic from the same keep rule instead of duplicating conditions.
static func complement(predicate: Callable) -> Callable:
	return func(value: Variant) -> bool:
		return not predicate.call(value)

## Combines two predicates with logical AND.
##
## Signature: both(left_predicate: Callable, right_predicate: Callable) -> Callable
##
## User Stories:
##   - As an ECS system author, I express admission rules as one reusable arrow.
static func both(left_predicate: Callable, right_predicate: Callable) -> Callable:
	return func(value: Variant) -> bool:
		return left_predicate.call(value) and right_predicate.call(value)

## Combines two predicates with logical OR.
##
## Signature: either_pred(left_predicate: Callable, right_predicate: Callable) -> Callable
##
## User Stories:
##   - As a matcher author, I accept values matching either rule without nested branches.
static func either_pred(left_predicate: Callable, right_predicate: Callable) -> Callable:
	return func(value: Variant) -> bool:
		return left_predicate.call(value) or right_predicate.call(value)

## Folds many predicates into one predicate requiring every rule to pass.
##
## Signature: all_pass(predicates: Array) -> Callable
##
## User Stories:
##   - As a domain gatekeeper, I keep admission rules data-driven and reusable.
static func all_pass(predicates: Array) -> Callable:
	return func(value: Variant) -> bool:
		return fold_array(predicates, true, func(acc, predicate): return acc and predicate.call(value))

## Folds many predicates into one predicate accepting any passing rule.
##
## Signature: any_pass(predicates: Array) -> Callable
##
## User Stories:
##   - As an event router, I collapse an allow-list of matchers into one callable.
static func any_pass(predicates: Array) -> Callable:
	return func(value: Variant) -> bool:
		return fold_array(predicates, false, func(acc, predicate): return acc or predicate.call(value))

## Projects one input through two functions and combines their outputs.
##
## Signature: converge2(combine: Callable, first: Callable, second: Callable) -> Callable
##
## User Stories:
##   - As a selector author, I derive one value from two projections without naming intermediate bindings.
static func converge2(combine: Callable, first: Callable, second: Callable) -> Callable:
	return func(value: Variant) -> Variant:
		return combine.call(first.call(value), second.call(value))

## Projects one input through two functions and returns both outputs.
##
## Signature: juxt2(first: Callable, second: Callable) -> Callable
##
## User Stories:
##   - As a layout author, I fan one config value into two derived values while preserving unary composition.
static func juxt2(first: Callable, second: Callable) -> Callable:
	return func(value: Variant) -> Array:
		return [first.call(value), second.call(value)]

## Composes three unary functions left-to-right.
##
## Signature: pipe3(f: Callable, g: Callable, h: Callable) -> Callable
##
## User Stories:
##   - As a reducer author, I chain three stages using the same left-to-right `pipe` semantics.
static func pipe3(f: Callable, g: Callable, h: Callable) -> Callable:
	return func(value: Variant) -> Variant:
		return pipe(value, [f, g, h])

## Composes four unary functions left-to-right.
##
## Signature: pipe4(f: Callable, g: Callable, h: Callable, i: Callable) -> Callable
##
## User Stories:
##   - As a rendering author, I build a four-stage transform without nested calls.
static func pipe4(f: Callable, g: Callable, h: Callable, i: Callable) -> Callable:
	return func(value: Variant) -> Variant:
		return pipe(value, [f, g, h, i])

## Cookbook recipe: clamps a channel or coordinate into the byte range.
##
## Signature: example_clamp_channel() -> Callable
##
## User Stories:
##   - As a HUD or renderer author, I reuse one point-free clamp arrow for color channels and bar widths.
static func example_clamp_channel() -> Callable:
	return func(value: Variant) -> Variant:
		return pipe(value, [func(v): return max(v, 0), func(v): return min(v, 255)])

## Cookbook recipe: turns [current, max] into a safe fill ratio.
##
## Signature: example_fill_ratio() -> Callable
##
## User Stories:
##   - As a meter selector author, I derive ratios from current/max tuples through `converge2`.
static func example_fill_ratio() -> Callable:
	return converge2(
		func(current, max_value): return current / max(max_value, 0.000001),
		func(reading): return reading[0],
		func(reading): return reading[1]
	)

## Cookbook recipe: accepts entities that are both alive and visible.
##
## Signature: example_alive_and_visible() -> Callable
##
## User Stories:
##   - As an ECS system author, I reuse a fused admission predicate in queries and filters.
static func example_alive_and_visible() -> Callable:
	return both(
		func(flags): return flags[0],
		func(flags): return flags[1]
	)

# ---------------------------------------------------------------------------
# 5. Lazy (Deferred Evaluation) — FACTORY, NO CLASS
# ---------------------------------------------------------------------------
# C++: Lazy<T> { thunk, cached } / lazy(f) / eval(lz)
# Returns a Dictionary with { thunk, has_cached, cached_value } and eval() function.
# ---------------------------------------------------------------------------

## Wraps a thunk so it is evaluated once on first access.
## FACTORY: returns a Dictionary, not a class.
##
## Signature: lazy(thunk: Callable) -> Dictionary
##
## User Stories:
##   - As a developer, I want to defer an expensive calculation until it is absolutely needed, evaluating it just once.
static func lazy(thunk: Callable) -> Dictionary:
	# Use a mutable reference container for caching state
	var cache_ref = { "has_cached": false, "cached_value": null }
	return {
		"_thunk": thunk,
		"_cache_ref": cache_ref,
	}

## Forces a lazy value and memoizes the computed result.
##
## Signature: eval_lazy(lz: Dictionary) -> Variant
##
## User Stories:
##   - As a developer, I want to evaluate a deferred thunk, storing its result safely so subsequent evaluations use the cached value.
static func eval_lazy(lz: Dictionary) -> Variant:
	var cache: Dictionary = lz._cache_ref
	match cache.has_cached:
		false:
			cache.cached_value = lz._thunk.call()
			cache.has_cached = true
			return cache.cached_value
		_:
			return cache.cached_value

# ---------------------------------------------------------------------------
# 6. MemoizedLast (Last-Input Memoization) — FACTORY, NO CLASS
# ---------------------------------------------------------------------------
# C++: MemoizedLast<Signature> / memoizeLast<Sig>(f, comparator?)
# This is the canonical primitive for selector-style derived-data memoization.
# Returns a Dictionary with a call_args(args) closure.
# ---------------------------------------------------------------------------

## Default array equality comparator.
##
## Signature: _default_array_equals(a: Array, b: Array) -> bool
##
## User Stories:
##   - As an internal system component, I want a fallback mechanism to deeply compare two arrays to support cache invalidation checks.
static func _default_array_equals(a: Array, b: Array) -> bool:
	return a.size() == b.size() and range(a.size()).all(func(i): return typeof(a[i]) == typeof(b[i]) and a[i] == b[i])

## Memoizes the last invocation of a pure function.
## FACTORY: returns a Dictionary with call_args closure, not a class.
## Optionally accepts a custom equality comparator (must compare two Arrays).
##
## Signature: memoize_last(f: Callable, custom_equals: Callable = Callable()) -> Dictionary
##
## User Stories:
##   - As a UI or state developer, I want to avoid recomputing derived data if the input parameters haven't changed since the last invocation.
static func memoize_last(f: Callable, custom_equals: Callable = Callable()) -> Dictionary:
	var cmp: Callable = bool_match(
		custom_equals.is_valid(),
		func(): return custom_equals,
		func(): return _default_array_equals
	)
	# Mutable state container for memoization cache
	var memo_ref = { "has_cached": false, "last_args": [], "last_result": null }
	return {
		"call_args": func(args: Array) -> Variant:
			match memo_ref.has_cached and cmp.call(memo_ref.last_args, args):
				true: return memo_ref.last_result
				_:
					memo_ref.last_result = f.callv(args)
					memo_ref.last_args = args.duplicate(true)
					memo_ref.has_cached = true
					return memo_ref.last_result
	}

# ---------------------------------------------------------------------------
# 7. fmap — Functor map (Maybe, Either, Array)
# ---------------------------------------------------------------------------
# C++: fmap(Maybe, f) / fmap(Either, f) / fmap(vector, f)
# Unified mapping interface across container types.
# ---------------------------------------------------------------------------

## Maps a function across every element in an Array.
## C++: fmap(vector, f)
##
## Signature: fmap_array(arr: Array, f: Callable) -> Array
##
## User Stories:
##   - As a developer, I want a standard functional map interface over collections that aligns with fmap for Maybe and Either.
static func fmap_array(arr: Array, f: Callable) -> Array:
	return arr.map(f)

## Filters an Array through a predicate, returning only matching elements.
##
## Signature: filter_array(arr: Array, predicate: Callable) -> Array
##
## User Stories:
##   - As a developer, I want to sift through a collection keeping only elements that meet a strict condition in a pure functional manner.
static func filter_array(arr: Array, predicate: Callable) -> Array:
	return arr.filter(predicate)

## Reduces an Array to a single value via an accumulator function.
## Equivalent to Rust's iter().fold() or JS Array.reduce().
##
## Signature: fold_array(arr: Array, initial: Variant, reducer: Callable) -> Variant
##
## User Stories:
##   - As a developer, I want to accumulate or summarize data in a collection iteratively without mutating external state.
static func fold_array(arr: Array, initial: Variant, reducer: Callable) -> Variant:
	return arr.reduce(reducer, initial)

## Merges two dictionaries by creating a shallow duplicate of the target and overwriting with source.
##
## Signature: merge_dict(target: Dictionary, source: Dictionary) -> Dictionary
##
## User Stories:
##   - As a developer, I want to purely update a Dictionary's values without modifying the original source reference.
static func merge_dict(target: Dictionary, source: Dictionary) -> Dictionary:
	var res = target.duplicate()
	res.merge(source)
	return res

# ---------------------------------------------------------------------------
# 8. Dispatcher (type-safe dispatch table) — FACTORY, NO CLASS
# ---------------------------------------------------------------------------
# Rust: Dispatcher<K, A, R> / create_dispatcher / dispatcher_dispatch
# TS:   Dispatcher<K, R> / createDispatcher
# C++:  Dispatcher<Key, Result> / createDispatcher / dispatch
# Returns Maybe<Result> from dispatch — just(handler()) if key exists, nothing() if not.
# ---------------------------------------------------------------------------

## Creates an empty dispatcher. FACTORY: returns a Dictionary with closures.
##
## Signature: create_dispatcher() -> Dictionary
##
## User Stories:
##   - As a system architect, I want to build a runtime type-safe dispatch table that avoids large match statements and scales cleanly.
static func create_dispatcher() -> Dictionary:
	var table = {}
	return {
		"register_handler": func(key: Variant, handler: Callable) -> Dictionary:
			var next_table = table.duplicate()
			next_table[key] = handler
			return create_dispatcher_with_table(next_table),
		"dispatch": func(key: Variant, arg: Variant = null) -> Dictionary:
			return bool_match(
				table.has(key),
				func(): return just(table[key].call(arg)),
				func(): return nothing()
			),
		"dispatch_or_else": func(key: Variant, arg: Variant, fallback: Callable) -> Variant:
			return bool_match(
				table.has(key),
				func(): return table[key].call(arg),
				func(): return fallback.call(arg)
			),
		"has": func(key: Variant) -> bool:
			return table.has(key),
		"keys": func() -> Array:
			return table.keys(),
	}

## Internal helper to create a dispatcher pre-seeded with a table.
##
## Signature: create_dispatcher_with_table(table: Dictionary) -> Dictionary
##
## User Stories:
##   - As an internal API, I need to instantiate new versions of a dispatcher while preserving immutability as new handlers are registered.
static func create_dispatcher_with_table(table: Dictionary) -> Dictionary:
	return {
		"register_handler": func(key: Variant, handler: Callable) -> Dictionary:
			var next_table = table.duplicate()
			next_table[key] = handler
			return create_dispatcher_with_table(next_table),
		"dispatch": func(key: Variant, arg: Variant = null) -> Dictionary:
			return bool_match(
				table.has(key),
				func(): return just(table[key].call(arg)),
				func(): return nothing()
			),
		"dispatch_or_else": func(key: Variant, arg: Variant, fallback: Callable) -> Variant:
			return bool_match(
				table.has(key),
				func(): return table[key].call(arg),
				func(): return fallback.call(arg)
			),
		"has": func(key: Variant) -> bool:
			return table.has(key),
		"keys": func() -> Array:
			return table.keys(),
	}

## Creates a dispatcher pre-populated from an array of [key, handler] pairs.
## TS: createDispatcher(entries)
##
## Signature: create_dispatcher_from(entries: Array) -> Dictionary
##
## User Stories:
##   - As a system architect, I want to initialize a fully loaded functional dispatcher from a configuration array right away.
static func create_dispatcher_from(entries: Array) -> Dictionary:
	return create_dispatcher_with_table(entries.reduce(func(acc, entry):
		acc[entry[0]] = entry[1]
		return acc
	, {}))

# ---------------------------------------------------------------------------
# 9. multi_match (value-based pattern matching) — uses factory match_case
# ---------------------------------------------------------------------------
# Rust: multi_match(value, cases, wildcard)
# TS:   multiMatch(value, cases) -> Maybe<R>
# C++:  multi_match(value, cases) -> Maybe<R> / when(pred, handler) / wildcard() / equals(val)
# ---------------------------------------------------------------------------

## Builds a match case from a predicate (Callable or exact value) and a handler.
## FACTORY: returns a plain Dictionary, not a class.
## C++: when(pred, handler)
##
## Signature: match_case(predicate: Variant, handler: Callable) -> Dictionary
##
## User Stories:
##   - As a developer, I want to declare a structural pattern matching arm purely via data so it can be passed to a matching evaluator.
static func match_case(predicate: Variant, handler: Callable) -> Dictionary:
	return { "_is_match_case": true, "predicate": predicate, "handler": handler }

## Tests whether a match-case dictionary accepts a value.
##
## Signature: match_case_accepts(case_value: Variant, val: Variant) -> bool
##
## User Stories:
##   - As a pattern matching maintainer, I want Callable predicates and exact-value predicates to share one branch-free evaluator.
static func match_case_accepts(case_value: Variant, val: Variant) -> bool:
	return bool_match(
		case_value is Dictionary and case_value.get("_is_match_case", false),
		func():
			return bool_match(
				case_value.predicate is Callable,
				func(): return case_value.predicate.call(val) == true,
				func(): return case_value.predicate == val
			),
		func(): return false
	)

## Returns a predicate that matches every input (default arm).
## C++: wildcard<T>()
##
## Signature: wildcard() -> Callable
##
## User Stories:
##   - As a developer, I want a fallback matching condition so I can elegantly catch all unmatched scenarios.
static func wildcard() -> Callable:
	return func(_val: Variant) -> bool: return true

## Returns a predicate that matches a specific expected value.
## C++: equals<T>(expected)
##
## Signature: equals(expected: Variant) -> Callable
##
## User Stories:
##   - As a developer, I want an exact-value match predicate easily constructed for multi-case pattern matching.
static func equals(expected: Variant) -> Callable:
	return func(val: Variant) -> bool: return val == expected

## Evaluates cases in order and returns the first match.
## A case predicate can be a Callable returning bool, or an exact value for equality.
## Uses a wildcard fallback when no case matches.
## Rust: multi_match(val, cases, wildcard) — with explicit wildcard
##
## Signature: multi_match(val: Variant, cases: Array, wildcard_handler: Callable = Callable()) -> Variant
##
## User Stories:
##   - As a developer, I want a robust functional alternative to switch/match statements that operates over complex data types and exact states.
static func multi_match(val: Variant, cases: Array, wildcard_handler: Callable = Callable()) -> Variant:
	var match_fn = func(c):
		return match_case_accepts(c, val)
	var found = cases.filter(match_fn)
	return bool_match(
		not found.is_empty(),
		func(): return found.front().handler.call(val),
		func():
			return bool_match(
				wildcard_handler.is_valid(),
				func(): return wildcard_handler.call(val),
				func(): return null
			)
	)

## Evaluates cases in order and returns Maybe<R> (Nothing if no match).
## C++/TS: multi_match(value, cases) -> Maybe<R> — without explicit wildcard
##
## Signature: multi_match_maybe(val: Variant, cases: Array) -> Dictionary
##
## User Stories:
##   - As a developer, I want a pattern matching function that gracefully returns Nothing rather than erroring when no match applies.
static func multi_match_maybe(val: Variant, cases: Array) -> Dictionary:
	var match_fn = func(c):
		return match_case_accepts(c, val)
	var found = cases.filter(match_fn)
	return bool_match(
		not found.is_empty(),
		func(): return just(found.front().handler.call(val)),
		func(): return nothing()
	)

# ---------------------------------------------------------------------------
# 10. ValidationPipeline (Functional Validation Chain) — FACTORY, NO CLASS
# ---------------------------------------------------------------------------
# C++: ValidationPipeline<T, E> / validationPipeline() / addValidation / runValidation
# Each validator takes input and returns Either<Error, Result>.
# The pipeline short-circuits on first error.
# ---------------------------------------------------------------------------

## Creates an empty validation pipeline.
## FACTORY: returns a Dictionary with closures, not a class.
##
## Signature: validation_pipeline() -> Dictionary
##
## User Stories:
##   - As a data processing author, I want to construct a composable chain of validators that halts gracefully on the first failure.
static func validation_pipeline() -> Dictionary:
	return _build_validation_pipeline([])

## Internal helper for building a pipeline with pre-populated validators.
##
## Signature: _build_validation_pipeline(validators: Array) -> Dictionary
##
## User Stories:
##   - As an internal API, I need to immutably duplicate a validation chain while adding a new validation step.
static func _build_validation_pipeline(validators: Array) -> Dictionary:
	return {
		"add_validation": func(validator: Callable) -> Dictionary:
			return _build_validation_pipeline(validators + [validator]),
		"run": func(val: Variant) -> Dictionary:
			return validators.reduce(func(acc, v):
				return bool_match(
					is_right(acc),
					func(): return v.call(acc.value),
					func(): return acc
				)
			, right(val)),
	}

# ---------------------------------------------------------------------------
# 11. ConfigBuilder (Functional Configuration Builder) — FACTORY, NO CLASS
# ---------------------------------------------------------------------------
# C++: ConfigBuilder<Config> / configBuilder() / setMember / with / buildConfig
# A data-first builder for creating configuration objects using functional composition.
# ---------------------------------------------------------------------------

## Creates an empty functional configuration builder.
## FACTORY: returns a Dictionary with closures, not a class.
##
## Signature: config_builder() -> Dictionary
##
## User Stories:
##   - As a developer, I want a chainable builder to declaratively construct complex configuration objects without mutating partial state.
static func config_builder() -> Dictionary:
	return _build_config_builder([])

## Internal helper for building a config builder with pre-populated setters.
##
## Signature: _build_config_builder(setters: Array) -> Dictionary
##
## User Stories:
##   - As an internal API, I need to instantiate new versions of a config builder dynamically as new setters are requested.
static func _build_config_builder(setters: Array) -> Dictionary:
	return {
		"with_setter": func(setter: Callable) -> Dictionary:
			return _build_config_builder(setters + [setter]),
		"set_key": func(key: String, value: Variant) -> Dictionary:
			return _build_config_builder(setters + [func(config: Dictionary): config[key] = value]),
		"build_config": func(initial_config: Variant) -> Variant:
			return setters.reduce(func(acc, setter):
				setter.call(acc)
				return acc
			, initial_config),
	}

# ---------------------------------------------------------------------------
# 12. TestResult (Functional Testing Result) — FACTORY, NO CLASS
# ---------------------------------------------------------------------------
# C++: TestResult<T> / Success / Failure / withDetail / TryGetValue
# ---------------------------------------------------------------------------

## Builds a successful test result with an attached value. FACTORY.
##
## Signature: test_success(val: Variant = null, msg: String = "") -> Dictionary
##
## User Stories:
##   - As a test author, I want to return a functionally pure success indicator with optional metadata to my testing harness.
static func test_success(val: Variant = null, msg: String = "") -> Dictionary:
	return { "is_successful": true, "value": val, "message": msg, "details": {} }

## Builds a failed test result with a message. FACTORY.
##
## Signature: test_failure(msg: String) -> Dictionary
##
## User Stories:
##   - As a test author, I want to clearly communicate test failure states immutably, ensuring failure tracking is thread-safe and predictable.
static func test_failure(msg: String) -> Dictionary:
	return { "is_successful": false, "value": null, "message": msg, "details": {} }

## Attaches a string detail pair to a test result (immutable — returns new result).
##
## Signature: test_with_detail(result: Dictionary, key: String, val: Variant) -> Dictionary
##
## User Stories:
##   - As a test author, I want to attach extra context to a result without modifying the original result instance.
static func test_with_detail(result: Dictionary, key: String, val: Variant) -> Dictionary:
	var next = result.duplicate(true)
	next.details[key] = val
	return next

## Returns the value as a Maybe when the test succeeded.
## C++: TryGetValue()
##
## Signature: test_try_get_value(result: Dictionary) -> Dictionary
##
## User Stories:
##   - As a developer, I want to extract a payload from a test outcome safely through the Maybe monad without checking success explicitly.
static func test_try_get_value(result: Dictionary) -> Dictionary:
	return bool_match(
		result.is_successful,
		func(): return just(result.value),
		func(): return nothing()
	)

## Returns the value or fails fast when the result is unsuccessful.
## C++: getValue()
##
## Signature: test_get_value(result: Dictionary) -> Variant
##
## User Stories:
##   - As a developer, I want to aggressively retrieve a test payload when I'm certain the test succeeded, failing immediately otherwise.
static func test_get_value(result: Dictionary) -> Variant:
	match result.is_successful:
		true: return result.value
		_:
			push_error("TestResult: Cannot get value from failure — " + result.message)
			return null

# ---------------------------------------------------------------------------
# 13. AsyncResult (Functional Async Result Handling) — FACTORY, NO CLASS
# ---------------------------------------------------------------------------
# C++: AsyncResult<T> / create / then / catch_ / execute
# Safe for async callbacks via shared state.
# ---------------------------------------------------------------------------

## Builds an async result from an executor callback.
## FACTORY: returns a Dictionary with closures, not a class.
## Executor signature: func(resolve: Callable, reject: Callable)
##
## Signature: create_async_result(executor: Callable) -> Dictionary
##
## User Stories:
##   - As a developer, I want an awaitable functional construct mapping directly to JS Promises, enabling pure asynchronous compositions.
static func create_async_result(executor: Callable) -> Dictionary:
	return _wrap_async_state({
		"executor": executor,
		"success_handlers": [],
		"error_handlers": [],
	})

## Internal helper to re-wrap an existing async state reference.
##
## Signature: _wrap_async_state(state_ref: Dictionary) -> Dictionary
##
## User Stories:
##   - As an internal API, I need to create chainable `then` and `catch` methods sharing underlying state reference data.
static func _wrap_async_state(state_ref: Dictionary) -> Dictionary:
	return {
		"_state_ref": state_ref,
		"then": func(handler: Callable) -> Dictionary:
			state_ref.success_handlers.append(handler)
			return _wrap_async_state(state_ref),
		"catch_": func(handler: Callable) -> Dictionary:
			state_ref.error_handlers.append(handler)
			return _wrap_async_state(state_ref),
		"execute": func() -> void:
			var resolve_fn = func(val: Variant):
				state_ref.success_handlers.map(func(h): h.call(val))
			var reject_fn = func(err: Variant):
				state_ref.error_handlers.map(func(h): h.call(err))
			state_ref.executor.call(resolve_fn, reject_fn),
	}

# ---------------------------------------------------------------------------
# 14. HttpResult (Functional Http Request Wrapper) — FACTORY, NO CLASS
# ---------------------------------------------------------------------------
# C++: HttpResult<T> / Success / Failure
# ---------------------------------------------------------------------------

## Builds a successful HTTP result wrapper. FACTORY.
##
## Signature: http_success(data: Variant, code: int = 200) -> Dictionary
##
## User Stories:
##   - As a developer, I want a consistent success response format carrying payload data and the corresponding HTTP status.
static func http_success(data: Variant, code: int = 200) -> Dictionary:
	return { "is_successful": true, "response_code": code, "data": data, "error": "" }

## Builds a failed HTTP result wrapper. FACTORY.
##
## Signature: http_failure(err: String, code: int = 0) -> Dictionary
##
## User Stories:
##   - As a developer, I want a consistent error response format to neatly handle failed HTTP requests in functional pipelines.
static func http_failure(err: String, code: int = 0) -> Dictionary:
	return { "is_successful": false, "response_code": code, "data": null, "error": err }

# ---------------------------------------------------------------------------
# 15. AsyncChain (Helpers for chaining AsyncResults) — NO CLASS
# ---------------------------------------------------------------------------
# C++: AsyncChain::then<T, U>(res, f) -> AsyncResult<U>
# Chains one AsyncResult into another async-producing transformation.
# ---------------------------------------------------------------------------

## Chains one AsyncResult into another async-producing transformation.
## C++: AsyncChain::then<T, U>(res, f)
## f receives the resolved value and must return a new AsyncResult (Dictionary).
##
## Signature: async_chain_then(res: Dictionary, f: Callable) -> Dictionary
##
## User Stories:
##   - As a developer, I want to pipe an asynchronous resolution straight into a secondary asynchronous operation sequentially.
static func async_chain_then(res: Dictionary, f: Callable) -> Dictionary:
	return create_async_result(func(resolve: Callable, reject: Callable):
		res.then.call(func(val: Variant):
			var next_result: Dictionary = f.call(val)
			next_result.then.call(resolve).catch_.call(reject).execute.call()
		).catch_.call(reject).execute.call()
	)


# ---------------------------------------------------------------------------
# 14. Partial application / memoize / tap / trampoline (parity additions)
# ---------------------------------------------------------------------------
# Rust: partial_apply / partial_apply2 / memoize / tap / tap_mut / trampoline
# TS:   partialApply / memoize / tap / tapMut / trampoline
# C++:  partial_apply / MemoizedLast / tap / tap_mut / trampoline
# Brings the GDScript FP substrate to parity with the Rust/TS/C++ cores.
# ---------------------------------------------------------------------------

## Binds the first argument of a 2-arg function ahead of time.
##
## Signature: partial_apply(f: Callable, a: Variant) -> Callable
##
## User Stories:
##   - As a configuration author, I bind a function's leading argument up front so callers only supply the remaining one at the call site.
static func partial_apply(f: Callable, a: Variant) -> Callable:
	return func(b: Variant) -> Variant:
		return f.call(a, b)

## Binds the first two arguments of a 3-arg function ahead of time.
##
## Signature: partial_apply2(f: Callable, a: Variant, b: Variant) -> Callable
##
## User Stories:
##   - As a configuration author, I pre-bind two arguments so the resulting unary function slots into a pipeline.
static func partial_apply2(f: Callable, a: Variant, b: Variant) -> Callable:
	return func(c: Variant) -> Variant:
		return f.call(a, b, c)

## Memoizes a pure function across ALL inputs (keyed by stringified args).
## FACTORY: returns a Dictionary with a call_args(args) closure, like memoize_last.
##
## Signature: memoize(f: Callable) -> Dictionary
##
## User Stories:
##   - As a performance-minded author, I cache a pure function so repeated calls with the same arguments skip recomputation.
static func memoize(f: Callable) -> Dictionary:
	var cache_ref = { "cache": {} }
	return {
		"call_args": func(args: Array) -> Variant:
			var key = str(args)
			match cache_ref.cache.has(key):
				true: return cache_ref.cache[key]
				_:
					var result: Variant = f.callv(args)
					cache_ref.cache[key] = result
					return result
	}

## Runs a side effect on a value and returns it unchanged (chain-friendly).
##
## Signature: tap(f: Callable) -> Callable
##
## User Stories:
##   - As a pipeline author, I insert logging or assertions into a pipe chain without breaking the data flow.
static func tap(f: Callable) -> Callable:
	return func(value: Variant) -> Variant:
		f.call(value)
		return value

## Mutable-reference variant of tap, kept for parity with the Rust/C++ seam.
##
## Signature: tap_mut(f: Callable) -> Callable
##
## User Stories:
##   - As a pipeline author, I need a functional hook to mutate state externally inside an immutable chain.
static func tap_mut(f: Callable) -> Callable:
	return func(value: Variant) -> Variant:
		f.call(value)
		return value

## Builds a "call again" trampoline step carrying the next argument.
##
## Signature: bounce_call(arg: Variant) -> Dictionary
##
## User Stories:
##   - As an author of deep recursion, I want to defer my next iterative call to the trampoline evaluator to prevent call stack overflow.
static func bounce_call(arg: Variant) -> Dictionary:
	return { "_tag": "Call", "arg": arg }

## Builds a "done" trampoline step carrying the final result.
##
## Signature: bounce_done(result: Variant) -> Dictionary
##
## User Stories:
##   - As an author of deep recursion, I want to indicate that my loop has resolved into its final value.
static func bounce_done(result: Variant) -> Dictionary:
	return { "_tag": "Done", "result": result }

## Iterates a step function on the heap until it returns a Done bounce, so deep
## recursion never grows the native call stack.
##
## Signature: trampoline(initial: Variant, step: Callable) -> Variant
##
## User Stories:
##   - As an author of deep recursion, I trampoline a step function so large iteration counts run without a stack overflow.
static func trampoline(initial: Variant, step: Callable) -> Variant:
	var current: Dictionary = step.call(initial)
	while current._tag == "Call":
		current = step.call(current.arg)
	return current.result
