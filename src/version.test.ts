import { describe, expect, it } from 'vitest';

import { isPowerVersion, nextPowerVersion, parsePowerVersion } from './version.ts';

describe('parsePowerVersion', () => {
  it('accepts Power Versions with SemVer metadata', () => {
    expect(parsePowerVersion('7.11.33-rc.1+moon.full')).toEqual({
      aura: 33,
      build: 'moon.full',
      master: 11,
      power: 7,
      prerelease: 'rc.1',
    });
  });

  it.each(['0.1.1', '10.11.22', '7.11', 'v7.11.33'])('rejects profane version %s', (version) => {
    expect(isPowerVersion(version)).toBe(false);
  });
});

describe('nextPowerVersion', () => {
  it.each([
    ['7.11.22', 'aura', '7.11.33'],
    ['7.11.33', 'aura', '7.22.1'],
    ['7.33.33', 'aura', '8.1.1'],
    ['7.11.22', 'master', '7.22.1'],
    ['7.33.22', 'master', '8.1.1'],
    ['7.11.22', 'power', '8.1.1'],
  ] as const)('advances %s at %s to %s', (version, component, expected) => {
    expect(nextPowerVersion(version, component)).toBe(expected);
  });

  it('recognizes completion', () => {
    expect(() => nextPowerVersion('33.33.33')).toThrow('Archive the project');
  });
});
