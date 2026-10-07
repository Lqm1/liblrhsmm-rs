"""Compile both upstream precision modes and derive binary64 literal tables."""
import argparse
from pathlib import Path
import re
import struct
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--upstream', type=Path, required=True)
    parser.add_argument('--cc', default='gcc')
    parser.add_argument('--zig', action='store_true')
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    tests = Path(__file__).resolve().parent
    source = (args.upstream/'math-funcs.c').read_text()
    for symbol, filename in [('lrh_lse_table', 'lse-table-f64.bin'), ('lrh_exp_table', 'exp-table-f64.bin')]:
        match = re.search(r'FP_TYPE\s+'+symbol+r'\[[^]]+\]\s*=\s*\{([^}]+)\}', source)
        if match is None:
            raise ValueError('Missing table: '+symbol)
        values = [float(value.strip()) for value in match.group(1).split(',') if value.strip()]
        assert len(values) == 7000
        content = struct.pack('<7000d', *values)
        destination = tests.parent/'src'/filename
        if args.check:
            if destination.read_bytes() != content:
                raise ValueError('Literal table changed: '+filename)
        else:
            destination.write_bytes(content)
    with tempfile.TemporaryDirectory() as temporary:
        for precision in ['float', 'double']:
            executable = Path(temporary)/('oracle-'+precision+('.exe' if args.zig else ''))
            command = [args.cc] + (['cc'] if args.zig else []) + [
                '-std=c99', '-O2', '-DFP_TYPE='+precision, '-I', str(args.upstream),
                str(tests/'reference'/'numerics_oracle.c'), str(args.upstream/'math-funcs.c'),
                '-lm', '-o', str(executable),
            ]
            subprocess.run(command, check=True)
            content = subprocess.check_output([str(executable)])
            width, count = struct.unpack_from('<II', content, 4)
            assert content[:4] == b'NUM1' and width == (4 if precision == 'float' else 8) and count == 7000
            if precision == 'double':
                assert content[12:12+56000] == (tests.parent/'src'/'lse-table-f64.bin').read_bytes()
                assert content[12+56000:12+112000] == (tests.parent/'src'/'exp-table-f64.bin').read_bytes()
            else:
                assert content[12:12+28000] == (tests.parent/'src'/'lse-table.bin').read_bytes()
                assert content[12+28000:12+56000] == (tests.parent/'src'/'exp-table.bin').read_bytes()
            destination = tests/'fixtures'/('c-numerics-'+precision+'.bin')
            if args.check:
                if destination.read_bytes() != content:
                    raise ValueError('C numerics reference changed: '+precision)
            else:
                destination.write_bytes(content)
            print(precision, '14000 table entries and 42008 helper outputs verified')


if __name__ == '__main__':
    main()
