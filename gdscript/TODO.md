# TODO — GDScript

Godot has **no first-party package manager**. Distribution is the official Asset
Library plus two community package managers. All three are worth doing; they
reach different users.

## 1. Godot Asset Library (official)

The canonical channel, browsable from inside the Godot editor
(AssetLib tab).

- [ ] Create a Godot account at <https://godotengine.org/register>
- [ ] Prepare the submission — the Asset Library pulls from a **git repo + tag**,
      so cut a release tag first
      ```bash
      git tag gdscript-v0.1.0 && git push origin gdscript-v0.1.0
      ```
- [ ] Submit at <https://godotengine.org/asset-library/asset/new>
      - **Category:** Scripts
      - **Godot version:** 4.x
      - **Repository URL:** `https://github.com/functional-programming-composition/fp`
      - **Commit/tag:** the tag above
      - **Icon:** 128×128 PNG (needs creating)
      - **License:** MIT
- [ ] Wait for moderator review — submissions are **manually reviewed**, so
      expect days, not minutes, and expect feedback on the description/icon

**Caveat:** the Asset Library expects the repo to look like a Godot project or
addon. Because this repo is multi-language, consider either (a) submitting a
dedicated `godot-fp` mirror repo laid out as `addons/fp/`, or (b) restructuring
`gdscript/` into `addons/fp/` with a `plugin.cfg`. Option (a) keeps this repo
clean; option (b) avoids a second source of truth.

- [ ] Decide (a) mirror repo vs (b) `addons/fp/` layout here

## 2. gd-plug (community)

A plugin manager where users declare dependencies in a `plug.gd` script — no
central registry, it installs straight from git.

- [ ] Nothing to submit. Just document the one-liner for users:
      ```gdscript
      # plug.gd
      plug("functional-programming-composition/fp", {"include": ["gdscript"]})
      ```
- [ ] Verify the `include` path resolves correctly from this repo's layout
- [ ] Add that snippet to `README.md` once verified

## 3. godot-package-manager (community)

Uses an npm-style `godot.package` manifest and can resolve from npm or git.

- [ ] Decide whether to publish the GDScript core to **npm** as well (gpm can
      consume npm packages) or leave it git-only
- [ ] If npm: pick a name (e.g. `godot-functional-composition`) and add a minimal
      `godot.package` manifest
- [ ] Document the install line for gpm users

## Before any submission

- [ ] Add a `plugin.cfg` if going the addon route
- [ ] Create the 128×128 icon the Asset Library requires
- [ ] Test `fp.gd` against a clean Godot 4 project (verify no editor warnings)
- [ ] Document the `multi_match` evaluation contract prominently — this
      implementation filters before selecting, so **every predicate runs**,
      unlike the short-circuiting TypeScript port
