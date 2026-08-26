import { describe, expect, it } from 'vitest';

import { reachNirvana } from './index.ts';

describe('reachNirvana', () => {
  it('achieves the default state of enlightenment', () => {
    expect(reachNirvana()).toBe('testing nirvana achieved, world ✨');
  });

  it('addresses a named seeker', () => {
    expect(reachNirvana({ name: 'grug' })).toBe('testing nirvana achieved, grug ✨');
  });
});
