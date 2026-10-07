"""Compile the original model and density functions at both precisions."""
import argparse
from pathlib import Path
import subprocess
import struct
import tempfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--upstream', type=Path, required=True)
    parser.add_argument('--cc', default='gcc')
    parser.add_argument('--zig', action='store_true')
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    tests = Path(__file__).resolve().parent
    units = ['common.c', 'math-funcs.c', 'model.c', 'data.c', 'inference.c', 'mempool.c']
    with tempfile.TemporaryDirectory() as temporary:
        for precision in ['float', 'double']:
            executable = Path(temporary)/('oracle-'+precision+('.exe' if args.zig else ''))
            command = [args.cc] + (['cc'] if args.zig else []) + [
                '-std=c99', '-O2', '-ffp-contract=off', '-DFP_TYPE='+precision, '-I', str(args.upstream),
                str(tests/'reference'/'precision_oracle.c'),
                *[str(args.upstream/unit) for unit in units], '-lm', '-o', str(executable),
            ]
            subprocess.run(command, check=True)
            content = subprocess.check_output([str(executable)]).replace(b'\r\n', b'\n')
            destination = tests/'fixtures'/('c-precision-'+precision+'.txt')
            if args.check:
                # C libm may differ by a few binary64 ulps between hosts.
                expected = destination.read_text().splitlines()
                actual = content.decode().splitlines()
                assert len(expected) == len(actual) == 72
                maximum = 0.0
                maximum_float_ulps = 0
                worst = None
                for before, after in zip(expected, actual):
                    old, new = before.split(), after.split()
                    assert old[:3] == new[:3] and len(old) == len(new)
                    for position, (a, b) in enumerate(zip(old[3:], new[3:])):
                        a, b = float(a), float(b)
                        float_output = precision == 'float' or (
                            old[0] in ['component', 'output'] and position == 1
                        ) or (old[0] == 'duration' and position == 2)
                        if float_output:
                            def ordered(value):
                                bits = struct.unpack('<I', struct.pack('<f', value))[0]
                                return (~bits & 0xffffffff) if bits >> 31 else bits | 0x80000000
                            ulps = abs(ordered(a)-ordered(b))
                            if ulps > maximum_float_ulps:
                                maximum_float_ulps = ulps
                                worst = (old[:3], position, a, b)
                        else:
                            maximum = max(maximum, abs(a-b)/max(1.0, abs(a)))
                print(precision, '72 source records; binary64 normalized difference', maximum,
                      'binary32-output ulps', maximum_float_ulps, 'worst', worst, flush=True)
                assert maximum_float_ulps == 0, worst
                assert maximum <= 2e-14, maximum
            else:
                assert len(content.splitlines()) == 72
                destination.write_bytes(content)
                print(precision, '72 source records generated')


if __name__ == '__main__':
    main()
