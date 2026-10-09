const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
import('./wasm_builder_resize_checks.mjs').then(({verifyBuilderResize}) => {
    console.log('Liblrhsmm builder resize actual Node.js:', JSON.stringify(verifyBuilderResize(api)));
}).catch(error => {console.error(error); process.exitCode = 1;});
