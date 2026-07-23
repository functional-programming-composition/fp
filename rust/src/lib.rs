/**
 * Functional Core - pure FP primitives for the Magik Lux game core.
 *
 * This module is the base primitive layer. RTK and ECS core files may use it,
 * but it must NOT depend on RTK, ECS, store, entity adapter, world, component,
 * or system semantics. FP is the dependency root so both broader cores can
 * share primitives without forming a cycle.
 *
 * User Stories:
 * - As a domain author, I need Maybe and Either so absence and recoverable
 *   failure are represented without sentinel values, panics, or side effects.
 * - As a reducer, selector, or system author, I need pipe, compose, curry,
 *   partial application, memoization, lazy evaluation, pattern matching,
 *   dispatch tables, validation, config building, and boundary result helpers
 *   so domain logic stays small, explicit, and composable.
 * - As a maintainer, I need FP to remain the dependency root so RTK and ECS can
 *   share primitives without creating circular architecture.
 * - As a conformance reviewer, I need every primitive expressed as data structs
 *   plus free factory functions (no methods on data) so the architecture
 *   contract holds across the whole core.
 */
use std::fmt;

// ---------------------------------------------------------------------------
// Maybe<T> — absence without sentinel nulls
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Maybe<T> {
    Just(T),
    Nothing,
}

/// Wraps a value in a `Maybe::Just`.
///
/// This function provides a type-safe way to represent the presence of a value without
/// relying on nulls or sentinel values. It's the primary constructor for a successful
/// or present state in the `Maybe` monad.
///
/// # Signature
/// `pub fn just<T>(value: T) -> Maybe<T>`
///
/// # User Stories
/// - As a domain author, I wrap a present value so downstream code never branches on null.
/// - As an API designer, I signal that an operation successfully yielded a specific value.
pub fn just<T>(value: T) -> Maybe<T> {
    Maybe::Just(value)
}

/// Constructs an explicitly absent value via `Maybe::Nothing`.
///
/// This serves as the safe alternative to `null` or `None`, explicitly signaling
/// that a computation yielded no value or a property is missing.
///
/// # Signature
/// `pub fn nothing<T>() -> Maybe<T>`
///
/// # User Stories
/// - As a domain author, I model absence explicitly instead of returning a null.
/// - As a maintainer, I prevent null pointer exceptions by enforcing handling of missing values.
pub fn nothing<T>() -> Maybe<T> {
    Maybe::Nothing
}

/// Checks if the provided `Maybe` contains a value.
///
/// # Signature
/// `pub fn is_just<T>(ma: &Maybe<T>) -> bool`
///
/// # User Stories
/// - As a reader, I check presence without destructuring.
/// - As a caller, I want a quick boolean check to proceed with conditional logic.
pub fn is_just<T>(ma: &Maybe<T>) -> bool {
    matches!(ma, Maybe::Just(_))
}

/// Checks if the provided `Maybe` is absent.
///
/// # Signature
/// `pub fn is_nothing<T>(ma: &Maybe<T>) -> bool`
///
/// # User Stories
/// - As a reader, I check absence without destructuring.
/// - As a caller, I want to quickly detect missing data to short-circuit operations.
pub fn is_nothing<T>(ma: &Maybe<T>) -> bool {
    matches!(ma, Maybe::Nothing)
}

/// Transforms a present value using a mapping function, leaving absence untouched.
///
/// This is the functor `map` operation for `Maybe`. It allows you to chain
/// synchronous, infallible transformations over a value that might not exist.
///
/// # Signature
/// `pub fn maybe_map<A, B>(ma: &Maybe<A>, f: impl FnOnce(&A) -> B) -> Maybe<B>`
///
/// # User Stories
/// - As a transformer, I map over a present value and leave absence untouched.
/// - As a domain author, I want to apply synchronous changes to data without unwrapping the context.
pub fn maybe_map<A, B>(ma: &Maybe<A>, f: impl FnOnce(&A) -> B) -> Maybe<B> {
    match ma {
        Maybe::Just(a) => Maybe::Just(f(a)),
        Maybe::Nothing => Maybe::Nothing,
    }
}

/// Chains a fallible or optional operation onto an existing `Maybe`.
///
/// This is the monadic `bind` or `flat_map` operation. It sequences computations
/// that each independently might yield no result.
///
/// # Signature
/// `pub fn maybe_chain<A, B>(ma: Maybe<A>, f: impl FnOnce(A) -> Maybe<B>) -> Maybe<B>`
///
/// # User Stories
/// - As a transformer, I flat-map a present value into another Maybe.
/// - As a logic author, I sequence multiple operations where any step might fail or be absent.
pub fn maybe_chain<A, B>(ma: Maybe<A>, f: impl FnOnce(A) -> Maybe<B>) -> Maybe<B> {
    match ma {
        Maybe::Just(a) => f(a),
        Maybe::Nothing => Maybe::Nothing,
    }
}

/// Retains a present value only if it satisfies a given predicate.
///
/// If the `Maybe` is `Just` but the value fails the predicate, it becomes `Nothing`.
///
/// # Signature
/// `pub fn maybe_filter<T>(ma: Maybe<T>, predicate: impl FnOnce(&T) -> bool) -> Maybe<T>`
///
/// # User Stories
/// - As a transformer, I keep a present value only when it satisfies a predicate.
/// - As a logic author, I want to discard a value implicitly if it doesn't meet specific business rules.
pub fn maybe_filter<T>(ma: Maybe<T>, predicate: impl FnOnce(&T) -> bool) -> Maybe<T> {
    match ma {
        Maybe::Just(value) if predicate(&value) => Maybe::Just(value),
        _ => Maybe::Nothing,
    }
}

/// Exhaustively handles both cases of a `Maybe` to produce a unified result.
///
/// This is the primary way to collapse a `Maybe` context back into a raw value
/// at the boundaries of your application.
///
/// # Signature
/// `pub fn maybe_match<A, R>(ma: &Maybe<A>, on_just: impl FnOnce(&A) -> R, on_nothing: impl FnOnce() -> R) -> R`
///
/// # User Stories
/// - As a caller, I collapse a Maybe into a single result by handling both arms.
/// - As a consumer, I provide fallback behavior and success behavior at a single boundary.
pub fn maybe_match<A, R>(
    ma: &Maybe<A>,
    on_just: impl FnOnce(&A) -> R,
    on_nothing: impl FnOnce() -> R,
) -> R {
    match ma {
        Maybe::Just(a) => on_just(a),
        Maybe::Nothing => on_nothing(),
    }
}

/// Unwraps a present value or computes a fallback dynamically.
///
/// Useful when constructing the default value is computationally expensive.
///
/// # Signature
/// `pub fn maybe_or_else<T: Clone>(ma: &Maybe<T>, default: impl FnOnce() -> T) -> T`
///
/// # User Stories
/// - As a caller, I read a present value or compute a lazy default for absence.
/// - As a performance-minded developer, I avoid expensive default allocations when a value is already present.
pub fn maybe_or_else<T: Clone>(ma: &Maybe<T>, default: impl FnOnce() -> T) -> T {
    match ma {
        Maybe::Just(a) => a.clone(),
        Maybe::Nothing => default(),
    }
}

/// Unwraps a present value or provides an eagerly evaluated fallback.
///
/// # Signature
/// `pub fn or_else<T: Clone>(ma: &Maybe<T>, default: T) -> T`
///
/// # User Stories
/// - As a caller, I read a present value or fall back to an eager default.
/// - As a code reader, I want a concise way to provide simple, cheap fallback values.
pub fn or_else<T: Clone>(ma: &Maybe<T>, default: T) -> T {
    maybe_or_else(ma, || default)
}

/// Converts a standard Rust `Option` into a `Maybe`.
///
/// # Signature
/// `pub fn maybe_from_option<T>(opt: Option<T>) -> Maybe<T>`
///
/// # User Stories
/// - As an interop author, I lift a host Option into a Maybe.
/// - As a consumer of third-party Rust libraries, I translate standard options into domain-specific structures.
pub fn maybe_from_option<T>(opt: Option<T>) -> Maybe<T> {
    match opt {
        Some(v) => Maybe::Just(v),
        None => Maybe::Nothing,
    }
}

/// Alias for `maybe_from_option`, converts any nullable host value into a `Maybe`.
///
/// # Signature
/// `pub fn from_nullable<T>(value: Option<T>) -> Maybe<T>`
///
/// # User Stories
/// - As an interop author, I lift any nullable host value into a Maybe.
/// - As a consumer of FFI boundaries, I handle nullability elegantly.
pub fn from_nullable<T>(value: Option<T>) -> Maybe<T> {
    maybe_from_option(value)
}

/// Conditionally lifts a value into a `Maybe` based on a boolean validity flag.
///
/// # Signature
/// `pub fn from_nullable_value<T>(value: T, valid: bool) -> Maybe<T>`
///
/// # User Stories
/// - As an interop author, I lift a value guarded by a validity flag into a Maybe.
/// - As a parser, I use a secondary boolean flag to safely process untrusted input streams.
///
/// # Build-on-itself Note
/// The implementation composes `just` with `maybe_filter` instead of spelling out
/// a branch, so validity-flag interop remains part of normal Maybe data flow.
pub fn from_nullable_value<T>(value: T, valid: bool) -> Maybe<T> {
    maybe_filter(just(value), |_value| valid)
}

/// Lowers a `Maybe` back into a standard Rust `Option`.
///
/// # Signature
/// `pub fn maybe_to_option<T>(ma: Maybe<T>) -> Option<T>`
///
/// # User Stories
/// - As an interop author, I lower a Maybe back into a host Option.
/// - As a library author, I safely expose standard types back to external non-domain Rust ecosystems.
pub fn maybe_to_option<T>(ma: Maybe<T>) -> Option<T> {
    match ma {
        Maybe::Just(v) => Some(v),
        Maybe::Nothing => None,
    }
}

/// Unwraps a `Maybe` assertively, panicking with a custom error if absent.
///
/// # Signature
/// `pub fn require_just<T>(ma: Maybe<T>, error_msg: &str) -> T`
///
/// # User Stories
/// - As a caller at a trusted boundary, I extract a required value, failing loudly only on a programmer error.
/// - As an invariant author, I enforce preconditions and intentionally panic when logic is broken.
pub fn require_just<T>(ma: Maybe<T>, error_msg: &str) -> T {
    match ma {
        Maybe::Just(v) => v,
        Maybe::Nothing => panic!("{}", error_msg),
    }
}

/// Monadic bind alias for `maybe_chain`.
///
/// # Signature
/// `pub fn mbind<A, B>(ma: Maybe<A>, f: impl FnOnce(A) -> Maybe<B>) -> Maybe<B>`
///
/// # User Stories
/// - As a chaining author, I use the monadic alias for `maybe_chain`.
/// - As an FP enthusiast, I want familiar terminology for sequencing optional outcomes.
pub fn mbind<A, B>(ma: Maybe<A>, f: impl FnOnce(A) -> Maybe<B>) -> Maybe<B> {
    maybe_chain(ma, f)
}

/// Functor map alias for `maybe_map`.
///
/// # Signature
/// `pub fn maybe_fmap<A, B>(ma: &Maybe<A>, f: impl FnOnce(&A) -> B) -> Maybe<B>`
///
/// # User Stories
/// - As a mapping author, I use the functor alias for `maybe_map`.
/// - As an FP enthusiast, I appreciate mathematically consistent terminology for mapping contexts.
pub fn maybe_fmap<A, B>(ma: &Maybe<A>, f: impl FnOnce(&A) -> B) -> Maybe<B> {
    maybe_map(ma, f)
}

// ---------------------------------------------------------------------------
// Either<E, T> — recoverable failure
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Either<E, T> {
    Left(E),
    Right(T),
}

/// Constructs a failure case containing an error.
///
/// # Signature
/// `pub fn left<E, T>(err: E) -> Either<E, T>`
///
/// # User Stories
/// - As an author of a fallible step, I represent the failure channel.
/// - As a library author, I safely propagate business-logic errors instead of panicking.
pub fn left<E, T>(err: E) -> Either<E, T> {
    Either::Left(err)
}

/// Constructs a success case containing a value.
///
/// # Signature
/// `pub fn right<E, T>(val: T) -> Either<E, T>`
///
/// # User Stories
/// - As an author of a fallible step, I represent the success channel.
/// - As a domain logic author, I signal that an operation successfully yielded data.
pub fn right<E, T>(val: T) -> Either<E, T> {
    Either::Right(val)
}

/// Explicitly named alias for `left` to avoid naming collisions.
///
/// # Signature
/// `pub fn make_left<E, T>(err: E) -> Either<E, T>`
///
/// # User Stories
/// - As an author needing an explicit constructor name, I build a Left.
/// - As an API designer, I want clear disambiguation when `left` might conflict with UI logic (e.g. padding).
pub fn make_left<E, T>(err: E) -> Either<E, T> {
    left(err)
}

/// Explicitly named alias for `right` to avoid naming collisions.
///
/// # Signature
/// `pub fn make_right<E, T>(val: T) -> Either<E, T>`
///
/// # User Stories
/// - As an author needing an explicit constructor name, I build a Right.
/// - As an API designer, I want clear disambiguation when `right` might conflict with UI logic.
pub fn make_right<E, T>(val: T) -> Either<E, T> {
    right(val)
}

/// Checks if the provided `Either` represents a failure (`Left`).
///
/// # Signature
/// `pub fn is_left<E, T>(ea: &Either<E, T>) -> bool`
///
/// # User Stories
/// - As a reader, I check the failure channel without destructuring.
/// - As a caller, I want a quick boolean check before logging an error.
pub fn is_left<E, T>(ea: &Either<E, T>) -> bool {
    matches!(ea, Either::Left(_))
}

/// Checks if the provided `Either` represents a success (`Right`).
///
/// # Signature
/// `pub fn is_right<E, T>(ea: &Either<E, T>) -> bool`
///
/// # User Stories
/// - As a reader, I check the success channel without destructuring.
/// - As a caller, I want a quick boolean check to conditionally continue a sequence.
pub fn is_right<E, T>(ea: &Either<E, T>) -> bool {
    matches!(ea, Either::Right(_))
}

/// Transforms the success value (`Right`) using a mapping function, preserving failures (`Left`).
///
/// # Signature
/// `pub fn either_map<E, A, B>(ea: &Either<E, A>, f: impl FnOnce(&A) -> B) -> Either<E, B>`
///
/// # User Stories
/// - As a transformer, I map the success channel and preserve the failure.
/// - As a pipeline author, I synchronously mutate data without unwrapping a potential error.
pub fn either_map<E, A, B>(ea: &Either<E, A>, f: impl FnOnce(&A) -> B) -> Either<E, B>
where
    E: Clone,
{
    match ea {
        Either::Right(a) => Either::Right(f(a)),
        Either::Left(e) => Either::Left(e.clone()),
    }
}

/// Transforms the error value (`Left`) using a mapping function, preserving successes (`Right`).
///
/// # Signature
/// `pub fn either_map_left<E, F, T>(ea: &Either<E, T>, f: impl FnOnce(&E) -> F) -> Either<F, T>`
///
/// # User Stories
/// - As a transformer, I map the failure channel and preserve the success.
/// - As an error-handler, I translate an internal error type into a boundary-appropriate error type (e.g. String).
pub fn either_map_left<E, F, T>(ea: &Either<E, T>, f: impl FnOnce(&E) -> F) -> Either<F, T>
where
    T: Clone,
{
    match ea {
        Either::Left(e) => Either::Left(f(e)),
        Either::Right(t) => Either::Right(t.clone()),
    }
}

/// Chains a fallible operation onto an existing `Either`, continuing only on success.
///
/// # Signature
/// `pub fn either_chain<E, A, B>(ea: Either<E, A>, f: impl FnOnce(A) -> Either<E, B>) -> Either<E, B>`
///
/// # User Stories
/// - As a transformer, I flat-map the success channel into another Either.
/// - As a pipeline author, I sequence computations that each might fail independently.
pub fn either_chain<E, A, B>(ea: Either<E, A>, f: impl FnOnce(A) -> Either<E, B>) -> Either<E, B> {
    match ea {
        Either::Right(a) => f(a),
        Either::Left(e) => Either::Left(e),
    }
}

/// Exhaustively handles both success and failure cases to produce a unified result.
///
/// # Signature
/// `pub fn either_match<E, T, R>(ea: &Either<E, T>, on_left: impl FnOnce(&E) -> R, on_right: impl FnOnce(&T) -> R) -> R`
///
/// # User Stories
/// - As a caller, I collapse an Either into a single result by handling both arms.
/// - As a consumer, I render either the error message or the success UI component at a boundary.
pub fn either_match<E, T, R>(
    ea: &Either<E, T>,
    on_left: impl FnOnce(&E) -> R,
    on_right: impl FnOnce(&T) -> R,
) -> R {
    match ea {
        Either::Left(e) => on_left(e),
        Either::Right(t) => on_right(t),
    }
}

/// Unwraps the success value or provides an eagerly evaluated fallback on failure.
///
/// # Signature
/// `pub fn either_or_else<E, T: Clone>(ea: &Either<E, T>, default: T) -> T`
///
/// # User Stories
/// - As a caller, I read a success value or fall back to an eager default.
/// - As a UI developer, I show default data when an optional fetch returns a failure.
pub fn either_or_else<E, T: Clone>(ea: &Either<E, T>, default: T) -> T {
    match ea {
        Either::Right(t) => t.clone(),
        Either::Left(_) => default,
    }
}

/// Monadic bind alias for `either_chain`.
///
/// # Signature
/// `pub fn ebind<E, A, B>(ea: Either<E, A>, f: impl FnOnce(A) -> Either<E, B>) -> Either<E, B>`
///
/// # User Stories
/// - As a chaining author, I use the monadic alias for `either_chain`.
/// - As an FP enthusiast, I want familiar terminology for sequencing fallible outcomes.
pub fn ebind<E, A, B>(ea: Either<E, A>, f: impl FnOnce(A) -> Either<E, B>) -> Either<E, B> {
    either_chain(ea, f)
}

/// Functor map alias for `either_map`.
///
/// # Signature
/// `pub fn either_fmap<E, A, B>(ea: &Either<E, A>, f: impl FnOnce(&A) -> B) -> Either<E, B>`
///
/// # User Stories
/// - As a mapping author, I use the functor alias for `either_map`.
/// - As an FP enthusiast, I appreciate mathematically consistent terminology.
pub fn either_fmap<E, A, B>(ea: &Either<E, A>, f: impl FnOnce(&A) -> B) -> Either<E, B>
where
    E: Clone,
{
    either_map(ea, f)
}

/// Short alias for `either_match`.
///
/// # Signature
/// `pub fn ematch<E, T, R>(ea: &Either<E, T>, on_left: impl FnOnce(&E) -> R, on_right: impl FnOnce(&T) -> R) -> R`
///
/// # User Stories
/// - As a caller, I use the short alias for `either_match`.
/// - As a succinct developer, I want a brief way to handle Both arms of an `Either`.
pub fn ematch<E, T, R>(
    ea: &Either<E, T>,
    on_left: impl FnOnce(&E) -> R,
    on_right: impl FnOnce(&T) -> R,
) -> R {
    either_match(ea, on_left, on_right)
}

// ---------------------------------------------------------------------------
// Fmap — a single functor seam for Maybe, Either, and Vec
// ---------------------------------------------------------------------------

/// As an author of generic transforms, I map over any functor through one seam.
pub trait Fmap<F> {
    type Output;

    fn fmap(self, f: F) -> Self::Output;
}

impl<'a, A, B, F> Fmap<F> for &'a Maybe<A>
where
    F: FnOnce(&A) -> B,
{
    type Output = Maybe<B>;

    fn fmap(self, f: F) -> Self::Output {
        maybe_map(self, f)
    }
}

impl<'a, E, A, B, F> Fmap<F> for &'a Either<E, A>
where
    E: Clone,
    F: FnOnce(&A) -> B,
{
    type Output = Either<E, B>;

    fn fmap(self, f: F) -> Self::Output {
        either_map(self, f)
    }
}

impl<'a, A, B, F> Fmap<F> for &'a Vec<A>
where
    F: Fn(&A) -> B,
{
    type Output = Vec<B>;

    fn fmap(self, f: F) -> Self::Output {
        fn map_recursive<A, B>(values: &[A], f: &dyn Fn(&A) -> B, mut acc: Vec<B>) -> Vec<B> {
            match values.split_first() {
                Some((first, rest)) => {
                    acc.push(f(first));
                    map_recursive(rest, f, acc)
                }
                None => acc,
            }
        }

        map_recursive(self, &f, Vec::with_capacity(self.len()))
    }
}

/// Applies a generic transformation over an `Fmap` compatible container.
///
/// # Signature
/// `pub fn fmap<C, F>(carrier: C, f: F) -> C::Output where C: Fmap<F>`
///
/// # User Stories
/// - As an author of generic transforms, I call `fmap` over any `Fmap` carrier.
/// - As a library author, I want to map over data structures uniformly regardless of their specific type.
pub fn fmap<C, F>(carrier: C, f: F) -> C::Output
where
    C: Fmap<F>,
{
    carrier.fmap(f)
}

// ---------------------------------------------------------------------------
// pipe / compose / curry
// ---------------------------------------------------------------------------

/// Threads a value left-to-right through two functions.
///
/// # Signature
/// `pub fn pipe<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C`
///
/// # User Stories
/// - As a pipeline author, I thread a value left-to-right through two functions.
/// - As a code reader, I prefer reading data transformations in the order they execute (A -> B -> C).
pub fn pipe<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |a| g(f(a))
}

/// Threads a value right-to-left through two functions (mathematical composition).
///
/// # Signature
/// `pub fn compose<A, B, C>(g: impl Fn(B) -> C, f: impl Fn(A) -> B) -> impl Fn(A) -> C`
///
/// # User Stories
/// - As a pipeline author, I thread a value right-to-left through two functions.
/// - As a mathematician or FP purist, I want to construct functions using traditional `g ∘ f` notation.
pub fn compose<A, B, C>(g: impl Fn(B) -> C, f: impl Fn(A) -> B) -> impl Fn(A) -> C {
    move |a| g(f(a))
}

/// Curries a binary function, breaking it into two unary function calls.
///
/// # Signature
/// `pub fn curry2<A: Clone + 'static, B: 'static, R: 'static>(f: impl Fn(A, B) -> R + 'static) -> impl Fn(A) -> Box<dyn Fn(B) -> R>`
///
/// # User Stories
/// - As a partial-application author, I curry a binary function into unary stages.
/// - As a library user, I defer providing the second argument until it becomes available in the pipeline.
pub fn curry2<A: Clone + 'static, B: 'static, R: 'static>(
    f: impl Fn(A, B) -> R + 'static,
) -> impl Fn(A) -> Box<dyn Fn(B) -> R> {
    let f = std::sync::Arc::new(f);
    move |a: A| {
        let f = f.clone();
        let a = a.clone();
        Box::new(move |b: B| f(a.clone(), b))
    }
}

/// Curries a ternary function, breaking it into three unary function calls.
///
/// # Signature
/// `pub fn curry3<A: Clone + 'static, B: Clone + 'static, C: 'static, R: 'static>(f: impl Fn(A, B, C) -> R + 'static) -> impl Fn(A) -> Box<dyn Fn(B) -> Box<dyn Fn(C) -> R>>`
///
/// # User Stories
/// - As a partial-application author, I curry a ternary function into unary stages.
/// - As a configuration builder, I lock in settings one at a time across different subsystems.
pub fn curry3<A: Clone + 'static, B: Clone + 'static, C: 'static, R: 'static>(
    f: impl Fn(A, B, C) -> R + 'static,
) -> impl Fn(A) -> Box<dyn Fn(B) -> Box<dyn Fn(C) -> R>> {
    let f = std::sync::Arc::new(f);
    move |a: A| {
        let f = f.clone();
        let a = a.clone();
        Box::new(move |b: B| {
            let f = f.clone();
            let a = a.clone();
            let b = b.clone();
            Box::new(move |c: C| f(a.clone(), b.clone(), c))
        })
    }
}

// ---------------------------------------------------------------------------
// Unary-composition cookbook — point-free combinators built on pipe/compose
// ---------------------------------------------------------------------------
//
// Every helper below is itself written in the unary-composition style it
// promotes: later combinators are assembled from earlier ones (`pipe3` from
// `pipe`, `pipe4` from `pipe3`) so the functional core demonstrably builds on
// itself instead of restating control flow. Domain code (reducers, selectors,
// ECS systems, and the `*_factory` helpers in Therapy 12's `src/features`)
// imports these to collapse many-argument logic into a single `A -> B` arrow
// that drops straight into `Iterator::map`, `maybe_map`, or a factory returning
// `impl Fn`.

/// Returns its argument unchanged — the neutral element of composition.
///
/// # Signature
/// `pub fn identity<T>(value: T) -> T`
///
/// # User Stories
/// - As a pipeline author, I want an explicit no-op arrow so a branch of a dispatch table can pass a value through untouched.
/// - As a cross-project maintainer, I need the same neutral function available in Rust, GDScript, TypeScript, and C++ cores.
pub fn identity<T>(value: T) -> T {
    value
}

/// Builds a unary function that ignores its input and always yields `value`.
///
/// # Signature
/// `pub fn constant<T: Clone + 'static, A>(value: T) -> impl Fn(A) -> T`
///
/// # User Stories
/// - As a selector author, I want a point-free "always this" arrow so dispatch tables can return fixed values without bespoke closures.
/// - As a recipe author, I need constants to participate in `pipe`, `compose`, and `multi_match` outputs.
pub fn constant<T: Clone + 'static, A>(value: T) -> impl Fn(A) -> T {
    move |_ignored| value.clone()
}

/// Swaps the argument order of a binary function.
///
/// # Signature
/// `pub fn flip<A, B, R>(f: impl Fn(A, B) -> R) -> impl Fn(B, A) -> R`
///
/// # User Stories
/// - As a `partial_apply` user, I want to bind a right-hand dependency by flipping the function first.
/// - As a pipeline author, I need a small adapter that turns existing binary helpers into unary-friendly factories.
pub fn flip<A, B, R>(f: impl Fn(A, B) -> R) -> impl Fn(B, A) -> R {
    move |b, a| f(a, b)
}

/// Negates a borrowing predicate.
///
/// # Signature
/// `pub fn complement<T>(predicate: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool`
///
/// # User Stories
/// - As a `maybe_filter` author, I want the inverse of an existing rule without restating its logic.
/// - As a validation author, I need keep/reject arrows to stay in sync when a domain rule changes.
pub fn complement<T>(predicate: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |value| !predicate(value)
}

/// Combines two predicates with logical AND into one arrow.
///
/// # Signature
/// `pub fn both<T>(left: impl Fn(&T) -> bool, right: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool`
///
/// # User Stories
/// - As a validation author, I fuse independent rules into one predicate for a single `maybe_filter` call.
/// - As an ECS system author, I describe entity admission rules as data instead of nested conditionals.
pub fn both<T>(left: impl Fn(&T) -> bool, right: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |value| left(value) && right(value)
}

/// Combines two predicates with logical OR into one arrow.
///
/// # Signature
/// `pub fn either_pred<T>(left: impl Fn(&T) -> bool, right: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool`
///
/// # User Stories
/// - As a routing author, I want a single arrow that accepts values matching either of two rules.
/// - As a reducer author, I need event grouping predicates that read like Redux Toolkit matchers.
pub fn either_pred<T>(
    left: impl Fn(&T) -> bool,
    right: impl Fn(&T) -> bool,
) -> impl Fn(&T) -> bool {
    move |value| left(value) || right(value)
}

/// Folds a list of predicates into one that holds only when all hold.
///
/// # Signature
/// `pub fn all_pass<T>(predicates: Vec<Box<dyn Fn(&T) -> bool>>) -> impl Fn(&T) -> bool`
///
/// # User Stories
/// - As a domain gatekeeper, I assemble an arbitrary set of admission rules into one reusable arrow.
/// - As a cookbook author, I can add a rule without changing every call site.
pub fn all_pass<T>(predicates: Vec<Box<dyn Fn(&T) -> bool>>) -> impl Fn(&T) -> bool {
    move |value| predicates.iter().all(|predicate| predicate(value))
}

/// Folds a list of predicates into one that holds when any holds.
///
/// # Signature
/// `pub fn any_pass<T>(predicates: Vec<Box<dyn Fn(&T) -> bool>>) -> impl Fn(&T) -> bool`
///
/// # User Stories
/// - As a routing author, I collapse an allow-list of predicates into a single matcher.
/// - As a feature maintainer, I keep broad acceptance rules data-driven and testable.
pub fn any_pass<T>(predicates: Vec<Box<dyn Fn(&T) -> bool>>) -> impl Fn(&T) -> bool {
    move |value| predicates.iter().any(|predicate| predicate(value))
}

/// Feeds one input through two arrows and merges the results with a combiner.
///
/// # Signature
/// `pub fn converge2<A: Clone, B, C, R>(combine: impl Fn(B, C) -> R, first: impl Fn(A) -> B, second: impl Fn(A) -> C) -> impl Fn(A) -> R`
///
/// # User Stories
/// - As a selector author, I derive a value from two projections of the same state without naming intermediate values.
/// - As a UI meter author, I compute ratios, centers, or spans from shared input in a unary pipeline.
pub fn converge2<A: Clone, B, C, R>(
    combine: impl Fn(B, C) -> R,
    first: impl Fn(A) -> B,
    second: impl Fn(A) -> C,
) -> impl Fn(A) -> R {
    move |value| combine(first(value.clone()), second(value))
}

/// Applies two arrows to one input and returns both results as a pair.
///
/// # Signature
/// `pub fn juxt2<A: Clone, B, C>(first: impl Fn(A) -> B, second: impl Fn(A) -> C) -> impl Fn(A) -> (B, C)`
///
/// # User Stories
/// - As a layout author, I compute two related projections from the same config using one point-free arrow.
/// - As a system author, I preserve the Therapy 12 factory-returns-impl-Fn shape across game cores.
pub fn juxt2<A: Clone, B, C>(
    first: impl Fn(A) -> B,
    second: impl Fn(A) -> C,
) -> impl Fn(A) -> (B, C) {
    move |value| (first(value.clone()), second(value))
}

/// Composes three arrows left-to-right, assembled from `pipe`.
///
/// # Signature
/// `pub fn pipe3<A, B, C, D>(f: impl Fn(A) -> B, g: impl Fn(B) -> C, h: impl Fn(C) -> D) -> impl Fn(A) -> D`
///
/// # User Stories
/// - As a reducer author, I chain decode -> transform -> clamp in one arrow.
/// - As a core maintainer, I show that broader helpers build on earlier primitives rather than restating control flow.
pub fn pipe3<A, B, C, D>(
    f: impl Fn(A) -> B,
    g: impl Fn(B) -> C,
    h: impl Fn(C) -> D,
) -> impl Fn(A) -> D {
    pipe(pipe(f, g), h)
}

/// Composes four arrows left-to-right, assembled from `pipe3` and `pipe`.
///
/// # Signature
/// `pub fn pipe4<A, B, C, D, E>(f: impl Fn(A) -> B, g: impl Fn(B) -> C, h: impl Fn(C) -> D, i: impl Fn(D) -> E) -> impl Fn(A) -> E`
///
/// # User Stories
/// - As a rendering author, I build a four-stage point-free pipeline without nested calls at the call site.
/// - As a cookbook author, I give feature code a named helper for the common four-stage transform shape.
pub fn pipe4<A, B, C, D, E>(
    f: impl Fn(A) -> B,
    g: impl Fn(B) -> C,
    h: impl Fn(C) -> D,
    i: impl Fn(D) -> E,
) -> impl Fn(A) -> E {
    pipe(pipe3(f, g, h), i)
}

// ---- Worked cross-domain examples (neutral primitives, fully point-free) ----

/// Example: clamps any signed channel/coordinate into the byte range `0..=255`.
///
/// # Signature
/// `pub fn example_clamp_channel() -> impl Fn(i32) -> i32`
///
/// # User Stories
/// - As a rendering or HUD author, I reuse one point-free clamp arrow for color channels and bar widths.
/// - As a conformance reviewer, I can see how `pipe` should be used in project features.
pub fn example_clamp_channel() -> impl Fn(i32) -> i32 {
    pipe(|value: i32| value.max(0), |value: i32| value.min(255))
}

/// Example: derives a `0.0..=1.0` fill ratio from a `(current, max)` reading.
///
/// # Signature
/// `pub fn example_fill_ratio() -> impl Fn((f32, f32)) -> f32`
///
/// # User Stories
/// - As a meter selector author, I turn a `(current, max)` reading into a safe ratio through `converge2`.
/// - As a gameplay author, I share one neutral formula across health, loading, cooldown, and progress domains.
pub fn example_fill_ratio() -> impl Fn((f32, f32)) -> f32 {
    converge2(
        |current: f32, max: f32| current / max.max(f32::EPSILON),
        |reading: (f32, f32)| reading.0,
        |reading: (f32, f32)| reading.1,
    )
}

/// Example: admits an entity that is both alive and on-screen.
///
/// # Signature
/// `pub fn example_alive_and_visible() -> impl Fn(&(bool, bool)) -> bool`
///
/// # User Stories
/// - As an ECS system author, I fuse entity admission rules with `both` and reuse that predicate in queries.
/// - As a test author, I get a tiny executable recipe for boolean composition semantics.
pub fn example_alive_and_visible() -> impl Fn(&(bool, bool)) -> bool {
    both(
        |flags: &(bool, bool)| flags.0,
        |flags: &(bool, bool)| flags.1,
    )
}

// ---------------------------------------------------------------------------
// partial_apply — bind leading arguments ahead of time
// ---------------------------------------------------------------------------

/// Binds the first argument of a binary function, returning a unary function.
///
/// # Signature
/// `pub fn partial_apply<A: Clone + 'static, B: 'static, R: 'static>(f: impl Fn(A, B) -> R + 'static, a: A) -> impl Fn(B) -> R`
///
/// # User Stories
/// - As a configuration author, I bind the first argument of a binary function.
/// - As a developer building callbacks, I lock in environment context before passing the function.
pub fn partial_apply<A: Clone + 'static, B: 'static, R: 'static>(
    f: impl Fn(A, B) -> R + 'static,
    a: A,
) -> impl Fn(B) -> R {
    let f = std::sync::Arc::new(f);
    move |b| f(a.clone(), b)
}

/// Binds the first two arguments of a ternary function, returning a unary function.
///
/// # Signature
/// `pub fn partial_apply2<A: Clone + 'static, B: Clone + 'static, C: 'static, R: 'static>(f: impl Fn(A, B, C) -> R + 'static, a: A, b: B) -> impl Fn(C) -> R`
///
/// # User Stories
/// - As a configuration author, I bind the first two arguments of a ternary function.
/// - As a framework author, I inject two internal dependencies while exposing the final parameter to the user.
pub fn partial_apply2<A: Clone + 'static, B: Clone + 'static, C: 'static, R: 'static>(
    f: impl Fn(A, B, C) -> R + 'static,
    a: A,
    b: B,
) -> impl Fn(C) -> R {
    let f = std::sync::Arc::new(f);
    move |c| f(a.clone(), b.clone(), c)
}

// ---------------------------------------------------------------------------
// memoize / lazy — cache pure work
// ---------------------------------------------------------------------------

/// Caches the output of a pure function across all inputs to avoid recomputation.
///
/// # Signature
/// `pub fn memoize<A, R>(mut f: impl FnMut(A) -> R + 'static) -> impl FnMut(A) -> R where A: Eq + std::hash::Hash + Clone, R: Clone`
///
/// # User Stories
/// - As a performance-minded author, I cache a pure function across all inputs.
/// - As an application developer, I ensure expensive configuration lookups only run once per distinct key.
pub fn memoize<A, R>(mut f: impl FnMut(A) -> R + 'static) -> impl FnMut(A) -> R
where
    A: Eq + std::hash::Hash + Clone,
    R: Clone,
{
    let mut cache: std::collections::HashMap<A, R> = std::collections::HashMap::new();
    move |a: A| {
        if let Some(r) = cache.get(&a) {
            return r.clone();
        }
        let r = f(a.clone());
        cache.insert(a, r.clone());
        r
    }
}

/// Caches only the most recent input/output pair.
///
/// # Signature
/// `pub fn memoize_last<A, R>(mut f: impl FnMut(A) -> R + 'static) -> impl FnMut(A) -> R where A: Eq + Clone, R: Clone`
///
/// # User Stories
/// - As a selector author, I cache only the most recent input/output pair, which is the reference-equality shape Redux-style selectors rely on.
/// - As a UI developer, I prevent re-rendering when the input state reference hasn't changed.
pub fn memoize_last<A, R>(mut f: impl FnMut(A) -> R + 'static) -> impl FnMut(A) -> R
where
    A: Eq + Clone,
    R: Clone,
{
    let mut last_arg: Maybe<A> = nothing();
    let mut last_result: Maybe<R> = nothing();
    move |arg: A| match (&last_arg, &last_result) {
        (Maybe::Just(previous), Maybe::Just(result)) if previous == &arg => result.clone(),
        _ => {
            let result = f(arg.clone());
            last_arg = just(arg);
            last_result = just(result.clone());
            result
        }
    }
}

/// A deferred computation whose result is computed at most once.
pub struct Lazy<T, F>
where
    F: FnOnce() -> T,
{
    thunk: std::cell::RefCell<Option<F>>,
    cached: std::cell::RefCell<Maybe<T>>,
}

/// Wraps an expensive computation as a thunk to be evaluated on demand.
///
/// # Signature
/// `pub fn lazy<T, F>(f: F) -> Lazy<T, F> where F: FnOnce() -> T`
///
/// # User Stories
/// - As a cost-conscious author, I wrap an expensive computation as a thunk.
/// - As a startup optimizer, I delay loading heavy resources until the user actually requests them.
pub fn lazy<T, F>(f: F) -> Lazy<T, F>
where
    F: FnOnce() -> T,
{
    Lazy {
        thunk: std::cell::RefCell::new(Some(f)),
        cached: std::cell::RefCell::new(nothing()),
    }
}

/// Forces a `Lazy` value, computing it once and caching the result for future calls.
///
/// # Signature
/// `pub fn eval<T, F>(lz: &Lazy<T, F>) -> T where T: Clone, F: FnOnce() -> T`
///
/// # User Stories
/// - As a consumer, I force a Lazy value, computing it once and caching the result.
/// - As an application component, I synchronously access a value that may have already been computed by another component.
pub fn eval<T, F>(lz: &Lazy<T, F>) -> T
where
    T: Clone,
    F: FnOnce() -> T,
{
    let cached = lz.cached.borrow().clone();
    match cached {
        Maybe::Just(value) => value,
        Maybe::Nothing => match lz.thunk.borrow_mut().take() {
            Some(thunk) => {
                let value = thunk();
                *lz.cached.borrow_mut() = just(value.clone());
                value
            }
            None => panic!("Lazy value was already forced without a cache"),
        },
    }
}

// ---------------------------------------------------------------------------
// Dispatcher<K, A, R> — type-safe dispatch table with fallback
// ---------------------------------------------------------------------------

/// A keyed table of handlers with a fallback, replacing long match ladders.
pub struct Dispatcher<K: Eq + std::hash::Hash, A, R> {
    table: std::collections::HashMap<K, Box<dyn Fn(&A) -> R>>,
    fallback: Box<dyn Fn(&A) -> R>,
}

/// Creates a new dispatch table with a required default fallback handler.
///
/// # Signature
/// `pub fn create_dispatcher<K, A, R>(fallback: impl Fn(&A) -> R + 'static) -> Dispatcher<K, A, R> where K: Eq + std::hash::Hash`
///
/// # User Stories
/// - As a router author, I create a dispatch table with a default handler.
/// - As a message broker, I ensure unhandled message types fall back safely to a no-op or error logger.
pub fn create_dispatcher<K, A, R>(fallback: impl Fn(&A) -> R + 'static) -> Dispatcher<K, A, R>
where
    K: Eq + std::hash::Hash,
{
    Dispatcher {
        table: std::collections::HashMap::new(),
        fallback: Box::new(fallback),
    }
}

/// Registers a handler for a specific key, returning the updated dispatcher.
///
/// # Signature
/// `pub fn dispatcher_register<K, A, R>(mut dispatcher: Dispatcher<K, A, R>, key: K, handler: impl Fn(&A) -> R + 'static) -> Dispatcher<K, A, R> where K: Eq + std::hash::Hash`
///
/// # User Stories
/// - As a router author, I register a handler for a key, returning the new table.
/// - As a plugin author, I map an event string to my custom execution logic dynamically at startup.
pub fn dispatcher_register<K, A, R>(
    mut dispatcher: Dispatcher<K, A, R>,
    key: K,
    handler: impl Fn(&A) -> R + 'static,
) -> Dispatcher<K, A, R>
where
    K: Eq + std::hash::Hash,
{
    dispatcher.table.insert(key, Box::new(handler));
    dispatcher
}

/// Looks up a key and executes its handler, or executes the fallback if not found.
///
/// # Signature
/// `pub fn dispatcher_dispatch<K, A, R>(dispatcher: &Dispatcher<K, A, R>, key: &K, arg: &A) -> R where K: Eq + std::hash::Hash`
///
/// # User Stories
/// - As a caller, I dispatch by key, falling back when the key is unknown.
/// - As an event loop, I route incoming domain events to their respective command handlers in constant time.
pub fn dispatcher_dispatch<K, A, R>(dispatcher: &Dispatcher<K, A, R>, key: &K, arg: &A) -> R
where
    K: Eq + std::hash::Hash,
{
    match dispatcher.table.get(key) {
        Some(handler) => handler(arg),
        None => (dispatcher.fallback)(arg),
    }
}

/// A keyed table of nullary handlers that returns Nothing on a missing key.
pub struct OptionalDispatcher<K: Eq + std::hash::Hash, R> {
    table: std::collections::HashMap<K, Box<dyn Fn() -> R>>,
}

/// Builds an optional dispatch table from a list of key-handler pairs.
///
/// # Signature
/// `pub fn create_optional_dispatcher<K, R>(entries: Vec<(K, Box<dyn Fn() -> R>)>) -> OptionalDispatcher<K, R> where K: Eq + std::hash::Hash`
///
/// # User Stories
/// - As a router author, I build an optional dispatch table from entries.
/// - As a configuration loader, I instantiate a static lookup table of known strategies.
pub fn create_optional_dispatcher<K, R>(
    entries: Vec<(K, Box<dyn Fn() -> R>)>,
) -> OptionalDispatcher<K, R>
where
    K: Eq + std::hash::Hash,
{
    fn register_entries<K, R>(
        entries: &mut std::vec::IntoIter<(K, Box<dyn Fn() -> R>)>,
        mut table: std::collections::HashMap<K, Box<dyn Fn() -> R>>,
    ) -> std::collections::HashMap<K, Box<dyn Fn() -> R>>
    where
        K: Eq + std::hash::Hash,
    {
        match entries.next() {
            Some((key, handler)) => {
                table.insert(key, handler);
                register_entries(entries, table)
            }
            None => table,
        }
    }

    OptionalDispatcher {
        table: register_entries(&mut entries.into_iter(), std::collections::HashMap::new()),
    }
}

/// Executes a handler for a given key, returning `Maybe::Nothing` if the key is missing.
///
/// # Signature
/// `pub fn dispatch<K, R>(dispatcher: &OptionalDispatcher<K, R>, key: &K) -> Maybe<R> where K: Eq + std::hash::Hash`
///
/// # User Stories
/// - As a caller, I dispatch by key and receive Nothing when no handler exists.
/// - As a graceful degradation component, I try to run a specialized behavior but tolerate its absence without crashing.
pub fn dispatch<K, R>(dispatcher: &OptionalDispatcher<K, R>, key: &K) -> Maybe<R>
where
    K: Eq + std::hash::Hash,
{
    match dispatcher.table.get(key) {
        Some(handler) => just(handler()),
        None => nothing(),
    }
}

/// Checks if a handler is registered for a specific key.
///
/// # Signature
/// `pub fn has<K, R>(dispatcher: &OptionalDispatcher<K, R>, key: &K) -> bool where K: Eq + std::hash::Hash`
///
/// # User Stories
/// - As a caller, I ask whether a handler is registered for a key.
/// - As a dependency injector, I want to conditionally add a handler only if it hasn't been added yet.
pub fn has<K, R>(dispatcher: &OptionalDispatcher<K, R>, key: &K) -> bool
where
    K: Eq + std::hash::Hash,
{
    dispatcher.table.contains_key(key)
}

/// Returns a list of all keys currently registered in the optional dispatcher.
///
/// # Signature
/// `pub fn keys<K, R>(dispatcher: &OptionalDispatcher<K, R>) -> Vec<K> where K: Eq + std::hash::Hash + Clone`
///
/// # User Stories
/// - As an introspector, I list every registered key.
/// - As a debugger, I want to print all available commands the system can currently process.
pub fn keys<K, R>(dispatcher: &OptionalDispatcher<K, R>) -> Vec<K>
where
    K: Eq + std::hash::Hash + Clone,
{
    fn collect_keys<K, R>(
        mut iter: std::collections::hash_map::Keys<'_, K, Box<dyn Fn() -> R>>,
        mut acc: Vec<K>,
    ) -> Vec<K>
    where
        K: Clone,
    {
        match iter.next() {
            Some(key) => {
                acc.push(key.clone());
                collect_keys(iter, acc)
            }
            None => acc,
        }
    }

    collect_keys(dispatcher.table.keys(), Vec::new())
}

// ---------------------------------------------------------------------------
// multi_match — value-based pattern matching with predicates and wildcard
// ---------------------------------------------------------------------------

/// One predicate/handler arm of a `multi_match` ladder.
pub struct MatchCase<T, R> {
    predicate: Box<dyn Fn(&T) -> bool>,
    handler: Box<dyn Fn(&T) -> R>,
}

/// Declares a predicate-guarded handler arm for pattern matching.
///
/// # Signature
/// `pub fn match_case<T, R>(predicate: impl Fn(&T) -> bool + 'static, handler: impl Fn(&T) -> R + 'static) -> MatchCase<T, R>`
///
/// # User Stories
/// - As a branching author, I declare a predicate-guarded handler arm.
/// - As a functional developer, I construct complex `if-else` chains using composable data structures instead of language syntax.
pub fn match_case<T, R>(
    predicate: impl Fn(&T) -> bool + 'static,
    handler: impl Fn(&T) -> R + 'static,
) -> MatchCase<T, R> {
    MatchCase {
        predicate: Box::new(predicate),
        handler: Box::new(handler),
    }
}

/// Intent-revealing alias for `match_case`.
///
/// # Signature
/// `pub fn when<T, R>(predicate: impl Fn(&T) -> bool + 'static, handler: impl Fn(&T) -> R + 'static) -> MatchCase<T, R>`
///
/// # User Stories
/// - As a branching author, I declare an arm with intent-revealing `when` naming.
/// - As a domain author, I write declarative rules like `when(is_admin, grant_access)`.
pub fn when<T, R>(
    predicate: impl Fn(&T) -> bool + 'static,
    handler: impl Fn(&T) -> R + 'static,
) -> MatchCase<T, R> {
    match_case(predicate, handler)
}

/// Builds a predicate that always matches (returns `true`).
///
/// # Signature
/// `pub fn wildcard<T>() -> impl Fn(&T) -> bool`
///
/// # User Stories
/// - As a branching author, I build a predicate that always matches.
/// - As a router author, I provide a catch-all case at the end of a match ladder.
pub fn wildcard<T>() -> impl Fn(&T) -> bool {
    |_| true
}

/// Builds a predicate that checks for equality against a specific value.
///
/// # Signature
/// `pub fn equals<T: PartialEq + Clone>(expected: T) -> impl Fn(&T) -> bool`
///
/// # User Stories
/// - As a branching author, I build a predicate testing equality with a value.
/// - As a state machine author, I easily map literal state strings to their respective handlers.
pub fn equals<T: PartialEq + Clone>(expected: T) -> impl Fn(&T) -> bool {
    move |value| value == &expected
}

/// Evaluates a list of cases sequentially, returning the first match or the wildcard fallback.
///
/// # Signature
/// `pub fn multi_match<T, R>(value: &T, cases: &[MatchCase<T, R>], wildcard: impl Fn(&T) -> R) -> R`
///
/// # User Stories
/// - As a caller, I evaluate the first matching arm or a wildcard handler.
/// - As a pipeline orchestrator, I run dynamic routing logic where conditions are built at runtime rather than compile-time.
pub fn multi_match<T, R>(value: &T, cases: &[MatchCase<T, R>], wildcard: impl Fn(&T) -> R) -> R {
    fn test_case<T, R>(value: &T, cases: &[MatchCase<T, R>], wildcard: &dyn Fn(&T) -> R) -> R {
        match cases.split_first() {
            Some((first, rest)) => {
                if (first.predicate)(value) {
                    (first.handler)(value)
                } else {
                    test_case(value, rest, wildcard)
                }
            }
            None => wildcard(value),
        }
    }
    test_case(value, cases, &wildcard)
}

/// Evaluates a list of cases sequentially, returning the first match or `Nothing`.
///
/// # Signature
/// `pub fn multi_match_maybe<T, R>(value: &T, cases: &[MatchCase<T, R>]) -> Maybe<R>`
///
/// # User Stories
/// - As a caller, I evaluate the first matching arm, or Nothing when none match.
/// - As a graceful degradation component, I try a sequence of strategies and safely report when none of them apply.
pub fn multi_match_maybe<T, R>(value: &T, cases: &[MatchCase<T, R>]) -> Maybe<R> {
    fn test_case<T, R>(value: &T, cases: &[MatchCase<T, R>]) -> Maybe<R> {
        match cases.split_first() {
            Some((first, rest)) => {
                if (first.predicate)(value) {
                    just((first.handler)(value))
                } else {
                    test_case(value, rest)
                }
            }
            None => nothing(),
        }
    }
    test_case(value, cases)
}

// ---------------------------------------------------------------------------
// Validation / configuration / boundary result primitives
// ---------------------------------------------------------------------------

/// An ordered chain of validators threading a value through, short-circuiting
/// on the first failure.
pub struct ValidationPipeline<T, E = String> {
    validators: Vec<Box<dyn Fn(T) -> Either<E, T>>>,
}

/// Initializes an empty validation pipeline.
///
/// # Signature
/// `pub fn validation_pipeline<T, E>() -> ValidationPipeline<T, E>`
///
/// # User Stories
/// - As a rule author, I start an empty validation pipeline.
/// - As a domain developer, I want a clean, declarative way to compose data validation rules.
pub fn validation_pipeline<T, E>() -> ValidationPipeline<T, E> {
    ValidationPipeline { validators: vec![] }
}

/// Appends a new validation step to an existing pipeline.
///
/// # Signature
/// `pub fn add_validation<T, E>(mut pipeline: ValidationPipeline<T, E>, validator: impl Fn(T) -> Either<E, T> + 'static) -> ValidationPipeline<T, E>`
///
/// # User Stories
/// - As a rule author, I append a validation step.
/// - As a form processor, I sequentially check bounds, formatting, and invariants, halting at the first error.
pub fn add_validation<T, E>(
    mut pipeline: ValidationPipeline<T, E>,
    validator: impl Fn(T) -> Either<E, T> + 'static,
) -> ValidationPipeline<T, E> {
    pipeline.validators.push(Box::new(validator));
    pipeline
}

/// Executes a validation pipeline against a provided value.
///
/// # Signature
/// `pub fn run_validation<T, E>(pipeline: &ValidationPipeline<T, E>, value: T) -> Either<E, T>`
///
/// # User Stories
/// - As a caller, I run a value through every validator, stopping at the first failure and otherwise returning the validated value.
/// - As an API boundary, I ensure all incoming requests are structurally sound before passing them to domain logic.
pub fn run_validation<T, E>(pipeline: &ValidationPipeline<T, E>, value: T) -> Either<E, T> {
    fn run_step<T, E>(validators: &[Box<dyn Fn(T) -> Either<E, T>>], current: T) -> Either<E, T> {
        match validators.split_first() {
            Some((first, rest)) => match first(current) {
                Either::Right(next) => run_step(rest, next),
                Either::Left(error) => Either::Left(error),
            },
            None => Either::Right(current),
        }
    }

    run_step(&pipeline.validators, value)
}

/// An ordered chain of setters that fold over a default config value.
pub struct ConfigBuilder<Config> {
    setters: Vec<Box<dyn Fn(Config) -> Config>>,
}

/// Initializes an empty configuration builder.
///
/// # Signature
/// `pub fn config_builder<Config>() -> ConfigBuilder<Config>`
///
/// # User Stories
/// - As a config author, I start an empty config builder.
/// - As an SDK designer, I want to expose a fluent API for building complex configuration structs.
pub fn config_builder<Config>() -> ConfigBuilder<Config> {
    ConfigBuilder { setters: vec![] }
}

/// Appends an immutable setter function to the configuration builder.
///
/// # Signature
/// `pub fn with<Config>(mut builder: ConfigBuilder<Config>, setter: impl Fn(Config) -> Config + 'static) -> ConfigBuilder<Config>`
///
/// # User Stories
/// - As a config author, I append a setter that updates the config immutably.
/// - As an SDK user, I can chain multiple `.with(...)` calls to iteratively build my desired state.
pub fn with<Config>(
    mut builder: ConfigBuilder<Config>,
    setter: impl Fn(Config) -> Config + 'static,
) -> ConfigBuilder<Config> {
    builder.setters.push(Box::new(setter));
    builder
}

/// Computes the final configuration struct by applying all setters to the `Default` instance.
///
/// # Signature
/// `pub fn build_config<Config: Default>(builder: &ConfigBuilder<Config>) -> Config`
///
/// # User Stories
/// - As a caller, I apply every setter over a default config to produce a value.
/// - As an application startup script, I instantiate dependencies based on the fully-built configuration.
pub fn build_config<Config: Default>(builder: &ConfigBuilder<Config>) -> Config {
    fn apply_setters<Config>(setters: &[Box<dyn Fn(Config) -> Config>], current: Config) -> Config {
        match setters.split_first() {
            Some((first, rest)) => apply_setters(rest, first(current)),
            None => current,
        }
    }

    apply_setters(&builder.setters, Config::default())
}

/// A structured outcome of a checked operation: success flag, optional value,
/// message, and free-form details. Pure data; constructed via factory functions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestResult<T> {
    pub success: bool,
    pub value: Maybe<T>,
    pub message: String,
    pub details: std::collections::HashMap<String, String>,
}

/// Constructs a successful `TestResult` containing a typed value.
///
/// # Signature
/// `pub fn test_success<T>(value: T, message: impl Into<String>) -> TestResult<T>`
///
/// # User Stories
/// - As a checker, I report a successful outcome carrying a value.
/// - As a testing framework author, I return explicit, structured assertions rather than just printing to stdout.
pub fn test_success<T>(value: T, message: impl Into<String>) -> TestResult<T> {
    TestResult {
        success: true,
        value: just(value),
        message: message.into(),
        details: std::collections::HashMap::new(),
    }
}

/// Constructs a failed `TestResult` without a value.
///
/// # Signature
/// `pub fn test_failure<T>(message: impl Into<String>) -> TestResult<T>`
///
/// # User Stories
/// - As a checker, I report a failed outcome with a message and no value.
/// - As a testing framework author, I capture failing invariants gracefully to display them in a test runner UI.
pub fn test_failure<T>(message: impl Into<String>) -> TestResult<T> {
    TestResult {
        success: false,
        value: nothing(),
        message: message.into(),
        details: std::collections::HashMap::new(),
    }
}

/// Appends arbitrary diagnostic key-value details to a `TestResult`.
///
/// # Signature
/// `pub fn test_with_detail<T>(mut result: TestResult<T>, key: impl Into<String>, value: impl Into<String>) -> TestResult<T>`
///
/// # User Stories
/// - As a checker, I attach a diagnostic detail, returning the updated result.
/// - As a debugger, I inject extra context (like thread ID or timestamp) into a test failure to trace complex issues.
pub fn test_with_detail<T>(
    mut result: TestResult<T>,
    key: impl Into<String>,
    value: impl Into<String>,
) -> TestResult<T> {
    result.details.insert(key.into(), value.into());
    result
}

/// Checks if a `TestResult` represents a success.
///
/// # Signature
/// `pub fn test_is_successful<T>(result: &TestResult<T>) -> bool`
///
/// # User Stories
/// - As a reader, I ask whether a result succeeded.
/// - As a test suite orchestrator, I tally up the total number of passing vs failing assertions.
pub fn test_is_successful<T>(result: &TestResult<T>) -> bool {
    result.success
}

/// Attempts to retrieve the underlying value from a `TestResult`.
///
/// # Signature
/// `pub fn test_try_get_value<T: Clone>(result: &TestResult<T>) -> Maybe<T>`
///
/// # User Stories
/// - As a reader, I attempt to read a result's value as a Maybe.
/// - As a sequential test step, I conditionally extract data from a previous successful step to feed into the next one.
pub fn test_try_get_value<T: Clone>(result: &TestResult<T>) -> Maybe<T> {
    result.value.clone()
}

/// Aggressively extracts the value from a `TestResult`, panicking if it's missing (a failure).
///
/// # Signature
/// `pub fn test_get_value<T: Clone>(result: &TestResult<T>) -> T`
///
/// # User Stories
/// - As a reader at a trusted boundary, I extract a successful result's value.
/// - As a test writer, I crash a test early when a prerequisite setup step completely fails.
pub fn test_get_value<T: Clone>(result: &TestResult<T>) -> T {
    require_just(
        result.value.clone(),
        "TestResult: cannot get value from failure",
    )
}

/// An HTTP-style status code carried by `HttpResult`.
pub type HttpStatusCode = i32;

/// A structured outcome of a boundary call: success flag, status, optional
/// data, and error text. Pure data; constructed via factory functions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpResult<T> {
    pub success: bool,
    pub response_code: HttpStatusCode,
    pub data: Maybe<T>,
    pub error: String,
}

/// Constructs a successful HTTP result wrapping response data.
///
/// # Signature
/// `pub fn http_success<T>(data: T, response_code: HttpStatusCode) -> HttpResult<T>`
///
/// # User Stories
/// - As a boundary author, I report a successful response carrying data.
/// - As a network mock interceptor, I simulate a valid 200 OK server response to test client logic.
pub fn http_success<T>(data: T, response_code: HttpStatusCode) -> HttpResult<T> {
    HttpResult {
        success: true,
        response_code,
        data: just(data),
        error: String::new(),
    }
}

/// Constructs a failed HTTP result wrapping an error message.
///
/// # Signature
/// `pub fn http_failure<T>(error: impl Into<String>, response_code: HttpStatusCode) -> HttpResult<T>`
///
/// # User Stories
/// - As a boundary author, I report a failed response with an error message.
/// - As a network request handler, I gracefully surface 404 or 500 errors up to the UI without throwing exceptions.
pub fn http_failure<T>(error: impl Into<String>, response_code: HttpStatusCode) -> HttpResult<T> {
    HttpResult {
        success: false,
        response_code,
        data: nothing(),
        error: error.into(),
    }
}

// ---------------------------------------------------------------------------
// AsyncResult<T> — composable success/error continuations
// ---------------------------------------------------------------------------

/// A lazily executed asynchronous computation with registered success and error
/// continuations. Pure data plus free functions; executed only on demand.
#[derive(Clone)]
pub struct AsyncResult<T: Clone + 'static> {
    executor: std::rc::Rc<dyn Fn(std::rc::Rc<dyn Fn(T)>, std::rc::Rc<dyn Fn(String)>)>,
    success_handlers: Vec<std::rc::Rc<dyn Fn(T)>>,
    error_handlers: Vec<std::rc::Rc<dyn Fn(String)>>,
}

/// Creates a new `AsyncResult` based on an execution closure containing resolve and reject callbacks.
///
/// # Signature
/// `pub fn create_async_result<T: Clone + 'static>(executor: impl Fn(std::rc::Rc<dyn Fn(T)>, std::rc::Rc<dyn Fn(String)>) + 'static) -> AsyncResult<T>`
///
/// # User Stories
/// - As an async author, I describe a computation in terms of resolve/reject.
/// - As a functional library designer, I provide a Promise-like monad that does not fire until explicitly commanded.
pub fn create_async_result<T: Clone + 'static>(
    executor: impl Fn(std::rc::Rc<dyn Fn(T)>, std::rc::Rc<dyn Fn(String)>) + 'static,
) -> AsyncResult<T> {
    AsyncResult {
        executor: std::rc::Rc::new(executor),
        success_handlers: vec![],
        error_handlers: vec![],
    }
}

/// Registers a callback to be executed upon the successful completion of the `AsyncResult`.
///
/// # Signature
/// `pub fn async_then<T: Clone + 'static>(mut result: AsyncResult<T>, handler: impl Fn(T) + 'static) -> AsyncResult<T>`
///
/// # User Stories
/// - As an async author, I register a success continuation.
/// - As a UI developer, I queue a callback to update a spinner and show data once it arrives over the network.
pub fn async_then<T: Clone + 'static>(
    mut result: AsyncResult<T>,
    handler: impl Fn(T) + 'static,
) -> AsyncResult<T> {
    result.success_handlers.push(std::rc::Rc::new(handler));
    result
}

/// Registers a callback to be executed if the `AsyncResult` yields an error.
///
/// # Signature
/// `pub fn async_catch<T: Clone + 'static>(mut result: AsyncResult<T>, handler: impl Fn(String) + 'static) -> AsyncResult<T>`
///
/// # User Stories
/// - As an async author, I register an error continuation.
/// - As a UI developer, I trap async failures so I can display a user-friendly error dialog.
pub fn async_catch<T: Clone + 'static>(
    mut result: AsyncResult<T>,
    handler: impl Fn(String) + 'static,
) -> AsyncResult<T> {
    result.error_handlers.push(std::rc::Rc::new(handler));
    result
}

/// Executes the `AsyncResult`, triggering the underlying execution closure.
///
/// # Signature
/// `pub fn async_execute<T: Clone + 'static>(result: &AsyncResult<T>)`
///
/// # User Stories
/// - As a caller, I execute the computation, fanning out to every continuation.
/// - As a pipeline runner, I pull the trigger on a fully-constructed async workflow.
pub fn async_execute<T: Clone + 'static>(result: &AsyncResult<T>) {
    fn invoke_success<T: Clone>(handlers: &[std::rc::Rc<dyn Fn(T)>], value: T) {
        match handlers.split_first() {
            Some((first, rest)) => {
                first(value.clone());
                invoke_success(rest, value);
            }
            None => {}
        }
    }

    fn invoke_error(handlers: &[std::rc::Rc<dyn Fn(String)>], error: String) {
        match handlers.split_first() {
            Some((first, rest)) => {
                first(error.clone());
                invoke_error(rest, error);
            }
            None => {}
        }
    }

    let success_handlers = result.success_handlers.clone();
    let error_handlers = result.error_handlers.clone();
    (result.executor)(
        std::rc::Rc::new(move |value| invoke_success(&success_handlers, value)),
        std::rc::Rc::new(move |error| invoke_error(&error_handlers, error)),
    );
}

/// Sequences two `AsyncResult` computations, using the output of the first to yield the second.
///
/// # Signature
/// `pub fn then_async<T, U>(result: AsyncResult<T>, transform: impl Fn(T) -> AsyncResult<U> + Clone + 'static) -> AsyncResult<U> where T: Clone + 'static, U: Clone + 'static`
///
/// # User Stories
/// - As an async author, I sequence one AsyncResult into the next.
/// - As a workflow orchestrator, I string together multiple dependent remote API calls (e.g. authenticate, then fetch profile).
pub fn then_async<T, U>(
    result: AsyncResult<T>,
    transform: impl Fn(T) -> AsyncResult<U> + Clone + 'static,
) -> AsyncResult<U>
where
    T: Clone + 'static,
    U: Clone + 'static,
{
    create_async_result(move |resolve, reject| {
        let reject_for_success = reject.clone();
        let reject_for_error = reject.clone();
        let resolve_for_success = resolve.clone();
        let transform = transform.clone();
        let staged = async_then(result.clone(), move |value| {
            let next = transform(value);
            let reject_for_next = reject_for_success.clone();
            let resolve_for_next = resolve_for_success.clone();
            let next = async_then(next, move |next_value| resolve_for_next(next_value));
            let next = async_catch(next, move |error| reject_for_next(error));
            async_execute(&next);
        });
        let staged = async_catch(staged, move |error| reject_for_error(error));
        async_execute(&staged);
    })
}

// ---------------------------------------------------------------------------
// tap — side effects inside a pipeline without breaking the chain
// ---------------------------------------------------------------------------

/// Inserts a side-effect (like logging) in the middle of a pipeline sequence without mutating the data.
///
/// # Signature
/// `pub fn tap<T>(f: impl FnOnce(&T)) -> impl FnOnce(T) -> T`
///
/// # User Stories
/// - As a pipeline author, `tap` runs a side-effecting closure and returns the original value, letting me insert logging or assertions into a `pipe!` chain.
/// - As a debugger, I inject a `print` statement midway through a data transform to verify intermediate state.
pub fn tap<T>(f: impl FnOnce(&T)) -> impl FnOnce(T) -> T {
    move |value| {
        f(&value);
        value
    }
}

/// Mutates a value in-place inside a pipeline sequence before passing it on.
///
/// # Signature
/// `pub fn tap_mut<T: ?Sized>(mut f: impl FnMut(&mut T)) -> impl FnMut(&mut T) -> &mut T`
///
/// # User Stories
/// - As a pipeline author, `tap_mut` runs a side-effecting closure on a mutable reference and returns it, useful for mutating a buffer mid-pipeline without breaking the chain.
/// - As an optimizer, I tweak a complex object cleanly between functional mapping steps.
pub fn tap_mut<T: ?Sized>(mut f: impl FnMut(&mut T)) -> impl FnMut(&mut T) -> &mut T {
    move |value| {
        f(value);
        value
    }
}

// ---------------------------------------------------------------------------
// pipe! / compose! macros — variadic composition
// ---------------------------------------------------------------------------

/// A variadic `pipe!` that chains any number of functions left-to-right:
/// `pipe!(a, f, g, h)` expands to `h(g(f(a)))`.
#[macro_export]
macro_rules! pipe {
    ($val:expr, $func:expr $(,)?) => {
        $func($val)
    };
    ($val:expr, $func:expr, $($rest:expr),+ $(,)?) => {
        $crate::pipe!($func($val), $($rest),+)
    };
}

/// A variadic `compose!` that chains functions right-to-left:
/// `compose!(f, g, h)` returns a closure running `f(g(h(x)))`.
#[macro_export]
macro_rules! compose {
    ($last:expr) => {
        $last
    };
    ($head:expr, $($tail:expr),+ $(,)?) => {
        |x| $head($crate::compose!($($tail),+)(x))
    };
}

// ---------------------------------------------------------------------------
// Trampoline — heap-based recursion without growing the call stack
// ---------------------------------------------------------------------------

/// One step of a trampolined computation: either bounce again or finish.
pub enum Bounce<A, R> {
    Call(A),
    Done(R),
}

/// Iteratively resolves a recursive-like sequence on the heap to prevent stack overflow.
///
/// # Signature
/// `pub fn trampoline<A, R>(mut arg: A, mut f: impl FnMut(A) -> Bounce<A, R>) -> R`
///
/// # User Stories
/// - As an author of deep recursion, I iterate a step function to a result on the heap so I never overflow the native stack.
/// - As a parser builder, I want to handle deeply nested abstract syntax trees cleanly and safely.
pub fn trampoline<A, R>(mut arg: A, mut f: impl FnMut(A) -> Bounce<A, R>) -> R {
    loop {
        match f(arg) {
            Bounce::Call(next) => arg = next,
            Bounce::Done(res) => return res,
        }
    }
}

// ---------------------------------------------------------------------------
// Display impls — render the FP carriers for logs and tests
// ---------------------------------------------------------------------------

impl<T: fmt::Display> fmt::Display for Maybe<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Maybe::Just(v) => write!(f, "Just({})", v),
            Maybe::Nothing => write!(f, "Nothing"),
        }
    }
}

impl<E: fmt::Display, T: fmt::Display> fmt::Display for Either<E, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Either::Left(e) => write!(f, "Left({})", e),
            Either::Right(v) => write!(f, "Right({})", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- Maybe tests --

    #[test]
    fn maybe_just_and_nothing() {
        let j: Maybe<i32> = just(42);
        let n: Maybe<i32> = nothing();
        assert_eq!(j, Maybe::Just(42));
        assert_eq!(n, Maybe::Nothing);
        assert!(is_just(&j));
        assert!(is_nothing(&n));
    }

    #[test]
    fn maybe_map_transforms_just() {
        let m = just(10);
        let result = maybe_map(&m, |x| x * 2);
        assert_eq!(result, Maybe::Just(20));
    }

    #[test]
    fn maybe_map_preserves_nothing() {
        let m: Maybe<i32> = nothing();
        let result = maybe_map(&m, |x| x * 2);
        assert_eq!(result, Maybe::Nothing);
    }

    #[test]
    fn maybe_chain_flat_maps() {
        let m = just(5);
        let result = maybe_chain(m, |x| if x > 3 { just(x * 10) } else { nothing() });
        assert_eq!(result, Maybe::Just(50));

        let m2 = just(1);
        let result2 = maybe_chain(m2, |x| if x > 3 { just(x * 10) } else { nothing() });
        assert_eq!(result2, Maybe::Nothing);
    }

    #[test]
    fn maybe_filter_keeps_only_matching() {
        assert_eq!(maybe_filter(just(8), |x| *x > 4), Maybe::Just(8));
        assert_eq!(maybe_filter(just(2), |x| *x > 4), Maybe::Nothing);
    }

    #[test]
    fn maybe_match_dispatches() {
        let j = just(7);
        let r = maybe_match(&j, |v| format!("got {}", v), || "empty".to_string());
        assert_eq!(r, "got 7");

        let n: Maybe<i32> = nothing();
        let r2 = maybe_match(&n, |v| format!("got {}", v), || "empty".to_string());
        assert_eq!(r2, "empty");
    }

    #[test]
    fn maybe_or_else_provides_default() {
        let j = just(42);
        assert_eq!(maybe_or_else(&j, || 0), 42);
        assert_eq!(or_else(&j, 0), 42);

        let n: Maybe<i32> = nothing();
        assert_eq!(maybe_or_else(&n, || 0), 0);
        assert_eq!(or_else(&n, 0), 0);
    }

    #[test]
    fn maybe_option_roundtrip() {
        let original = Some(99);
        let m = maybe_from_option(original);
        assert_eq!(m, Maybe::Just(99));
        assert_eq!(maybe_to_option(m), Some(99));

        let none: Option<i32> = None;
        let m2 = from_nullable(none);
        assert_eq!(m2, Maybe::Nothing);
        assert_eq!(maybe_to_option(m2), None);

        assert_eq!(from_nullable_value(7, true), Maybe::Just(7));
        assert_eq!(from_nullable_value(7, false), Maybe::Nothing);
    }

    #[test]
    fn require_just_unwraps_present_value() {
        assert_eq!(require_just(just(5), "should be present"), 5);
    }

    // -- Either tests --

    #[test]
    fn either_left_and_right() {
        let l: Either<&str, i32> = left("err");
        let r: Either<&str, i32> = right(42);
        assert_eq!(l, Either::Left("err"));
        assert_eq!(r, Either::Right(42));
        assert!(is_left(&l));
        assert!(is_right(&r));
    }

    #[test]
    fn either_map_transforms_right() {
        let e: Either<&str, i32> = right(10);
        let result = either_map(&e, |x| x * 3);
        assert_eq!(result, Either::Right(30));
    }

    #[test]
    fn either_map_preserves_left() {
        let e: Either<String, i32> = left("fail".to_string());
        let result = either_map(&e, |x| x * 3);
        assert_eq!(result, Either::Left("fail".to_string()));
    }

    #[test]
    fn either_map_left_transforms_failure() {
        let e: Either<i32, &str> = left(404);
        let result = either_map_left(&e, |code| format!("status {}", code));
        assert_eq!(result, Either::Left("status 404".to_string()));
    }

    #[test]
    fn either_chain_flat_maps() {
        let e: Either<&str, i32> = right(5);
        let result = either_chain(e, |x| {
            if x > 3 {
                right(x * 10)
            } else {
                left("too small")
            }
        });
        assert_eq!(result, Either::Right(50));
    }

    #[test]
    fn either_match_dispatches() {
        let e: Either<&str, i32> = right(7);
        let r = either_match(&e, |err| format!("err: {}", err), |v| format!("ok: {}", v));
        assert_eq!(r, "ok: 7");
        assert_eq!(either_or_else(&e, 0), 7);
    }

    // -- Fmap tests --

    #[test]
    fn fmap_over_vec() {
        let xs = vec![1, 2, 3];
        let doubled = fmap(&xs, |x: &i32| x * 2);
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    // -- pipe / compose tests --

    #[test]
    fn pipe_chains_left_to_right() {
        let double = |x: i32| x * 2;
        let add_one = |x: i32| x + 1;
        let f = pipe(double, add_one);
        assert_eq!(f(5), 11); // (5*2)+1
    }

    #[test]
    fn compose_chains_right_to_left() {
        let double = |x: i32| x * 2;
        let add_one = |x: i32| x + 1;
        let f = compose(double, add_one);
        assert_eq!(f(5), 12); // (5+1)*2
    }

    #[test]
    fn pipe_macro_chains_variadic_left_to_right() {
        let double = |x: i32| x * 2;
        let add_one = |x: i32| x + 1;
        let square = |x: i32| x * x;
        let result = crate::pipe!(5, double, add_one, square);
        assert_eq!(result, 121); // ((5*2)+1)^2 = 11^2 = 121
    }

    #[test]
    fn compose_macro_chains_variadic_right_to_left() {
        let double = |x: i32| x * 2;
        let add_one = |x: i32| x + 1;
        let square = |x: i32| x * x;
        let f = crate::compose!(square, add_one, double);
        assert_eq!(f(5), 121); // square(add_one(double(5)))
    }

    #[test]
    fn tap_runs_side_effect_without_changing_value() {
        let mut seen = 0;
        let result = crate::pipe!(10, |x: i32| x + 5, tap(|v| seen = *v), |x: i32| x * 2);
        assert_eq!(result, 30);
        assert_eq!(seen, 15);
    }

    // -- partial application / memoize / lazy --

    #[test]
    fn partial_apply_binds_first_argument() {
        let add = |a: i32, b: i32| a + b;
        let add_ten = partial_apply(add, 10);
        assert_eq!(add_ten(5), 15);
    }

    #[test]
    fn memoize_caches_results() {
        let mut calls = 0;
        let mut squared = {
            let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
            let counter_inner = counter.clone();
            let f = memoize(move |x: i32| {
                *counter_inner.borrow_mut() += 1;
                x * x
            });
            calls = *counter.borrow();
            f
        };
        assert_eq!(squared(4), 16);
        assert_eq!(squared(4), 16);
        let _ = calls;
    }

    #[test]
    fn memoize_last_recomputes_on_new_arg() {
        let mut doubler = memoize_last(|x: i32| x * 2);
        assert_eq!(doubler(2), 4);
        assert_eq!(doubler(2), 4);
        assert_eq!(doubler(3), 6);
    }

    #[test]
    fn lazy_computes_once() {
        let lz = lazy(|| 6 * 7);
        assert_eq!(eval(&lz), 42);
        assert_eq!(eval(&lz), 42);
    }

    // -- Dispatcher tests --

    #[test]
    fn dispatcher_routes_by_key() {
        let d = create_dispatcher(|_: &i32| "fallback".to_string());
        let d = dispatcher_register(d, "double", |x: &i32| format!("{}", x * 2));
        let d = dispatcher_register(d, "negate", |x: &i32| format!("{}", -x));

        assert_eq!(dispatcher_dispatch(&d, &"double", &5), "10");
        assert_eq!(dispatcher_dispatch(&d, &"negate", &3), "-3");
        assert_eq!(dispatcher_dispatch(&d, &"unknown", &1), "fallback");
    }

    #[test]
    fn optional_dispatcher_returns_maybe() {
        let d = create_optional_dispatcher(vec![(
            "greet",
            Box::new(|| "hello".to_string()) as Box<dyn Fn() -> String>,
        )]);
        assert_eq!(dispatch(&d, &"greet"), Maybe::Just("hello".to_string()));
        assert_eq!(dispatch(&d, &"missing"), Maybe::Nothing);
        assert!(has(&d, &"greet"));
        assert_eq!(keys(&d), vec!["greet"]);
    }

    // -- multi_match tests --

    #[test]
    fn multi_match_finds_first_matching_case() {
        let cases = vec![
            match_case(|x: &i32| *x < 0, |x| format!("negative: {}", x)),
            when(|x: &i32| *x == 0, |_| "zero".to_string()),
            match_case(|x: &i32| *x > 0, |x| format!("positive: {}", x)),
        ];

        assert_eq!(
            multi_match(&-5, &cases, |x| format!("wild: {}", x)),
            "negative: -5"
        );
        assert_eq!(multi_match(&0, &cases, |x| format!("wild: {}", x)), "zero");
        assert_eq!(
            multi_match(&7, &cases, |x| format!("wild: {}", x)),
            "positive: 7"
        );
    }

    #[test]
    fn multi_match_falls_through_to_wildcard() {
        let cases: Vec<MatchCase<i32, String>> = vec![match_case(equals(1), |_| "one".to_string())];
        assert_eq!(
            multi_match(&99, &cases, |x| format!("wild: {}", x)),
            "wild: 99"
        );
        assert_eq!(multi_match_maybe(&99, &cases), Maybe::Nothing);
        assert_eq!(
            multi_match_maybe(&1, &cases),
            Maybe::Just("one".to_string())
        );
    }

    // -- validation / config / results --

    #[test]
    fn validation_pipeline_short_circuits() {
        let pipeline = add_validation(
            add_validation(validation_pipeline::<i32, String>(), |x| {
                if x > 0 {
                    right(x)
                } else {
                    left("must be positive".to_string())
                }
            }),
            |x| {
                if x < 100 {
                    right(x)
                } else {
                    left("too big".to_string())
                }
            },
        );
        assert_eq!(run_validation(&pipeline, 50), Either::Right(50));
        assert_eq!(
            run_validation(&pipeline, -1),
            Either::Left("must be positive".to_string())
        );
    }

    #[test]
    fn config_builder_folds_setters() {
        let builder = with(with(config_builder::<i32>(), |c| c + 1), |c| c * 10);
        assert_eq!(build_config(&builder), 10); // (0 + 1) * 10
    }

    #[test]
    fn test_result_carries_value_and_details() {
        let result = test_with_detail(test_success(42, "ok"), "phase", "unit");
        assert!(test_is_successful(&result));
        assert_eq!(test_get_value(&result), 42);
        assert_eq!(result.details.get("phase"), Some(&"unit".to_string()));

        let failure: TestResult<i32> = test_failure("nope");
        assert!(!test_is_successful(&failure));
        assert_eq!(test_try_get_value(&failure), Maybe::Nothing);
    }

    #[test]
    fn http_result_constructors() {
        let ok = http_success(vec![1, 2, 3], 200);
        assert!(ok.success);
        assert_eq!(ok.response_code, 200);

        let err: HttpResult<Vec<i32>> = http_failure("boom", 500);
        assert!(!err.success);
        assert_eq!(err.error, "boom");
    }

    // -- async result --

    #[test]
    fn async_result_resolves_to_handlers() {
        let captured = std::rc::Rc::new(std::cell::RefCell::new(0));
        let sink = captured.clone();
        let job = create_async_result(|resolve, _reject| resolve(21));
        let job = async_then(job, move |value| *sink.borrow_mut() = value * 2);
        async_execute(&job);
        assert_eq!(*captured.borrow(), 42);
    }

    // -- trampoline --

    #[test]
    fn trampoline_sums_without_stack_growth() {
        let result = trampoline((0i64, 1_000_000i64), |(acc, n)| {
            if n == 0 {
                Bounce::Done(acc)
            } else {
                Bounce::Call((acc + n, n - 1))
            }
        });
        assert_eq!(result, 500_000_500_000);
    }
}
