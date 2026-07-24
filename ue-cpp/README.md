# ue-cpp — Unreal Engine C++ functional core

The **Unreal C++** implementation of the [fp core](https://github.com/functional-programming-composition/fp).
Header-only, C++11, 29 headers under one `fp.hpp` aggregate, namespace `func`.

> **This is not portable standalone C++.** It includes `CoreMinimal.h` and uses
> `TArray` / `TMap` / `FString` in several headers. It targets an Unreal module.
> See [TODO.md](TODO.md) for the engine-independent port.

## Install

Copy `fp.hpp` and the `FP/` directory into your module's `Public/Core/`, then:

```cpp
#include "Core/fp.hpp"

using namespace func;

const Maybe<int32> Doubled = fmap(just(21), [](int32 N) { return N * 2; });
const FString Label = match(Doubled,
    [](int32 V) { return FString::Printf(TEXT("Just(%d)"), V); },
    []            { return FString(TEXT("Nothing")); });
```

`fp.hpp` is an aggregate — it includes every module below, so one include gets
the whole surface.

## Design constraints

- **Strict C++11.** No C++14/17 features; index sequences and tuple expansion
  are backported where needed.
- **Data structs, free functions.** No classes with methods, no inheritance —
  behavior lives in free functions in `namespace func`.
- **Value semantics.** Sum types store the inactive field physically and
  default-construct it, so `T` must be default-constructible, and equality must
  compare the **tag and active branch** rather than dummy payloads.
- **Portable across Unreal's compilers.** No MSVC-, Clang-, or GCC-only quirks.

## Ownership is part of the API

The C++ surface makes lifetime explicit in ways a garbage-collected port hides:

| Shape | Meaning |
| --- | --- |
| `Maybe<Value>` from a map lookup | Owns a **copied** value |
| `Maybe<const Value*>` from a map lookup | **Borrows** storage from the original `TMap` |
| `from_nullable(const T*) -> Maybe<T>` | **Copies** the pointee; does not preserve pointer identity |
| Lazy / memoized `const T&` | Borrows a **cache-owned** result |

A borrowed pointer or reference must not outlive the container or cache that
owns it. Mutating a `TMap` can invalidate a borrowed pointer, and a
memoize-last cache can replace its internal result — so retaining a reference
across another call may observe a different value.

## Modules

| Module | Contents |
| --- | --- |
| `Maybe` / `Either` / `Nullable` | Carriers, `fmap`, `chain`, `match`, boundary lifts |
| `Validation` | Accumulating validation (applicative, not fail-fast `Either`) |
| `Monad` / `Functor` | Law-bearing operations over the carriers |
| `Composition` / `Pipeline` / `Curry` | `compose`, value-threading pipelines, currying |
| `Combinator` | `both`, `either_pred`, `all_pass`, `complement`, `identity`, `flip` |
| `Match` / `Dispatcher` | Predicate case tables and keyed handler tables |
| `Trampoline` | `Call`/`Done` iteration without `while` |
| `Lazy` / `Memoization` | Deferred values and bounded/last-input caches |
| `Vector` / `Array` / `Range` / `Map` / `Sequence` / `Indexed` | Collection ops incl. `TArray`/`TMap` overloads |
| `Catalog` | Grouped declaration atoms folded through one composer |
| `Async` | `AsyncResult`, `AsyncChain`, `AsyncVoid` |
| `Config` / `Error` / `Prelude` / `Test` | Support surfaces |

`Catalog` deserves a note: it exists for the recurring problem of many parallel
declarations differing only by name, path, conversion, or predicate. Prefer one
grouped declaration atom folded through a single composer over repeated typed
factory families — parallel arrays of names and validators silently drift.

## Trampoline caveat

The historical helper in the source SDK re-entered its trampoline
**recursively**, which is not stack-safe. Verify the interpreter is iterative
before using it for unbounded work.

## License

MIT — see [NOTICE.md](../NOTICE.md) for provenance. Relicensed from
Copyright (c) 2026 ForbocAI, Inc.
