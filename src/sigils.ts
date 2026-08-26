/** Canonical glyphs. Consumers MUST NOT infer behavior from visually similar Unicode. */
export const SIGILS = {
  aura: '🫧',
  chaotic: '🌀',
  cursed: '☄️',
  fullMoon: '🌕',
  good: '🪷',
  immaculate: '✨',
  master: '🔮',
  mercury: '☿️',
  neutral: '🧘',
  power: '⚡',
} as const;

export type Sigil = (typeof SIGILS)[keyof typeof SIGILS];
