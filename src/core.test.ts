import { describe, expect, it } from 'vitest';

import { inspectCommitAura, reduceToPowerNumber } from './core.ts';

describe('reduceToPowerNumber', () => {
  it.each([
    [7, 7],
    [11, 11],
    [22, 22],
    [33, 33],
    [34, 7],
    [987, 6],
  ] as const)('reduces %i to %i without diminishing master numbers', (value, expected) =>
    expect(reduceToPowerNumber(value)).toBe(expected),
  );

  it.each([0, -1, Number.NaN, Number.POSITIVE_INFINITY])('rejects profane total %s', (value) => {
    expect(() => reduceToPowerNumber(value)).toThrow(RangeError);
  });
});

describe('inspectCommitAura', () => {
  it('calculates a hexadecimal commit aura deterministically', () => {
    expect(inspectCommitAura('a1b2c3')).toEqual({
      destinyNumber: 3,
      hash: 'a1b2c3',
      total: 12,
      vibe: 'NEUTRAL',
    });
  });

  it('curses the impossible all-zero hash', () => {
    expect(inspectCommitAura('0000000')).toEqual({
      destinyNumber: 0,
      hash: '0000000',
      total: 0,
      vibe: 'CURSED',
    });
  });

  it('normalizes superficial casing and whitespace', () => {
    expect(inspectCommitAura('  AAAAAA  ').hash).toBe('aaaaaa');
  });

  it('rejects matter that cannot belong to a commit hash', () => {
    expect(() => inspectCommitAura('mercury')).toThrow(TypeError);
  });
});
