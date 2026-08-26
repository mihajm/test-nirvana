#!/usr/bin/env node
import { parseArgs } from 'node:util';

import { reachNirvana } from './index.ts';

const { values } = parseArgs({
  options: {
    help: { short: 'h', type: 'boolean' },
    name: { short: 'n', type: 'string' },
    version: { short: 'v', type: 'boolean' },
  },
  strict: true,
});

if (values.help) {
  console.log(`test-nirvana

Usage:
  test-nirvana [--name <name>]

Options:
  -n, --name       Name the enlightened one
  -h, --help       Show this help
  -v, --version    Show the version`);
} else if (values.version) {
  const { version } = await import('../package.json', { with: { type: 'json' } });
  console.log(version);
} else {
  console.log(reachNirvana(values.name === undefined ? {} : { name: values.name }));
}
