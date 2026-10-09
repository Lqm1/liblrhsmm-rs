const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
import('./wasm_posterior_storage_checks.mjs').then(({verifyPosteriorStorage}) => {
    console.log('Liblrhsmm posterior storage actual Node.js:', JSON.stringify(verifyPosteriorStorage(api)));
}).catch(error => { console.error(error); process.exitCode = 1; });
