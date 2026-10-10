function assert(condition, message) {
    if (!condition) throw new Error(message);
}
function throws(action, message) {
    let failed = false;
    try { action(); } catch { failed = true; }
    assert(failed, message);
}
function shape(pool) {
    return [pool.page_size, pool.page_count, pool.total_capacity, pool.allocated_bytes];
}
export async function verifyMemoryPoolBindings(api, readFixture) {
    const text = new TextDecoder().decode(await readFixture('c-memory-pool.txt'));
    const records = text.trim().split(/\r?\n/);
    const requests = [[false, 1, 0], [false, 1, 3], [true, 1, 5], [false, 1, 1],
        [true, 3, 4], [true, 0, 17], [false, 1, 2], [true, 2, 3]];
    let compared = 0;
    let pageChecks = 0;
    let byteChecks = 0;
    for (const [scenario, pageSize] of [8, 0].entries()) {
        const pool = new api.MemoryPool(pageSize);
        assert(JSON.stringify(shape(pool)) === JSON.stringify([pageSize, 1, pageSize, 0]), 'initial page');
        const initialPages = pool.pages();
        const allocations = [];
        let lastTop = 0;
        let lastPage = 1;
        let allocated = 0;
        for (const [index, [zeroed, number, size]] of requests.entries()) {
            const allocation = zeroed ? pool.allocate_zeroed(number, size) : pool.allocate(number * size);
            const length = number * size;
            assert(allocation.length === length && allocation.is_empty() === (length === 0), 'region metadata');
            assert([...pool.get(allocation)].every(value => value === 0), 'initialized allocation');
            pool.set_bytes(allocation, 0, new Uint8Array(length).fill(index + 1));
            const offset = lastPage === pool.page_count ? lastTop : 0;
            lastTop = offset + length;
            lastPage = pool.page_count;
            allocated += length;
            assert(pool.allocated_bytes === allocated, 'allocated accounting');
            const actual = `A ${scenario} ${index} ${pool.page_count} ${pool.total_capacity} ${lastTop} ${offset} ${length} 1`;
            assert(actual === records[compared++], 'original C allocation record: ' + actual);
            const iterator = pool.pages();
            assert(iterator.length === pool.page_count && !iterator.is_empty(), 'exact-size page iterator');
            let usage = 0;
            let capacity = 0;
            for (let pageIndex = 0; pageIndex < pool.page_count; pageIndex++) {
                const page = iterator.next();
                const indexed = pool.page(pageIndex);
                assert(page.used_bytes === indexed.used_bytes && page.capacity === indexed.capacity, 'ordered page accounting');
                assert(iterator.length === pool.page_count - pageIndex - 1, 'remaining iterator length');
                usage += page.used_bytes;
                capacity += page.capacity;
                page.free(); indexed.free(); pageChecks++;
            }
            assert(iterator.is_empty() && iterator.next() === null && iterator.next_back() === null, 'fused exhaustion');
            assert(usage === pool.allocated_bytes && capacity === pool.total_capacity, 'complete page sums');
            iterator.free();
            allocations.push(allocation);
        }
        for (const [index, allocation] of allocations.entries()) {
            const values = pool.get(allocation);
            const checksum = [...values].reduce((sum, value) => sum + value, 0);
            assert(`V ${scenario} ${index} ${checksum}` === records[compared++], 'original C retained bytes');
            byteChecks += values.length;
            values.fill(99);
            assert([...pool.get(allocation)].every(value => value === index + 1), 'detached region copies');
        }
        const region = allocations[1];
        const shared = region.cloned();
        pool.set_bytes(shared, 1, new Uint8Array([11, 13]));
        assert(JSON.stringify([...pool.get(region)]) === '[2,11,13]', 'shared region identity');
        assert(JSON.stringify([...pool.get_range(shared, 1, 2)]) === '[11,13]', 'partial reads');
        assert(pool.get_range(shared, 3, 0).length === 0, 'empty terminal read');
        pool.set_bytes(shared, 3, new Uint8Array([]));
        const beforeShape = JSON.stringify(shape(pool));
        const beforeBytes = JSON.stringify([...pool.get(shared)]);
        for (const action of [
            () => pool.allocate_zeroed(2 ** 53, 2),
            () => pool.allocate(2 ** 53),
            () => pool.allocate(Infinity),
            () => pool.get_range(shared, 0xffffffff, 2),
            () => pool.get_range(shared, 4, 0),
            () => pool.set_bytes(shared, 2, new Uint8Array([1, 2])),
            () => pool.set_bytes(shared, 0xffffffff, new Uint8Array([1, 2])),
        ]) {
            throws(action, 'invalid pool allocation or range');
            assert(JSON.stringify(shape(pool)) === beforeShape && JSON.stringify([...pool.get(shared)]) === beforeBytes, 'failed operation atomicity');
        }
        assert(pool.page(pool.page_count) === null && pool.page(0xffffffff) === null, 'absent page index');
        const other = new api.MemoryPool(pageSize);
        assert(other.get(shared) === null && other.get(allocations[0]) === null, 'cross-pool rejection including empty region');
        throws(() => other.get_range(shared, 0, 0), 'cross-pool empty read');
        throws(() => other.set_bytes(shared, 0, new Uint8Array([])), 'cross-pool empty write');
        const snapshot = pool.pages();
        const snapshotClone = snapshot.cloned();
        const mixed = snapshot.cloned();
        const first = mixed.next();
        const last = mixed.next_back();
        const firstExpected = pool.page(0);
        const lastExpected = pool.page(pool.page_count - 1);
        assert(first.used_bytes === firstExpected.used_bytes && first.capacity === firstExpected.capacity
            && last.used_bytes === lastExpected.used_bytes && last.capacity === lastExpected.capacity, 'mixed-direction page iteration');
        first.free(); last.free(); firstExpected.free(); lastExpected.free();
        assert(mixed.length === pool.page_count - 2, 'mixed remaining length');
        const mixedClone = mixed.cloned();
        while (!mixed.is_empty()) {
            const left = mixed.next_back();
            const right = mixedClone.next_back();
            assert(left.used_bytes === right.used_bytes && left.capacity === right.capacity, 'partially consumed iterator cloning');
            left.free(); right.free();
        }
        mixed.free(); mixedClone.free();
        const retainedPage = pool.page(0);
        retainedPage.used_bytes = 123;
        retainedPage.capacity = 456;
        const nativePage = pool.page(0);
        assert(nativePage.used_bytes !== 123 && nativePage.capacity !== 456, 'mutable page copies do not change pool');
        nativePage.free();
        for (const allocation of allocations) allocation.free();
        assert(JSON.stringify([...pool.get(shared)]) === '[2,11,13]', 'original handle release retains clone region');
        const retainedShape = shape(pool);
        pool.free();
        assert(shared.length === 3 && !shared.is_empty() && other.get(shared) === null, 'retained metadata after pool release');
        const postReleaseClone = shared.cloned();
        assert(postReleaseClone.length === 3 && other.get(postReleaseClone) === null, 'retained identity cannot match another pool');
        postReleaseClone.free(); shared.free(); other.free();
        assert(snapshot.length === retainedShape[1] && snapshotClone.length === snapshot.length, 'independent page snapshots');
        while (!snapshot.is_empty()) {
            const left = snapshot.next();
            const right = snapshotClone.next();
            assert(left.used_bytes === right.used_bytes && left.capacity === right.capacity, 'cloned snapshot values');
            left.free(); right.free();
        }
        snapshot.free(); snapshotClone.free();
        assert(initialPages.length === 1, 'initial snapshot retained before growth');
        const initialPage = initialPages.next_back();
        assert(initialPage.used_bytes === 0 && initialPage.capacity === pageSize && initialPages.is_empty(), 'reverse copied page iteration');
        initialPage.free(); initialPages.free();
        assert(retainedPage.used_bytes === 123 && retainedPage.capacity === 456, 'retained editable page fields');
        retainedPage.free();
    }
    throws(() => new api.MemoryPool(Number.MAX_SAFE_INTEGER + 1), 'unrepresentable initial page');
    assert(compared === records.length && compared === 32 && pageChecks === 52 && byteChecks === 58, 'complete original C pool corpus');
    return {records: compared, pageChecks, byteChecks};
}
