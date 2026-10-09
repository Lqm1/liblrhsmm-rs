const fs = require('node:fs/promises');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
import('./wasm_duration_checks.mjs').then(async ({ verifyDurationBindings }) => {
    console.log('Liblrhsmm duration actual Node.js:', JSON.stringify(await verifyDurationBindings(api, name => fs.readFile(path.join(__dirname, 'fixtures', name)), process.argv[3] !== '--no-serialization')));
}).catch(error => { console.error(error); process.exitCode = 1; });
