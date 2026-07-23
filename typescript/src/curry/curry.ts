type FunctionValue = (...arguments_: any[]) => any;

/** Curries a two-argument function.
 * @signature export function curry<A, B, R>(function_: (a: A, b: B) => R): { (a: A, b: B): R; (a: A): (b: B) => R; };
 * User Story: As a core maintainer, I need curry so the core workflow remains explicit and composable.
 */
export function curry<A, B, R>(function_: (a: A, b: B) => R): {
  (a: A, b: B): R;
  (a: A): (b: B) => R;
};

/** Curries a three-argument function.
 * @signature export function curry<A, B, C, R>(function_: (a: A, b: B, c: C) => R): { (a: A, b: B, c: C): R; (a: A, b: B): (c: C) => R; (a: A): { (b: B, c: C): R; (b: B): (c: C) => R }; };
 * User Story: As a core maintainer, I need curry so the core workflow remains explicit and composable.
 */
export function curry<A, B, C, R>(function_: (a: A, b: B, c: C) => R): {
  (a: A, b: B, c: C): R;
  (a: A, b: B): (c: C) => R;
  (a: A): { (b: B, c: C): R; (b: B): (c: C) => R };
};

/** Collects arguments until the original function arity is met.
 * @signature export function curry(function_: FunctionValue): FunctionValue
 * User Story: As a core maintainer, I need curry so the core workflow remains explicit and composable.
 */
export function curry(function_: FunctionValue): FunctionValue {
  const arity = function_.length;
  /**
   * @signature const curried = (...arguments_: any[]): any =>
   * User Story: As a core maintainer, I need curried so the core workflow remains explicit and composable.
   */
  const curried = (...arguments_: any[]): any =>
    arguments_.length >= arity
      ? function_(...arguments_)
      : (...remainingArguments: any[]) => curried(...arguments_, ...remainingArguments);
  return curried;
}
