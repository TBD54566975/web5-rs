import { fileURLToPath } from 'node:url'

import esbuild from 'esbuild'
import path from 'node:path'
import fs from 'node:fs'

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

esbuild.buildSync({
  format: 'esm',
  bundle: true,
  minify: true,
  entryPoints: [`${__dirname}/../pkg/web5_wasm.js`],
  outfile: `${__dirname}/../dist/bundle.js`,
  allowOverwrite: true,
})

fs.copyFileSync(`${__dirname}/../pkg/web5_wasm.d.ts`, `${__dirname}/../dist/web5_wasm.d.ts`)
