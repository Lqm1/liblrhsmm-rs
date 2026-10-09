const assert = require('node:assert/strict');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
for (const suffix of ['F32', 'F64']) for (const name of ['Stream', 'StreamBuilder', 'PreparedStream', 'StreamCollection', 'Model', 'ModelBuilder', 'PreparedModel', 'DecodedStream', 'DecodedModel']) assert.equal(name + suffix in api, false, name + suffix);
console.log('Actual feature-disabled container interface: 18 owners absent');
