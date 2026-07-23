import { fpRuntime, type EitherTag } from '../runtime/runtimeAdapters';

export interface Either<E, T> {
  readonly _tag: EitherTag;
  /**
   * @signature map<U>(transform: (value: T) => U): Either<E, U>;
   * User Story: As a core maintainer, I need map so the core workflow remains explicit and composable.
   */
  map<U>(transform: (value: T) => U): Either<E, U>;
  /**
   * @signature chain<U>(transform: (value: T) => Either<E, U>): Either<E, U>;
   * User Story: As a core maintainer, I need chain so the core workflow remains explicit and composable.
   */
  chain<U>(transform: (value: T) => Either<E, U>): Either<E, U>;
  /**
   * @signature getOrElse(defaultValue: T): T;
   * User Story: As a core maintainer, I need getOrElse so the core workflow remains explicit and composable.
   */
  getOrElse(defaultValue: T): T;
  /**
   * @signature match<R>(cases: { left: (error: E) => R; right: (value: T) => R }): R;
   * User Story: As a core maintainer, I need match so the core workflow remains explicit and composable.
   */
  match<R>(cases: { left: (error: E) => R; right: (value: T) => R }): R;
}

/** Creates a failing Either branch.
 * @signature export const left = <E, T>(error: E): Either<E, T> =>
 * User Story: As a core maintainer, I need left so the core workflow remains explicit and composable.
 */
export const left = <E, T>(error: E): Either<E, T> => ({
  _tag: fpRuntime.tags.either.left,
  /**
   * @signature map: <U>(_transform: (value: T) => U): Either<E, U> =>
   * User Story: As a core maintainer, I need map so the core workflow remains explicit and composable.
   */
  map: <U>(_transform: (value: T) => U): Either<E, U> => left<E, U>(error),
  /**
   * @signature chain: <U>(_transform: (value: T) => Either<E, U>): Either<E, U> =>
   * User Story: As a core maintainer, I need chain so the core workflow remains explicit and composable.
   */
  chain: <U>(_transform: (value: T) => Either<E, U>): Either<E, U> => left<E, U>(error),
  /**
   * @signature getOrElse: (defaultValue: T): T =>
   * User Story: As a core maintainer, I need getOrElse so the core workflow remains explicit and composable.
   */
  getOrElse: (defaultValue: T): T => defaultValue,
  /**
   * @signature match: <R>(cases: { left: (current: E) => R; right: (value: T) => R }): R =>
   * User Story: As a core maintainer, I need match so the core workflow remains explicit and composable.
   */
  match: <R>(cases: { left: (current: E) => R; right: (value: T) => R }): R => cases.left(error),
});

/** Creates a successful Either branch.
 * @signature export const right = <E, T>(value: T): Either<E, T> =>
 * User Story: As a core maintainer, I need right so the core workflow remains explicit and composable.
 */
export const right = <E, T>(value: T): Either<E, T> => ({
  _tag: fpRuntime.tags.either.right,
  /**
   * @signature map: <U>(transform: (current: T) => U): Either<E, U> =>
   * User Story: As a core maintainer, I need map so the core workflow remains explicit and composable.
   */
  map: <U>(transform: (current: T) => U): Either<E, U> => right<E, U>(transform(value)),
  /**
   * @signature chain: <U>(transform: (current: T) => Either<E, U>): Either<E, U> =>
   * User Story: As a core maintainer, I need chain so the core workflow remains explicit and composable.
   */
  chain: <U>(transform: (current: T) => Either<E, U>): Either<E, U> => transform(value),
  /**
   * @signature getOrElse: (_defaultValue: T): T =>
   * User Story: As a core maintainer, I need getOrElse so the core workflow remains explicit and composable.
   */
  getOrElse: (_defaultValue: T): T => value,
  /**
   * @signature match: <R>(cases: { left: (error: E) => R; right: (current: T) => R }): R =>
   * User Story: As a core maintainer, I need match so the core workflow remains explicit and composable.
   */
  match: <R>(cases: { left: (error: E) => R; right: (current: T) => R }): R => cases.right(value),
});

/** Maps the success branch of an Either.
 * @signature export const efmap = <E, T, U>( value: Either<E, T>, transform: (current: T) => U, ): Either<E, U> =>
 * User Story: As a core maintainer, I need efmap so the core workflow remains explicit and composable.
 */
export const efmap = <E, T, U>(
  value: Either<E, T>,
  transform: (current: T) => U,
): Either<E, U> => value.map(transform);

/** Chains an Either-producing function over a successful value.
 * @signature export const ebind = <E, T, U>( value: Either<E, T>, transform: (current: T) => Either<E, U>, ): Either<E, U> =>
 * User Story: As a core maintainer, I need ebind so the core workflow remains explicit and composable.
 */
export const ebind = <E, T, U>(
  value: Either<E, T>,
  transform: (current: T) => Either<E, U>,
): Either<E, U> => value.chain(transform);

/** Pattern-matches an Either into a single return value.
 * @signature export const ematch = <E, T, R>( value: Either<E, T>, onLeft: (error: E) => R, onRight: (current: T) => R, ): R =>
 * User Story: As a core maintainer, I need ematch so the core workflow remains explicit and composable.
 */
export const ematch = <E, T, R>(
  value: Either<E, T>,
  onLeft: (error: E) => R,
  onRight: (current: T) => R,
): R => value.match({ left: onLeft, right: onRight });

/** Identifies the error branch of an Either.
 * @signature export const isLeft = <E, T>(value: Either<E, T>): boolean =>
 * User Story: As a core maintainer, I need isLeft so the core workflow remains explicit and composable.
 */
export const isLeft = <E, T>(value: Either<E, T>): boolean =>
  value._tag === fpRuntime.tags.either.left;

/** Identifies the success branch of an Either.
 * @signature export const isRight = <E, T>(value: Either<E, T>): boolean =>
 * User Story: As a core maintainer, I need isRight so the core workflow remains explicit and composable.
 */
export const isRight = <E, T>(value: Either<E, T>): boolean =>
  value._tag === fpRuntime.tags.either.right;
