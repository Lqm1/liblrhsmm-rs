"""Measure optimizer differences without replacing strict C reference fixtures."""
import argparse
import hashlib
import json
from pathlib import Path
import struct
import subprocess
import tempfile


def records(content, width):
    assert content[:4] == b'NUM1'
    assert struct.unpack_from('<II', content, 4) == (width, 7000)
    offset = 12 + 14000 * width
    count, = struct.unpack_from('<I', content, offset)
    assert count == 42008
    offset += 4
    assert len(content) == offset + count * (4 + 3 * width)
    for _ in range(count):
        kind, = struct.unpack_from('<I', content, offset)
        offset += 4
        values = [content[offset + i * width:offset + (i + 1) * width] for i in range(3)]
        offset += 3 * width
        yield kind, values
    assert offset == len(content)


def compare(strict, optimized, width):
    scalar = '<f' if width == 4 else '<d'
    result = {name: {'samples': 0, 'changed_inputs': 0, 'changed_outputs': 0,
                     'maximum_absolute_error': 0.0, 'maximum_normalized_error': 0.0,
                     'first_difference': None}
              for name in ('log_add', 'negative_exponential', 'exponential')}
    assert strict[:12 + 14000 * width] == optimized[:12 + 14000 * width]
    for index, ((kind, old), (new_kind, new)) in enumerate(zip(records(strict, width), records(optimized, width))):
        assert kind == new_kind
        case = result[('log_add', 'negative_exponential', 'exponential')[kind]]
        case['samples'] += 1
        case['changed_inputs'] += old[:2] != new[:2]
        if old[2] != new[2]:
            case['changed_outputs'] += 1
            a, b = (struct.unpack(scalar, payload)[0] for payload in (old[2], new[2]))
            error = abs(a - b)
            case['maximum_absolute_error'] = max(case['maximum_absolute_error'], error)
            case['maximum_normalized_error'] = max(case['maximum_normalized_error'], error / max(1.0, abs(a)))
            if case['first_difference'] is None:
                case['first_difference'] = {'record': index,
                    'strict_inputs': [struct.unpack(scalar, value)[0] for value in old[:2]],
                    'optimized_inputs': [struct.unpack(scalar, value)[0] for value in new[:2]],
                    'strict_output': a, 'optimized_output': b}
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
    report = {'compiler': subprocess.check_output(compiler + ['--version'], text=True).splitlines()[0],
              'source_commit': subprocess.check_output(['git', '-C', str(args.upstream), 'rev-parse', 'HEAD'], text=True).strip(),
              'common_flags': ['-std=c99', '-ffp-contract=off'],
              'driver_adaptation': 'Use finite FLT_MAX directions instead of infinity; strict payload equality is required',
              'results': {}, 'index_probes': {}, 'payload_sha256': {}, 'effective_macros': {}}
    with tempfile.TemporaryDirectory() as directory:
        driver = Path(directory)/'finite_numerics_oracle.c'
        original = (tests/'reference'/'numerics_oracle.c').read_text()
        driver.write_text('#include <float.h>\n'+original.replace('INFINITY', 'FLT_MAX'))
        for precision, width in [('float', 4), ('double', 8)]:
            variants = {}
            for flag in ['-O2', '-Ofast', '-O3 -ffast-math', '-Ofast -fno-reciprocal-math', '-Ofast -fno-unsafe-math-optimizations']:
                executable = Path(directory)/(precision + ('.exe' if args.zig else ''))
                subprocess.run(compiler + ['-std=c99', *flag.split(), '-ffp-contract=off', '-DFP_TYPE='+precision,
                    '-I', str(args.upstream), str(driver),
                    str(args.upstream/'math-funcs.c'), '-lm', '-o', str(executable)], check=True)
                variants[flag] = subprocess.check_output([str(executable)])
            strict = variants['-O2']
            assert strict == (tests/'fixtures'/f'c-numerics-{precision}.bin').read_bytes(), 'strict baseline changed'
            report['results'][precision] = {flag: compare(strict, content, width) for flag, content in variants.items() if flag != '-O2'}
            report['payload_sha256'][precision] = {flag: hashlib.sha256(content).hexdigest() for flag, content in variants.items()}
            report['index_probes'][precision] = {}
            report['effective_macros'][precision] = {}
            for flag in variants:
                common = compiler + ['-std=c99', *flag.split(), '-ffp-contract=off', '-DFP_TYPE='+precision]
                index_driver = tests/'reference'/'numerics_index_oracle.c'
                macros = subprocess.check_output(common + ['-dM', '-E', str(index_driver)], text=True)
                report['effective_macros'][precision][flag] = {line.split()[1]: ' '.join(line.split()[2:])
                    for line in macros.splitlines() if line.startswith('#define ')
                    and line.split()[1] in ('__FAST_MATH__', '__FINITE_MATH_ONLY__', '__NO_MATH_ERRNO__')}
                subprocess.run(common + [str(index_driver), '-o', str(executable)], check=True)
                indices = [int(value) for value in subprocess.check_output([str(executable)]).split()]
                assembly = Path(directory)/'index.s'
                subprocess.run(common + ['-S', str(index_driver), '-o', str(assembly)], check=True)
                operations = [line.strip() for line in assembly.read_text().splitlines()
                              if any(operation in line for operation in ('divsd', 'mulsd', 'cvttsd2si'))]
                report['index_probes'][precision][flag] = {'indices': indices, 'assembly_operations': operations}
            print(precision, {flag: {name: data['changed_outputs'] for name, data in results.items()}
                              for flag, results in report['results'][precision].items()}, flush=True)
    args.output.write_text(json.dumps(report, indent=2, allow_nan=False)+'\n', encoding='utf-8')


if __name__ == '__main__':
    main()
