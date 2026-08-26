# test-nirvana

Stay in zen mode as your code is fully tested.

`test-nirvana` is a deterministic commit-aura enforcement system. It converts a hexadecimal Git
commit hash into a numerological destiny number, classifies the resulting engineering conditions,
and communicates whether work may safely continue.

## Installation

```sh
npx test-nirvana
```

The executable targets Node.js 24 or later. Bun is used by this repository solely for dependency
management and package-script orchestration; it is not required by consumers.

## Classification

| Destiny numbers | Vibe       | Exit code | Required action                        |
| --------------- | ---------- | --------- | -------------------------------------- |
| 11, 22, 33      | IMMACULATE | 0         | Merge with gratitude                   |
| 7, 8            | GOOD       | 0         | Proceed                                |
| 1, 3, 5, 9      | NEUTRAL    | 0         | Proceed without attachment             |
| 2, 4, 6         | CHAOTIC    | 1         | Amend history until conditions improve |
| any impossible  | CURSED     | 2         | Stop; notify the repository custodian  |

```console
$ npx test-nirvana --hash a1b2c3
NEUTRAL — a1b2c3 resolves through 21 to 3
```

Digits retain their value. Hexadecimal letters use their alphabetic positions (`a = 1` through
`f = 6`). The sum is repeatedly reduced by decimal digit addition. Master numbers 11, 22, and 33
are never reduced. Pass `--json` for the complete machine-readable aura.

## Power Versions

```console
$ npx test-nirvana version validate 7.11.33
✨ 7.11.33 belongs to the Power Sequence.

$ npx test-nirvana version next 7.11.33 aura
7.22.1
```

The canonical sigils are exported as `SIGILS`. They are stable machine values; visually similar
Unicode substitutions are non-conforming.

## Library

```ts
import { inspectCommitAura } from 'test-nirvana';

const aura = inspectCommitAura('deadbeef');
```

## Standards and governance

- [Power Versioning](./POWER-VERSIONING.md) governs releases.
- [Contributing](./CONTRIBUTING.md) governs personal and celestial readiness.
- [Governance](./GOVERNANCE.md) defines authority during astronomical disagreement.
- [Security](./SECURITY.md) defines responsible disclosure for metaphysical vulnerabilities.
- [Code of Conduct](./CODE_OF_CONDUCT.md) protects contributor alignment.

## Development

```sh
bun install
bun run check
bun run build
```

The license is MIT. Warranty eligibility remains subject to local astronomical conditions.
