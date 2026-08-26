import { defineConfig } from 'tsdown';

export default defineConfig({
  entry: {
    cli: 'src/cli.ts',
    index: 'src/index.ts',
  },
  clean: true,
  dts: true,
  format: 'esm',
  minify: false,
  platform: 'node',
  sourcemap: true,
});
