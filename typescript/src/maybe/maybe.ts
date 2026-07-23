import { fpRuntime, type MaybeTag } from '../runtime/runtimeAdapters';

export interface Maybe<T> {
  readonly _tag: MaybeTag;
  readonly value?: T;
  /**
   * @signature map<U>(transform: (value: T) => U): Maybe<U>;
   * User Story: As a core maintainer, I need map so the core workflow remains explicit and composable.
   */
  map<U>(transform: (value: T) => U): Maybe<U>;
  /**
   * @signature chain<U>(transform: (value: T) => Maybe<U>): Maybe<U>;
   * User Story: As a core maintainer, I need chain so the core workflow remains explicit and composable.
   */
  chain<U>(transform: (value: T) => Maybe<U>): Maybe<U>;
  /**
   * @signature getOrElse(defaultValue: T): T;
   * User Story: As a core maintainer, I need getOrElse so the core workflow remains explicit and composable.
   */
  getOrElse(defaultValue: T): T;
  /**
   * @signature match<R>(cases: { just: (value: T) => R; nothing: () => R }): R;
   * User Story: As a core maintainer, I need match so the core workflow remains explicit and composable.
   */
  match<R>(cases: { just: (value: T) => R; nothing: () => R }): R;
}

interface Just<T> extends Maybe<T> {
  readonly value: T;
}

interface Nothing<T> extends Maybe<T> {
  readonly value?: never;
}

/** Creates a present Maybe value.
 * @signature export const just = <T>(value: T): Maybe<T> =>
 * User Story: As a core maintainer, I need just so the core workflow remains explicit and composable.
 */
export const just = <T>(value: T): Maybe<T> => ({
  _tag: fpRuntime.tags.maybe.just,
  value,
  /**
   * @signature map: <U>(transform: (current: T) => U): Maybe<U> =>
   * User Story: As a core maintainer, I need map so the core workflow remains explicit and composable.
   */
  map: <U>(transform: (current: T) => U): Maybe<U> => just(transform(value)),
  /**
   * @signature chain: <U>(transform: (current: T) => Maybe<U>): Maybe<U> =>
   * User Story: As a core maintainer, I need chain so the core workflow remains explicit and composable.
   */
  chain: <U>(transform: (current: T) => Maybe<U>): Maybe<U> => transform(value),
  /**
   * @signature getOrElse: (_defaultValue: T): T =>
   * User Story: As a core maintainer, I need getOrElse so the core workflow remains explicit and composable.
   */
  getOrElse: (_defaultValue: T): T => value,
  /**
   * @signature match: <R>(cases: { just: (current: T) => R; nothing: () => R }): R =>
   * User Story: As a core maintainer, I need match so the core workflow remains explicit and composable.
   */
  match: <R>(cases: { just: (current: T) => R; nothing: () => R }): R => cases.just(value),
});

/** Creates an absent Maybe value.
 * @signature export const nothing = <T>(): Maybe<T> =>
 * User Story: As a core maintainer, I need nothing so the core workflow remains explicit and composable.
 */
export const nothing = <T>(): Maybe<T> => ({
  _tag: fpRuntime.tags.maybe.nothing,
  /**
   * @signature map: <U>(_transform: (current: T) => U): Maybe<U> =>
   * User Story: As a core maintainer, I need map so the core workflow remains explicit and composable.
   */
  map: <U>(_transform: (current: T) => U): Maybe<U> => nothing<U>(),
  /**
   * @signature chain: <U>(_transform: (current: T) => Maybe<U>): Maybe<U> =>
   * User Story: As a core maintainer, I need chain so the core workflow remains explicit and composable.
   */
  chain: <U>(_transform: (current: T) => Maybe<U>): Maybe<U> => nothing<U>(),
  /**
   * @signature getOrElse: (defaultValue: T): T =>
   * User Story: As a core maintainer, I need getOrElse so the core workflow remains explicit and composable.
   */
  getOrElse: (defaultValue: T): T => defaultValue,
  /**
   * @signature match: <R>(cases: { just: (current: T) => R; nothing: () => R }): R =>
   * User Story: As a core maintainer, I need match so the core workflow remains explicit and composable.
   */
  match: <R>(cases: { just: (current: T) => R; nothing: () => R }): R => cases.nothing(),
});

/** Maps a present Maybe value and preserves Nothing.
 * @signature export const fmap = <T, U>(value: Maybe<T>, transform: (current: T) => U): Maybe<U> =>
 * User Story: As a core maintainer, I need fmap so the core workflow remains explicit and composable.
 */
export const fmap = <T, U>(value: Maybe<T>, transform: (current: T) => U): Maybe<U> =>
  value.map(transform);

/** Chains a Maybe-producing function over a Maybe value.
 * @signature export const mbind = <T, U>(value: Maybe<T>, transform: (current: T) => Maybe<U>): Maybe<U> =>
 * User Story: As a core maintainer, I need mbind so the core workflow remains explicit and composable.
 */
export const mbind = <T, U>(value: Maybe<T>, transform: (current: T) => Maybe<U>): Maybe<U> =>
  value.chain(transform);

/** Extracts a Maybe payload or returns the provided default.
 * @signature export const orElse = <T>(value: Maybe<T>, defaultValue: T): T =>
 * User Story: As a core maintainer, I need orElse so the core workflow remains explicit and composable.
 */
export const orElse = <T>(value: Maybe<T>, defaultValue: T): T => value.getOrElse(defaultValue);

/** Pattern-matches a Maybe into a single return value.
 * @signature export const match = <T, R>( value: Maybe<T>, onJust: (current: T) => R, onNothing: () => R, ): R =>
 * User Story: As a core maintainer, I need match so the core workflow remains explicit and composable.
 */
export const match = <T, R>(
  value: Maybe<T>,
  onJust: (current: T) => R,
  onNothing: () => R,
): R => value.match({ just: onJust, nothing: onNothing });

/** Narrows a Maybe to Just.
 * @signature export const isJust = <T>(value: Maybe<T>): value is Just<T> =>
 * User Story: As a core maintainer, I need isJust so the core workflow remains explicit and composable.
 */
export const isJust = <T>(value: Maybe<T>): value is Just<T> =>
  value._tag === fpRuntime.tags.maybe.just;

/** Narrows a Maybe to Nothing.
 * @signature export const isNothing = <T>(value: Maybe<T>): value is Nothing<T> =>
 * User Story: As a core maintainer, I need isNothing so the core workflow remains explicit and composable.
 */
export const isNothing = <T>(value: Maybe<T>): value is Nothing<T> =>
  value._tag === fpRuntime.tags.maybe.nothing;

/** Lifts a nullable value into Maybe.
 * @signature export const fromNullable = <T>(value: T | null | undefined): Maybe<T> =>
 * User Story: As a core maintainer, I need fromNullable so the core workflow remains explicit and composable.
 */
export const fromNullable = <T>(value: T | null | undefined): Maybe<T> =>
  value != null ? just(value) : nothing<T>();

/** Extracts a Maybe payload or throws at a required-value boundary.
 * @signature export const requireJust = <T>(value: Maybe<T>, errorMessage: string): T =>
 * User Story: As a core maintainer, I need requireJust so the core workflow remains explicit and composable.
 */
export const requireJust = <T>(value: Maybe<T>, errorMessage: string): T =>
  value.match({
    /**
     * @signature just: current =>
     * User Story: As a core maintainer, I need just so the core workflow remains explicit and composable.
     */
    just: current => current,
    /**
     * @signature nothing: () =>
     * User Story: As a core maintainer, I need nothing so the core workflow remains explicit and composable.
     */
    nothing: () => {
      throw new Error(errorMessage);
    },
  });
