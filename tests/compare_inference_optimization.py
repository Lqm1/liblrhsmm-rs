"""Measure fast-math effects on inference paths and expectation statistics."""
import argparse
import hashlib
import json
import math
from pathlib import Path
import re
import shutil
import struct
import subprocess
import tempfile


def strict_reference(expected, actual, precision):
    old, new = expected.splitlines(), actual.splitlines()
    if len(old) != len(new):
        raise ValueError('Strict reference record count changed')
    for first, second in zip(old, new):
        a, b = first.split(), second.split()
        if len(a) != len(b):
            raise ValueError('Strict reference field count changed')
        for x, y in zip(a, b):
            try:
                x, y = float(x), float(y)
            except ValueError:
                if x != y:
                    raise ValueError('Strict reference label changed')
                continue
            if precision == 'float':
                same = struct.pack('<f', x) == struct.pack('<f', y)
            else:
                same = math.isfinite(x) and math.isfinite(y) and abs(x-y) / max(1, abs(x)) <= 2e-14
            if not same:
                raise ValueError(f'Strict reference value changed: {first} / {second}')


def compare(strict, fast):
    decisions = ('path', 'boundary')
    prefixes = {'scores': 6, 'total': 6, 'occupancy': 6, 'math': 2,
                'score': 3, 'duration': 4, 'mixture': 5}
    old_paths = [line for line in strict.splitlines() if line.split()[0] in decisions]
    new_paths = [line for line in fast.splitlines() if line.split()[0] in decisions]
    result = {'discrete_records': len(old_paths), 'discrete_records_identical': old_paths == new_paths,
              'numeric_values': 0, 'changed_values': 0, 'maximum_absolute_error': 0.0,
              'maximum_normalized_error': 0.0, 'first_difference': None}
    old = [line for line in strict.splitlines() if line.split()[0] not in decisions]
    new = [line for line in fast.splitlines() if line.split()[0] not in decisions]
    if len(old) != len(new):
        raise ValueError('Numeric record count changed')
    for first, second in zip(old, new):
        a, b = first.split(), second.split()
        # Estimation totals use mode and temperature, inference totals have five keys.
        prefix = 3 if a[0] == 'total' and len(a) == 4 else prefixes[a[0]]
        if a[:prefix] != b[:prefix] or len(a) != len(b):
            raise ValueError(f'Input/identifier fields changed: {first} / {second}')
        for field, (x, y) in enumerate(zip(a[prefix:], b[prefix:]), prefix):
            x, y = float(x), float(y)
            if not math.isfinite(x) or not math.isfinite(y):
                raise ValueError('Nonfinite workflow output')
            result['numeric_values'] += 1
            if x != y:
                result['changed_values'] += 1
                error = abs(x-y)
                result['maximum_absolute_error'] = max(result['maximum_absolute_error'], error)
                result['maximum_normalized_error'] = max(result['maximum_normalized_error'], error/max(1, abs(x)))
                if result['first_difference'] is None:
                    result['first_difference'] = {'record': a[:prefix], 'field': field, 'strict': x, 'fast': y}
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--upstream', type=Path, required=True)
    parser.add_argument('--cc', default='gcc')
    parser.add_argument('--zig', action='store_true')
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    tests = Path(__file__).resolve().parent
    compiler = [args.cc] + (['cc'] if args.zig else [])
    units = ['common.c', 'math-funcs.c', 'model.c', 'data.c', 'inference.c', 'mempool.c', 'estimate.c']
    cases = [('geometric', 'geometric_oracle.c', 'c-geometric.txt', 'c-double-geometric-posteriors.txt'),
             ('hsmm', 'hsmm_oracle.c', 'c-hsmm.txt', 'c-double-hsmm.txt'),
             ('estimation', 'estimation_oracle.c', 'corrected-c-estimation.txt', 'c-double-estimation.txt')]
    report = {'compiler': subprocess.check_output(compiler+['--version'], text=True).splitlines()[0],
              'source_commit': subprocess.check_output(['git', '-C', str(args.upstream), 'rev-parse', 'HEAD'], text=True).strip(),
              'flags': {'strict': ['-O2', '-ffp-contract=off'], 'fast': ['-O3', '-ffast-math', '-ffp-contract=off']},
              'source_correction': 'Only estimation uses the disclosed duration-initialization.patch in a temporary copy',
              'results': {}, 'effective_fast_macros': {}}
    with tempfile.TemporaryDirectory(prefix='inference-optimization-') as directory:
        root = Path(directory)
        corrected = root/'corrected'
        shutil.copytree(args.upstream, corrected, ignore=shutil.ignore_patterns('.git', 'build'))
        patch = (tests/'reference/duration-initialization.patch').read_text().splitlines()
        removed = [line[1:] for line in patch if line.startswith('-') and not line.startswith('---')]
        added = [line[1:] for line in patch if line.startswith('+') and not line.startswith('+++')]
        source = (corrected/'inference.c').read_text()
        if len(removed) != 1 or len(added) != 1 or source.count(removed[0]) != 1:
            raise ValueError('Cannot apply unique disclosed duration correction')
        (corrected/'inference.c').write_text(source.replace(removed[0], added[0]))
        for precision in ['float', 'double']:
            macros = subprocess.check_output(compiler+['-O3', '-ffast-math', '-ffp-contract=off', '-dM', '-E', '-x', 'c', '-'], input='', text=True)
            effective = [line for line in macros.splitlines() if any(name in line for name in ['__FAST_MATH__', '__FINITE_MATH_ONLY__'])]
            if '#define __FAST_MATH__ 1' not in effective:
                raise ValueError('Requested fast math is not enabled')
            report['effective_fast_macros'][precision] = effective
            report['results'][precision] = {}
            for family, filename, float_fixture, double_fixture in cases:
                upstream = corrected if family == 'estimation' else args.upstream
                driver = (tests/'reference'/filename).read_text()
                driver = re.sub(r'\bfloat\b', 'FP_TYPE', driver)
                driver = re.sub(r'(\d[\d.]*(?:e[+-]?\d+)?)f\b', r'\1', driver)
                driver = driver.replace('%.9g', '%.17g')
                path = root/'driver.c'
                path.write_text(driver)
                outputs = {}
                for mode, flags in report['flags'].items():
                    executable = root/('driver.exe' if args.zig else 'driver')
                    subprocess.run(compiler+['-std=c99', *flags, '-DFP_TYPE='+precision, '-I', str(upstream),
                        str(path), *[str(upstream/unit) for unit in units], '-lm', '-o', str(executable)], check=True, timeout=120)
                    outputs[mode] = subprocess.check_output([str(executable)], timeout=30).decode().replace('\r\n', '\n')
                fixture = tests/'fixtures'/(float_fixture if precision == 'float' else double_fixture)
                strict_reference(fixture.read_text(), outputs['strict'], precision)
                result = compare(outputs['strict'], outputs['fast'])
                result['sha256'] = {mode: hashlib.sha256(content.encode()).hexdigest() for mode, content in outputs.items()}
                report['results'][precision][family] = result
                print(precision, family, result, flush=True)
    args.output.write_text(json.dumps(report, indent=2, allow_nan=False)+'\n')


if __name__ == '__main__':
    main()
