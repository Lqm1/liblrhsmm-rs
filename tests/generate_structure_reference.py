"""Verify prepared-cache and pool-page metadata against unmodified C."""
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
    with tempfile.TemporaryDirectory() as directory:
        for precision in ('float', 'double'):
            executable = Path(directory)/(precision+('.exe' if args.zig else ''))
            command = [args.cc]+(['cc'] if args.zig else [])+[
                '-std=c99', '-O2', '-ffp-contract=off', f'-DFP_TYPE={precision}',
                '-I', str(args.upstream), str(tests/'reference'/'structure_oracle.c'),
                *[str(args.upstream/unit) for unit in ['common.c', 'math-funcs.c', 'model.c', 'mempool.c']],
                '-lm', '-o', str(executable),
            ]
            subprocess.run(command, check=True)
            content = subprocess.check_output([str(executable)]).replace(b'\r\n', b'\n')
            fixture = tests/'fixtures'/f'c-structures-{precision}.txt'
            if args.check:
                assert fixture.read_bytes() == content
                print(precision, len(content.splitlines()), 'records verified byte-for-byte', flush=True)
            else:
                fixture.write_bytes(content)
                print(precision, len(content.splitlines()), 'records generated', flush=True)


if __name__ == '__main__':
    main()
