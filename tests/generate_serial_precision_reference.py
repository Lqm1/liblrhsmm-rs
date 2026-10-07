"""Reproduce the C double jump-reader defect and verify corrected wire round trips."""
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
    fixtures = tests/'fixtures'
    with tempfile.TemporaryDirectory() as directory:
        directory = Path(directory)
        source = directory/'source'
        shutil.copytree(args.upstream, source, ignore=shutil.ignore_patterns('.git', 'build'))
        outputs = directory/'outputs'
        outputs.mkdir()
        executable = directory/('oracle.exe' if args.zig else 'oracle')
        command = [args.cc]+(['cc'] if args.zig else [])+[
            '-std=c99', '-O2', '-ffp-contract=off', '-DFP_TYPE=double',
            '-I', str(source), str(tests/'reference'/'serial_precision_oracle.c'),
            *[str(source/unit) for unit in ['common.c', 'math-funcs.c', 'model.c', 'data.c', 'serial.c', 'external/cmp/cmp.c']],
            '-lm', '-o', str(executable),
        ]
        subprocess.run(command, check=True, capture_output=True)
        result = subprocess.run([str(executable), str(fixtures), str(outputs)], capture_output=True)
        assert result.returncode == 9, (result.returncode, result.stdout, result.stderr)
        print('original double reader reproduced:', result.stdout.decode().strip(), flush=True)
        patch = (tests/'reference'/'serial-double-jump.patch').read_text().splitlines()
        old = '\n'.join(line[1:] for line in patch if line.startswith('-') and not line.startswith('---'))
        new = '\n'.join(line[1:] for line in patch if line.startswith('+') and not line.startswith('+++'))
        serial = source/'serial.c'
        content = serial.read_text()
        assert content.count(old) == 1
        serial.write_text(content.replace(old, new))
        subprocess.run(command, check=True)
        subprocess.run([str(executable), str(fixtures), str(outputs)], check=True)
        count = 0
        for output in outputs.iterdir():
            fixture = fixtures/output.name
            if output.name == 'c-double-wire-duration.bin' and not args.check:
                fixture.write_bytes(output.read_bytes())
            assert output.read_bytes() == fixture.read_bytes(), output.name
            count += 1
        assert count == 9
        print(count, 'double C wire outputs verified byte-for-byte', flush=True)


if __name__ == '__main__':
    main()
