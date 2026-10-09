// Check both directions: either missing precision or a different exported
// method/getter/setter is a binding gap. Numerical behavior is verified by the
// shared-module computational families, not inferred from this structural check.
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
const serialization = process.argv[3] !== '--no-serialization';

function shape(value) {
    return Object.getOwnPropertyNames(value)
        .filter(name => !['name', 'length', 'prototype', 'constructor', '__destroy_into_raw'].includes(name))
        .sort()
        .map(name => {
            const descriptor = Object.getOwnPropertyDescriptor(value, name);
            return [name, typeof descriptor.value, descriptor.value?.length, typeof descriptor.get, typeof descriptor.set];
        });
}

let pairs = 0;
for (const name of Object.keys(api).filter(name => /F(?:32|64)$|_f(?:32|64)$/.test(name))) {
    const single = /F32$|_f32$/.test(name);
    const mate = single ? name.replace(/32$/, '64') : name.replace(/64$/, '32');
    if (!(mate in api)) throw new Error('Missing precision counterpart: ' + mate);
    const first = api[name], second = api[mate];
    if (typeof first !== typeof second || first.length !== second.length) throw new Error('Export arity/type mismatch: ' + name);
    if (JSON.stringify(shape(first)) !== JSON.stringify(shape(second))) throw new Error('Static method mismatch: ' + name);
    if (Boolean(first.prototype) !== Boolean(second.prototype)) throw new Error('Owner/function mismatch: ' + name);
    if (first.prototype && JSON.stringify(shape(first.prototype)) !== JSON.stringify(shape(second.prototype))) {
        throw new Error('Instance method/getter/setter mismatch: ' + name);
    }
    if (single) pairs++;
}
const expected = serialization ? 67 : 59;
if (pairs !== expected) throw new Error(`Audited precision pair count changed: ${pairs} != ${expected}`);
console.log('Liblrhsmm actual Node.js precision exports:', JSON.stringify({pairs, serialization}));
