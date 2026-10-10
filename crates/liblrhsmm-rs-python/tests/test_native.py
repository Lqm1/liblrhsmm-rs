"""Python model, allocation, and inference checks against the original C data."""
from importlib.metadata import version
from pathlib import Path
import struct

import pytest
import liblrhsmm_rs as api

FIXTURES = Path(__file__).resolve().parents[3] / "tests" / "fixtures"


@pytest.mark.parametrize("suffix", ["F32", "F64"])
@pytest.mark.parametrize("filename", ["current-c-model.hsmm", "cmu-arctic-all-speakers.hsmm"])
def test_existing_model_roundtrip(suffix, filename):
    wire = (FIXTURES / filename).read_bytes()
    model = getattr(api, f"Model{suffix}").read(wire)
    model.validate()
    encoding = 1 if filename.startswith("cmu-") else 0
    assert model.write_with_encoding(encoding) == wire
    clone = model.cloned()
    model.close()
    assert clone.write_with_encoding(encoding) == wire
    with pytest.raises(RuntimeError):
        model.write()


@pytest.mark.parametrize("suffix", ["F32", "F64"])
@pytest.mark.parametrize("fractional", [False, True])
def test_hsmm_c_scores(suffix, fractional):
    scalar = (lambda x: struct.unpack("f", struct.pack("f", x))[0]) if suffix == "F32" else float
    cases = {}

    def compute(scenario):
        if scenario in cases:
            return cases[scenario]
        model = getattr(api, f"Model{suffix}")([], 4)
        segmentation = getattr(api, f"Segmentation{suffix}")(0, 4)
        options = getattr(api, f"HsmmOptions{suffix}")()
        for state in range(4):
            duration = model.duration(state)
            duration.set_values([2.5 + state * 0.25, 0.75 + state * 0.25, 0])
            duration.set_constraints([2 if state == 1 else -1, 3 if scenario == 3 and state == 1 else 7, 0])
            model.set_duration(state, duration)
        segmentation.set_boundaries([3, 6, 9, 12])
        segmentation.set_duration_states([0, 1, 2, 3])
        if scenario > 0:
            for source, delta, probability in [(0, 2, 0.2), (1, 0, 0.15), (2, -1, 0.1)]:
                segmentation.add_jump(source, delta, probability)
            for state in range(3):
                segmentation.add_jump(state, 1, 0.8)
        desired = [0, 0, 2, 2, 1, 1, 1, 1, 2, 3, 3, 3]
        values = [0 if state == desired[frame] else -50 for frame in range(12) for state in range(4)] if scenario == 3 else [-(frame - state * 3) ** 2 * 0.125 for frame in range(12) for state in range(4)]
        emissions = getattr(api, f"LogEmissions{suffix}")(12, 4, values)
        options.temperature = 0.5 if scenario == 2 else 1
        options.duration_weight = 0.75
        options.state_radius = [0.25, 0.5, 1.75, 2.5][scenario] if fractional else 1 if scenario == 2 else 5
        options.duration_extra = 3
        options.duration_extra_factor = 0.5
        forward = model.forward_hsmm(segmentation, emissions, options)
        backward = model.backward_hsmm(segmentation, emissions, options)
        path = model.viterbi_hsmm(segmentation, emissions, options)
        scores = [forward.begin().values(), forward.end().values(), backward.begin().values(), backward.end().values()]
        cases[scenario] = scores, path.occurrences()
        return cases[scenario]

    if suffix == "F32":
        filename = "c-fractional-hsmm.txt" if fractional else "c-hsmm.txt"
    else:
        filename = "c-double-hsmm-fractional.txt" if fractional else "c-double-hsmm.txt"
    for line in (FIXTURES / filename).read_text().splitlines():
        fields = line.split()
        scores, occurrences = compute(int(fields[1]))
        index = int(fields[2])
        if fields[0] == "path":
            assert occurrences[index * 2] == int(fields[4])
            assert occurrences[index * 2 + 1] == int(fields[3])
        else:
            assert fields[0] == "score"
            for plane, field in zip(scores, fields[3:], strict=True):
                expected = scalar(float(field))
                tolerance = 0 if suffix == "F32" else 2e-14 * max(1, abs(expected))
                assert abs(plane[index] - expected) <= tolerance


def test_allocation_ownership_and_bounds():
    pool = api.MemoryPool(16)
    block = pool.allocate_zeroed(4, 2)
    assert pool.get(block) == bytes(8)
    pool.set_bytes(block, 2, b"abcd")
    assert pool.get_range(block, 2, 4) == b"abcd"
    with pytest.raises(ValueError):
        pool.set_bytes(block, 7, b"ab")
    assert pool.get(block) == b"\0\0abcd\0\0"
    pool.close()
    with pytest.raises(RuntimeError):
        pool.get(block)


def test_version():
    assert api.__version__.replace("-alpha.", "a") == version("liblrhsmm-rs")
    assert api.ModelF32.__module__ == "liblrhsmm_rs"



def test_installed_type_stub_matches_public_runtime_api():
    import ast
    stub = Path(api.__file__).with_name("__init__.pyi")
    assert stub.is_file()
    assert stub.with_name("py.typed").is_file()
    for definition in ast.parse(stub.read_text()).body:
        if isinstance(definition, (ast.ClassDef, ast.FunctionDef)):
            exported = getattr(api, definition.name)
            if isinstance(definition, ast.ClassDef):
                for member in definition.body:
                    if isinstance(member, ast.FunctionDef):
                        assert hasattr(exported, member.name), f"{definition.name}.{member.name}"
