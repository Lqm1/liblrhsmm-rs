const assert = require('node:assert/strict');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
for (const suffix of ['F32', 'F64']) for (const name of ['ObservationSetBuilder', 'SegmentationSetBuilder', 'Dataset', 'DecodedObservationSet', 'DecodedSegmentationSet']) assert.equal(name + suffix in api, false, name + suffix);
for (const precision of ['f32', 'f64']) assert.equal('regroup_data_' + precision in api, false);
console.log('Actual feature-disabled dataset interface: 10 owners and2 functions absent');
