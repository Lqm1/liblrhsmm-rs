/* Actual C ownership and original-source duration density comparisons. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include "liblrhsmm_rs.h"

static void check_f32(void) {
    _Alignas(8) LiblrhsmmRsDurationParametersF32 parameters;
    assert(liblrhsmm_rs_duration_default_f32(&parameters)==0);
    assert(parameters.mean==0 && parameters.variance==0 && parameters.minimum==-1);
    assert(parameters.maximum==-1 && parameters.fixed_mean==-1 && parameters.variance_floor==0);
    LiblrhsmmRsDurationF32 *owner=NULL, *copy=NULL;
    assert(liblrhsmm_rs_duration_create_f32(&parameters,&owner)==0);
    _Alignas(8) double density=71;
    assert(liblrhsmm_rs_duration_probability_f32(owner,3,1,1,&density)==3 && density==71);
    parameters.mean=4.25; parameters.variance=2.5; parameters.minimum=2;
    parameters.maximum=6; parameters.fixed_mean=17; parameters.variance_floor=.125;
    assert(liblrhsmm_rs_duration_set_f32(owner,&parameters)==0);
    FILE *file=fopen("tests/fixtures/c-precision-float.txt","r"); assert(file);
    char line[1024], operation[32]; unsigned frames, index; unsigned compared=0;
    while(fgets(line,sizeof(line),file)) {
        _Alignas(8) double expected[3];
        if(sscanf(line,"%31s %u %u %lf %lf %lf",operation,&frames,&index,
            &expected[0],&expected[1],&expected[2])!=6 || strcmp(operation,"duration")) continue;
        float temperatures[]={.5,1,1.75}, log=71, annealed=71;
        assert(index<3);
        assert(liblrhsmm_rs_duration_log_probability_f32(owner,(float)frames,.75,&log)==0);
        assert(liblrhsmm_rs_duration_annealed_log_probability_f32(owner,frames,.75,temperatures[index],&annealed)==0);
        assert(liblrhsmm_rs_duration_probability_f32(owner,frames,.75,temperatures[index],&density)==0);
        _Alignas(8) double actual[]={log,annealed,density};
        for(unsigned i=0;i<3;i++) { assert(fabs(actual[i]-expected[i])/fmax(fabs(expected[i]),1)<=2e-14); compared++; }
    }
    assert(fclose(file)==0 && compared==81);
    assert(liblrhsmm_rs_duration_clone_f32(owner,&copy)==0);
    parameters.mean=8;
    assert(liblrhsmm_rs_duration_set_f32(copy,&parameters)==0);
    assert(liblrhsmm_rs_duration_get_f32(owner,&parameters)==0);
    assert(parameters.mean==4.25 && parameters.fixed_mean==17 && parameters.variance_floor==.125);
    assert(liblrhsmm_rs_duration_set_f32(owner,NULL)==1);
    assert(liblrhsmm_rs_duration_create_f32(&parameters,NULL)==1);
    assert(liblrhsmm_rs_duration_clone_f32(owner,NULL)==1);
    assert(liblrhsmm_rs_duration_get_f32(owner,NULL)==1);
    assert(liblrhsmm_rs_duration_default_f32(NULL)==1);
    float output=71;
    assert(liblrhsmm_rs_duration_annealed_log_probability_f32(owner,UINTPTR_MAX,.75,1,&output)==3 && output==71);
    assert(liblrhsmm_rs_duration_log_probability_f32(owner,NAN,.75,&output)==3 && output==71);
    assert(liblrhsmm_rs_duration_annealed_log_probability_f32(owner,1,.75,0,&output)==0 && output==(float)-1e15);
    assert(liblrhsmm_rs_duration_annealed_log_probability_f32(owner,3,.75,0,&output)==0 && output==0);
    assert(liblrhsmm_rs_duration_probability_f32(owner,8,.75,1,&density)==0 && density>0);
    assert(liblrhsmm_rs_duration_release_f32(&owner)==0 && owner==NULL);
    assert(liblrhsmm_rs_duration_release_f32(&owner)==0);
    assert(liblrhsmm_rs_duration_get_f32(copy,&parameters)==0 && parameters.mean==8);
    assert(liblrhsmm_rs_duration_release_f32(&copy)==0 && copy==NULL);
    assert(liblrhsmm_rs_duration_release_f32(NULL)==1);
}

static void check_f64(void) {
    _Alignas(8) LiblrhsmmRsDurationParametersF64 parameters;
    assert(liblrhsmm_rs_duration_default_f64(&parameters)==0);
    assert(parameters.mean==0 && parameters.variance==0 && parameters.minimum==-1);
    assert(parameters.maximum==-1 && parameters.fixed_mean==-1 && parameters.variance_floor==0);
    LiblrhsmmRsDurationF64 *owner=NULL, *copy=NULL;
    assert(liblrhsmm_rs_duration_create_f64(&parameters,&owner)==0);
    _Alignas(8) double density=71;
    assert(liblrhsmm_rs_duration_probability_f64(owner,3,1,1,&density)==3 && density==71);
    parameters.mean=4.25; parameters.variance=2.5; parameters.minimum=2;
    parameters.maximum=6; parameters.fixed_mean=17; parameters.variance_floor=.125;
    assert(liblrhsmm_rs_duration_set_f64(owner,&parameters)==0);
    FILE *file=fopen("tests/fixtures/c-precision-double.txt","r"); assert(file);
    char line[1024], operation[32]; unsigned frames, index; unsigned compared=0;
    while(fgets(line,sizeof(line),file)) {
        _Alignas(8) double expected[3];
        if(sscanf(line,"%31s %u %u %lf %lf %lf",operation,&frames,&index,
            &expected[0],&expected[1],&expected[2])!=6 || strcmp(operation,"duration")) continue;
        _Alignas(8) double temperatures[]={.5,1,1.75}, log=71, annealed=71;
        assert(index<3);
        assert(liblrhsmm_rs_duration_log_probability_f64(owner,(double)frames,.75,&log)==0);
        assert(liblrhsmm_rs_duration_annealed_log_probability_f64(owner,frames,.75,temperatures[index],&annealed)==0);
        assert(liblrhsmm_rs_duration_probability_f64(owner,frames,.75,temperatures[index],&density)==0);
        _Alignas(8) double actual[]={log,annealed,density};
        for(unsigned i=0;i<3;i++) { assert(fabs(actual[i]-expected[i])/fmax(fabs(expected[i]),1)<=2e-14); compared++; }
    }
    assert(fclose(file)==0 && compared==81);
    assert(liblrhsmm_rs_duration_clone_f64(owner,&copy)==0);
    parameters.mean=8;
    assert(liblrhsmm_rs_duration_set_f64(copy,&parameters)==0);
    assert(liblrhsmm_rs_duration_get_f64(owner,&parameters)==0);
    assert(parameters.mean==4.25 && parameters.fixed_mean==17 && parameters.variance_floor==.125);
    assert(liblrhsmm_rs_duration_set_f64(owner,NULL)==1);
    assert(liblrhsmm_rs_duration_create_f64(&parameters,NULL)==1);
    assert(liblrhsmm_rs_duration_clone_f64(owner,NULL)==1);
    assert(liblrhsmm_rs_duration_get_f64(owner,NULL)==1);
    assert(liblrhsmm_rs_duration_default_f64(NULL)==1);
    _Alignas(8) double output=71;
    assert(liblrhsmm_rs_duration_annealed_log_probability_f64(owner,UINTPTR_MAX,.75,1,&output)==3 && output==71);
    assert(liblrhsmm_rs_duration_log_probability_f64(owner,NAN,.75,&output)==3 && output==71);
    assert(liblrhsmm_rs_duration_annealed_log_probability_f64(owner,1,.75,0,&output)==0 && output==(double)-1e15);
    assert(liblrhsmm_rs_duration_annealed_log_probability_f64(owner,3,.75,0,&output)==0 && output==0);
    assert(liblrhsmm_rs_duration_probability_f64(owner,8,.75,1,&density)==0 && density>0);
    assert(liblrhsmm_rs_duration_release_f64(&owner)==0 && owner==NULL);
    assert(liblrhsmm_rs_duration_release_f64(&owner)==0);
    assert(liblrhsmm_rs_duration_get_f64(copy,&parameters)==0 && parameters.mean==8);
    assert(liblrhsmm_rs_duration_release_f64(&copy)==0 && copy==NULL);
    assert(liblrhsmm_rs_duration_release_f64(NULL)==1);
}

int main(void) { check_f32(); check_f64(); puts("C duration: all18 exports and162 original C values passed"); return 0; }
