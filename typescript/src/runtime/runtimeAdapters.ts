// Runtime constants for the functional core.
//
// These were previously loaded from an external `data/fp/runtime.json` so a host
// application could re-tag the carriers. This package is standalone, so they are
// inlined here as the single source of truth. The shape is unchanged, so code
// written against the data-driven version still type-checks against this one.
const runtimeData = {
  tags: {
    maybe: { just: 'Just', nothing: 'Nothing' },
    either: { left: 'Left', right: 'Right' },
  },
  match: {
    wildcardSymbolKey: 'fp_wildcard',
    functionType: 'function',
    emptyLength: 0,
    firstCaseIndex: 0,
    nextCaseOffset: 1,
  },
  composition: {
    innerOffset: 1,
    startIndex: 0,
    endOffset: -1,
  },
} as const;

export type MaybeTag = Capitalize<keyof typeof runtimeData.tags.maybe>;
export type EitherTag = Capitalize<keyof typeof runtimeData.tags.either>;

export const fpRuntime = Object.freeze({
  tags: Object.freeze({
    maybe: Object.freeze({
      just: runtimeData.tags.maybe.just as MaybeTag,
      nothing: runtimeData.tags.maybe.nothing as MaybeTag,
    }),
    either: Object.freeze({
      left: runtimeData.tags.either.left as EitherTag,
      right: runtimeData.tags.either.right as EitherTag,
    }),
  }),
  match: Object.freeze(runtimeData.match),
  composition: Object.freeze(runtimeData.composition),
});
