const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
import('./wasm_ownership_checks.mjs').then(({verifyOwnershipBindings}) => {
    console.log('Liblrhsmm ownership actual Node.js:', JSON.stringify(verifyOwnershipBindings(api)));
}).catch(error => { console.error(error); process.exitCode = 1; });
