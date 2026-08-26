import { POWER_NUMBERS, type PowerNumber } from './core.ts';

export type PowerVersionComponent = 'aura' | 'master' | 'power';

export interface PowerVersion {
  readonly aura: PowerNumber;
  readonly build?: string;
  readonly master: PowerNumber;
  readonly power: PowerNumber;
  readonly prerelease?: string;
}

const POWER_VERSION_PATTERN =
  /^(?<power>\d+)\.(?<master>\d+)\.(?<aura>\d+)(?:-(?<prerelease>[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+(?<build>[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$/u;

function assertPowerNumber(
  value: number,
  component: PowerVersionComponent,
): asserts value is PowerNumber {
  if (!(POWER_NUMBERS as readonly number[]).includes(value)) {
    throw new RangeError(
      `${component.toUpperCase()} must belong to the Power Sequence; received ${value}.`,
    );
  }
}

export function parsePowerVersion(version: string): PowerVersion {
  const match = POWER_VERSION_PATTERN.exec(version);
  if (!match?.groups) {
    throw new TypeError(`Invalid Power Version: ${version}.`);
  }

  const power = Number(match.groups.power);
  const master = Number(match.groups.master);
  const aura = Number(match.groups.aura);
  assertPowerNumber(power, 'power');
  assertPowerNumber(master, 'master');
  assertPowerNumber(aura, 'aura');

  return {
    aura,
    ...(match.groups.build === undefined ? {} : { build: match.groups.build }),
    master,
    power,
    ...(match.groups.prerelease === undefined ? {} : { prerelease: match.groups.prerelease }),
  };
}

export function isPowerVersion(version: string): boolean {
  try {
    parsePowerVersion(version);
    return true;
  } catch {
    return false;
  }
}

function successor(value: PowerNumber): PowerNumber | undefined {
  return POWER_NUMBERS[POWER_NUMBERS.indexOf(value) + 1];
}

export function nextPowerVersion(
  version: string,
  component: PowerVersionComponent = 'aura',
): string {
  const current = parsePowerVersion(version);
  let { aura, master, power } = current;

  if (component === 'power') {
    const next = successor(power);
    if (next === undefined) throw new RangeError('33.33.33 has no successor. Archive the project.');
    return `${next}.1.1`;
  }

  if (component === 'master') {
    aura = 1;
    const next = successor(master);
    if (next !== undefined) return `${power}.${next}.1`;
    master = 1;
  } else {
    const next = successor(aura);
    if (next !== undefined) return `${power}.${master}.${next}`;
    aura = 1;
    const nextMaster = successor(master);
    if (nextMaster !== undefined) return `${power}.${nextMaster}.${aura}`;
    master = 1;
  }

  const nextPower = successor(power);
  if (nextPower === undefined)
    throw new RangeError('33.33.33 has no successor. Archive the project.');
  return `${nextPower}.${master}.${aura}`;
}
