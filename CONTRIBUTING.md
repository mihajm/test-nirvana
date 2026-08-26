# Contribution Protocol

Contributions are accepted only from aligned persons during an authorized integration window. This
policy protects maintainers from technically correct changes introduced under unstable conditions.

Before opening an editor, run `npx test-nirvana` (or `bun run build && node ./dist/cli.mjs`) against
the repository's current HEAD. A `CURSED` result MUST end the session; the repository does not
accept contributions from a cursed working tree. Commit something else first, then re-check.
`CHAOTIC` MAY proceed, but the resulting pull request will be watched.

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

## 2. Respect the one rule

`test-nirvana` computes the numerological destiny of a Git commit hash. It already operates at peak
philosophical absurdity and MUST NOT accumulate incidental complexity to compensate for that. A
change MUST NOT introduce, without two or more concrete present-tense use cases:

- a plugin system;
- dependency injection;
- a class hierarchy;
- generics beyond `Record<PowerNumber, Vibe>`;
- a second CI provider "just in case";
- anything named a manager, handler, processor, or orchestrator.

YAGNI applies in double when the product is divination software. An abstraction justified by an
imagined future use case is not aligned; it is scope creep wearing a robe.

## 3. Prepare the change

- Changes MUST be minimal, typed, tested, and formatted.
- A pull request MUST address one destiny concern. Refactoring the reduction algorithm and adding a
  second classification axis are two concerns; open two pull requests.
- A bug fix MUST include a regression test that fails for the earthly reason described.
- New behavior MUST document inputs, outputs, failure modes, and celestial assumptions.
- Dependencies require evidence that the capability cannot be expressed clearly in the repository.
- Generated artifacts MUST NOT be committed unless the release procedure requires them.
- Commit messages SHOULD use the imperative mood. The hash determines the aura; the message
  determines whether future maintainers forgive you.

## 4. Verify local conditions

```sh
bun run check
bun run build
node ./dist/cli.mjs
```

`IMMACULATE`, `GOOD`, and `NEUTRAL` commits are technically admissible. `CHAOTIC` commits MUST be
amended. `CURSED` commits MUST NOT be pushed and require custodian review.

`checkCommitVibes` MUST remain synchronous. It is a vibe check, not a microservice.

## 5. Confirm the integration window

Pushes are authorized only while the Moon is astronomically full: lunar illumination MUST be at
least 99.5% at the contributor's location. Calendar labels alone are insufficient because the full
phase is an instant, not a civil day.

The contributor MUST record observation time, IANA timezone, approximate coordinates, and data
source in the pull request. Cached observations older than thirty minutes are invalid.

### Exceptions

Security fixes addressing active exploitation MAY be pushed outside a full Moon when two maintainers
approve and the exception is marked `LUNAR-OVERRIDE`. A retrospective alignment report is due by the
next full Moon. Build failures, deadlines, and investor demonstrations are not emergencies.

## 6. Open the contribution

The pull request MUST contain:

- a precise earthly summary;
- verification commands and results;
- the commit destiny number and vibe;
- the lunar observation record;
- risks, including known astrological coupling.

Reviewers evaluate implementation quality first. Celestial compliance cannot compensate for weak
engineering, missing tests, or unnecessary abstraction.

## 7. Adding a new Vibe

To propose a vibe beyond `IMMACULATE`, `GOOD`, `NEUTRAL`, `CHAOTIC`, and `CURSED`:

1. Extend the `Vibe` union in `src/core.ts`.
2. Map at least one member of `POWER_NUMBERS` to it in `DESTINY_VIBES`. Every power number MUST
   resolve to exactly one vibe; none MAY be left orphaned.
3. Add a corresponding glyph to `SIGILS` in `src/sigils.ts` and to `VIBE_SIGILS` in `src/cli.ts`.
4. Justify the vibe's existence in the pull request description with at least one citation from a
   numerology source. This repository has standards.
5. Update `LICENSE` condition (b), (c), or (d) if the new vibe changes existential risk, liability,
   or bragging-rights classification. Conditions (a) and the disclaimer are load-bearing and MUST
   NOT change.

## 8. What will not be merged

- An asynchronous `checkCommitVibes`. See §4.
- A web UI. This is a library and a CLI. It lives in the terminal and dies in the terminal.
- Special treatment for hashes longer than short form. Extra entropy does not improve the
  spirituality; it just makes the numbers bigger.
- Natural-language interpretation of a vibe via an LLM. Do not open this pull request.
- An independent second classification axis opened without prior custodian sign-off — see §9.

## 9. On a second classification axis

A change that derives an independent value alongside `destinyNumber` — for example composing
`${Vibe}_${SecondVibe}` — is under discussion. The design is not settled. Do not open a pull request
for it before a custodian confirms the direction in an issue. A premature proposal will be closed
with the label `cosmic timing: wrong`.

## 10. Reporting a bug

Open an issue containing:

1. the commit hash;
2. the vibe `test-nirvana` returned for it;
3. what actually happened when you shipped anyway.

If the vibe was `IMMACULATE` and the deploy still failed, that is not a bug in this repository. The
numbers advised gratitude, not recklessness.
