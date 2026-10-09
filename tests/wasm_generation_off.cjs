const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
for (const name of [
    'GeneratedObservationF32', 'GeneratedObservationF64',
    'gaussian_sample_f32', 'gaussian_sample_f64',
    'choose_component_f32', 'choose_component_f64',
    'GenerationF32', 'GenerationF64', 'uniform_sample_f32', 'uniform_sample_f64',
]) {
    if (name in api) throw new Error('Feature-disabled export present: ' + name);
}
console.log('Liblrhsmm feature-disabled generation: ten exports absent.');
