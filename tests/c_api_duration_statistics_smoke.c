/* Actual optimized C caller for duration statistics and maximization. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include "liblrhsmm_rs.h"

static void check_f32(void) {
    _Alignas(8) LiblrhsmmRsDurationStatisticsValuesF32 values={99,99,99}, fetched;
    assert(liblrhsmm_rs_duration_statistics_default_f32(&values)==0);
    assert(values.value_sum==0 && values.squared_sum==0 && values.occupancy==0);
    _Alignas(8) LiblrhsmmRsDurationParametersF32 parameters;
    assert(liblrhsmm_rs_duration_default_f32(&parameters)==0);
    LiblrhsmmRsDurationF32 *duration=NULL;
    LiblrhsmmRsDurationStatisticsF32 *statistics=NULL, *copy=NULL, *direct=NULL;
    assert(liblrhsmm_rs_duration_create_f32(&parameters,&duration)==0);
    assert(liblrhsmm_rs_duration_statistics_from_duration_f32(duration,&statistics)==0);
    assert(liblrhsmm_rs_duration_release_f32(&duration)==0);
    values.value_sum=6; values.squared_sum=20; values.occupancy=2;
    assert(liblrhsmm_rs_duration_statistics_set_f32(statistics,&values)==0);
    assert(liblrhsmm_rs_duration_statistics_clone_f32(statistics,&copy)==0);
    assert(liblrhsmm_rs_duration_statistics_clear_f32(statistics)==0);
    assert(liblrhsmm_rs_duration_statistics_get_f32(statistics,&fetched)==0);
    assert(fetched.value_sum==0 && fetched.squared_sum==0 && fetched.occupancy==0);
    assert(liblrhsmm_rs_duration_statistics_get_f32(copy,&fetched)==0);
    assert(fetched.value_sum==6 && fetched.squared_sum==20 && fetched.occupancy==2);
    assert(liblrhsmm_rs_duration_statistics_create_f32(&values,&direct)==0);
    assert(liblrhsmm_rs_duration_statistics_release_f32(&direct)==0 && direct==NULL);
    assert(liblrhsmm_rs_duration_statistics_release_f32(&direct)==0);
    assert(liblrhsmm_rs_duration_statistics_create_f32(NULL,&direct)==1);
    assert(liblrhsmm_rs_duration_statistics_create_f32(&values,NULL)==1);
    assert(liblrhsmm_rs_duration_statistics_from_duration_f32(NULL,&direct)==1);
    assert(liblrhsmm_rs_duration_statistics_get_f32(copy,NULL)==1);
    assert(liblrhsmm_rs_duration_statistics_set_f32(copy,NULL)==1);
    assert(liblrhsmm_rs_duration_statistics_clone_f32(copy,NULL)==1);
    assert(liblrhsmm_rs_duration_statistics_clear_f32(NULL)==1);
    assert(liblrhsmm_rs_duration_statistics_default_f32(NULL)==1);
    assert(liblrhsmm_rs_duration_statistics_release_f32(NULL)==1);
    assert(liblrhsmm_rs_duration_statistics_release_f32(&copy)==0);
    _Alignas(8) LiblrhsmmRsUpdateOptionsF32 options;
    assert(liblrhsmm_rs_update_options_default_f32(&options)==0);
    assert(options.duration_mode==LIBLRHSMM_RS_DURATION_NORMAL);
    assert(options.duration_variance_floor==(float).05 && options.emission_variance_floor==(float).01);
    parameters.mean=7; parameters.variance=3;
    assert(liblrhsmm_rs_duration_create_f32(&parameters,&duration)==0);
    FILE *file=fopen("tests/fixtures/c-updates.txt","r"); assert(file);
    char line[1024], operation[32]; unsigned scenario, compared=0;
    while(fgets(line,sizeof(line),file)) {
        _Alignas(8) double expected_mean, expected_variance;
        if(sscanf(line,"%31s %u %lf %lf",operation,&scenario,&expected_mean,&expected_variance)!=4 || strcmp(operation,"duration")) continue;
        parameters.mean=7; parameters.variance=3; parameters.variance_floor=scenario==2?2.5:0;
        values.value_sum=scenario==3?1.5:9; values.squared_sum=29; values.occupancy=scenario==4?(float)1e-8:3;
        options.duration_mode=scenario==3?LIBLRHSMM_RS_DURATION_GEOMETRIC:LIBLRHSMM_RS_DURATION_NORMAL;
        assert(liblrhsmm_rs_duration_set_f32(duration,&parameters)==0);
        assert(liblrhsmm_rs_duration_statistics_set_f32(statistics,&values)==0);
        uint32_t updated=99;
        assert(liblrhsmm_rs_duration_update_f32(duration,statistics,&options,&updated)==0);
        assert(updated==(scenario==4?0u:1u));
        assert(liblrhsmm_rs_duration_get_f32(duration,&parameters)==0);
        assert(parameters.mean==(float)expected_mean && parameters.variance==(float)expected_variance);
        compared+=2;
    }
    assert(fclose(file)==0 && compared==10);
    options.duration_mode=LIBLRHSMM_RS_DURATION_NORMAL;
    parameters.fixed_mean=4; parameters.variance_floor=0;
    values.value_sum=6; values.squared_sum=20; values.occupancy=2;
    assert(liblrhsmm_rs_duration_set_f32(duration,&parameters)==0);
    assert(liblrhsmm_rs_duration_statistics_set_f32(statistics,&values)==0);
    uint32_t updated=99;
    assert(liblrhsmm_rs_duration_update_f32(duration,statistics,&options,&updated)==0 && updated==1);
    assert(liblrhsmm_rs_duration_get_f32(duration,&parameters)==0 && parameters.mean==4 && parameters.variance==2);
    options.duration_mode=99; updated=99;
    assert(liblrhsmm_rs_duration_update_f32(duration,statistics,&options,&updated)==3 && updated==99);
    assert(liblrhsmm_rs_duration_get_f32(duration,&parameters)==0 && parameters.mean==4 && parameters.variance==2);
    options.duration_mode=LIBLRHSMM_RS_DURATION_NORMAL;
    assert(liblrhsmm_rs_duration_update_f32(duration,statistics,&options,NULL)==1);
    assert(liblrhsmm_rs_duration_get_f32(duration,&parameters)==0 && parameters.mean==4 && parameters.variance==2);
    options.emission_variance_floor=NAN;
    assert(liblrhsmm_rs_duration_update_f32(duration,statistics,&options,&updated)==3 && updated==99);
    options.emission_variance_floor=(float).01;
    values.value_sum=NAN;
    assert(liblrhsmm_rs_duration_statistics_set_f32(statistics,&values)==0);
    assert(liblrhsmm_rs_duration_statistics_get_f32(statistics,&fetched)==0 && isnan(fetched.value_sum));
    assert(liblrhsmm_rs_duration_update_f32(duration,statistics,&options,&updated)==3 && updated==99);
    assert(liblrhsmm_rs_duration_get_f32(duration,&parameters)==0 && parameters.mean==4 && parameters.variance==2);
    assert(liblrhsmm_rs_duration_update_f32(duration,NULL,&options,&updated)==1);
    assert(liblrhsmm_rs_duration_update_f32(duration,statistics,NULL,&updated)==1);
    assert(liblrhsmm_rs_duration_update_f32(NULL,statistics,&options,&updated)==1);
    assert(liblrhsmm_rs_update_options_default_f32(NULL)==1);
    assert(liblrhsmm_rs_duration_statistics_clear_f32(statistics)==0);
    assert(liblrhsmm_rs_duration_statistics_release_f32(&statistics)==0 && statistics==NULL);
    assert(liblrhsmm_rs_duration_release_f32(&duration)==0 && duration==NULL);
}

static void check_f64(void) {
    _Alignas(8) LiblrhsmmRsDurationStatisticsValuesF64 values={99,99,99}, fetched;
    assert(liblrhsmm_rs_duration_statistics_default_f64(&values)==0);
    assert(values.value_sum==0 && values.squared_sum==0 && values.occupancy==0);
    _Alignas(8) LiblrhsmmRsDurationParametersF64 parameters;
    assert(liblrhsmm_rs_duration_default_f64(&parameters)==0);
    LiblrhsmmRsDurationF64 *duration=NULL;
    LiblrhsmmRsDurationStatisticsF64 *statistics=NULL, *copy=NULL, *direct=NULL;
    assert(liblrhsmm_rs_duration_create_f64(&parameters,&duration)==0);
    assert(liblrhsmm_rs_duration_statistics_from_duration_f64(duration,&statistics)==0);
    assert(liblrhsmm_rs_duration_release_f64(&duration)==0);
    values.value_sum=6; values.squared_sum=20; values.occupancy=2;
    assert(liblrhsmm_rs_duration_statistics_set_f64(statistics,&values)==0);
    assert(liblrhsmm_rs_duration_statistics_clone_f64(statistics,&copy)==0);
    assert(liblrhsmm_rs_duration_statistics_clear_f64(statistics)==0);
    assert(liblrhsmm_rs_duration_statistics_get_f64(statistics,&fetched)==0);
    assert(fetched.value_sum==0 && fetched.squared_sum==0 && fetched.occupancy==0);
    assert(liblrhsmm_rs_duration_statistics_get_f64(copy,&fetched)==0);
    assert(fetched.value_sum==6 && fetched.squared_sum==20 && fetched.occupancy==2);
    assert(liblrhsmm_rs_duration_statistics_create_f64(&values,&direct)==0);
    assert(liblrhsmm_rs_duration_statistics_release_f64(&direct)==0 && direct==NULL);
    assert(liblrhsmm_rs_duration_statistics_release_f64(&direct)==0);
    assert(liblrhsmm_rs_duration_statistics_create_f64(NULL,&direct)==1);
    assert(liblrhsmm_rs_duration_statistics_create_f64(&values,NULL)==1);
    assert(liblrhsmm_rs_duration_statistics_from_duration_f64(NULL,&direct)==1);
    assert(liblrhsmm_rs_duration_statistics_get_f64(copy,NULL)==1);
    assert(liblrhsmm_rs_duration_statistics_set_f64(copy,NULL)==1);
    assert(liblrhsmm_rs_duration_statistics_clone_f64(copy,NULL)==1);
    assert(liblrhsmm_rs_duration_statistics_clear_f64(NULL)==1);
    assert(liblrhsmm_rs_duration_statistics_default_f64(NULL)==1);
    assert(liblrhsmm_rs_duration_statistics_release_f64(NULL)==1);
    assert(liblrhsmm_rs_duration_statistics_release_f64(&copy)==0);
    _Alignas(8) LiblrhsmmRsUpdateOptionsF64 options;
    assert(liblrhsmm_rs_update_options_default_f64(&options)==0);
    assert(options.duration_mode==LIBLRHSMM_RS_DURATION_NORMAL);
    assert(options.duration_variance_floor==(double).05 && options.emission_variance_floor==(double).01);
    parameters.mean=7; parameters.variance=3;
    assert(liblrhsmm_rs_duration_create_f64(&parameters,&duration)==0);
    FILE *file=fopen("tests/fixtures/c-double-update.txt","r"); assert(file);
    char line[1024], operation[32]; unsigned scenario, compared=0;
    while(fgets(line,sizeof(line),file)) {
        _Alignas(8) double expected_mean, expected_variance;
        if(sscanf(line,"%31s %u %lf %lf",operation,&scenario,&expected_mean,&expected_variance)!=4 || strcmp(operation,"duration")) continue;
        parameters.mean=7; parameters.variance=3; parameters.variance_floor=scenario==2?2.5:0;
        values.value_sum=scenario==3?1.5:9; values.squared_sum=29; values.occupancy=scenario==4?(double)1e-8:3;
        options.duration_mode=scenario==3?LIBLRHSMM_RS_DURATION_GEOMETRIC:LIBLRHSMM_RS_DURATION_NORMAL;
        assert(liblrhsmm_rs_duration_set_f64(duration,&parameters)==0);
        assert(liblrhsmm_rs_duration_statistics_set_f64(statistics,&values)==0);
        uint32_t updated=99;
        assert(liblrhsmm_rs_duration_update_f64(duration,statistics,&options,&updated)==0);
        assert(updated==(scenario==4?0u:1u));
        assert(liblrhsmm_rs_duration_get_f64(duration,&parameters)==0);
        assert(fabs(parameters.mean-expected_mean)/fmax(fabs(expected_mean),1)<=2e-14); assert(fabs(parameters.variance-expected_variance)/fmax(fabs(expected_variance),1)<=2e-14);
        compared+=2;
    }
    assert(fclose(file)==0 && compared==10);
    options.duration_mode=LIBLRHSMM_RS_DURATION_NORMAL;
    parameters.fixed_mean=4; parameters.variance_floor=0;
    values.value_sum=6; values.squared_sum=20; values.occupancy=2;
    assert(liblrhsmm_rs_duration_set_f64(duration,&parameters)==0);
    assert(liblrhsmm_rs_duration_statistics_set_f64(statistics,&values)==0);
    uint32_t updated=99;
    assert(liblrhsmm_rs_duration_update_f64(duration,statistics,&options,&updated)==0 && updated==1);
    assert(liblrhsmm_rs_duration_get_f64(duration,&parameters)==0 && parameters.mean==4 && parameters.variance==2);
    options.duration_mode=99; updated=99;
    assert(liblrhsmm_rs_duration_update_f64(duration,statistics,&options,&updated)==3 && updated==99);
    assert(liblrhsmm_rs_duration_get_f64(duration,&parameters)==0 && parameters.mean==4 && parameters.variance==2);
    options.duration_mode=LIBLRHSMM_RS_DURATION_NORMAL;
    assert(liblrhsmm_rs_duration_update_f64(duration,statistics,&options,NULL)==1);
    assert(liblrhsmm_rs_duration_get_f64(duration,&parameters)==0 && parameters.mean==4 && parameters.variance==2);
    options.emission_variance_floor=NAN;
    assert(liblrhsmm_rs_duration_update_f64(duration,statistics,&options,&updated)==3 && updated==99);
    options.emission_variance_floor=(double).01;
    values.value_sum=NAN;
    assert(liblrhsmm_rs_duration_statistics_set_f64(statistics,&values)==0);
    assert(liblrhsmm_rs_duration_statistics_get_f64(statistics,&fetched)==0 && isnan(fetched.value_sum));
    assert(liblrhsmm_rs_duration_update_f64(duration,statistics,&options,&updated)==3 && updated==99);
    assert(liblrhsmm_rs_duration_get_f64(duration,&parameters)==0 && parameters.mean==4 && parameters.variance==2);
    assert(liblrhsmm_rs_duration_update_f64(duration,NULL,&options,&updated)==1);
    assert(liblrhsmm_rs_duration_update_f64(duration,statistics,NULL,&updated)==1);
    assert(liblrhsmm_rs_duration_update_f64(NULL,statistics,&options,&updated)==1);
    assert(liblrhsmm_rs_update_options_default_f64(NULL)==1);
    assert(liblrhsmm_rs_duration_statistics_clear_f64(statistics)==0);
    assert(liblrhsmm_rs_duration_statistics_release_f64(&statistics)==0 && statistics==NULL);
    assert(liblrhsmm_rs_duration_release_f64(&duration)==0 && duration==NULL);
}

int main(void) {check_f32();check_f64();puts("C duration statistics: all20 exports and20 original C values passed");return 0;}
