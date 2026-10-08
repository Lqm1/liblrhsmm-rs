/* Actual C caller for immutable prepared duration snapshots. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include "liblrhsmm_rs.h"
static void compare(double actual, double expected) {
    assert(fabs(actual-expected)/fmax(fabs(expected),1)<=2e-14);
}

static void check_f32(void) {
    LiblrhsmmRsDurationParametersF32 parameters={4.25,2.5,2,6,17,.125}, fetched;
    LiblrhsmmRsDurationPreparationF32 options, fetched_options;
    assert(liblrhsmm_rs_duration_preparation_default_f32(&options)==0);
    assert(options.duration_cache_length==300 && options.duration_weight==1 && options.temperature==1);
    LiblrhsmmRsDurationF32 *owner=NULL;
    LiblrhsmmRsPreparedDurationF32 *prepared=NULL, *copy=NULL;
    assert(liblrhsmm_rs_duration_create_f32(&parameters,&owner)==0);
    options.duration_cache_length=8; options.duration_weight=.5; options.temperature=.75;
    assert(liblrhsmm_rs_duration_prepare_f32(owner,&options,&prepared)==0);
    assert(liblrhsmm_rs_prepared_duration_parameters_f32(prepared,&fetched)==0);
    assert(fetched.mean==4.25 && fetched.variance==2.5 && fetched.minimum==2 && fetched.maximum==6);
    assert(fetched.fixed_mean==17 && fetched.variance_floor==.125);
    assert(liblrhsmm_rs_prepared_duration_options_f32(prepared,&fetched_options)==0);
    assert(fetched_options.duration_cache_length==8 && fetched_options.duration_weight==.5 && fetched_options.temperature==.75);
    uintptr_t length=99;
    assert(liblrhsmm_rs_prepared_duration_cache_length_f32(prepared,&length)==0 && length==8);
    float normalizer=71, cache[8];
    assert(liblrhsmm_rs_prepared_duration_log_normalizer_f32(prepared,&normalizer)==0);
    assert(liblrhsmm_rs_prepared_duration_cache_copy_f32(prepared,0,cache,8)==0);
    FILE *file=fopen("tests/fixtures/c-structures-float.txt","r"); assert(file);
    char line[1024], operation[32]; unsigned index, compared=0;
    while(fgets(line,sizeof(line),file)) {
        double expected;
        assert(sscanf(line,"%31s",operation)==1);
        if(!strcmp(operation,"duration")) {
            assert(sscanf(line,"%31s %lf",operation,&expected)==2);
            compare(normalizer,expected); compared++;
        }
        if(!strcmp(operation,"cache")) {
            assert(sscanf(line,"%31s %u %lf",operation,&index,&expected)==3);
            assert(index<8); compare(cache[index],expected); compared++;
        }
    }
    assert(fclose(file)==0 && compared==9);
    assert(liblrhsmm_rs_prepared_duration_clone_f32(prepared,&copy)==0);
    parameters.mean=99; parameters.minimum=99;
    assert(liblrhsmm_rs_duration_set_f32(owner,&parameters)==0);
    assert(liblrhsmm_rs_duration_release_f32(&owner)==0 && owner==NULL);
    assert(liblrhsmm_rs_prepared_duration_release_f32(&prepared)==0 && prepared==NULL);
    assert(liblrhsmm_rs_prepared_duration_release_f32(&prepared)==0);
    assert(liblrhsmm_rs_prepared_duration_parameters_f32(copy,&fetched)==0 && fetched.mean==4.25 && fetched.minimum==2);
    float copied_cache[8];
    assert(liblrhsmm_rs_prepared_duration_cache_copy_f32(copy,0,copied_cache,8)==0);
    for(unsigned i=0;i<8;i++) assert(copied_cache[i]==cache[i]);
    float log=71; double density=71;
    assert(liblrhsmm_rs_prepared_duration_log_probability_f32(copy,4,&log)==0 && log==cache[4]);
    assert(liblrhsmm_rs_prepared_duration_probability_f32(copy,4,&density)==0 && density>0);
    assert(liblrhsmm_rs_prepared_duration_cache_copy_f32(copy,8,NULL,0)==0);
    assert(liblrhsmm_rs_prepared_duration_cache_copy_f32(copy,9,NULL,0)==2);
    copied_cache[0]=71;
    assert(liblrhsmm_rs_prepared_duration_cache_copy_f32(copy,7,copied_cache,2)==2 && copied_cache[0]==71);
    assert(liblrhsmm_rs_prepared_duration_cache_copy_f32(copy,UINTPTR_MAX,copied_cache,1)==2 && copied_cache[0]==71);
    assert(liblrhsmm_rs_prepared_duration_cache_copy_f32(copy,0,NULL,1)==1);
    assert(liblrhsmm_rs_prepared_duration_log_probability_f32(copy,UINTPTR_MAX,&log)==3);
    log=71; density=71;
    assert(liblrhsmm_rs_prepared_duration_log_probability_f32(copy,UINTPTR_MAX,&log)==3 && log==71);
    assert(liblrhsmm_rs_prepared_duration_probability_f32(copy,UINTPTR_MAX,&density)==3 && density==71);
    assert(liblrhsmm_rs_prepared_duration_clone_f32(copy,NULL)==1);
    assert(liblrhsmm_rs_prepared_duration_parameters_f32(copy,NULL)==1);
    assert(liblrhsmm_rs_prepared_duration_options_f32(copy,NULL)==1);
    assert(liblrhsmm_rs_prepared_duration_log_normalizer_f32(copy,NULL)==1);
    assert(liblrhsmm_rs_prepared_duration_cache_length_f32(copy,NULL)==1);
    assert(liblrhsmm_rs_prepared_duration_release_f32(&copy)==0 && copy==NULL);
    assert(liblrhsmm_rs_prepared_duration_release_f32(NULL)==1);
    assert(liblrhsmm_rs_duration_preparation_default_f32(NULL)==1);
    parameters.mean=4.25; parameters.minimum=2;
    assert(liblrhsmm_rs_duration_create_f32(&parameters,&owner)==0);
    file=fopen("tests/fixtures/c-precision-float.txt","r"); assert(file);
    compared=0;
    while(fgets(line,sizeof(line),file)) {
        unsigned frames;
        double expected_log, expected_density, unused;
        if(sscanf(line,"%31s %u %u %lf %lf %lf",operation,&frames,&index,&unused,&expected_log,&expected_density)!=6 || strcmp(operation,"duration")) continue;
        float temperatures[]={.5,1,1.75}; assert(index<3);
        options.duration_cache_length=3; options.duration_weight=.75; options.temperature=temperatures[index];
        assert(liblrhsmm_rs_duration_prepare_f32(owner,&options,&prepared)==0);
        assert(liblrhsmm_rs_prepared_duration_log_probability_f32(prepared,frames,&log)==0);
        assert(liblrhsmm_rs_prepared_duration_probability_f32(prepared,frames,&density)==0);
        compare(log,expected_log); compare(density,expected_density); compared+=2;
        assert(liblrhsmm_rs_prepared_duration_release_f32(&prepared)==0);
    }
    assert(fclose(file)==0 && compared==54);

    file=fopen("tests/fixtures/c-preparation.txt","r"); assert(file);
    compared=0;
    while(fgets(line,sizeof(line),file)) {
        unsigned length_value, temperature_index, frames;
        double expected_log, expected_density;
        assert(sscanf(line,"%31s %u %u %u %lf %lf",operation,&length_value,&temperature_index,&frames,&expected_log,&expected_density)==6);
        assert(!strcmp(operation,"D"));
        options.duration_cache_length=length_value; options.duration_weight=.75; options.temperature=temperature_index*.5f;
        assert(liblrhsmm_rs_duration_prepare_f32(owner,&options,&prepared)==0);
        assert(liblrhsmm_rs_prepared_duration_log_probability_f32(prepared,frames,&log)==0);
        assert(liblrhsmm_rs_prepared_duration_probability_f32(prepared,frames,&density)==0);
        assert(log==(float)expected_log && density==(double)(float)expected_density); compared+=2;
        assert(liblrhsmm_rs_prepared_duration_release_f32(&prepared)==0);
    }
    assert(fclose(file)==0 && compared==144);

    options.duration_cache_length=0; options.temperature=0;
    assert(liblrhsmm_rs_duration_prepare_f32(owner,&options,&prepared)==0);
    assert(liblrhsmm_rs_prepared_duration_cache_length_f32(prepared,&length)==0 && length==0);
    assert(liblrhsmm_rs_prepared_duration_cache_copy_f32(prepared,0,NULL,0)==0);
    assert(liblrhsmm_rs_prepared_duration_log_probability_f32(prepared,1,&log)==0 && log==(float)-1e15);
    assert(liblrhsmm_rs_prepared_duration_log_probability_f32(prepared,3,&log)==0 && log==0);
    assert(liblrhsmm_rs_prepared_duration_release_f32(&prepared)==0);
    options.duration_cache_length=UINTPTR_MAX;
    assert(liblrhsmm_rs_duration_prepare_f32(owner,&options,&prepared)==3 && prepared==NULL);
    assert(liblrhsmm_rs_duration_prepare_f32(owner,NULL,&prepared)==1 && prepared==NULL);
    assert(liblrhsmm_rs_duration_prepare_f32(owner,&options,NULL)==1);
    assert(liblrhsmm_rs_duration_prepare_f32(NULL,&options,&prepared)==1);
    assert(liblrhsmm_rs_duration_release_f32(&owner)==0 && owner==NULL);
}

static void check_f64(void) {
    LiblrhsmmRsDurationParametersF64 parameters={4.25,2.5,2,6,17,.125}, fetched;
    LiblrhsmmRsDurationPreparationF64 options, fetched_options;
    assert(liblrhsmm_rs_duration_preparation_default_f64(&options)==0);
    assert(options.duration_cache_length==300 && options.duration_weight==1 && options.temperature==1);
    LiblrhsmmRsDurationF64 *owner=NULL;
    LiblrhsmmRsPreparedDurationF64 *prepared=NULL, *copy=NULL;
    assert(liblrhsmm_rs_duration_create_f64(&parameters,&owner)==0);
    options.duration_cache_length=8; options.duration_weight=.5; options.temperature=.75;
    assert(liblrhsmm_rs_duration_prepare_f64(owner,&options,&prepared)==0);
    assert(liblrhsmm_rs_prepared_duration_parameters_f64(prepared,&fetched)==0);
    assert(fetched.mean==4.25 && fetched.variance==2.5 && fetched.minimum==2 && fetched.maximum==6);
    assert(fetched.fixed_mean==17 && fetched.variance_floor==.125);
    assert(liblrhsmm_rs_prepared_duration_options_f64(prepared,&fetched_options)==0);
    assert(fetched_options.duration_cache_length==8 && fetched_options.duration_weight==.5 && fetched_options.temperature==.75);
    uintptr_t length=99;
    assert(liblrhsmm_rs_prepared_duration_cache_length_f64(prepared,&length)==0 && length==8);
    double normalizer=71, cache[8];
    assert(liblrhsmm_rs_prepared_duration_log_normalizer_f64(prepared,&normalizer)==0);
    assert(liblrhsmm_rs_prepared_duration_cache_copy_f64(prepared,0,cache,8)==0);
    FILE *file=fopen("tests/fixtures/c-structures-double.txt","r"); assert(file);
    char line[1024], operation[32]; unsigned index, compared=0;
    while(fgets(line,sizeof(line),file)) {
        double expected;
        assert(sscanf(line,"%31s",operation)==1);
        if(!strcmp(operation,"duration")) {
            assert(sscanf(line,"%31s %lf",operation,&expected)==2);
            compare(normalizer,expected); compared++;
        }
        if(!strcmp(operation,"cache")) {
            assert(sscanf(line,"%31s %u %lf",operation,&index,&expected)==3);
            assert(index<8); compare(cache[index],expected); compared++;
        }
    }
    assert(fclose(file)==0 && compared==9);
    assert(liblrhsmm_rs_prepared_duration_clone_f64(prepared,&copy)==0);
    parameters.mean=99; parameters.minimum=99;
    assert(liblrhsmm_rs_duration_set_f64(owner,&parameters)==0);
    assert(liblrhsmm_rs_duration_release_f64(&owner)==0 && owner==NULL);
    assert(liblrhsmm_rs_prepared_duration_release_f64(&prepared)==0 && prepared==NULL);
    assert(liblrhsmm_rs_prepared_duration_release_f64(&prepared)==0);
    assert(liblrhsmm_rs_prepared_duration_parameters_f64(copy,&fetched)==0 && fetched.mean==4.25 && fetched.minimum==2);
    double copied_cache[8];
    assert(liblrhsmm_rs_prepared_duration_cache_copy_f64(copy,0,copied_cache,8)==0);
    for(unsigned i=0;i<8;i++) assert(copied_cache[i]==cache[i]);
    double log=71; double density=71;
    assert(liblrhsmm_rs_prepared_duration_log_probability_f64(copy,4,&log)==0 && log==cache[4]);
    assert(liblrhsmm_rs_prepared_duration_probability_f64(copy,4,&density)==0 && density>0);
    assert(liblrhsmm_rs_prepared_duration_cache_copy_f64(copy,8,NULL,0)==0);
    assert(liblrhsmm_rs_prepared_duration_cache_copy_f64(copy,9,NULL,0)==2);
    copied_cache[0]=71;
    assert(liblrhsmm_rs_prepared_duration_cache_copy_f64(copy,7,copied_cache,2)==2 && copied_cache[0]==71);
    assert(liblrhsmm_rs_prepared_duration_cache_copy_f64(copy,UINTPTR_MAX,copied_cache,1)==2 && copied_cache[0]==71);
    assert(liblrhsmm_rs_prepared_duration_cache_copy_f64(copy,0,NULL,1)==1);
    assert(liblrhsmm_rs_prepared_duration_log_probability_f64(copy,UINTPTR_MAX,&log)==3);
    log=71; density=71;
    assert(liblrhsmm_rs_prepared_duration_log_probability_f64(copy,UINTPTR_MAX,&log)==3 && log==71);
    assert(liblrhsmm_rs_prepared_duration_probability_f64(copy,UINTPTR_MAX,&density)==3 && density==71);
    assert(liblrhsmm_rs_prepared_duration_clone_f64(copy,NULL)==1);
    assert(liblrhsmm_rs_prepared_duration_parameters_f64(copy,NULL)==1);
    assert(liblrhsmm_rs_prepared_duration_options_f64(copy,NULL)==1);
    assert(liblrhsmm_rs_prepared_duration_log_normalizer_f64(copy,NULL)==1);
    assert(liblrhsmm_rs_prepared_duration_cache_length_f64(copy,NULL)==1);
    assert(liblrhsmm_rs_prepared_duration_release_f64(&copy)==0 && copy==NULL);
    assert(liblrhsmm_rs_prepared_duration_release_f64(NULL)==1);
    assert(liblrhsmm_rs_duration_preparation_default_f64(NULL)==1);
    parameters.mean=4.25; parameters.minimum=2;
    assert(liblrhsmm_rs_duration_create_f64(&parameters,&owner)==0);
    file=fopen("tests/fixtures/c-precision-double.txt","r"); assert(file);
    compared=0;
    while(fgets(line,sizeof(line),file)) {
        unsigned frames;
        double expected_log, expected_density, unused;
        if(sscanf(line,"%31s %u %u %lf %lf %lf",operation,&frames,&index,&unused,&expected_log,&expected_density)!=6 || strcmp(operation,"duration")) continue;
        double temperatures[]={.5,1,1.75}; assert(index<3);
        options.duration_cache_length=3; options.duration_weight=.75; options.temperature=temperatures[index];
        assert(liblrhsmm_rs_duration_prepare_f64(owner,&options,&prepared)==0);
        assert(liblrhsmm_rs_prepared_duration_log_probability_f64(prepared,frames,&log)==0);
        assert(liblrhsmm_rs_prepared_duration_probability_f64(prepared,frames,&density)==0);
        compare(log,expected_log); compare(density,expected_density); compared+=2;
        assert(liblrhsmm_rs_prepared_duration_release_f64(&prepared)==0);
    }
    assert(fclose(file)==0 && compared==54);

    options.duration_cache_length=0; options.temperature=0;
    assert(liblrhsmm_rs_duration_prepare_f64(owner,&options,&prepared)==0);
    assert(liblrhsmm_rs_prepared_duration_cache_length_f64(prepared,&length)==0 && length==0);
    assert(liblrhsmm_rs_prepared_duration_cache_copy_f64(prepared,0,NULL,0)==0);
    assert(liblrhsmm_rs_prepared_duration_log_probability_f64(prepared,1,&log)==0 && log==(double)-1e15);
    assert(liblrhsmm_rs_prepared_duration_log_probability_f64(prepared,3,&log)==0 && log==0);
    assert(liblrhsmm_rs_prepared_duration_release_f64(&prepared)==0);
    options.duration_cache_length=UINTPTR_MAX;
    assert(liblrhsmm_rs_duration_prepare_f64(owner,&options,&prepared)==3 && prepared==NULL);
    assert(liblrhsmm_rs_duration_prepare_f64(owner,NULL,&prepared)==1 && prepared==NULL);
    assert(liblrhsmm_rs_duration_prepare_f64(owner,&options,NULL)==1);
    assert(liblrhsmm_rs_duration_prepare_f64(NULL,&options,&prepared)==1);
    assert(liblrhsmm_rs_duration_release_f64(&owner)==0 && owner==NULL);
}

int main(void) { check_f32(); check_f64(); puts("C prepared duration: all22 exports and270 original C values passed"); return 0; }
