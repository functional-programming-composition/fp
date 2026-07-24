import { describe, expect, it } from 'vitest';
import {
  ebind,
  efmap,
  ematch,
  fmap,
  isJust,
  isLeft,
  isNothing,
  isRight,
  just,
  left,
  match,
  mbind,
  nothing,
  orElse,
  right,
} from '../index';
import fixture from './fixtures/maybe-either.json';

describe(fixture.suites.maybe, () => {
  it(fixture.maybe.justMap.label, () => {
    const data = fixture.maybe.justMap;
    const result = just(data.input).map(value => value + data.delta);
    expect(result.getOrElse(data.fallback)).toBe(data.expected);
    expect(isJust(result)).toBe(data.expectedJust);
  });

  it(fixture.maybe.nothingMap.label, () => {
    const data = fixture.maybe.nothingMap;
    const result = nothing<number>().map(value => value + data.delta);
    expect(result.getOrElse(data.fallback)).toBe(data.expected);
    expect(isNothing(result)).toBe(data.expectedNothing);
  });

  it(fixture.maybe.chainPass.label, () => {
    const data = fixture.maybe.chainPass;
    const result = just(data.input).chain(value =>
      value > data.threshold ? just(value) : nothing()
    );
    expect(result.getOrElse(data.fallback)).toBe(data.expected);
  });

  it(fixture.maybe.chainFail.label, () => {
    const data = fixture.maybe.chainFail;
    const result = just(data.input).chain(value =>
      value > data.threshold ? just(value) : nothing()
    );
    expect(result.getOrElse(data.fallback)).toBe(data.expected);
  });

  it(fixture.maybe.nothingChain.label, () => {
    const data = fixture.maybe.nothingChain;
    const result = nothing<number>().chain(value => just(value * data.multiplier));
    expect(result.getOrElse(data.fallback)).toBe(data.expected);
  });

  it(fixture.maybe.nothingOrElse.label, () => {
    const data = fixture.maybe.nothingOrElse;
    expect(nothing<number>().getOrElse(data.fallback)).toBe(data.expected);
  });

  it(fixture.maybe.justOrElse.label, () => {
    const data = fixture.maybe.justOrElse;
    expect(just(data.input).getOrElse(data.fallback)).toBe(data.expected);
  });

  it(fixture.maybe.justMatch.label, () => {
    const data = fixture.maybe.justMatch;
    const result = just(data.input).match({
      /**
       * @signature just: value =>
       * User Story: As a core maintainer, I need just so the core workflow remains explicit and composable.
       */
      just: value => value * data.multiplier,
      /**
       * @signature nothing: () =>
       * User Story: As a core maintainer, I need nothing so the core workflow remains explicit and composable.
       */
      nothing: () => data.nothingValue,
    });
    expect(result).toBe(data.expected);
  });

  it(fixture.maybe.nothingMatch.label, () => {
    const data = fixture.maybe.nothingMatch;
    const result = nothing<number>().match({
      /**
       * @signature just: value =>
       * User Story: As a core maintainer, I need just so the core workflow remains explicit and composable.
       */
      just: value => value * data.multiplier,
      /**
       * @signature nothing: () =>
       * User Story: As a core maintainer, I need nothing so the core workflow remains explicit and composable.
       */
      nothing: () => data.nothingValue,
    });
    expect(result).toBe(data.expected);
  });

  it(fixture.maybe.fmap.label, () => {
    const data = fixture.maybe.fmap;
    expect(fmap(just(data.input), value => value + data.delta).getOrElse(data.fallback))
      .toBe(data.expected);
  });

  it(fixture.maybe.mbind.label, () => {
    const data = fixture.maybe.mbind;
    expect(mbind(just(data.input), value => just(value * data.multiplier))
      .getOrElse(data.fallback)).toBe(data.expected);
  });

  it(fixture.maybe.freeMatch.label, () => {
    const data = fixture.maybe.freeMatch;
    expect(match(just(data.input), value => value * data.multiplier,
      () => data.nothingValue)).toBe(data.expectedJust);
    expect(match(nothing<number>(), value => value * data.multiplier,
      () => data.nothingValue)).toBe(data.expectedNothing);
  });

  it(fixture.maybe.freeOrElse.label, () => {
    const data = fixture.maybe.freeOrElse;
    expect(orElse(nothing<number>(), data.fallback)).toBe(data.expectedNothing);
    expect(orElse(just(data.justValue), data.fallback)).toBe(data.expectedJust);
  });
});

describe(fixture.suites.either, () => {
  it(fixture.either.rightMap.label, () => {
    const data = fixture.either.rightMap;
    const result = right<string, number>(data.input).map(value => value + data.delta);
    expect(result.getOrElse(data.fallback)).toBe(data.expected);
    expect(isRight(result)).toBe(data.expectedRight);
  });

  it(fixture.either.leftMap.label, () => {
    const data = fixture.either.leftMap;
    const result = left<string, number>(data.error).map(value => value + data.delta);
    expect(result.getOrElse(data.fallback)).toBe(data.expected);
    expect(isLeft(result)).toBe(data.expectedLeft);
  });

  it(fixture.either.rightChain.label, () => {
    const data = fixture.either.rightChain;
    const result = right<string, number>(data.input).chain(value =>
      value > data.threshold
        ? right<string, number>(value * data.multiplier)
        : left<string, number>(data.error)
    );
    expect(result.getOrElse(data.fallback)).toBe(data.expected);
  });

  it(fixture.either.chainToLeft.label, () => {
    const data = fixture.either.chainToLeft;
    const result = right<string, number>(data.input).chain(value =>
      value > data.threshold
        ? right<string, number>(value * data.multiplier)
        : left<string, number>(data.error)
    );
    expect(result.match({ /**
     * @signature left: error =>
     * User Story: As a core maintainer, I need left so the core workflow remains explicit and composable.
     */
    left: error => error, /**
     * @signature right: () =>
     * User Story: As a core maintainer, I need right so the core workflow remains explicit and composable.
     */
    right: () => data.success }))
      .toBe(data.expected);
  });

  it(fixture.either.leftChain.label, () => {
    const data = fixture.either.leftChain;
    const result = left<string, number>(data.error).chain(value =>
      right<string, number>(value * data.multiplier)
    );
    expect(result.match({ /**
     * @signature left: error =>
     * User Story: As a core maintainer, I need left so the core workflow remains explicit and composable.
     */
    left: error => error, /**
     * @signature right: () =>
     * User Story: As a core maintainer, I need right so the core workflow remains explicit and composable.
     */
    right: () => data.success }))
      .toBe(data.expected);
  });

  it(fixture.either.leftMatch.label, () => {
    const data = fixture.either.leftMatch;
    const result = left<string, number>(data.error).match({
      /**
       * @signature left: error =>
       * User Story: As a core maintainer, I need left so the core workflow remains explicit and composable.
       */
      left: error => data.leftPrefix + error,
      /**
       * @signature right: () =>
       * User Story: As a core maintainer, I need right so the core workflow remains explicit and composable.
       */
      right: () => data.rightValue,
    });
    expect(result).toBe(data.expected);
  });

  it(fixture.either.rightMatch.label, () => {
    const data = fixture.either.rightMatch;
    const result = right<string, number>(data.input).match({
      /**
       * @signature left: () =>
       * User Story: As a core maintainer, I need left so the core workflow remains explicit and composable.
       */
      left: () => data.leftValue,
      /**
       * @signature right: value =>
       * User Story: As a core maintainer, I need right so the core workflow remains explicit and composable.
       */
      right: value => data.rightPrefix + value,
    });
    expect(result).toBe(data.expected);
  });

  it(fixture.either.efmap.label, () => {
    const data = fixture.either.efmap;
    expect(efmap(right<string, number>(data.input), value => value + data.delta)
      .getOrElse(data.fallback)).toBe(data.expected);
  });

  it(fixture.either.ebind.label, () => {
    const data = fixture.either.ebind;
    expect(ebind(right<string, number>(data.input), value =>
      right<string, number>(value * data.multiplier)
    ).getOrElse(data.fallback)).toBe(data.expected);
  });

  it(fixture.either.ematch.label, () => {
    const data = fixture.either.ematch;
    expect(ematch(left<string, number>(data.leftError),
      error => data.leftPrefix + error,
      value => data.rightPrefix + value)).toBe(data.expectedLeft);
    expect(ematch(right<string, number>(data.rightInput),
      error => data.leftPrefix + error,
      value => data.rightPrefix + value)).toBe(data.expectedRight);
  });
});
