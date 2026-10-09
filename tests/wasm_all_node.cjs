const fs = require('node:fs/promises');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
import('./wasm_all_checks.mjs').then(async ({verifyAllBindings}) => {
    const result = await verifyAllBindings(api, name => fs.readFile(path.join(__dirname, 'fixtures', name)), process.argv[3] !== '--no-serialization');
    console.log('Liblrhsmm all families actual Node.js:', JSON.stringify(result));
}).catch(error => {console.error(error); process.exitCode = 1;});
