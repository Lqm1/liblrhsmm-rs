"""Copy the first complete stream from the original pretrained model without re-encoding."""
from pathlib import Path
import hashlib

source=Path('tests/fixtures/cmu-arctic-all-speakers.hsmm').read_bytes()
position=0

def take(count):
    global position
    value=source[position:position+count]
    assert len(value)==count
    position+=count
    return value

def array():
    marker=take(1)[0]
    if 0x90<=marker<=0x9f:
        return marker & 15
    assert marker in (0xdc,0xdd)
    return int.from_bytes(take(2 if marker==0xdc else 4),'big')

def skip():
    marker=source[position]
    if 0x90<=marker<=0x9f or marker in (0xdc,0xdd):
        for _ in range(array()):
            skip()
    else:
        take(1)
        if marker<=0x7f or marker>=0xe0:
            return
        sizes={0xca:4,0xcb:8,0xcc:1,0xcd:2,0xce:4,0xcf:8,0xd0:1,0xd1:2,0xd2:4,0xd3:8}
        assert marker in sizes,hex(marker)
        take(sizes[marker])

assert array()==2
assert array()>0
begin=position
skip()
output=source[begin:position]
Path('tests/fixtures/c-historical-standalone-stream.bin').write_bytes(output)
print(f'Copied original stream [{begin}:{position}], {len(output)} bytes, SHA256 {hashlib.sha256(output).hexdigest()}')
