#!/usr/bin/env node
import { parseArgs } from 'node:util';

import {
  SIGILS,
  getLatestCommitHash,
  inspectCommitAura,
  nextPowerVersion,
  parsePowerVersion,
  type PowerVersionComponent,
  type Vibe,
} from './index.ts';

const EXIT_CODES: Readonly<Record<Vibe, number>> = {
  IMMACULATE: 0,
  GOOD: 0,
  NEUTRAL: 0,
  CHAOTIC: 1,
  CURSED: 2,
};

const VIBE_SIGILS: Readonly<Record<Vibe, string>> = {
  IMMACULATE: SIGILS.immaculate,
  GOOD: SIGILS.good,
  NEUTRAL: SIGILS.neutral,
  CHAOTIC: SIGILS.chaotic,
  CURSED: SIGILS.cursed,
};

const { positionals, values } = parseArgs({
  allowPositionals: true,
  options: {
    hash: { type: 'string' },
    help: { short: 'h', type: 'boolean' },
    json: { type: 'boolean' },
    version: { short: 'v', type: 'boolean' },
  },
  strict: true,
});

if (values.help) {
  console.log(`test-nirvana — deterministic commit-aura enforcement

Usage:
  test-nirvana [--hash <hex>] [--json]
  test-nirvana version validate <version>
  test-nirvana version next <version> [aura|master|power]

Options:
      --hash       Inspect a specific hexadecimal hash instead of HEAD
      --json       Emit the complete aura as JSON
  -h, --help       Show this canonical guidance
  -v, --version    Show the Power Version`);
} else if (values.version && positionals.length === 0) {
  const { version } = await import('../package.json', { with: { type: 'json' } });
  console.log(version);
} else if (positionals[0] === 'version') {
  const [, operation, version, component = 'aura'] = positionals;
  if (operation === 'validate' && version !== undefined) {
    parsePowerVersion(version);
    console.log(`${SIGILS.immaculate} ${version} belongs to the Power Sequence.`);
  } else if (
    operation === 'next' &&
    version !== undefined &&
    ['aura', 'master', 'power'].includes(component)
  ) {
    console.log(nextPowerVersion(version, component as PowerVersionComponent));
  } else {
    throw new TypeError('Invalid version operation. Consult --help before proceeding.');
  }
} else {
  if (positionals.length > 0) {
    throw new TypeError(`Unknown rite: ${positionals.join(' ')}.`);
  }
  const aura = inspectCommitAura(values.hash ?? getLatestCommitHash());
  console.log(
    values.json
      ? JSON.stringify(aura, undefined, 2)
      : `${VIBE_SIGILS[aura.vibe]} ${aura.vibe} — ${aura.hash} resolves through ${aura.total} to ${aura.destinyNumber}`,
  );
  process.exitCode = EXIT_CODES[aura.vibe];
}
