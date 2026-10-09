const fs = require('node:fs/promises');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
import('./wasm_statistics_checks.mjs').then(async ({ verifyStatisticsBindings }) => {
    console.log('Liblrhsmm statistics actual Node.js:', JSON.stringify(await verifyStatisticsBindings(api, name => fs.readFile(path.join(__dirname, 'fixtures', name)), process.argv[3] !== '--no-serialization')));
}).catch(error => { console.error(error); process.exitCode = 1; });
