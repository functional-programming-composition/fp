import { just, nothing, type Maybe } from '../maybe/maybe';
import { fpRuntime } from '../runtime/runtimeAdapters';

export const _ = Symbol.for(fpRuntime.match.wildcardSymbolKey);

export type Predicate<T> = ((value: T) => boolean) | typeof _;

/**
 * @signature const testCase = <T, R>( value: T, predicate: Predicate<T> | T, handler: (current: T) => R, ): Maybe<R> =>
 * User Story: As a core maintainer, I need testCase so the core workflow remains explicit and composable.
 */
const testCase = <T, R>(
  value: T,
  predicate: Predicate<T> | T,
  handler: (current: T) => R,
): Maybe<R> =>
  predicate === _
    ? just(handler(value))
    : typeof predicate === fpRuntime.match.functionType
      ? (predicate as (current: T) => boolean)(value) ? just(handler(value)) : nothing<R>()
      : predicate === value ? just(handler(value)) : nothing<R>();

/** Evaluates predicate or literal cases in order and returns the first match.
 * @signature export const multiMatch = <T, R>( value: T, cases: [Predicate<T> | T, (current: T) => R][], ): Maybe<R> =>
 * User Story: As a core maintainer, I need multiMatch so the core workflow remains explicit and composable.
 */
export const multiMatch = <T, R>(
  value: T,
  cases: [Predicate<T> | T, (current: T) => R][],
): Maybe<R> => {
  if (cases.length === fpRuntime.match.emptyLength) return nothing<R>();
  const [predicate, handler] = cases[fpRuntime.match.firstCaseIndex];
  return testCase(value, predicate, handler).match({
        /**
         * @signature just: current =>
         * User Story: As a core maintainer, I need just so the core workflow remains explicit and composable.
         */
        just: current => just<R>(current),
        /**
         * @signature nothing: () =>
         * User Story: As a core maintainer, I need nothing so the core workflow remains explicit and composable.
         */
        nothing: () => multiMatch(value, cases.slice(fpRuntime.match.nextCaseOffset)),
  });
};
