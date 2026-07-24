import { describe, expect, it } from 'vitest';
import {
  _,
  compose,
  createDispatcher,
  curry,
  filter,
  fold,
  isNothing,
  just,
  multiMatch,
  nothing,
  traverse,
} from '../index';
import fixture from './fixtures/composition.json';

describe(fixture.suites.compose, () => {
  it(fixture.compose.binary.label, () => {
    const data = fixture.compose.binary;
    const composed = compose(
      (value: number) => value + data.delta,
      (value: number) => value * data.multiplier,
    );
    expect(composed(data.input)).toBe(data.expected);
  });

  it(fixture.compose.ternary.label, () => {
    const data = fixture.compose.ternary;
    const composed = compose(
      String,
      (value: number) => value + data.delta,
      (value: number) => value * data.multiplier,
    );
    expect(composed(data.input)).toBe(data.expected);
  });

  it(fixture.compose.identity.label, () => {
    const data = fixture.compose.identity;
    const composed = compose((value: number) => value);
    expect(composed(data.input)).toBe(data.expected);
  });
});

describe(fixture.suites.curry, () => {
  it(fixture.curry.full.label, () => {
    const data = fixture.curry.full;
    const add = curry((left: number, right: number) => left + right);
    expect(add(data.left, data.right)).toBe(data.expected);
  });

  it(fixture.curry.partial.label, () => {
    const data = fixture.curry.partial;
    const add = curry((first: number, second: number, third: number) =>
      first + second + third
    );
    expect(add(data.first)(data.second)(data.third)).toBe(data.expected);
  });

  it(fixture.curry.mixed.label, () => {
    const data = fixture.curry.mixed;
    const add = curry((first: number, second: number, third: number) =>
      first + second + third
    );
    expect(add(data.first, data.second)(data.third)).toBe(data.expected);
  });

  it(fixture.curry.function.label, () => {
    const data = fixture.curry.function;
    const add = curry((left: number, right: number) => left + right);
    const addBound = add(data.bound);
    expect(typeof addBound).toBe(data.expectedType);
    expect(addBound(data.input)).toBe(data.expected);
  });
});

describe(fixture.suites.collections, () => {
  it(fixture.collections.fold.label, () => {
    const data = fixture.collections.fold;
    expect(fold(data.values, data.seed, (sum, value) => sum + value))
      .toBe(data.expected);
  });

  it(fixture.collections.filter.label, () => {
    const data = fixture.collections.filter;
    expect(filter(data.values, value => value === data.selectedValue))
      .toEqual(data.expected);
  });

  it(fixture.collections.traverse.label, () => {
    const data = fixture.collections.traverse;
    const result = traverse(data.values, value =>
      just(value * data.multiplier)
    );
    expect(result.getOrElse(data.fallback)).toEqual(data.expected);
  });

  it(fixture.collections.traverseFailure.label, () => {
    const data = fixture.collections.traverseFailure;
    const result = traverse(data.values, value =>
      value === data.rejectedValue ? nothing<number>() : just(value)
    );
    expect(isNothing(result)).toBe(data.expectedNothing);
  });
});

describe(fixture.suites.dispatcher, () => {
  it(fixture.dispatcher.registered.label, () => {
    const data = fixture.dispatcher.registered;
    const dispatcher = createDispatcher<string, number>(
      data.entries.map(entry => [entry.key, () => entry.value]),
    );
    expect(dispatcher.dispatch(data.selectedKey).getOrElse(data.fallback))
      .toBe(data.expected);
  });

  it(fixture.dispatcher.missing.label, () => {
    const data = fixture.dispatcher.missing;
    const dispatcher = createDispatcher<string, number>([
      [data.entryKey, () => data.entryValue],
    ]);
    expect(dispatcher.dispatch(data.missingKey).getOrElse(data.fallback))
      .toBe(data.expected);
    expect(isNothing(dispatcher.dispatch(data.missingKey))).toBe(data.expectedNothing);
  });

  it(fixture.dispatcher.handlers.label, () => {
    const data = fixture.dispatcher.handlers;
    const dispatcher = createDispatcher<string, string>(
      data.entries.map(entry => [entry.key, () => entry.value]),
    );
    expect(dispatcher.dispatch(data.firstKey).getOrElse(data.fallback))
      .toBe(data.firstExpected);
    expect(dispatcher.dispatch(data.secondKey).getOrElse(data.fallback))
      .toBe(data.secondExpected);
  });

  it(fixture.dispatcher.arguments.label, () => {
    const data = fixture.dispatcher.arguments;
    const dispatcher = createDispatcher<string, number>([
      [data.key, (left: number, right: number) => left + right],
    ]);
    expect(dispatcher.dispatch(data.key, data.left, data.right).getOrElse(data.fallback))
      .toBe(data.expected);
  });

  it(fixture.dispatcher.has.label, () => {
    const data = fixture.dispatcher.has;
    const dispatcher = createDispatcher<string, number>([
      [data.entryKey, () => data.entryValue],
    ]);
    expect(dispatcher.has(data.presentKey)).toBe(data.expectedPresent);
    expect(dispatcher.has(data.missingKey)).toBe(data.expectedMissing);
  });

  it(fixture.dispatcher.keys.label, () => {
    const data = fixture.dispatcher.keys;
    const dispatcher = createDispatcher<string, number>(
      data.entries.map(entry => [entry.key, () => entry.value]),
    );
    expect(dispatcher.keys()).toEqual(data.expected);
  });
});

describe(fixture.suites.multiMatch, () => {
  it(fixture.multiMatch.first.label, () => {
    const data = fixture.multiMatch.first;
    const result = multiMatch(data.input, [
      [value => value > data.highThreshold, () => data.highValue],
      [value => value > data.middleThreshold, () => data.middleValue],
      [value => value > data.lowThreshold, () => data.lowValue],
    ]);
    expect(result.getOrElse(data.fallback)).toBe(data.expected);
  });

  it(fixture.multiMatch.none.label, () => {
    const data = fixture.multiMatch.none;
    const result = multiMatch(data.input, [
      [value => value > data.highThreshold, () => data.highValue],
      [value => value > data.lowThreshold, () => data.lowValue],
    ]);
    expect(isNothing(result)).toBe(data.expectedNothing);
  });

  it(fixture.multiMatch.wildcard.label, () => {
    const data = fixture.multiMatch.wildcard;
    const result = multiMatch(data.input, [
      [value => value > data.threshold, () => data.matchedValue],
      [_, () => data.defaultValue],
    ]);
    expect(result.getOrElse(data.fallback)).toBe(data.expected);
  });

  it(fixture.multiMatch.equality.label, () => {
    const data = fixture.multiMatch.equality;
    const result = multiMatch(data.input, [
      [data.firstInput, () => data.firstValue],
      [data.secondInput, () => data.secondValue],
    ]);
    expect(result.getOrElse(data.fallback)).toBe(data.expected);
  });

  it(fixture.multiMatch.predicateValue.label, () => {
    const data = fixture.multiMatch.predicateValue;
    const result = multiMatch(data.input, [
      [value => value === data.expectedInput, value => data.prefix + value],
    ]);
    expect(result.getOrElse(data.fallback)).toBe(data.expected);
  });

  it(fixture.multiMatch.wildcardValue.label, () => {
    const data = fixture.multiMatch.wildcardValue;
    const result = multiMatch(data.input, [
      [_, value => data.prefix + value],
    ]);
    expect(result.getOrElse(data.fallback)).toBe(data.expected);
  });
});
