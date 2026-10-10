const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const { pathToFileURL } = require('node:url');
const { test } = require('node:test');
const api = require('..');
const root = path.resolve(__dirname, '../../..');

test('all original C inference, training, codec, and ownership oracles', async () => {
  const { verifyAllBindings } = await import(pathToFileURL(path.join(__dirname, 'oracles/native_all_checks.mjs')));
  const report = await verifyAllBindings(api, name => fs.readFile(path.join(root, 'tests/fixtures', name)));
  assert.equal(report.families, 18);
  assert.equal(report.passes, 2);
});

test('native package version and checked bounds', () => {
  assert.equal(api.version(), '0.1.0');
  const pool = new api.MemoryPool(16);
  const allocation = pool.allocate_zeroed(4, 2);
  assert.deepEqual(Array.from(pool.get(allocation)), Array(8).fill(0));
  pool.set_bytes(allocation, 2, new Uint8Array([1, 2, 3]));
  assert.deepEqual(Array.from(pool.get_range(allocation, 2, 3)), [1, 2, 3]);
  assert.throws(() => pool.set_bytes(allocation, 7, new Uint8Array([1, 2])));
  pool.close();
  assert.throws(() => pool.get(allocation), /consumed/);
});
