"""Compile binary64 inference and preparation against pinned original C code."""
import argparse
from pathlib import Path
import re
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
    cases = [('geometric', 'geometric_oracle.c', []),
             ('hsmm', 'hsmm_oracle.c', []),
             ('hsmm-fractional', 'hsmm_oracle.c', ['-DFRACTIONAL_ORACLE']),
             ('preparation', 'preparation_oracle.c', []),
             ('emissions', 'emission_table_oracle.c', []),
             ('emissions-fractional', 'emission_table_oracle.c', ['-DFRACTIONAL_ORACLE']),
             ('hsmm-viterbi', 'hsmm_oracle.c', ['-DVITERBI_SCORE_ORACLE']),
             ('hsmm-viterbi-fractional', 'hsmm_oracle.c', ['-DVITERBI_SCORE_ORACLE', '-DFRACTIONAL_ORACLE'])]
    units = ['common.c', 'math-funcs.c', 'model.c', 'data.c', 'inference.c', 'mempool.c']
    with tempfile.TemporaryDirectory() as temporary:
        temporary = Path(temporary)
        score_source = temporary/'score-source'
        shutil.copytree(args.upstream, score_source, ignore=shutil.ignore_patterns('.git', 'build'))
        subprocess.run(['git', 'apply', str(tests/'reference'/'viterbi-score-reference.patch')], cwd=score_source, check=True)
        for family, filename, defines in cases:
            upstream = score_source if '-DVITERBI_SCORE_ORACLE' in defines else args.upstream
            driver = (tests/'reference'/filename).read_text()
            if family == 'geometric':
                start = driver.index('                float total =')
                end = driver.index('                free(forward)', start)
                driver = driver[:start]+driver[end:]
            driver = re.sub(r'\bfloat\b', 'FP_TYPE', driver)
            driver = re.sub(r'(\d[\d.]*(?:e[+-]?\d+)?)f\b', r'\1', driver)
            driver = driver.replace('%.9g', '%.17g')
            driver_path = temporary/(family+'.c')
            driver_path.write_text(driver)
            executable = temporary/(family+('.exe' if args.zig else ''))
            command = [args.cc]+(['cc'] if args.zig else [])+[
                '-std=c99', '-O2', '-ffp-contract=off', '-DFP_TYPE=double',
                *defines, '-I', str(upstream), str(driver_path),
                *[str(upstream/unit) for unit in units], '-lm', '-o', str(executable),
            ]
            subprocess.run(command, check=True)
            content = subprocess.check_output([str(executable)]).replace(b'\r\n', b'\n')
            destination = tests/'fixtures'/('c-double-'+family+'.txt')
            if args.check:
                old_lines = destination.read_text().splitlines()
                new_lines = content.decode().splitlines()
                assert len(old_lines) == len(new_lines)
                maximum = 0.0
                for old, new in zip(old_lines, new_lines):
                    old, new = old.split(), new.split()
                    assert len(old) == len(new)
                    for a, b in zip(old, new):
                        try:
                            a, b = float(a), float(b)
                        except ValueError:
                            assert a == b
                            continue
                        maximum = max(maximum, abs(a-b)/max(1.0, abs(a)))
                assert maximum <= 2e-14, (family, maximum)
                print(family, len(old_lines), 'records verified; normalized difference', maximum, flush=True)
            else:
                destination.write_bytes(content)
                print(family, len(content.splitlines()), 'records generated', flush=True)


if __name__ == '__main__':
    main()
