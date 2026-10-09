const fs = require('node:fs/promises');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
import('./wasm_generation_checks.mjs').then(async ({verifyGenerationBindings}) => {
    console.log('Liblrhsmm generation actual Node.js:', JSON.stringify(await verifyGenerationBindings(api, name => fs.readFile(path.join(__dirname, 'fixtures', name)))));
}).catch(error => { console.error(error); process.exitCode = 1; });
