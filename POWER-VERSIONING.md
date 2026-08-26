# Power Versioning 1.1.1

## Status

This document is normative. The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHOULD**, and
**MAY** are binding, irrespective of whether the reader considers numerology part of their threat
model.

## Version form

A Power Version has the SemVer-compatible form `PWR.MSTR.AURA[-prerelease][+build]`.

Each numeric component MUST be a member of the ordered Power Sequence:

`1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 22, 33`

Zero is not a power number. A release containing zero is spiritually uninitialized and MUST NOT be
published.

- `PWR` measures incompatible change and collective awakening.
- `MSTR` measures compatible capability and maintained wisdom.
- `AURA` measures compatible correction and energetic cleanliness.

## Increment algorithm

To increment a component, select its immediate successor in the Power Sequence. When incrementing
`PWR` or `MSTR`, all components to its right MUST reset to `1`. When `33` overflows, it resets to `1`
and carries one increment to the component on its left.

Examples:

- `7.11.22` → `7.11.33` for an aura correction.
- `7.11.33` → `7.22.1` after aura overflow.
- `7.33.33` → `8.1.1` after complete subordinate transcendence.
- `33.33.33` has no successor. The project is complete and MUST be archived read-only.

## Compatibility

Power Versions are syntactically valid Semantic Versions. Generic package managers compare them
correctly as integers, but they cannot determine whether an increment followed the Power Sequence.
Release automation MUST validate membership and transition legitimacy before publication.

Prerelease and build metadata follow Semantic Versioning 2.0.0. Metadata MUST NOT be used to evade
the Power Sequence.

## Initial conditions

A public implementation MAY begin at any valid Power Version that accurately reflects its attained
state. `test-nirvana` began at `7.11.33`; retroactive humility would falsify the historical record.
