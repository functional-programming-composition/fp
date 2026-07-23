import { fmap, just, mbind, type Maybe } from '../maybe/maybe';

/** Folds immutable values through a pure accumulator step.
 * @signature export const fold = <T, Accumulator>( values: readonly T[], seed: Accumulator, step: (accumulator: Accumulator, value: T) => Accumulator, ): Accumulator =>
 * User Story: As a core maintainer, I need fold so the core workflow remains explicit and composable.
 */
export const fold = <T, Accumulator>(
  values: readonly T[],
  seed: Accumulator,
  step: (accumulator: Accumulator, value: T) => Accumulator,
): Accumulator => values.reduce(step, seed);

/** Keeps immutable values that satisfy a predicate.
 * @signature export const filter = <T>( values: readonly T[], predicate: (value: T) => boolean, ): T[] =>
 * User Story: As a core maintainer, I need filter so the core workflow remains explicit and composable.
 */
export const filter = <T>(
  values: readonly T[],
  predicate: (value: T) => boolean,
): T[] => values.filter(predicate);

/** Maps immutable values through Maybe and fails when any result is Nothing.
 * @signature export const traverse = <T, U>( values: readonly T[], transform: (value: T) => Maybe<U>, ): Maybe<U[]> =>
 * User Story: As a core maintainer, I need traverse so the core workflow remains explicit and composable.
 */
export const traverse = <T, U>(
  values: readonly T[],
  transform: (value: T) => Maybe<U>,
): Maybe<U[]> => fold<T, Maybe<U[]>>(
  values,
  just<U[]>([]),
  (collected, value) => mbind(
    collected,
    results => fmap(transform(value), result => [...results, result]),
  ),
);
