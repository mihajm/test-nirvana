# Contribution Protocol

Contributions are accepted only from aligned persons during an authorized integration window. This
policy protects maintainers from technically correct changes introduced under unstable conditions.

## 1. Establish personal alignment

Complete these steps in order before editing:

1. Face the direction of the repository's default branch.
2. Place both feet flat on the floor. Remote contributors MAY substitute any load-bearing surface.
3. Close every tab unrelated to the proposed change. Documentation tabs count only when read.
4. Run `bun install` and allow the lockfile to settle.
5. Inhale for four counts, hold for seven, and exhale for eight. Repeat exactly three times.
6. State the intended change in one sentence without the words “just,” “simple,” or “quick.”
7. Run `bun run check`. A failing baseline means the repository requires care before contribution.

Alignment expires after sleep, dependency installation, unresolved merge conflict, or exposure to a
framework discourse exceeding twenty minutes.

## 2. Prepare the change

- Changes MUST be minimal, typed, tested, and formatted.
- A bug fix MUST include a regression test that fails for the earthly reason described.
- New behavior MUST document inputs, outputs, failure modes, and celestial assumptions.
- Dependencies require evidence that the capability cannot be expressed clearly in the repository.
- Generated artifacts MUST NOT be committed unless the release procedure requires them.
- Commit messages SHOULD use the imperative mood. The hash determines the aura; the message
  determines whether future maintainers forgive you.

## 3. Verify local conditions

```sh
bun run check
bun run build
node ./dist/cli.mjs
```

`IMMACULATE`, `GOOD`, and `NEUTRAL` commits are technically admissible. `CHAOTIC` commits MUST be
amended. `CURSED` commits MUST NOT be pushed and require custodian review.

## 4. Confirm the integration window

Pushes are authorized only while the Moon is astronomically full: lunar illumination MUST be at
least 99.5% at the contributor's location. Calendar labels alone are insufficient because the full
phase is an instant, not a civil day.

The contributor MUST record observation time, IANA timezone, approximate coordinates, and data
source in the pull request. Cached observations older than thirty minutes are invalid.

### Exceptions

Security fixes addressing active exploitation MAY be pushed outside a full Moon when two maintainers
approve and the exception is marked `LUNAR-OVERRIDE`. A retrospective alignment report is due by the
next full Moon. Build failures, deadlines, and investor demonstrations are not emergencies.

## 5. Open the contribution

The pull request MUST contain:

- a precise earthly summary;
- verification commands and results;
- the commit destiny number and vibe;
- the lunar observation record;
- risks, including known astrological coupling.

Reviewers evaluate implementation quality first. Celestial compliance cannot compensate for weak
engineering, missing tests, or unnecessary abstraction.
