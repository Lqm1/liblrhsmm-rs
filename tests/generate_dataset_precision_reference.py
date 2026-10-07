"""Check double regrouping against the disclosed corrected C implementation."""
import argparse
from pathlib import Path
import shutil
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
        directory = Path(directory)
        source = directory/'source'
        shutil.copytree(args.upstream, source, ignore=shutil.ignore_patterns('.git', 'build'))
        data = source/'data.c'
        content = data.read_text()
        # Apply each complete hunk from the existing disclosed patch; offsets are
        # irrelevant after line-ending normalization. Each old block must be unique.
        blocks = (tests/'reference'/'data-regroup.patch').read_text().split('@@')
        for block in blocks[2::2]:
            lines = block.splitlines()[1:]
            old = '\n'.join(line[1:] for line in lines if line.startswith((' ', '-')))
            new = '\n'.join(line[1:] for line in lines if line.startswith((' ', '+')))
            assert old and content.count(old) == 1
            content = content.replace(old, new)
        data.write_text(content)
        executable = directory/('oracle.exe' if args.zig else 'oracle')
        command = [args.cc]+(['cc'] if args.zig else [])+[
            '-std=c99', '-O2', '-ffp-contract=off', '-DFP_TYPE=double',
            '-I', str(source), str(tests/'reference'/'dataset_precision_oracle.c'),
            str(data), str(source/'common.c'), '-lm', '-o', str(executable),
        ]
        subprocess.run(command, check=True)
        content = subprocess.check_output([str(executable)]).replace(b'\r\n', b'\n')
        fixture = tests/'fixtures'/'corrected-c-double-datasets.txt'
        if args.check:
            assert fixture.read_bytes() == content
            print(len(content.splitlines()), 'records verified byte-for-byte', flush=True)
        else:
            fixture.write_bytes(content)
            print(len(content.splitlines()), 'records generated', flush=True)


if __name__ == '__main__':
    main()
