#include "liblrhsmm_rs.h"
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <math.h>
#include <string.h>
typedef struct Draws { size_t index, fail; int32_t maximum; } Draws;

typedef struct RefF32 { int32_t maximum; float gaussian[6]; size_t choice[6]; int32_t boundaries[2][2]; float frames[2][4][3]; size_t draws[2]; } RefF32;
static const RefF32 references_f32[] = {
{2147483647,{-0.750000834,-2.61162353,-0.75,-0.15034315,-0.172228724,-0.75},{0,1,1,2,2,2},{{2,4},{1,2}},{{{1,-0.165086791,3.25838709},{-0.832543373,0.879256308,2.99999952},{2.46449208,2.5,2},{2,0.834913194,2.25838709}},{{1,1.5,3},{1,1.5,2},{0,0,0},{0,0,0}}},{36,20}},
};

static float draw_f32(Draws *source) {
    int32_t maximum = source->maximum, values[6] = {maximum/8,maximum/4,maximum/2,maximum/4*3,maximum/8*7,maximum};
    volatile float numerator = (float)values[source->index++ % 6], denominator = (float)maximum;
    return numerator / denominator;
}
static uint32_t callback_f32(void *context, float *output) {
    Draws *source = context;
    if(source->index == source->fail) { ++source->index; return 99; }
    *output = draw_f32(source); return 0;
}
static uint32_t word_f32(void *context, uint32_t *output) {
    Draws *source = context;
    if(source->index == source->fail) { ++source->index; return 77; }
    const uint32_t words[6] = {0, UINT32_MAX, UINT32_C(0x80000000), UINT32_C(0x12345678), 1, UINT32_MAX - 1};
    *output = words[source->index++ % 6]; return 0;
}
static uint32_t converted_f32(void *context, float *output) {
    uint32_t value; uint32_t status = word_f32(context, &value);
    if(status) return status;
    *output = (float)((value >> 8) + 1) * 0x1p-24f; return 0;
}
static void make_f32(int scenario, LiblrhsmmRsModelF32 **model, LiblrhsmmRsSegmentationF32 **sequence) {
    LiblrhsmmRsStreamF32 *streams[2]; const LiblrhsmmRsStreamF32 *borrowed_streams[2];
    for(size_t stream = 0; stream < 2; ++stream) {
        size_t components = stream == 0 ? 2 : 1, dimensions = stream == 0 ? 2 : 1;
        LiblrhsmmRsGaussianF32 *gaussians[2]; const LiblrhsmmRsGaussianF32 *borrowed[2];
        for(size_t state = 0; state < 2; ++state) {
            float weights[2] = {0.25,0.75}, means[4], variances[4], floors[4] = {0};
            if(stream) weights[0] = 1;
            for(size_t index = 0; index < components * dimensions; ++index) {
                means[index] = state + index * 0.5 + stream * 2;
                variances[index] = scenario == 0 ? 0.25 + index * 0.25 : 0;
            }
            _Alignas(8) LiblrhsmmRsGaussianParametersF32 parameters = {components,dimensions,weights,means,variances,floors};
            assert(liblrhsmm_rs_gaussian_create_f32(&parameters, &gaussians[state]) == 0); borrowed[state] = gaussians[state];
        }
        assert(liblrhsmm_rs_stream_create_f32(1, borrowed, 2, &streams[stream]) == 0); borrowed_streams[stream] = streams[stream];
        for(size_t state = 0; state < 2; ++state) assert(liblrhsmm_rs_gaussian_release_f32(&gaussians[state]) == 0);
    }
    LiblrhsmmRsDurationF32 *durations[2]; const LiblrhsmmRsDurationF32 *borrowed[2];
    for(size_t state = 0; state < 2; ++state) {
        _Alignas(8) LiblrhsmmRsDurationParametersF32 parameters; assert(liblrhsmm_rs_duration_default_f32(&parameters) == 0);
        parameters.mean = scenario == 0 ? 2.5 - state : 0; parameters.variance = scenario == 0 ? 0.75 - state * 0.5 : 0; parameters.minimum = 9;
        assert(liblrhsmm_rs_duration_create_f32(&parameters, &durations[state]) == 0); borrowed[state] = durations[state];
    }
    assert(liblrhsmm_rs_model_create_f32(borrowed_streams, 2, borrowed, 2, model) == 0);
    for(size_t index = 0; index < 2; ++index) { assert(liblrhsmm_rs_stream_release_f32(&streams[index]) == 0); assert(liblrhsmm_rs_duration_release_f32(&durations[index]) == 0); }
    int32_t boundaries[2] = {999,999}, duration_states[2] = {0,1}, labels[2][2] = {{0,1},{1,0}};
    const int32_t *rows[2] = {labels[0],labels[1]};
    _Alignas(8) LiblrhsmmRsJumpF32 jump = {1,1}; _Alignas(8) LiblrhsmmRsJumpSliceF32 jumps[2] = {{&jump,1},{&jump,1}};
    assert(liblrhsmm_rs_segmentation_create_f32(2,2,boundaries,duration_states,rows,jumps,sequence) == 0);
}
static size_t read_f32(LiblrhsmmRsObservationF32 *owner, float values[384]) {
    _Alignas(8) LiblrhsmmRsObservationLayout shape; assert(liblrhsmm_rs_observation_layout_f32(owner, &shape) == 0);
    assert(shape.streams == 2 && shape.frames <= 128);
    for(size_t stream = 0; stream < 2; ++stream) {
        LiblrhsmmRsObservationStreamF32 *child = NULL; _Alignas(8) LiblrhsmmRsObservationStreamLayout layout;
        assert(liblrhsmm_rs_observation_get_stream_f32(owner, stream, &child) == 0);
        assert(liblrhsmm_rs_observation_stream_layout_f32(child, &layout) == 0);
        size_t dimensions = stream == 0 ? 2 : 1; assert(layout.frames == shape.frames && layout.dimensions == dimensions);
        float input[256]; assert(liblrhsmm_rs_observation_stream_copy_values_f32(child, 0, input, layout.values) == 0);
        for(size_t frame = 0; frame < shape.frames; ++frame) for(size_t dimension = 0; dimension < dimensions; ++dimension) values[frame*3+(stream==0?dimension:2)] = input[frame*dimensions+dimension];
        assert(liblrhsmm_rs_observation_stream_release_f32(&child) == 0);
    }
    return shape.frames;
}
static void check_f32(float actual, float expected) { assert(memcmp(&actual,&expected,sizeof(actual)) == 0); }
static void exercise_f32(void) {
    for(size_t corpus = 0; corpus < 1; ++corpus) {
        const RefF32 *ref = &references_f32[corpus];
        for(size_t scenario = 0; scenario < 2; ++scenario) {
            LiblrhsmmRsModelF32 *model = NULL; LiblrhsmmRsSegmentationF32 *sequence = NULL;
            make_f32(scenario, &model, &sequence);
            Draws draws = {0,SIZE_MAX,ref->maximum}; _Alignas(8) LiblrhsmmRsUniformSourceF32 source = {&draws,callback_f32};
            LiblrhsmmRsObservationF32 *observation = NULL;
            assert(liblrhsmm_rs_model_generate_with_uniform_f32(model, sequence, &source, &observation) == 0);
            assert(draws.index == ref->draws[scenario]);
            int32_t boundaries[2]; assert(liblrhsmm_rs_segmentation_copy_boundaries_f32(sequence,0,boundaries,2) == 0);
            assert(memcmp(boundaries, ref->boundaries[scenario], sizeof(boundaries)) == 0);
            float values[384]; size_t frames = read_f32(observation, values); assert(frames == (size_t)ref->boundaries[scenario][1]);
            for(size_t frame = 0; frame < frames; ++frame) for(size_t dimension = 0; dimension < 3; ++dimension) check_f32(values[frame*3+dimension], ref->frames[scenario][frame][dimension]);
            if(scenario == 0) for(size_t index = 0; index < 6; ++index) {
                Draws draws = {index,SIZE_MAX,ref->maximum}; float radial = draw_f32(&draws), angular = draw_f32(&draws), result;
                assert(liblrhsmm_rs_gaussian_sample_f32(-0.75,1.25,radial,angular,&result) == 0); check_f32(result, ref->gaussian[index]);
                float weights[3] = {0.25,0.5,0.1}; size_t choice;
                assert(liblrhsmm_rs_choose_component_f32(weights,3,radial,&choice) == 0); assert(choice == ref->choice[index]);
            }
            assert(liblrhsmm_rs_model_release_f32(&model) == 0); assert(liblrhsmm_rs_segmentation_release_f32(&sequence) == 0);
            float retained[384]; assert(read_f32(observation, retained) == frames && memcmp(values, retained, frames*3*sizeof(float)) == 0);
            assert(liblrhsmm_rs_observation_release_f32(&observation) == 0);
        }
    }
    Draws words = {0,SIZE_MAX,0}; _Alignas(8) LiblrhsmmRsRandomSource random = {&words,word_f32,NULL}; float sampled;
    assert(liblrhsmm_rs_uniform_sample_f32(&random, &sampled) == 0 && sampled == 0x1p-24f);
    assert(liblrhsmm_rs_uniform_sample_f32(&random, &sampled) == 0 && sampled == 1);
    words.fail = words.index; sampled = 99;
    assert(liblrhsmm_rs_uniform_sample_f32(&random, &sampled) == 3 && sampled == 99);
    size_t calls = words.index; assert(liblrhsmm_rs_uniform_sample_f32(&random, NULL) == 1 && words.index == calls);
    LiblrhsmmRsModelF32 *model = NULL; LiblrhsmmRsSegmentationF32 *sequence = NULL, *copy = NULL;
    make_f32(0, &model, &sequence); assert(liblrhsmm_rs_segmentation_clone_f32(sequence, &copy) == 0);
    words.index = 0; words.fail = SIZE_MAX;
    Draws converted = {0,SIZE_MAX,0}; _Alignas(8) LiblrhsmmRsUniformSourceF32 scalar = {&converted,converted_f32};
    LiblrhsmmRsObservationF32 *raw = NULL, *uniform = NULL;
    assert(liblrhsmm_rs_model_generate_f32(model, sequence, &random, &raw) == 0);
    assert(liblrhsmm_rs_model_generate_with_uniform_f32(model, copy, &scalar, &uniform) == 0);
    assert(words.index == converted.index);
    float a[384], b[384]; size_t frames = read_f32(raw, a); assert(frames == read_f32(uniform, b)); assert(memcmp(a,b,frames*3*sizeof(float)) == 0);
    int32_t first[2], second[2]; assert(liblrhsmm_rs_segmentation_copy_boundaries_f32(sequence,0,first,2) == 0); assert(liblrhsmm_rs_segmentation_copy_boundaries_f32(copy,0,second,2) == 0); assert(memcmp(first,second,sizeof(first)) == 0);
    assert(liblrhsmm_rs_observation_release_f32(&raw) == 0); assert(liblrhsmm_rs_observation_release_f32(&uniform) == 0);
    const size_t failures[5] = {0,1,4,5,10};
    for(size_t index = 0; index < 5; ++index) {
        int32_t original[2] = {999,999}; assert(liblrhsmm_rs_segmentation_set_boundaries_f32(sequence,0,original,2) == 0);
        LiblrhsmmRsObservationF32 *sentinel = (LiblrhsmmRsObservationF32 *)(uintptr_t)0x1000;
        words.index = 0; words.fail = failures[index];
        assert(liblrhsmm_rs_model_generate_f32(model,sequence,&random,&sentinel) == 3); assert((uintptr_t)sentinel == 0x1000 && words.index == failures[index]+1);
        assert(liblrhsmm_rs_segmentation_copy_boundaries_f32(sequence,0,first,2) == 0 && memcmp(first,original,sizeof(first)) == 0);
        Draws bad = {0,failures[index],INT32_MAX}; _Alignas(8) LiblrhsmmRsUniformSourceF32 invalid = {&bad,callback_f32};
        assert(liblrhsmm_rs_model_generate_with_uniform_f32(model,sequence,&invalid,&sentinel) == 3); assert((uintptr_t)sentinel == 0x1000 && bad.index == failures[index]+1);
        assert(liblrhsmm_rs_segmentation_copy_boundaries_f32(sequence,0,first,2) == 0 && memcmp(first,original,sizeof(first)) == 0);
    }
    assert(liblrhsmm_rs_model_release_f32(&model) == 0); assert(liblrhsmm_rs_segmentation_release_f32(&sequence) == 0); assert(liblrhsmm_rs_segmentation_release_f32(&copy) == 0);
}

typedef struct RefF64 { int32_t maximum; _Alignas(8) double gaussian[6]; size_t choice[6]; int32_t boundaries[2][2]; _Alignas(8) double frames[2][4][3]; size_t draws[2]; } RefF64;
static const RefF64 references_f64[] = {
{32767,{-0.74967332375023821,-2.6116848817116134,-0.75056739531642613,-0.15095269475316742,-0.17176668734493616,-0.75},{0,0,1,1,2,2},{{3,4},{1,2}},{{{1,-0.16514157927632889,3.2585937987903226},{-0.83257078963816444,0.87887078215322534,3.0001460940602205},{1.4640200473622826,1.5,2.9997462531004713},{2,0.83485842072367111,2.2585937987903226}},{{1,1.5,3},{1,1.5,2},{0,0,0},{0,0,0}}},{36,20}},
{2147483647,{-0.75000085285620599,-2.6116236288498937,-0.75,-0.15034315363317063,-0.17222871724413669,-0.75},{0,0,1,1,2,2},{{2,4},{1,2}},{{{1,-0.16508679305128049,3.2583871727378724},{-0.83254339652564024,0.87925628980656934,2.9999996185911098},{2.4644921958835981,2.5,2},{2,0.83491320694871951,2.2583871727378724}},{{1,1.5,3},{1,1.5,2},{0,0,0},{0,0,0}}},{36,20}},
};

static double draw_f64(Draws *source) {
    int32_t maximum = source->maximum, values[6] = {maximum/8,maximum/4,maximum/2,maximum/4*3,maximum/8*7,maximum};
    volatile double numerator = (double)values[source->index++ % 6], denominator = (double)maximum;
    return numerator / denominator;
}
static uint32_t callback_f64(void *context, double *output) {
    Draws *source = context;
    if(source->index == source->fail) { ++source->index; return 99; }
    *output = draw_f64(source); return 0;
}
static uint32_t word_f64(void *context, uint64_t *output) {
    Draws *source = context;
    if(source->index == source->fail) { ++source->index; return 77; }
    const uint64_t words[6] = {0, UINT64_MAX, UINT64_C(0x8000000000000000), UINT64_C(0x123456789abcdef0), 1, UINT64_MAX - 1};
    *output = words[source->index++ % 6]; return 0;
}
static uint32_t converted_f64(void *context, double *output) {
    uint64_t value; uint32_t status = word_f64(context, &value);
    if(status) return status;
    *output = (double)((value >> 11) + 1) * 0x1p-53; return 0;
}
static void make_f64(int scenario, LiblrhsmmRsModelF64 **model, LiblrhsmmRsSegmentationF64 **sequence) {
    LiblrhsmmRsStreamF64 *streams[2]; const LiblrhsmmRsStreamF64 *borrowed_streams[2];
    for(size_t stream = 0; stream < 2; ++stream) {
        size_t components = stream == 0 ? 2 : 1, dimensions = stream == 0 ? 2 : 1;
        LiblrhsmmRsGaussianF64 *gaussians[2]; const LiblrhsmmRsGaussianF64 *borrowed[2];
        for(size_t state = 0; state < 2; ++state) {
            _Alignas(8) double weights[2] = {0.25,0.75}, means[4], variances[4], floors[4] = {0};
            if(stream) weights[0] = 1;
            for(size_t index = 0; index < components * dimensions; ++index) {
                means[index] = state + index * 0.5 + stream * 2;
                variances[index] = scenario == 0 ? 0.25 + index * 0.25 : 0;
            }
            _Alignas(8) LiblrhsmmRsGaussianParametersF64 parameters = {components,dimensions,weights,means,variances,floors};
            assert(liblrhsmm_rs_gaussian_create_f64(&parameters, &gaussians[state]) == 0); borrowed[state] = gaussians[state];
        }
        assert(liblrhsmm_rs_stream_create_f64(1, borrowed, 2, &streams[stream]) == 0); borrowed_streams[stream] = streams[stream];
        for(size_t state = 0; state < 2; ++state) assert(liblrhsmm_rs_gaussian_release_f64(&gaussians[state]) == 0);
    }
    LiblrhsmmRsDurationF64 *durations[2]; const LiblrhsmmRsDurationF64 *borrowed[2];
    for(size_t state = 0; state < 2; ++state) {
        _Alignas(8) LiblrhsmmRsDurationParametersF64 parameters; assert(liblrhsmm_rs_duration_default_f64(&parameters) == 0);
        parameters.mean = scenario == 0 ? 2.5 - state : 0; parameters.variance = scenario == 0 ? 0.75 - state * 0.5 : 0; parameters.minimum = 9;
        assert(liblrhsmm_rs_duration_create_f64(&parameters, &durations[state]) == 0); borrowed[state] = durations[state];
    }
    assert(liblrhsmm_rs_model_create_f64(borrowed_streams, 2, borrowed, 2, model) == 0);
    for(size_t index = 0; index < 2; ++index) { assert(liblrhsmm_rs_stream_release_f64(&streams[index]) == 0); assert(liblrhsmm_rs_duration_release_f64(&durations[index]) == 0); }
    int32_t boundaries[2] = {999,999}, duration_states[2] = {0,1}, labels[2][2] = {{0,1},{1,0}};
    const int32_t *rows[2] = {labels[0],labels[1]};
    _Alignas(8) LiblrhsmmRsJumpF64 jump = {1,1}; _Alignas(8) LiblrhsmmRsJumpSliceF64 jumps[2] = {{&jump,1},{&jump,1}};
    assert(liblrhsmm_rs_segmentation_create_f64(2,2,boundaries,duration_states,rows,jumps,sequence) == 0);
}
static size_t read_f64(LiblrhsmmRsObservationF64 *owner, double values[384]) {
    _Alignas(8) LiblrhsmmRsObservationLayout shape; assert(liblrhsmm_rs_observation_layout_f64(owner, &shape) == 0);
    assert(shape.streams == 2 && shape.frames <= 128);
    for(size_t stream = 0; stream < 2; ++stream) {
        LiblrhsmmRsObservationStreamF64 *child = NULL; _Alignas(8) LiblrhsmmRsObservationStreamLayout layout;
        assert(liblrhsmm_rs_observation_get_stream_f64(owner, stream, &child) == 0);
        assert(liblrhsmm_rs_observation_stream_layout_f64(child, &layout) == 0);
        size_t dimensions = stream == 0 ? 2 : 1; assert(layout.frames == shape.frames && layout.dimensions == dimensions);
        _Alignas(8) double input[256]; assert(liblrhsmm_rs_observation_stream_copy_values_f64(child, 0, input, layout.values) == 0);
        for(size_t frame = 0; frame < shape.frames; ++frame) for(size_t dimension = 0; dimension < dimensions; ++dimension) values[frame*3+(stream==0?dimension:2)] = input[frame*dimensions+dimension];
        assert(liblrhsmm_rs_observation_stream_release_f64(&child) == 0);
    }
    return shape.frames;
}
static void check_f64(double actual, double expected) { assert(isfinite(actual) && fabs(actual-expected)/fmax(1.0,fabs(expected)) <= 2e-14); }
static void exercise_f64(void) {
    for(size_t corpus = 0; corpus < 2; ++corpus) {
        const RefF64 *ref = &references_f64[corpus];
        for(size_t scenario = 0; scenario < 2; ++scenario) {
            LiblrhsmmRsModelF64 *model = NULL; LiblrhsmmRsSegmentationF64 *sequence = NULL;
            make_f64(scenario, &model, &sequence);
            Draws draws = {0,SIZE_MAX,ref->maximum}; _Alignas(8) LiblrhsmmRsUniformSourceF64 source = {&draws,callback_f64};
            LiblrhsmmRsObservationF64 *observation = NULL;
            assert(liblrhsmm_rs_model_generate_with_uniform_f64(model, sequence, &source, &observation) == 0);
            assert(draws.index == ref->draws[scenario]);
            int32_t boundaries[2]; assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(sequence,0,boundaries,2) == 0);
            assert(memcmp(boundaries, ref->boundaries[scenario], sizeof(boundaries)) == 0);
            _Alignas(8) double values[384]; size_t frames = read_f64(observation, values); assert(frames == (size_t)ref->boundaries[scenario][1]);
            for(size_t frame = 0; frame < frames; ++frame) for(size_t dimension = 0; dimension < 3; ++dimension) check_f64(values[frame*3+dimension], ref->frames[scenario][frame][dimension]);
            if(scenario == 0) for(size_t index = 0; index < 6; ++index) {
                Draws draws = {index,SIZE_MAX,ref->maximum}; _Alignas(8) double radial = draw_f64(&draws), angular = draw_f64(&draws), result;
                assert(liblrhsmm_rs_gaussian_sample_f64(-0.75,1.25,radial,angular,&result) == 0); check_f64(result, ref->gaussian[index]);
                _Alignas(8) double weights[3] = {0.25,0.5,0.1}; size_t choice;
                assert(liblrhsmm_rs_choose_component_f64(weights,3,radial,&choice) == 0); assert(choice == ref->choice[index]);
            }
            assert(liblrhsmm_rs_model_release_f64(&model) == 0); assert(liblrhsmm_rs_segmentation_release_f64(&sequence) == 0);
            _Alignas(8) double retained[384]; assert(read_f64(observation, retained) == frames && memcmp(values, retained, frames*3*sizeof(double)) == 0);
            assert(liblrhsmm_rs_observation_release_f64(&observation) == 0);
        }
    }
    Draws words = {0,SIZE_MAX,0}; _Alignas(8) LiblrhsmmRsRandomSource random = {&words,NULL,word_f64}; _Alignas(8) double sampled;
    assert(liblrhsmm_rs_uniform_sample_f64(&random, &sampled) == 0 && sampled == 0x1p-53);
    assert(liblrhsmm_rs_uniform_sample_f64(&random, &sampled) == 0 && sampled == 1);
    words.fail = words.index; sampled = 99;
    assert(liblrhsmm_rs_uniform_sample_f64(&random, &sampled) == 3 && sampled == 99);
    size_t calls = words.index; assert(liblrhsmm_rs_uniform_sample_f64(&random, NULL) == 1 && words.index == calls);
    LiblrhsmmRsModelF64 *model = NULL; LiblrhsmmRsSegmentationF64 *sequence = NULL, *copy = NULL;
    make_f64(0, &model, &sequence); assert(liblrhsmm_rs_segmentation_clone_f64(sequence, &copy) == 0);
    words.index = 0; words.fail = SIZE_MAX;
    Draws converted = {0,SIZE_MAX,0}; _Alignas(8) LiblrhsmmRsUniformSourceF64 scalar = {&converted,converted_f64};
    LiblrhsmmRsObservationF64 *raw = NULL, *uniform = NULL;
    assert(liblrhsmm_rs_model_generate_f64(model, sequence, &random, &raw) == 0);
    assert(liblrhsmm_rs_model_generate_with_uniform_f64(model, copy, &scalar, &uniform) == 0);
    assert(words.index == converted.index);
    _Alignas(8) double a[384], b[384]; size_t frames = read_f64(raw, a); assert(frames == read_f64(uniform, b)); assert(memcmp(a,b,frames*3*sizeof(double)) == 0);
    int32_t first[2], second[2]; assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(sequence,0,first,2) == 0); assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(copy,0,second,2) == 0); assert(memcmp(first,second,sizeof(first)) == 0);
    assert(liblrhsmm_rs_observation_release_f64(&raw) == 0); assert(liblrhsmm_rs_observation_release_f64(&uniform) == 0);
    const size_t failures[5] = {0,1,4,5,10};
    for(size_t index = 0; index < 5; ++index) {
        int32_t original[2] = {999,999}; assert(liblrhsmm_rs_segmentation_set_boundaries_f64(sequence,0,original,2) == 0);
        LiblrhsmmRsObservationF64 *sentinel = (LiblrhsmmRsObservationF64 *)(uintptr_t)0x1000;
        words.index = 0; words.fail = failures[index];
        assert(liblrhsmm_rs_model_generate_f64(model,sequence,&random,&sentinel) == 3); assert((uintptr_t)sentinel == 0x1000 && words.index == failures[index]+1);
        assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(sequence,0,first,2) == 0 && memcmp(first,original,sizeof(first)) == 0);
        Draws bad = {0,failures[index],INT32_MAX}; _Alignas(8) LiblrhsmmRsUniformSourceF64 invalid = {&bad,callback_f64};
        assert(liblrhsmm_rs_model_generate_with_uniform_f64(model,sequence,&invalid,&sentinel) == 3); assert((uintptr_t)sentinel == 0x1000 && bad.index == failures[index]+1);
        assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(sequence,0,first,2) == 0 && memcmp(first,original,sizeof(first)) == 0);
    }
    assert(liblrhsmm_rs_model_release_f64(&model) == 0); assert(liblrhsmm_rs_segmentation_release_f64(&sequence) == 0); assert(liblrhsmm_rs_segmentation_release_f64(&copy) == 0);
}

int main(void) { exercise_f32(); exercise_f64(); puts("Generation C: all10 exports, original C corpora and synchronous callback transactions passed"); return 0; }
