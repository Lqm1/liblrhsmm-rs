#ifdef NDEBUG
#undef NDEBUG
#endif
#include "liblrhsmm_rs.h"
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

static void check_f32(void) {
 LiblrhsmmRsGaussianF32 *source=NULL,*editable=NULL; LiblrhsmmRsPreparedGaussianF32 *prepared=NULL,*cloned=NULL;
 float weights[1]={1},means[2]={1.25,-2.5},variances[2]={2.5,0.75},floors[2]={0.1,0.2};
 _Alignas(8) LiblrhsmmRsGaussianParametersF32 input={1,2,weights,means,variances,floors};
 assert(liblrhsmm_rs_gaussian_create_f32(&input,&source)==0);
 assert(liblrhsmm_rs_gaussian_increase_mixtures_f32(source,5,(float)0.2)==0);
 assert(liblrhsmm_rs_gaussian_prepare_f32(source,&prepared)==0);
 assert(liblrhsmm_rs_prepared_gaussian_parameters_f32(prepared,&editable)==0);
 assert(liblrhsmm_rs_prepared_gaussian_clone_f32(prepared,&cloned)==0);
 float changed=999;
 assert(liblrhsmm_rs_gaussian_set_field_f32(source,1,0,&changed,1)==0);
 assert(liblrhsmm_rs_gaussian_release_f32(&source)==0);
 assert(liblrhsmm_rs_gaussian_set_field_f32(editable,1,0,&changed,1)==0);
 assert(liblrhsmm_rs_gaussian_release_f32(&editable)==0);
 size_t count=0;assert(liblrhsmm_rs_prepared_gaussian_cache_length_f32(prepared,&count)==0 && count==5);
 float cache[5];assert(liblrhsmm_rs_prepared_gaussian_cache_copy_f32(prepared,0,cache,5)==0);
 assert(liblrhsmm_rs_prepared_gaussian_cache_copy_f32(prepared,5,NULL,0)==0);
 FILE *oracle=fopen("tests/fixtures/c-precision-float.txt","r");assert(oracle);
 char line[2048],operation[32];unsigned compared=0;
 while(fgets(line,sizeof line,oracle)) {
  if(strncmp(line,"component ",10)!=0 && strncmp(line,"output ",7)!=0)continue;
  size_t index,secondary;_Alignas(8) double expected_log,expected_probability;
  assert(sscanf(line,"%31s %zu %zu %lf %lf",operation,&index,&secondary,&expected_log,&expected_probability)==5);
  float observation[2]={(float)((double)index*0.3-0.5),(float)((double)index*(-0.2)+0.75)},result=77;
  _Alignas(8) double probability=77;
  if(strcmp(operation,"component")==0) {
   assert(liblrhsmm_rs_prepared_gaussian_component_log_probability_f32(prepared,observation,2,secondary,&result)==0);
   assert(liblrhsmm_rs_prepared_gaussian_component_probability_f32(prepared,observation,2,secondary,&probability)==0);
  } else {
   float temperatures[3]={0.5,1,1.75};
   assert(liblrhsmm_rs_prepared_gaussian_log_probability_f32(prepared,observation,2,temperatures[secondary],&result)==0);
   assert(liblrhsmm_rs_prepared_gaussian_probability_f32(prepared,observation,2,temperatures[secondary],&probability)==0);
  }
  _Alignas(8) double target=(float)expected_log;
  assert(fabs((double)result-target)<=2e-14*fmax(1,fabs(target)));
  assert(fabs(probability-expected_probability)<=2e-14*fmax(1,fabs(expected_probability)));compared+=2;
 }
 fclose(oracle);assert(compared==80);
 float observation[2]={0,0},a=77,b=77;
 assert(liblrhsmm_rs_prepared_gaussian_log_probability_f32(prepared,observation,2,1,&a)==0);
 assert(liblrhsmm_rs_prepared_gaussian_log_probability_f32(cloned,observation,2,1,&b)==0);assert(a==b);
 a=77;assert(liblrhsmm_rs_prepared_gaussian_component_log_probability_f32(prepared,observation,2,5,&a)==3 && a==77);
 assert(liblrhsmm_rs_prepared_gaussian_cache_copy_f32(prepared,5,&a,1)==2 && a==77);
 assert(liblrhsmm_rs_prepared_gaussian_release_f32(&cloned)==0 && cloned==NULL);
 assert(liblrhsmm_rs_prepared_gaussian_release_f32(&prepared)==0 && prepared==NULL);assert(liblrhsmm_rs_prepared_gaussian_release_f32(&prepared)==0);
 float cache_weights[3]={(float)(1.0/3.0),(float)(1.0/3.0),(float)(1.0/3.0)};
 float cache_means[6]={0},cache_variances[6]={0.75,1.25,2.5,3.75,0.125,0.5},cache_floors[6]={0};
 _Alignas(8) LiblrhsmmRsGaussianParametersF32 cache_input={3,2,cache_weights,cache_means,cache_variances,cache_floors};
 assert(liblrhsmm_rs_gaussian_create_f32(&cache_input,&source)==0);assert(liblrhsmm_rs_gaussian_prepare_f32(source,&prepared)==0);
 assert(liblrhsmm_rs_prepared_gaussian_cache_copy_f32(prepared,0,cache,3)==0);
 oracle=fopen("tests/fixtures/c-structures-float.txt","r");assert(oracle);compared=0;
 while(fgets(line,sizeof line,oracle)) {
  if(strncmp(line,"gaussian ",9)!=0)continue;
  size_t index;_Alignas(8) double expected;assert(sscanf(line,"gaussian %zu %lf",&index,&expected)==2);
  _Alignas(8) double target=(float)expected;assert(fabs((double)cache[index]-target)<=2e-14*fmax(1,fabs(target)));compared++;
 }
 fclose(oracle);assert(compared==3);
 assert(liblrhsmm_rs_gaussian_release_f32(&source)==0);assert(liblrhsmm_rs_prepared_gaussian_release_f32(&prepared)==0);
}

static void check_f64(void) {
 LiblrhsmmRsGaussianF64 *source=NULL,*editable=NULL; LiblrhsmmRsPreparedGaussianF64 *prepared=NULL,*cloned=NULL;
 _Alignas(8) double weights[1]={1},means[2]={1.25,-2.5},variances[2]={2.5,0.75},floors[2]={0.1,0.2};
 _Alignas(8) LiblrhsmmRsGaussianParametersF64 input={1,2,weights,means,variances,floors};
 assert(liblrhsmm_rs_gaussian_create_f64(&input,&source)==0);
 assert(liblrhsmm_rs_gaussian_increase_mixtures_f64(source,5,(double)0.2)==0);
 assert(liblrhsmm_rs_gaussian_prepare_f64(source,&prepared)==0);
 assert(liblrhsmm_rs_prepared_gaussian_parameters_f64(prepared,&editable)==0);
 assert(liblrhsmm_rs_prepared_gaussian_clone_f64(prepared,&cloned)==0);
 _Alignas(8) double changed=999;
 assert(liblrhsmm_rs_gaussian_set_field_f64(source,1,0,&changed,1)==0);
 assert(liblrhsmm_rs_gaussian_release_f64(&source)==0);
 assert(liblrhsmm_rs_gaussian_set_field_f64(editable,1,0,&changed,1)==0);
 assert(liblrhsmm_rs_gaussian_release_f64(&editable)==0);
 size_t count=0;assert(liblrhsmm_rs_prepared_gaussian_cache_length_f64(prepared,&count)==0 && count==5);
 _Alignas(8) double cache[5];assert(liblrhsmm_rs_prepared_gaussian_cache_copy_f64(prepared,0,cache,5)==0);
 assert(liblrhsmm_rs_prepared_gaussian_cache_copy_f64(prepared,5,NULL,0)==0);
 FILE *oracle=fopen("tests/fixtures/c-precision-double.txt","r");assert(oracle);
 char line[2048],operation[32];unsigned compared=0;
 while(fgets(line,sizeof line,oracle)) {
  if(strncmp(line,"component ",10)!=0 && strncmp(line,"output ",7)!=0)continue;
  size_t index,secondary;_Alignas(8) double expected_log,expected_probability;
  assert(sscanf(line,"%31s %zu %zu %lf %lf",operation,&index,&secondary,&expected_log,&expected_probability)==5);
  _Alignas(8) double observation[2]={(double)((double)index*0.3-0.5),(double)((double)index*(-0.2)+0.75)},result=77;
  _Alignas(8) double probability=77;
  if(strcmp(operation,"component")==0) {
   assert(liblrhsmm_rs_prepared_gaussian_component_log_probability_f64(prepared,observation,2,secondary,&result)==0);
   assert(liblrhsmm_rs_prepared_gaussian_component_probability_f64(prepared,observation,2,secondary,&probability)==0);
  } else {
   _Alignas(8) double temperatures[3]={0.5,1,1.75};
   assert(liblrhsmm_rs_prepared_gaussian_log_probability_f64(prepared,observation,2,temperatures[secondary],&result)==0);
   assert(liblrhsmm_rs_prepared_gaussian_probability_f64(prepared,observation,2,temperatures[secondary],&probability)==0);
  }
  _Alignas(8) double target=(double)expected_log;
  assert(fabs((double)result-target)<=2e-14*fmax(1,fabs(target)));
  assert(fabs(probability-expected_probability)<=2e-14*fmax(1,fabs(expected_probability)));compared+=2;
 }
 fclose(oracle);assert(compared==80);
 _Alignas(8) double observation[2]={0,0},a=77,b=77;
 assert(liblrhsmm_rs_prepared_gaussian_log_probability_f64(prepared,observation,2,1,&a)==0);
 assert(liblrhsmm_rs_prepared_gaussian_log_probability_f64(cloned,observation,2,1,&b)==0);assert(a==b);
 a=77;assert(liblrhsmm_rs_prepared_gaussian_component_log_probability_f64(prepared,observation,2,5,&a)==3 && a==77);
 assert(liblrhsmm_rs_prepared_gaussian_cache_copy_f64(prepared,5,&a,1)==2 && a==77);
 assert(liblrhsmm_rs_prepared_gaussian_release_f64(&cloned)==0 && cloned==NULL);
 assert(liblrhsmm_rs_prepared_gaussian_release_f64(&prepared)==0 && prepared==NULL);assert(liblrhsmm_rs_prepared_gaussian_release_f64(&prepared)==0);
 _Alignas(8) double cache_weights[3]={(double)(1.0/3.0),(double)(1.0/3.0),(double)(1.0/3.0)};
 _Alignas(8) double cache_means[6]={0},cache_variances[6]={0.75,1.25,2.5,3.75,0.125,0.5},cache_floors[6]={0};
 _Alignas(8) LiblrhsmmRsGaussianParametersF64 cache_input={3,2,cache_weights,cache_means,cache_variances,cache_floors};
 assert(liblrhsmm_rs_gaussian_create_f64(&cache_input,&source)==0);assert(liblrhsmm_rs_gaussian_prepare_f64(source,&prepared)==0);
 assert(liblrhsmm_rs_prepared_gaussian_cache_copy_f64(prepared,0,cache,3)==0);
 oracle=fopen("tests/fixtures/c-structures-double.txt","r");assert(oracle);compared=0;
 while(fgets(line,sizeof line,oracle)) {
  if(strncmp(line,"gaussian ",9)!=0)continue;
  size_t index;_Alignas(8) double expected;assert(sscanf(line,"gaussian %zu %lf",&index,&expected)==2);
  _Alignas(8) double target=(double)expected;assert(fabs((double)cache[index]-target)<=2e-14*fmax(1,fabs(target)));compared++;
 }
 fclose(oracle);assert(compared==3);
 assert(liblrhsmm_rs_gaussian_release_f64(&source)==0);assert(liblrhsmm_rs_prepared_gaussian_release_f64(&prepared)==0);
}
int main(void) {check_f32();check_f64();puts("C prepared Gaussian: all20 exports and166 original C cache/density values passed");return 0;}
