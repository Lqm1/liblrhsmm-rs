const fs = require('node:fs/promises');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
import('./wasm_emissions_checks.mjs').then(async ({verifyEmissionBindings}) => {
    console.log('Liblrhsmm emissions actual Node.js:', JSON.stringify(await verifyEmissionBindings(api, name => fs.readFile(path.join(__dirname, 'fixtures', name)))));
}).catch(error => { console.error(error); process.exitCode = 1; });
