# Provenance and attribution

The language cores in this repository were extracted from working codebases and
released here under the MIT license (see `LICENSE`). This file records where
each one came from, so downstream users know the lineage.

| Directory | Origin | Original license |
| --- | --- | --- |
| `typescript/` | ForbocAI SDK (`@forbocai/core`) | Copyright (c) 2026 ForbocAI, Inc. — relicensed MIT by the copyright holder for this release |
| `ue-cpp/` | ForbocAI Unreal SDK (`ForbocAI_SDK`) | Copyright (c) 2026 ForbocAI, Inc. — relicensed MIT by the copyright holder for this release |
| `rust/` | magik-lux (`core/src/functional.rs`) | previously unlicensed; released MIT |
| `gdscript/` | therapy-one (`src/fp.gd`) | already MIT |
| `javascript/` | Lanternbough / frontier-of-jefferson (`fp.js`) | previously unlicensed; released MIT |

The TypeScript core was modified during extraction to remove a host-application
dependency: it previously loaded its carrier tags and match/composition
constants from an external `data/fp/runtime.json`. Those values are now inlined
in `typescript/src/runtime/runtimeAdapters.ts` so the package is standalone. The
shape and values are otherwise unchanged.

The Unreal C++ core is intentionally **not** portable standalone C++ — it
depends on `CoreMinimal.h` and uses `TArray`/`TMap`/`FString` in several
headers. It is published as an Unreal Engine module, not a generic C++ library.
