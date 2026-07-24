# TODO — Unreal C++

Two separate tracks: **ship it to Unreal users** (Fab), and **free it from
Unreal** so general C++ package managers can carry it.

## Track A — Fab (Unreal marketplace)

Fab is the Unreal-native distribution channel. It takes a **clean source plugin
package**, not a build artifact.

- [ ] Wrap the core as a proper UE plugin: `.uplugin` descriptor, `Source/`
      module with a `Build.cs`, and a module entry header
- [ ] Package the allowlisted source tree — exclude `Intermediate/`, debug
      symbols, and local build residue
- [ ] Compile the staged plugin with Unreal Automation Tool for each supported
      target, as compilation evidence
- [ ] Produce a deterministic ZIP + SHA-256 + manifest
- [ ] Validate: required files present, module descriptor correct, no forbidden
      residue, no credential-shaped values
- [ ] Complete Fab publisher onboarding — Epic Distribution Agreement, publisher
      profile, trader verification, tax interview, payout setup. **These are
      human-only account operations**; never automate accepting legal terms,
      answering tax questions, or entering payout details
- [ ] Create the product, upload the ZIP and listing media (1920×1080), submit
      for review
- [ ] After approval, set the listing URL in the `.uplugin` and resubmit the
      descriptor update

## Track B — engine-independent port

The blocker for every general C++ package manager: this core `#include`s
`CoreMinimal.h` and uses `TArray` / `TMap` / `FString` in **6 headers**. Until
that is abstracted, vcpkg/Conan/CPM cannot consume it.

- [ ] Identify the 6 UE-dependent headers and isolate the engine types behind a
      thin adapter layer (`TArray` → `std::vector`, `TMap` → `std::unordered_map`,
      `FString` → `std::string`), selected at compile time
- [ ] Keep the UE overloads as an opt-in adapter so Unreal users lose nothing
- [ ] Verify it builds clean with GCC, Clang, and MSVC outside Unreal
- [ ] Keep it **strict C++11** — the existing backports (index sequences, tuple
      expansion) exist for that reason; do not raise the standard casually
- [ ] Add a CMakeLists.txt exporting a header-only INTERFACE target

Once the port compiles standalone:

### vcpkg
- [ ] Write a port: `vcpkg.json` manifest + `portfile.cmake`
- [ ] Test locally with `vcpkg install --overlay-ports=...`
- [ ] Open a PR against `microsoft/vcpkg` (review can be slow; expect iteration)

### Conan
- [ ] Write `conanfile.py` (header-only recipe, `package_type = "header-library"`)
- [ ] Test with `conan create .`
- [ ] Submit to Conan Center Index via PR

### CPM.cmake
- [ ] No registry — CPM consumes GitHub releases directly. Just cut a tag and
      document the one-liner:
      ```cmake
      CPMAddPackage("gh:functional-programming-composition/fp@0.1.0")
      ```
- [ ] This is the **cheapest** path to a consumable C++ package; do it first

## Migrate the tests from the source repo

**The tests were left behind in the extraction.** The Unreal SDK has a test tree
mirroring the `FP/` module layout:

| Source (`Forboc.AI/sdk-ue-5/Source/ForbocAI_SDK/Private/Tests/Core/FP/`) |
| --- |
| `Array/array_test.cpp` |
| `Composition/composition_test.cpp` |
| `Dispatcher/dispatcher_test.cpp` |
| `Match/match_test.cpp` |

- [ ] Copy the four files into `ue-cpp/Tests/` preserving the module layout
- [ ] Rewrite the includes — they resolve against the SDK module's
      `Private/Tests/...` root
- [ ] Note these are **Unreal Automation Tests**, so they need a UE module and
      the automation runner to execute; they cannot run in a bare C++ harness
- [ ] Coverage gap: only 4 of 24 modules have tests. `Maybe`, `Either`,
      `Validation`, `Trampoline`, `Memoization`, `Lazy`, `Curry`, `Monad`, and
      `Functor` are all untested — write those, prioritizing `Validation` and
      `Trampoline`
- [ ] `FP/Test/Test.hpp` is a **test helper**, not a suite. Do not mistake its
      presence for coverage
- [ ] If the Track B engine-independent port happens, the tests need a
      non-Unreal runner (Catch2 / doctest) alongside the automation ones

## Correctness follow-ups

- [ ] Fix the trampoline: the source helper re-enters **recursively**, which is
      not stack-safe. Make the interpreter iterative before advertising it for
      unbounded work
- [ ] Document equality semantics — sum types store the inactive branch
      physically and default-construct it, so comparisons must check the tag and
      active branch only
- [ ] Add law tests (Functor, Monad, Monoid, Applicative) to `FP/Test/`
