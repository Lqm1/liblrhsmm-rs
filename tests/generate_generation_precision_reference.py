"""Compile unmodified double C generation with controlled native rand draws."""
import argparse
from pathlib import Path
import re
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
    driver = (tests/'reference'/'generation_oracle.c').read_text()
    driver = re.sub(r'\bfloat\b', 'FP_TYPE', driver)
    driver = re.sub(r'(\d[\d.]*(?:e[+-]?\d+)?)f\b', r'\1', driver)
    driver = driver.replace('%.9g', '%.17g')
    driver = driver.replace('int main(void) {', 'int main(void) {\n    printf("maximum %d\\n", RAND_MAX);')
    with tempfile.TemporaryDirectory() as directory:
        directory = Path(directory)
        source = directory/'driver.c'
        source.write_text(driver)
        executable = directory/('oracle.exe' if args.zig else 'oracle')
        command = [args.cc]+(['cc'] if args.zig else [])+[
            '-std=c99', '-O2', '-ffp-contract=off', '-DFP_TYPE=double',
            '-I', str(args.upstream), str(source),
            *[str(args.upstream/unit) for unit in ['common.c', 'math-funcs.c', 'model.c', 'data.c', 'generate.c']],
            '-lm', '-o', str(executable),
        ]
        subprocess.run(command, check=True)
        content = subprocess.check_output([str(executable)]).replace(b'\r\n', b'\n')
        maximum = int(content.splitlines()[0].split()[1])
        assert maximum in (32767, 2147483647), maximum
        fixture = tests/'fixtures'/f'c-double-generation-{maximum}.txt'
        if args.check:
            assert fixture.read_bytes() == content
            print(maximum, len(content.splitlines()), 'records verified byte-for-byte', flush=True)
        else:
            fixture.write_bytes(content)
            print(maximum, len(content.splitlines()), 'records generated', flush=True)


if __name__ == '__main__':
    main()
