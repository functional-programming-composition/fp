import { fpRuntime } from '../runtime/runtimeAdapters';

type UnaryFunction = (...arguments_: any[]) => any;

/** Composes one function.
 * @signature export function compose<A, B>(transform: (value: A) => B): (value: A) => B;
 * User Story: As a core maintainer, I need compose so the core workflow remains explicit and composable.
 */
export function compose<A, B>(transform: (value: A) => B): (value: A) => B;

/** Composes two functions right-to-left.
 * @signature export function compose<A, B, C>( outer: (value: B) => C, inner: (value: A) => B, ): (value: A) => C;
 * User Story: As a core maintainer, I need compose so the core workflow remains explicit and composable.
 */
export function compose<A, B, C>(
  outer: (value: B) => C,
  inner: (value: A) => B,
): (value: A) => C;

/** Composes three functions right-to-left.
 * @signature export function compose<A, B, C, D>( outer: (value: C) => D, middle: (value: B) => C, inner: (value: A) => B, ): (value: A) => D;
 * User Story: As a core maintainer, I need compose so the core workflow remains explicit and composable.
 */
export function compose<A, B, C, D>(
  outer: (value: C) => D,
  middle: (value: B) => C,
  inner: (value: A) => B,
): (value: A) => D;

/** Implements right-to-left composition for one or more functions.
 * @signature export function compose(...transforms: UnaryFunction[]): UnaryFunction
 * User Story: As a core maintainer, I need compose so the core workflow remains explicit and composable.
 */
export function compose(...transforms: UnaryFunction[]): UnaryFunction {
  return (...arguments_: any[]) => {
    const inner = transforms[transforms.length - fpRuntime.composition.innerOffset];
    const outer = transforms.slice(
      fpRuntime.composition.startIndex,
      fpRuntime.composition.endOffset,
    );
    return outer.reduceRight((value, transform) => transform(value), inner(...arguments_));
  };
}
