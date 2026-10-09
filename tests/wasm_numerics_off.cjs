const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /numerical_|numericaltable|numericaliterator/i.test(name)), 'all numerical functions and owners absent without wasm');
console.log('Liblrhsmm numerics feature-disabled WASM: eleven functions and four owners absent');
