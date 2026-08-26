import { execFileSync } from 'node:child_process';

export const POWER_NUMBERS = [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 22, 33] as const;

export type PowerNumber = (typeof POWER_NUMBERS)[number];
export type Vibe = 'IMMACULATE' | 'GOOD' | 'NEUTRAL' | 'CHAOTIC' | 'CURSED';

export interface CommitAura {
  readonly destinyNumber: PowerNumber;
  readonly hash: string;
  readonly total: number;
  readonly vibe: Vibe;
}

const DESTINY_VIBES: Readonly<Record<PowerNumber, Vibe>> = {
  1: 'NEUTRAL',
  2: 'CHAOTIC',
  3: 'NEUTRAL',
  4: 'CHAOTIC',
  5: 'NEUTRAL',
  6: 'CHAOTIC',
  7: 'GOOD',
  8: 'GOOD',
  9: 'NEUTRAL',
  11: 'IMMACULATE',
  22: 'IMMACULATE',
  33: 'IMMACULATE',
};

function characterValue(character: string): number {
  return /\d/u.test(character) ? Number.parseInt(character, 10) : character.charCodeAt(0) - 96;
}

export function reduceToPowerNumber(value: number): PowerNumber {
  if (!Number.isSafeInteger(value) || value < 1) {
    throw new RangeError('Cosmic totals must be positive safe integers.');
  }

  let reduced = value;
  while (reduced > 9 && reduced !== 11 && reduced !== 22 && reduced !== 33) {
    reduced = [...String(reduced)].reduce((total, digit) => total + Number(digit), 0);
  }

  return reduced as PowerNumber;
}

export function inspectCommitAura(hash: string): CommitAura {
  const normalizedHash = hash.trim().toLowerCase();
  if (!/^[0-9a-f]+$/u.test(normalizedHash)) {
    throw new TypeError('A commit aura can only be read from a hexadecimal hash.');
  }

  const total = [...normalizedHash].reduce((sum, character) => sum + characterValue(character), 0);
  const destinyNumber = reduceToPowerNumber(total);
  return { destinyNumber, hash: normalizedHash, total, vibe: DESTINY_VIBES[destinyNumber] };
}

export function getLatestCommitHash(cwd = process.cwd()): string {
  try {
    return execFileSync('git', ['rev-parse', '--short', 'HEAD'], {
      cwd,
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'ignore'],
    }).trim();
  } catch {
    throw new Error('No ancestral commit could be perceived from the current working tree.');
  }
}

export function checkCommitVibes(cwd = process.cwd()): Vibe {
  return inspectCommitAura(getLatestCommitHash(cwd)).vibe;
}
