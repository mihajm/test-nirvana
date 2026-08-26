import packageManifest from '../package.json' with { type: 'json' };
import { parsePowerVersion } from '../src/version.ts';

const version = parsePowerVersion(packageManifest.version);
console.log(`✨ ${packageManifest.version} is a valid Power Version.`, version);
