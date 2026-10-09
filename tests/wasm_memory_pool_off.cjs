const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'liblrhsmm_rs.js'));
for (const name of ['MemoryPool', 'PoolAllocation', 'PoolPages', 'PoolPageUsage']) {
    if (name in api) throw new Error('Feature-disabled export present: ' + name);
}
console.log('Liblrhsmm feature-disabled memory pool: four owners absent.');
