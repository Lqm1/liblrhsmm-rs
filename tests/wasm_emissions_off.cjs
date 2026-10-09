const assert = require('node:assert/strict');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
for (const suffix of ['F32', 'F64']) for (const name of ['EmissionPruning', 'LogEmissions', 'EmissionDensities']) assert.equal(name + suffix in api, false, name + suffix);
console.log('Actual feature-disabled emissions interface: 6 owners absent');
