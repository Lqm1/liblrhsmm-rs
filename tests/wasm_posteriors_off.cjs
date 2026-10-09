const assert = require('node:assert/strict');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
for (const suffix of ['F32', 'F64']) for (const name of ['StateOccupancies', 'MixtureOccupancyMode']) assert.equal(name + suffix in api, false, name + suffix);
for (const name of ['TransitionPosterior', 'TransitionPosteriors', 'DurationPosterior', 'DurationPosteriors', 'MixtureWeights', 'MixturePosteriors']) assert.equal(name in api, false, name);
console.log('Actual feature-disabled posterior interface: 10 owners absent');
