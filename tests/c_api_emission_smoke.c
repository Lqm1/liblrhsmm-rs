#include "liblrhsmm_rs.h"
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <string.h>


static const float reference_f32[6][36]={{-2.08748507f,-1.71865296f,-9.99999987e+14f,-1.94407988f,-1.60787785f,-9.99999987e+14f,-1.83068848f,-1.52842724f,-9.99999987e+14f,-1.74680746f,-1.48006713f,-9.99999987e+14f,-1.69196475f,-1.46248507f,-1.71865296f,-1.6654948f,-1.47533f,-1.76412773f,-1.6668241f,-1.51818848f,-1.84092724f,-1.69539106f,-1.59055758f,-1.94881701f,-1.75057507f,-1.69196475f,-2.08748507f,-9.99999987e+14f,-1.82174468f,-2.25657988f,-9.99999987e+14f,-1.9793241f,-2.45568848f,-9.99999987e+14f,-2.16414094f,-2.68430758f},{-2.08748507f,-9.99999987e+14f,-9.99999987e+14f,-1.94407988f,-9.99999987e+14f,-9.99999987e+14f,-1.83068848f,-9.99999987e+14f,-9.99999987e+14f,-1.74680746f,-9.99999987e+14f,-9.99999987e+14f,-1.69196475f,-1.46248507f,-9.99999987e+14f,-1.6654948f,-1.47533f,-9.99999987e+14f,-1.6668241f,-1.51818848f,-9.99999987e+14f,-1.69539106f,-1.59055758f,-9.99999987e+14f,-9.99999987e+14f,-1.69196475f,-2.08748507f,-9.99999987e+14f,-1.82174468f,-2.25657988f,-9.99999987e+14f,-1.9793241f,-2.45568848f,-9.99999987e+14f,-2.16414094f,-2.68430758f},{-2.08748507f,-9.99999987e+14f,-9.99999987e+14f,-1.94407988f,-9.99999987e+14f,-9.99999987e+14f,-1.83068848f,-9.99999987e+14f,-9.99999987e+14f,-1.74680746f,-9.99999987e+14f,-9.99999987e+14f,-1.69196475f,-1.46248507f,-9.99999987e+14f,-1.6654948f,-1.47533f,-9.99999987e+14f,-1.6668241f,-1.51818848f,-9.99999987e+14f,-1.69539106f,-1.59055758f,-9.99999987e+14f,-1.75057507f,-1.69196475f,-2.08748507f,-9.99999987e+14f,-1.82174468f,-2.25657988f,-9.99999987e+14f,-1.9793241f,-2.45568848f,-9.99999987e+14f,-2.16414094f,-2.68430758f},{-2.08748507f,-9.99999987e+14f,-9.99999987e+14f,-1.94407988f,-9.99999987e+14f,-9.99999987e+14f,-1.83068848f,-9.99999987e+14f,-9.99999987e+14f,-1.74680746f,-9.99999987e+14f,-9.99999987e+14f,-1.69196475f,-1.46248507f,-9.99999987e+14f,-1.6654948f,-1.47533f,-9.99999987e+14f,-1.6668241f,-1.51818848f,-9.99999987e+14f,-1.69539106f,-1.59055758f,-9.99999987e+14f,-1.75057507f,-1.69196475f,-2.08748507f,-9.99999987e+14f,-1.82174468f,-2.25657988f,-9.99999987e+14f,-1.9793241f,-2.45568848f,-9.99999987e+14f,-2.16414094f,-2.68430758f},{-2.08748507f,-1.71865296f,-9.99999987e+14f,-1.94407988f,-1.60787785f,-9.99999987e+14f,-1.83068848f,-1.52842724f,-9.99999987e+14f,-1.74680746f,-1.48006713f,-9.99999987e+14f,-1.69196475f,-1.46248507f,-1.71865296f,-1.6654948f,-1.47533f,-1.76412773f,-1.6668241f,-1.51818848f,-1.84092724f,-1.69539106f,-1.59055758f,-1.94881701f,-1.75057507f,-1.69196475f,-2.08748507f,-1.83191288f,-1.82174468f,-2.25657988f,-1.93896246f,-1.9793241f,-2.45568848f,-2.07157469f,-2.16414094f,-2.68430758f},{-2.08748507f,-1.71865296f,-1.85274303f,-1.94407988f,-1.60787785f,-1.77161717f,-1.83068848f,-1.52842724f,-1.72227931f,-1.74680746f,-1.48006713f,-1.70463657f,-1.69196475f,-1.46248507f,-1.71865296f,-1.6654948f,-1.47533f,-1.76412773f,-1.6668241f,-1.51818848f,-1.84092724f,-1.69539106f,-1.59055758f,-1.94881701f,-1.75057507f,-1.69196475f,-2.08748507f,-1.83191288f,-1.82174468f,-2.25657988f,-1.93896246f,-1.9793241f,-2.45568848f,-2.07157469f,-2.16414094f,-2.68430758f}};
static const float density_reference_f32[36]={0.123998642f,0.179307938f,1.17551775e-38f,0.143112183f,0.2003088f,1.17551775e-38f,0.160301208f,0.216868401f,1.17551775e-38f,0.174330711f,0.227613449f,1.17551775e-38f,0.184158325f,0.23165226f,0.179307938f,0.189097404f,0.228694916f,0.171336174f,0.188845634f,0.219099998f,0.158668518f,0.1835289f,0.203807831f,0.142436028f,0.173674583f,0.184158325f,0.123998642f,1.17551775e-38f,0.16174221f,0.104705334f,1.17551775e-38f,0.138155937f,0.0858044624f,1.17551775e-38f,0.114844322f,0.0682649612f};

static void read_log_f32(LiblrhsmmRsLogEmissionsF32 *owner, float values[36]) {
    _Alignas(8) LiblrhsmmRsInferenceTableLayout layout;
    assert(liblrhsmm_rs_log_emissions_layout_f32(owner,&layout)==0);
    assert(layout.frames==12 && layout.states==3 && layout.values==36);
    assert(liblrhsmm_rs_log_emissions_copy_values_f32(owner,0,values,36)==0);
}
static void read_density_f32(LiblrhsmmRsEmissionDensitiesF32 *owner, float values[36]) {
    _Alignas(8) LiblrhsmmRsInferenceTableLayout layout;
    assert(liblrhsmm_rs_emission_densities_layout_f32(owner,&layout)==0);
    assert(layout.frames==12 && layout.states==3 && layout.values==36);
    assert(liblrhsmm_rs_emission_densities_copy_values_f32(owner,0,values,36)==0);
}
static void exercise_f32(void) {
    LiblrhsmmRsStreamF32 *streams[2]={0}; const LiblrhsmmRsStreamF32 *stream_inputs[2];
    LiblrhsmmRsObservationStreamF32 *features[2]={0}; const LiblrhsmmRsObservationStreamF32 *feature_inputs[2];
    for(size_t stream=0;stream<2;++stream) {
        size_t components=stream==0?2:1, dimensions=stream==0?2:1, count=components*dimensions;
        LiblrhsmmRsGaussianF32 *emissions[3]={0}; const LiblrhsmmRsGaussianF32 *emission_inputs[3];
        for(size_t state=0;state<3;++state) {
            float weights[2]={0.5,0.5}, means[4], variances[4], floors[4]={0};
            if(stream) weights[0]=1;
            for(size_t index=0;index<count;++index) { means[index]=state*0.5+index*0.25; variances[index]=0.75+index*0.25; }
            _Alignas(8) LiblrhsmmRsGaussianParametersF32 parameters={components,dimensions,weights,means,variances,floors};
            assert(liblrhsmm_rs_gaussian_create_f32(&parameters,&emissions[state])==0); emission_inputs[state]=emissions[state];
        }
        assert(liblrhsmm_rs_stream_create_f32(stream==0?0.5:1.25,emission_inputs,3,&streams[stream])==0); stream_inputs[stream]=streams[stream];
        for(size_t state=0;state<3;++state) assert(liblrhsmm_rs_gaussian_release_f32(&emissions[state])==0);
        float samples[24];
        for(size_t frame=0;frame<12;++frame) for(size_t dimension=0;dimension<dimensions;++dimension) samples[frame*dimensions+dimension]=frame*0.125+dimension*0.5;
        assert(liblrhsmm_rs_observation_stream_create_f32(dimensions,samples,12*dimensions,&features[stream])==0); feature_inputs[stream]=features[stream];
    }
    LiblrhsmmRsDurationF32 *durations[3]={0}; const LiblrhsmmRsDurationF32 *duration_inputs[3];
    for(size_t state=0;state<3;++state) {
        _Alignas(8) LiblrhsmmRsDurationParametersF32 parameters;
        assert(liblrhsmm_rs_duration_default_f32(&parameters)==0); parameters.mean=4;parameters.variance=1;
        assert(liblrhsmm_rs_duration_create_f32(&parameters,&durations[state])==0); duration_inputs[state]=durations[state];
    }
    LiblrhsmmRsModelF32 *model=NULL; LiblrhsmmRsObservationF32 *observation=NULL;
    assert(liblrhsmm_rs_model_create_f32(stream_inputs,2,duration_inputs,3,&model)==0);
    assert(liblrhsmm_rs_observation_create_f32(12,feature_inputs,2,&observation)==0);
    for(size_t state=0;state<3;++state) assert(liblrhsmm_rs_duration_release_f32(&durations[state])==0);
    for(size_t stream=0;stream<2;++stream) { assert(liblrhsmm_rs_stream_release_f32(&streams[stream])==0);assert(liblrhsmm_rs_observation_stream_release_f32(&features[stream])==0); }
    int32_t boundaries[3]={4,8,12},durations_labels[3]={0,1,2}, labels[2][3]={{0,1,2},{2,1,0}};
    const int32_t *output_states[2]={labels[0],labels[1]};
    _Alignas(8) LiblrhsmmRsJumpF32 normal={1,1}; _Alignas(8) LiblrhsmmRsJumpSliceF32 jumps[3]={{&normal,1},{&normal,1},{&normal,1}};
    LiblrhsmmRsSegmentationF32 *segmentation=NULL;
    assert(liblrhsmm_rs_segmentation_create_f32(2,3,boundaries,durations_labels,output_states,jumps,&segmentation)==0);
    _Alignas(8) LiblrhsmmRsDurationPreparationF32 preparation={2,1,1}; LiblrhsmmRsPreparedModelF32 *prepared=NULL;
    assert(liblrhsmm_rs_model_prepare_f32(model,&preparation,&prepared)==0);
    _Alignas(8) LiblrhsmmRsEmissionSettingsF32 defaults;
    assert(liblrhsmm_rs_emission_settings_default_f32(&defaults)==0);
    assert(defaults.temperature==1 && defaults.pruning==LIBLRHSMM_RS_EMISSION_PRUNING_NONE && defaults.radius==5);
    size_t comparisons=0;
    _Alignas(8) LiblrhsmmRsEmissionSettingsF32 options=defaults;
    for(size_t mode=0;mode<7;++mode) {
        options.temperature=0.75;
        options.pruning=mode==6?LIBLRHSMM_RS_EMISSION_PRUNING_NONE:(mode==1?LIBLRHSMM_RS_EMISSION_PRUNING_LINEAR:LIBLRHSMM_RS_EMISSION_PRUNING_BOUNDARIES);
        _Alignas(8) const double radii[6]={1,1,0.25,0.5,1.75,2.5}; options.radius=mode==6?NAN:radii[mode]; options.slope=mode==1?0.34:NAN;
        LiblrhsmmRsLogEmissionsF32 *log=NULL,*cached=NULL; LiblrhsmmRsEmissionDensitiesF32 *density=NULL,*cached_density=NULL;
        assert(liblrhsmm_rs_model_emission_log_probabilities_f32(model,observation,segmentation,&options,&log)==0);
        assert(liblrhsmm_rs_prepared_model_emission_log_probabilities_f32(prepared,observation,segmentation,&options,&cached)==0);
        assert(liblrhsmm_rs_model_emission_probabilities_f32(model,observation,segmentation,&options,&density)==0);
        assert(liblrhsmm_rs_prepared_model_emission_probabilities_f32(prepared,observation,segmentation,&options,&cached_density)==0);
        float raw[36], cached_values[36], raw_density[36], cached_density_values[36];
        read_log_f32(log,raw); read_log_f32(cached,cached_values); read_density_f32(density,raw_density); read_density_f32(cached_density,cached_density_values);
        assert(memcmp(raw,cached_values,sizeof(raw))==0);assert(memcmp(raw_density,cached_density_values,sizeof(raw_density))==0);
        if(mode<6) for(size_t index=0;index<36;++index) { assert(memcmp(&raw[index],&reference_f32[mode][index],sizeof(raw[index]))==0); ++comparisons; }
        if(mode==0) assert(memcmp(raw_density,density_reference_f32,sizeof(raw_density))==0);
        assert(liblrhsmm_rs_log_emissions_release_f32(&log)==0); assert(liblrhsmm_rs_log_emissions_release_f32(&cached)==0); assert(liblrhsmm_rs_emission_densities_release_f32(&density)==0); assert(liblrhsmm_rs_emission_densities_release_f32(&cached_density)==0);
    }
    assert(comparisons==216);
    LiblrhsmmRsLogEmissionsF32 *sentinel=(LiblrhsmmRsLogEmissionsF32 *)(uintptr_t)0x1000;
    options.pruning=99;
    assert(liblrhsmm_rs_model_emission_log_probabilities_f32(model,observation,segmentation,&options,&sentinel)==3);assert((uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_prepared_model_emission_log_probabilities_f32(prepared,observation,segmentation,&options,&sentinel)==3);assert((uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_model_emission_log_probabilities_f32(NULL,observation,segmentation,&defaults,&sentinel)==1);
    assert(liblrhsmm_rs_emission_settings_default_f32(NULL)==1);
    assert(liblrhsmm_rs_model_release_f32(&model)==0);
    LiblrhsmmRsLogEmissionsF32 *retained=NULL; LiblrhsmmRsEmissionDensitiesF32 *retained_density=NULL;
    assert(liblrhsmm_rs_prepared_model_emission_log_probabilities_f32(prepared,observation,segmentation,&defaults,&retained)==0);
    assert(liblrhsmm_rs_prepared_model_emission_probabilities_f32(prepared,observation,segmentation,&defaults,&retained_density)==0);
    assert(liblrhsmm_rs_prepared_model_release_f32(&prepared)==0); assert(liblrhsmm_rs_observation_release_f32(&observation)==0); assert(liblrhsmm_rs_segmentation_release_f32(&segmentation)==0);
    float copied[36]; read_log_f32(retained,copied);read_density_f32(retained_density,copied);
    assert(liblrhsmm_rs_log_emissions_release_f32(&retained)==0);assert(liblrhsmm_rs_emission_densities_release_f32(&retained_density)==0);
}

_Alignas(8) static const double reference_f64[6][36]={{-2.0874850001859877,-1.7186546620655272,-1000000000000000.0,-1.9440799799490185,-1.6078778502273772,-1000000000000000.0,-1.8306885060596678,-1.5284272131252501,-1000000000000000.0,-1.7468074855137083,-1.4800670739937041,-1000000000000000.0,-1.6919647154140685,-1.4624850001859877,-1.7186546620655272,-1.6654947397041533,-1.4753299799490187,-1.7641278502273772,-1.6668240126896117,-1.5181885060596678,-1.8409272131252501,-1.6953910586166594,-1.5905574855137083,-1.9488170739937039,-1.750575035505316,-1.6919647154140685,-2.0874850001859877,-1000000000000000.0,-1.8217447397041533,-2.2565799799490187,-1000000000000000.0,-1.9793240126896117,-2.4556885060596678,-1000000000000000.0,-2.164141058616659,-2.6843074855137088},{-2.0874850001859877,-1000000000000000.0,-1000000000000000.0,-1.9440799799490185,-1000000000000000.0,-1000000000000000.0,-1.8306885060596678,-1000000000000000.0,-1000000000000000.0,-1.7468074855137083,-1000000000000000.0,-1000000000000000.0,-1.6919647154140685,-1.4624850001859877,-1000000000000000.0,-1.6654947397041533,-1.4753299799490187,-1000000000000000.0,-1.6668240126896117,-1.5181885060596678,-1000000000000000.0,-1.6953910586166594,-1.5905574855137083,-1000000000000000.0,-1000000000000000.0,-1.6919647154140685,-2.0874850001859877,-1000000000000000.0,-1.8217447397041533,-2.2565799799490187,-1000000000000000.0,-1.9793240126896117,-2.4556885060596678,-1000000000000000.0,-2.164141058616659,-2.6843074855137088},{-2.0874850001859877,-1000000000000000.0,-1000000000000000.0,-1.9440799799490185,-1000000000000000.0,-1000000000000000.0,-1.8306885060596678,-1000000000000000.0,-1000000000000000.0,-1.7468074855137083,-1000000000000000.0,-1000000000000000.0,-1.6919647154140685,-1.4624850001859877,-1000000000000000.0,-1.6654947397041533,-1.4753299799490187,-1000000000000000.0,-1.6668240126896117,-1.5181885060596678,-1000000000000000.0,-1.6953910586166594,-1.5905574855137083,-1000000000000000.0,-1.750575035505316,-1.6919647154140685,-2.0874850001859877,-1000000000000000.0,-1.8217447397041533,-2.2565799799490187,-1000000000000000.0,-1.9793240126896117,-2.4556885060596678,-1000000000000000.0,-2.164141058616659,-2.6843074855137088},{-2.0874850001859877,-1000000000000000.0,-1000000000000000.0,-1.9440799799490185,-1000000000000000.0,-1000000000000000.0,-1.8306885060596678,-1000000000000000.0,-1000000000000000.0,-1.7468074855137083,-1000000000000000.0,-1000000000000000.0,-1.6919647154140685,-1.4624850001859877,-1000000000000000.0,-1.6654947397041533,-1.4753299799490187,-1000000000000000.0,-1.6668240126896117,-1.5181885060596678,-1000000000000000.0,-1.6953910586166594,-1.5905574855137083,-1000000000000000.0,-1.750575035505316,-1.6919647154140685,-2.0874850001859877,-1000000000000000.0,-1.8217447397041533,-2.2565799799490187,-1000000000000000.0,-1.9793240126896117,-2.4556885060596678,-1000000000000000.0,-2.164141058616659,-2.6843074855137088},{-2.0874850001859877,-1.7186546620655272,-1000000000000000.0,-1.9440799799490185,-1.6078778502273772,-1000000000000000.0,-1.8306885060596678,-1.5284272131252501,-1000000000000000.0,-1.7468074855137083,-1.4800670739937041,-1000000000000000.0,-1.6919647154140685,-1.4624850001859877,-1.7186546620655272,-1.6654947397041533,-1.4753299799490187,-1.7641278502273772,-1.6668240126896117,-1.5181885060596678,-1.8409272131252501,-1.6953910586166594,-1.5905574855137083,-1.9488170739937039,-1.750575035505316,-1.6919647154140685,-2.0874850001859877,-1.8319129046726439,-1.8217447397041533,-2.2565799799490187,-1.9389624592590544,-1.9793240126896117,-2.4556885060596678,-2.0715744790363524,-2.164141058616659,-2.6843074855137088},{-2.0874850001859877,-1.7186546620655272,-1.852742969684622,-1.9440799799490185,-1.6078778502273772,-1.7716171142387602,-1.8306885060596678,-1.5284272131252501,-1.7222792205143187,-1.7468074855137083,-1.4800670739937041,-1.7046365734863493,-1.6919647154140685,-1.4624850001859877,-1.7186546620655272,-1.6654947397041533,-1.4753299799490187,-1.7641278502273772,-1.6668240126896117,-1.5181885060596678,-1.8409272131252501,-1.6953910586166594,-1.5905574855137083,-1.9488170739937039,-1.750575035505316,-1.6919647154140685,-2.0874850001859877,-1.8319129046726439,-1.8217447397041533,-2.2565799799490187,-1.9389624592590544,-1.9793240126896117,-2.4556885060596678,-2.0715744790363524,-2.164141058616659,-2.6843074855137088}};

static void read_log_f64(LiblrhsmmRsLogEmissionsF64 *owner, double values[36]) {
    _Alignas(8) LiblrhsmmRsInferenceTableLayout layout;
    assert(liblrhsmm_rs_log_emissions_layout_f64(owner,&layout)==0);
    assert(layout.frames==12 && layout.states==3 && layout.values==36);
    assert(liblrhsmm_rs_log_emissions_copy_values_f64(owner,0,values,36)==0);
}
static void read_density_f64(LiblrhsmmRsEmissionDensitiesF64 *owner, double values[36]) {
    _Alignas(8) LiblrhsmmRsInferenceTableLayout layout;
    assert(liblrhsmm_rs_emission_densities_layout_f64(owner,&layout)==0);
    assert(layout.frames==12 && layout.states==3 && layout.values==36);
    assert(liblrhsmm_rs_emission_densities_copy_values_f64(owner,0,values,36)==0);
}
static void exercise_f64(void) {
    LiblrhsmmRsStreamF64 *streams[2]={0}; const LiblrhsmmRsStreamF64 *stream_inputs[2];
    LiblrhsmmRsObservationStreamF64 *features[2]={0}; const LiblrhsmmRsObservationStreamF64 *feature_inputs[2];
    for(size_t stream=0;stream<2;++stream) {
        size_t components=stream==0?2:1, dimensions=stream==0?2:1, count=components*dimensions;
        LiblrhsmmRsGaussianF64 *emissions[3]={0}; const LiblrhsmmRsGaussianF64 *emission_inputs[3];
        for(size_t state=0;state<3;++state) {
            _Alignas(8) double weights[2]={0.5,0.5}, means[4], variances[4], floors[4]={0};
            if(stream) weights[0]=1;
            for(size_t index=0;index<count;++index) { means[index]=state*0.5+index*0.25; variances[index]=0.75+index*0.25; }
            _Alignas(8) LiblrhsmmRsGaussianParametersF64 parameters={components,dimensions,weights,means,variances,floors};
            assert(liblrhsmm_rs_gaussian_create_f64(&parameters,&emissions[state])==0); emission_inputs[state]=emissions[state];
        }
        assert(liblrhsmm_rs_stream_create_f64(stream==0?0.5:1.25,emission_inputs,3,&streams[stream])==0); stream_inputs[stream]=streams[stream];
        for(size_t state=0;state<3;++state) assert(liblrhsmm_rs_gaussian_release_f64(&emissions[state])==0);
        _Alignas(8) double samples[24];
        for(size_t frame=0;frame<12;++frame) for(size_t dimension=0;dimension<dimensions;++dimension) samples[frame*dimensions+dimension]=frame*0.125+dimension*0.5;
        assert(liblrhsmm_rs_observation_stream_create_f64(dimensions,samples,12*dimensions,&features[stream])==0); feature_inputs[stream]=features[stream];
    }
    LiblrhsmmRsDurationF64 *durations[3]={0}; const LiblrhsmmRsDurationF64 *duration_inputs[3];
    for(size_t state=0;state<3;++state) {
        _Alignas(8) LiblrhsmmRsDurationParametersF64 parameters;
        assert(liblrhsmm_rs_duration_default_f64(&parameters)==0); parameters.mean=4;parameters.variance=1;
        assert(liblrhsmm_rs_duration_create_f64(&parameters,&durations[state])==0); duration_inputs[state]=durations[state];
    }
    LiblrhsmmRsModelF64 *model=NULL; LiblrhsmmRsObservationF64 *observation=NULL;
    assert(liblrhsmm_rs_model_create_f64(stream_inputs,2,duration_inputs,3,&model)==0);
    assert(liblrhsmm_rs_observation_create_f64(12,feature_inputs,2,&observation)==0);
    for(size_t state=0;state<3;++state) assert(liblrhsmm_rs_duration_release_f64(&durations[state])==0);
    for(size_t stream=0;stream<2;++stream) { assert(liblrhsmm_rs_stream_release_f64(&streams[stream])==0);assert(liblrhsmm_rs_observation_stream_release_f64(&features[stream])==0); }
    int32_t boundaries[3]={4,8,12},durations_labels[3]={0,1,2}, labels[2][3]={{0,1,2},{2,1,0}};
    const int32_t *output_states[2]={labels[0],labels[1]};
    _Alignas(8) LiblrhsmmRsJumpF64 normal={1,1}; _Alignas(8) LiblrhsmmRsJumpSliceF64 jumps[3]={{&normal,1},{&normal,1},{&normal,1}};
    LiblrhsmmRsSegmentationF64 *segmentation=NULL;
    assert(liblrhsmm_rs_segmentation_create_f64(2,3,boundaries,durations_labels,output_states,jumps,&segmentation)==0);
    _Alignas(8) LiblrhsmmRsDurationPreparationF64 preparation={2,1,1}; LiblrhsmmRsPreparedModelF64 *prepared=NULL;
    assert(liblrhsmm_rs_model_prepare_f64(model,&preparation,&prepared)==0);
    _Alignas(8) LiblrhsmmRsEmissionSettingsF64 defaults;
    assert(liblrhsmm_rs_emission_settings_default_f64(&defaults)==0);
    assert(defaults.temperature==1 && defaults.pruning==LIBLRHSMM_RS_EMISSION_PRUNING_NONE && defaults.radius==5);
    size_t comparisons=0;
    _Alignas(8) LiblrhsmmRsEmissionSettingsF64 options=defaults;
    for(size_t mode=0;mode<7;++mode) {
        options.temperature=0.75;
        options.pruning=mode==6?LIBLRHSMM_RS_EMISSION_PRUNING_NONE:(mode==1?LIBLRHSMM_RS_EMISSION_PRUNING_LINEAR:LIBLRHSMM_RS_EMISSION_PRUNING_BOUNDARIES);
        _Alignas(8) const double radii[6]={1,1,0.25,0.5,1.75,2.5}; options.radius=mode==6?NAN:radii[mode]; options.slope=mode==1?0.34:NAN;
        LiblrhsmmRsLogEmissionsF64 *log=NULL,*cached=NULL; LiblrhsmmRsEmissionDensitiesF64 *density=NULL,*cached_density=NULL;
        assert(liblrhsmm_rs_model_emission_log_probabilities_f64(model,observation,segmentation,&options,&log)==0);
        assert(liblrhsmm_rs_prepared_model_emission_log_probabilities_f64(prepared,observation,segmentation,&options,&cached)==0);
        assert(liblrhsmm_rs_model_emission_probabilities_f64(model,observation,segmentation,&options,&density)==0);
        assert(liblrhsmm_rs_prepared_model_emission_probabilities_f64(prepared,observation,segmentation,&options,&cached_density)==0);
        _Alignas(8) double raw[36], cached_values[36], raw_density[36], cached_density_values[36];
        read_log_f64(log,raw); read_log_f64(cached,cached_values); read_density_f64(density,raw_density); read_density_f64(cached_density,cached_density_values);
        assert(memcmp(raw,cached_values,sizeof(raw))==0);assert(memcmp(raw_density,cached_density_values,sizeof(raw_density))==0);
        if(mode<6) for(size_t index=0;index<36;++index) { assert(fabs(raw[index]-reference_f64[mode][index])/fmax(1.0,fabs(reference_f64[mode][index]))<=2e-14); ++comparisons; }

        assert(liblrhsmm_rs_log_emissions_release_f64(&log)==0); assert(liblrhsmm_rs_log_emissions_release_f64(&cached)==0); assert(liblrhsmm_rs_emission_densities_release_f64(&density)==0); assert(liblrhsmm_rs_emission_densities_release_f64(&cached_density)==0);
    }
    assert(comparisons==216);
    LiblrhsmmRsLogEmissionsF64 *sentinel=(LiblrhsmmRsLogEmissionsF64 *)(uintptr_t)0x1000;
    options.pruning=99;
    assert(liblrhsmm_rs_model_emission_log_probabilities_f64(model,observation,segmentation,&options,&sentinel)==3);assert((uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_prepared_model_emission_log_probabilities_f64(prepared,observation,segmentation,&options,&sentinel)==3);assert((uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_model_emission_log_probabilities_f64(NULL,observation,segmentation,&defaults,&sentinel)==1);
    assert(liblrhsmm_rs_emission_settings_default_f64(NULL)==1);
    assert(liblrhsmm_rs_model_release_f64(&model)==0);
    LiblrhsmmRsLogEmissionsF64 *retained=NULL; LiblrhsmmRsEmissionDensitiesF64 *retained_density=NULL;
    assert(liblrhsmm_rs_prepared_model_emission_log_probabilities_f64(prepared,observation,segmentation,&defaults,&retained)==0);
    assert(liblrhsmm_rs_prepared_model_emission_probabilities_f64(prepared,observation,segmentation,&defaults,&retained_density)==0);
    assert(liblrhsmm_rs_prepared_model_release_f64(&prepared)==0); assert(liblrhsmm_rs_observation_release_f64(&observation)==0); assert(liblrhsmm_rs_segmentation_release_f64(&segmentation)==0);
    _Alignas(8) double copied[36]; read_log_f64(retained,copied);read_density_f64(retained_density,copied);
    assert(liblrhsmm_rs_log_emissions_release_f64(&retained)==0);assert(liblrhsmm_rs_emission_densities_release_f64(&retained_density)==0);
}

int main(void) { exercise_f32();exercise_f64(); puts("Emission C: all10 exports,432 original C log values and36 densities passed");return 0;}
