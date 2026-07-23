// Canonical aggregate surface for the modular functional core.
export {
  just,
  nothing,
  fmap,
  mbind,
  match,
  orElse,
  isJust,
  isNothing,
  fromNullable,
  requireJust,
} from './maybe/maybe';
export type { Maybe } from './maybe/maybe';

export {
  left,
  right,
  efmap,
  ebind,
  ematch,
  isLeft,
  isRight,
} from './either/either';
export type { Either } from './either/either';

export { compose } from './composition/composition';
export { curry } from './curry/curry';
export { fold, filter, traverse } from './vector/vector';
export { createDispatcher } from './dispatcher/dispatcher';
export type { Dispatcher } from './dispatcher/dispatcher';
export { multiMatch, _ } from './match/match';
export type { Predicate } from './match/match';
