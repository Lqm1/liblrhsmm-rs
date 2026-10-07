"""Check duration-bound macro results in both source precision modes."""
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
    parser.add_argument('--optimization', choices=('strict', 'fast'), default='strict')
    args = parser.parse_args()
    tests = Path(__file__).resolve().parent
    with tempfile.TemporaryDirectory() as directory:
        for precision in ('float', 'double'):
            executable = Path(directory)/(precision+('.exe' if args.zig else ''))
            subprocess.run([args.cc]+(['cc'] if args.zig else [])+[
                '-std=c99', '-O2' if args.optimization == 'strict' else '-Ofast',
                '-ffp-contract=off', f'-DFP_TYPE={precision}',
                '-I', str(args.upstream), str(tests/'reference'/'duration_bounds_oracle.c'),
                str(args.upstream/'common.c'), '-lm', '-o', str(executable),
            ], check=True)
            content = subprocess.check_output([str(executable)]).replace(b'\r\n', b'\n')
            assert len(content.splitlines()) == 3024
            fixture = tests/'fixtures'/f'c-duration-bounds-{precision}.txt'
            if args.check:
                assert fixture.read_bytes() == content, precision
            else:
                fixture.write_bytes(content)
            print(precision, '3024 duration bounds verified' if args.check else '3024 duration bounds generated', flush=True)


if __name__ == '__main__':
    main()
