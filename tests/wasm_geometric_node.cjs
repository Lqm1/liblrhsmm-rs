const fs = require('node:fs/promises');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
import('./wasm_geometric_checks.mjs').then(async ({verifyGeometricBindings}) => {
    console.log('Liblrhsmm geometric actual Node.js:', JSON.stringify(await verifyGeometricBindings(api, name => fs.readFile(path.join(__dirname, 'fixtures', name)))));
}).catch(error => { console.error(error); process.exitCode = 1; });
