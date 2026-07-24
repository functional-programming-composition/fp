import { describe, expect, it } from 'vitest';
import {
  fromNullable,
  isJust,
  isNothing,
  just,
  nothing,
  requireJust,
} from '../index';
import fixture from './fixtures/nullable.json';

describe(fixture.suites.fromNullable, () => {
  it(fixture.fromNullable.nonNull.label, () => {
    const data = fixture.fromNullable.nonNull;
    const m = fromNullable(data.input);
    expect(isJust(m)).toBe(data.expectedJust);
    expect(m.getOrElse(data.fallback)).toBe(data.expected);
  });

  it(fixture.fromNullable.nullValue.label, () => {
    const data = fixture.fromNullable.nullValue;
    const m = fromNullable(null);
    expect(isNothing(m)).toBe(data.expectedNothing);
  });

  it(fixture.fromNullable.undefinedValue.label, () => {
    const data = fixture.fromNullable.undefinedValue;
    const m = fromNullable(undefined);
    expect(isNothing(m)).toBe(data.expectedNothing);
  });

  it(fixture.fromNullable.emptyString.label, () => {
    const data = fixture.fromNullable.emptyString;
    const m = fromNullable(data.input);
    expect(isJust(m)).toBe(data.expectedJust);
    expect(m.getOrElse(data.fallback)).toBe(data.expected);
  });

  it(fixture.fromNullable.zero.label, () => {
    const data = fixture.fromNullable.zero;
    const m = fromNullable(data.input);
    expect(isJust(m)).toBe(data.expectedJust);
    expect(m.getOrElse(data.fallback)).toBe(data.expected);
  });

  it(fixture.fromNullable.falseValue.label, () => {
    const data = fixture.fromNullable.falseValue;
    const m = fromNullable(data.input);
    expect(isJust(m)).toBe(data.expectedJust);
  });

  it(fixture.fromNullable.chain.label, () => {
    const data = fixture.fromNullable.chain;
    const result = fromNullable<string>(data.input)
      .map(s => s.toUpperCase())
      .getOrElse(data.fallback);
    expect(result).toBe(data.expected);
  });
});

describe(fixture.suites.requireJust, () => {
  it(fixture.requireJust.justValue.label, () => {
    const data = fixture.requireJust.justValue;
    const m = just(data.input);
    expect(requireJust(m, data.error)).toBe(data.expected);
  });

  it(fixture.requireJust.nothingValue.label, () => {
    const data = fixture.requireJust.nothingValue;
    const m = nothing<number>();
    expect(() => requireJust(m, data.error)).toThrow(data.error);
  });

  it(fixture.requireJust.nonNull.label, () => {
    const data = fixture.requireJust.nonNull;
    expect(requireJust(fromNullable(data.input), data.error)).toBe(data.expected);
  });

  it(fixture.requireJust.nullValue.label, () => {
    const data = fixture.requireJust.nullValue;
    expect(() => requireJust(fromNullable(null), data.error)).toThrow(data.error);
  });
});
