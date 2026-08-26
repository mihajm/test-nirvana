export interface NirvanaOptions {
  readonly name?: string;
}

/** Returns the sacred greeting without performing any side effects. */
export function reachNirvana({ name = 'world' }: NirvanaOptions = {}): string {
  return `testing nirvana achieved, ${name} ✨`;
}
