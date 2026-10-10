/* Assert-enabled optimized C caller; preserve every original NUM1 record. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "liblrhsmm_rs.h"
static uint32_t integer(FILE *file) {
    unsigned char bytes[4]; assert(fread(bytes,1,4,file)==4);
    return (uint32_t)bytes[0]|((uint32_t)bytes[1]<<8)|((uint32_t)bytes[2]<<16)|((uint32_t)bytes[3]<<24);
}
static uint32_t bits_f32(float value) { uint32_t bits;memcpy(&bits,&value,4);return bits; }
static uint64_t bits_f64(double value) { uint64_t bits;memcpy(&bits,&value,8);return bits; }
static float read_f32(FILE *file) { uint32_t bits=integer(file);float value;memcpy(&value,&bits,4);return value; }
static double read_f64(FILE *file) { uint64_t bits=integer(file);bits|=(uint64_t)integer(file)<<32;_Alignas(8) double value;memcpy(&value,&bits,8);return value; }

#define CHECK_PRECISION(SUFFIX,TYPE,READ,BITS,WIDTH,FIXTURE) \
static void check_##SUFFIX(void) { \
    FILE *file=fopen(FIXTURE,"rb");assert(file);char magic[4];assert(fread(magic,1,4,file)==4 && memcmp(magic,"NUM1",4)==0); \
    assert(integer(file)==WIDTH && integer(file)==7000); \
    _Alignas(8) TYPE coefficients[7000]; \
    for(uint32_t table=0;table<2;++table) { \
        assert(liblrhsmm_rs_table_copy_##SUFFIX(table,0,coefficients,7000)==0); \
        for(int i=0;i<7000;++i) assert(BITS(coefficients[i])==BITS(READ(file))); \
    } \
    assert(integer(file)==42008); \
    for(int record=0;record<42008;++record) { \
        uint32_t kind=integer(file); _Alignas(8) TYPE a=READ(file),b=READ(file),expected=READ(file),output=0; \
        uint32_t status=kind==0?liblrhsmm_rs_log_add_##SUFFIX(a,b,&output):kind==1?liblrhsmm_rs_negative_exponential_##SUFFIX(a,&output):liblrhsmm_rs_exponential_##SUFFIX(a,&output); \
        assert(kind<3 && status==0 && BITS(output)==BITS(expected)); \
    } \
    assert(fgetc(file)==EOF && !ferror(file));assert(fclose(file)==0); \
    for(uint32_t table=0;table<2;++table) { \
        _Alignas(8) TYPE original=0,previous=0,current=0; \
        assert(liblrhsmm_rs_table_get_##SUFFIX(table,0,&original)==0); \
        _Alignas(8) TYPE changed=table==0?(TYPE)0.125:(TYPE)0.375; \
        assert(liblrhsmm_rs_table_replace_##SUFFIX(table,0,changed,&previous)==0 && BITS(previous)==BITS(original)); \
        assert((table==0?liblrhsmm_rs_log_add_##SUFFIX(0,0,&current):liblrhsmm_rs_negative_exponential_##SUFFIX((TYPE)-0.001,&current))==0 && current==changed); \
        assert(liblrhsmm_rs_table_replace_##SUFFIX(table,0,1,NULL)==1); \
        assert(liblrhsmm_rs_table_get_##SUFFIX(table,0,&current)==0 && current==changed); \
        assert(liblrhsmm_rs_table_replace_##SUFFIX(table,0,original,&previous)==0 && previous==changed); \
        _Alignas(8) TYPE output[2]={(TYPE)71,(TYPE)71}; \
        assert(liblrhsmm_rs_table_copy_##SUFFIX(table,6999,output,2)==2 && output[0]==71 && output[1]==71); \
        assert(liblrhsmm_rs_table_copy_##SUFFIX(table,7000,NULL,0)==0); \
        assert(liblrhsmm_rs_table_copy_##SUFFIX(table,7001,NULL,0)==2); \
        assert(liblrhsmm_rs_table_get_##SUFFIX(99,0,&current)==3); \
        assert(liblrhsmm_rs_table_get_##SUFFIX(table,7000,&current)==2); \
    } \
}
CHECK_PRECISION(f32,float,read_f32,bits_f32,4,"tests/fixtures/c-numerics-float.bin")
CHECK_PRECISION(f64,double,read_f64,bits_f64,8,"tests/fixtures/c-numerics-double.bin")
int main(void) {
    assert(liblrhsmm_rs_abi_version()==1);
    _Alignas(8) LiblrhsmmRsNumericalConstants constants={0};
    assert(liblrhsmm_rs_numerical_constants(&constants)==0);
    assert(constants.log_add_count==7000 && constants.exponential_count==7000);
    assert(constants.log_add_step==0.001 && constants.exponential_step==0.002 && constants.occupancy_epsilon==1e-7);
    assert(constants.log_zero_f32==(float)-1e15 && constants.log_zero_f64==-1e15 && constants.pi==3.14159265358979323846);
    assert(liblrhsmm_rs_numerical_constants(NULL)==1);
    check_f32();check_f64();
    _Alignas(8) double result=0;
    assert(liblrhsmm_rs_log_add_f64(INFINITY,INFINITY,&result)==0 && result==INFINITY);
    assert(liblrhsmm_rs_negative_exponential_f64(-INFINITY,&result)==0 && result==0);
    assert(liblrhsmm_rs_exponential_f64(NAN,&result)==0 && isnan(result));
    assert(liblrhsmm_rs_exponential_f32(0,NULL)==1);
    return 0;
}
