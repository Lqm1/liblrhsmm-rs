"""Extract an unchanged standalone GMM from SHIRO's original pretrained model.

This copies a byte span; it never writes or normalizes MessagePack values.
Run from the package root. The source fixture is retained with its provenance.
"""
from pathlib import Path
import struct

source = Path("tests/fixtures/cmu-arctic-all-speakers.hsmm").read_bytes()
position = 0

def take(count):
    global position
    result = source[position:position + count]
    assert len(result) == count
    position += count
    return result

def array():
    marker = take(1)[0]
    if 0x90 <= marker <= 0x9f:
        return marker & 15
    assert marker in [0xdc, 0xdd]
    return int.from_bytes(take(2 if marker == 0xdc else 4), "big")

def integer():
    marker = take(1)[0]
    if marker <= 0x7f:
        return marker
    sizes = {0xcc: 1, 0xcd: 2, 0xce: 4, 0xd0: 1, 0xd1: 2, 0xd2: 4}
    assert marker in sizes
    return int.from_bytes(take(sizes[marker]), "big", signed=marker >= 0xd0)

def floating():
    assert take(1) == b"\xca"
    return struct.unpack(">f", take(4))[0]

assert array() == 2
assert array() > 0
assert array() == 2
floating()  # Stream weight.
assert array() > 0
begin = position
assert array() == 4
mixtures, dimensions = integer(), integer()
assert mixtures > 0 and dimensions > 0
assert array() == mixtures
for _ in range(mixtures):
    floating()
assert array() == mixtures * dimensions * 2
for _ in range(mixtures * dimensions * 2):
    floating()
output = source[begin:position]
Path("tests/fixtures/c-historical-standalone-gmm.bin").write_bytes(output)
print(f"Copied original GMM bytes [{begin}:{position}], {mixtures} components, {dimensions} dimensions, {len(output)} bytes")
