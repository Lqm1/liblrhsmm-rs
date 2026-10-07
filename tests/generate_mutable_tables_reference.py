"""Verify public table modification using unmodified float/double C sources."""
import argparse
from pathlib import Path
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
    units = ['common.c', 'math-funcs.c', 'model.c', 'data.c', 'inference.c', 'mempool.c', 'estimate.c']
    with tempfile.TemporaryDirectory() as directory:
        for precision in ('float', 'double'):
            executable = Path(directory)/(precision+('.exe' if args.zig else ''))
            command = [args.cc]+(['cc'] if args.zig else [])+[
                '-std=c99', '-O2', '-ffp-contract=off', f'-DFP_TYPE={precision}',
                '-I', str(args.upstream), str(tests/'reference'/'mutable_tables_oracle.c'),
                *[str(args.upstream/unit) for unit in units], '-lm', '-o', str(executable),
            ]
            subprocess.run(command, check=True)
            content = subprocess.check_output([str(executable)]).replace(b'\r\n', b'\n')
            fixture = tests/'fixtures'/f'c-mutable-tables-{precision}.txt'
            if args.check:
                assert fixture.read_bytes() == content, precision
            else:
                fixture.write_bytes(content)
            print(precision, len(content.splitlines()), 'records verified' if args.check else 'records generated', flush=True)


if __name__ == '__main__':
    main()
