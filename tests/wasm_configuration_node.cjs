const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
import('./wasm_configuration_checks.mjs').then(({verifyConfigurationBindings}) => {
    console.log('Liblrhsmm configuration actual Node.js:', JSON.stringify(verifyConfigurationBindings(api)));
}).catch(error => { console.error(error); process.exitCode = 1; });
