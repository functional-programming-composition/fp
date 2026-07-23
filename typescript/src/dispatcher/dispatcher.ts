import { just, nothing, type Maybe } from '../maybe/maybe';

export interface Dispatcher<K, R> {
  /**
   * @signature dispatch(key: K, ...arguments_: any[]): Maybe<R>;
   * User Story: As a core maintainer, I need dispatch so the core workflow remains explicit and composable.
   */
  dispatch(key: K, ...arguments_: any[]): Maybe<R>;
  /**
   * @signature has(key: K): boolean;
   * User Story: As a core maintainer, I need has so the core workflow remains explicit and composable.
   */
  has(key: K): boolean;
  /**
   * @signature keys(): K[];
   * User Story: As a core maintainer, I need keys so the core workflow remains explicit and composable.
   */
  keys(): K[];
}

/** Builds a keyed dispatcher that returns Nothing for missing handlers.
 * @signature export const createDispatcher = <K, R>( entries: [K, (...arguments_: any[]) => R][], ): Dispatcher<K, R> =>
 * User Story: As a core maintainer, I need createDispatcher so the core workflow remains explicit and composable.
 */
export const createDispatcher = <K, R>(
  entries: [K, (...arguments_: any[]) => R][],
): Dispatcher<K, R> => {
  const table = new Map<K, (...arguments_: any[]) => R>(entries);
  return {
    /**
     * @signature dispatch: (key: K, ...arguments_: any[]): Maybe<R> =>
     * User Story: As a core maintainer, I need dispatch so the core workflow remains explicit and composable.
     */
    dispatch: (key: K, ...arguments_: any[]): Maybe<R> => {
      const handler = table.get(key);
      return handler ? just(handler(...arguments_)) : nothing<R>();
    },
    /**
     * @signature has: (key: K): boolean =>
     * User Story: As a core maintainer, I need has so the core workflow remains explicit and composable.
     */
    has: (key: K): boolean => table.has(key),
    /**
     * @signature keys: (): K[] =>
     * User Story: As a core maintainer, I need keys so the core workflow remains explicit and composable.
     */
    keys: (): K[] => [...table.keys()],
  };
};
