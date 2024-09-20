import { fileURLToPath } from 'node:url'

import esbuild from 'esbuild'
import path from 'node:path'

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

esbuild.buildSync({
  entryPoints: [`${__dirname}/../dist/tests/**/*.test.js`],
  format: 'esm',
  bundle: true,
  sourcemap: true,
  platform: 'node',
  target: ['node18'],
  outdir: `${__dirname}/compiled/node`,
});
