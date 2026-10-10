#ifdef NDEBUG
#undef NDEBUG
#endif
#include "liblrhsmm_rs.h"
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

static void check_f32(void) {
 LiblrhsmmRsGaussianStatisticsF32 *statistics=NULL,*created=NULL,*copy=NULL,*matched=NULL; LiblrhsmmRsGaussianF32 *source=NULL;
 assert(liblrhsmm_rs_gaussian_statistics_new_f32(2,2,&statistics)==0);assert(liblrhsmm_rs_gaussian_new_f32(2,2,&source)==0);
 assert(liblrhsmm_rs_gaussian_statistics_from_gaussian_f32(source,&matched)==0);assert(liblrhsmm_rs_gaussian_release_f32(&source)==0);
 float sums[4]={-1,2,3,4},squares[4]={1,4,9,16},occupancies[2]={1,2};
 _Alignas(8) LiblrhsmmRsGaussianStatisticsValuesF32 input={2,2,3,sums,squares,occupancies};
 assert(liblrhsmm_rs_gaussian_statistics_replace_f32(statistics,&input)==0);assert(liblrhsmm_rs_gaussian_statistics_create_f32(&input,&created)==0);assert(liblrhsmm_rs_gaussian_statistics_clone_f32(statistics,&copy)==0);
 _Alignas(8) LiblrhsmmRsGaussianStatisticsLayoutF32 metadata={0};assert(liblrhsmm_rs_gaussian_statistics_layout_f32(statistics,&metadata)==0);
 assert(metadata.mixtures==2 && metadata.dimensions==2 && metadata.state_occupancy==3);
 float changed=-9,output=99;assert(liblrhsmm_rs_gaussian_statistics_set_field_f32(statistics,0,0,&changed,1)==0);
 assert(liblrhsmm_rs_gaussian_statistics_copy_field_f32(copy,0,0,&output,1)==0 && output==-1);
 assert(liblrhsmm_rs_gaussian_statistics_set_occupancy_f32(statistics,5)==0);assert(liblrhsmm_rs_gaussian_statistics_layout_f32(statistics,&metadata)==0 && metadata.state_occupancy==5);
 assert(liblrhsmm_rs_gaussian_statistics_copy_field_f32(statistics,0,4,NULL,0)==0);
 assert(liblrhsmm_rs_gaussian_statistics_set_field_f32(statistics,0,4,&changed,1)==2);
 assert(liblrhsmm_rs_gaussian_statistics_clear_f32(statistics)==0);assert(liblrhsmm_rs_gaussian_statistics_copy_field_f32(statistics,0,0,&output,1)==0 && output==0);
 assert(liblrhsmm_rs_gaussian_statistics_layout_f32(statistics,&metadata)==0 && metadata.state_occupancy==0);
 assert(liblrhsmm_rs_gaussian_statistics_release_f32(&statistics)==0);assert(liblrhsmm_rs_gaussian_statistics_release_f32(&created)==0);assert(liblrhsmm_rs_gaussian_statistics_release_f32(&copy)==0);assert(liblrhsmm_rs_gaussian_statistics_release_f32(&matched)==0);
 assert(liblrhsmm_rs_gaussian_statistics_release_f32(&statistics)==0);assert(liblrhsmm_rs_gaussian_statistics_new_f32(0,1,&statistics)==3 && statistics==NULL);
 FILE *oracle=fopen("tests/fixtures/c-updates.txt","r");assert(oracle);char line[2048];unsigned compared=0;
 while(fgets(line,sizeof line,oracle)) {
  if(strncmp(line,"mixture ",8)!=0)continue;
  size_t scenario,component;_Alignas(8) double expected[5];
  assert(sscanf(line,"mixture %zu %zu %lf %lf %lf %lf %lf",&scenario,&component,&expected[0],&expected[1],&expected[2],&expected[3],&expected[4])==7);
  size_t count=scenario==0?1:2;float weights[2]={0.5,0.5},means[4]={-7,-7,-7,-7},vars[4]={8,8,8,8},floors[4]={0,0.75,0,0.75};
  float sums[4]={-3.5,6,-3.5,6},squares[4]={9.25,12,9.25,12},occupancies[2]={2,3};
  if(scenario==2)occupancies[1]=(float)1e-8;
  _Alignas(8) LiblrhsmmRsGaussianParametersF32 model={count,2,weights,means,vars,floors};
  _Alignas(8) LiblrhsmmRsGaussianStatisticsValuesF32 values={count,2,scenario==3?(float)1e-8:4,sums,squares,occupancies};
  assert(liblrhsmm_rs_gaussian_create_f32(&model,&source)==0);assert(liblrhsmm_rs_gaussian_statistics_create_f32(&values,&statistics)==0);
  _Alignas(8) LiblrhsmmRsUpdateOptionsF32 options;assert(liblrhsmm_rs_update_options_default_f32(&options)==0);
  uint32_t updated=99;assert(liblrhsmm_rs_gaussian_update_f32(source,statistics,&options,&updated)==0 && updated==(scenario!=3));
  float weight,m[2],v[2];assert(liblrhsmm_rs_gaussian_copy_field_f32(source,0,component,&weight,1)==0);assert(liblrhsmm_rs_gaussian_copy_field_f32(source,1,2*component,m,2)==0);assert(liblrhsmm_rs_gaussian_copy_field_f32(source,2,2*component,v,2)==0);
  float actual[5]={weight,m[0],v[0],m[1],v[1]};
  for(unsigned index=0;index<5;index++){_Alignas(8) double target=(float)expected[index];assert(fabs((double)actual[index]-target)<=2e-14*fmax(1,fabs(target)));compared++;}
  options.duration_mode=99;updated=99;assert(liblrhsmm_rs_gaussian_update_f32(source,statistics,&options,&updated)==3 && updated==99);
  assert(liblrhsmm_rs_gaussian_statistics_release_f32(&statistics)==0);assert(liblrhsmm_rs_gaussian_release_f32(&source)==0);
 }
 fclose(oracle);assert(compared==35);
}

static void check_f64(void) {
 LiblrhsmmRsGaussianStatisticsF64 *statistics=NULL,*created=NULL,*copy=NULL,*matched=NULL; LiblrhsmmRsGaussianF64 *source=NULL;
 assert(liblrhsmm_rs_gaussian_statistics_new_f64(2,2,&statistics)==0);assert(liblrhsmm_rs_gaussian_new_f64(2,2,&source)==0);
 assert(liblrhsmm_rs_gaussian_statistics_from_gaussian_f64(source,&matched)==0);assert(liblrhsmm_rs_gaussian_release_f64(&source)==0);
 _Alignas(8) double sums[4]={-1,2,3,4},squares[4]={1,4,9,16},occupancies[2]={1,2};
 _Alignas(8) LiblrhsmmRsGaussianStatisticsValuesF64 input={2,2,3,sums,squares,occupancies};
 assert(liblrhsmm_rs_gaussian_statistics_replace_f64(statistics,&input)==0);assert(liblrhsmm_rs_gaussian_statistics_create_f64(&input,&created)==0);assert(liblrhsmm_rs_gaussian_statistics_clone_f64(statistics,&copy)==0);
 _Alignas(8) LiblrhsmmRsGaussianStatisticsLayoutF64 metadata={0};assert(liblrhsmm_rs_gaussian_statistics_layout_f64(statistics,&metadata)==0);
 assert(metadata.mixtures==2 && metadata.dimensions==2 && metadata.state_occupancy==3);
 _Alignas(8) double changed=-9,output=99;assert(liblrhsmm_rs_gaussian_statistics_set_field_f64(statistics,0,0,&changed,1)==0);
 assert(liblrhsmm_rs_gaussian_statistics_copy_field_f64(copy,0,0,&output,1)==0 && output==-1);
 assert(liblrhsmm_rs_gaussian_statistics_set_occupancy_f64(statistics,5)==0);assert(liblrhsmm_rs_gaussian_statistics_layout_f64(statistics,&metadata)==0 && metadata.state_occupancy==5);
 assert(liblrhsmm_rs_gaussian_statistics_copy_field_f64(statistics,0,4,NULL,0)==0);
 assert(liblrhsmm_rs_gaussian_statistics_set_field_f64(statistics,0,4,&changed,1)==2);
 assert(liblrhsmm_rs_gaussian_statistics_clear_f64(statistics)==0);assert(liblrhsmm_rs_gaussian_statistics_copy_field_f64(statistics,0,0,&output,1)==0 && output==0);
 assert(liblrhsmm_rs_gaussian_statistics_layout_f64(statistics,&metadata)==0 && metadata.state_occupancy==0);
 assert(liblrhsmm_rs_gaussian_statistics_release_f64(&statistics)==0);assert(liblrhsmm_rs_gaussian_statistics_release_f64(&created)==0);assert(liblrhsmm_rs_gaussian_statistics_release_f64(&copy)==0);assert(liblrhsmm_rs_gaussian_statistics_release_f64(&matched)==0);
 assert(liblrhsmm_rs_gaussian_statistics_release_f64(&statistics)==0);assert(liblrhsmm_rs_gaussian_statistics_new_f64(0,1,&statistics)==3 && statistics==NULL);
 FILE *oracle=fopen("tests/fixtures/c-double-update.txt","r");assert(oracle);char line[2048];unsigned compared=0;
 while(fgets(line,sizeof line,oracle)) {
  if(strncmp(line,"mixture ",8)!=0)continue;
  size_t scenario,component;_Alignas(8) double expected[5];
  assert(sscanf(line,"mixture %zu %zu %lf %lf %lf %lf %lf",&scenario,&component,&expected[0],&expected[1],&expected[2],&expected[3],&expected[4])==7);
  size_t count=scenario==0?1:2;_Alignas(8) double weights[2]={0.5,0.5},means[4]={-7,-7,-7,-7},vars[4]={8,8,8,8},floors[4]={0,0.75,0,0.75};
  _Alignas(8) double sums[4]={-3.5,6,-3.5,6},squares[4]={9.25,12,9.25,12},occupancies[2]={2,3};
  if(scenario==2)occupancies[1]=(double)1e-8;
  _Alignas(8) LiblrhsmmRsGaussianParametersF64 model={count,2,weights,means,vars,floors};
  _Alignas(8) LiblrhsmmRsGaussianStatisticsValuesF64 values={count,2,scenario==3?(double)1e-8:4,sums,squares,occupancies};
  assert(liblrhsmm_rs_gaussian_create_f64(&model,&source)==0);assert(liblrhsmm_rs_gaussian_statistics_create_f64(&values,&statistics)==0);
  _Alignas(8) LiblrhsmmRsUpdateOptionsF64 options;assert(liblrhsmm_rs_update_options_default_f64(&options)==0);
  uint32_t updated=99;assert(liblrhsmm_rs_gaussian_update_f64(source,statistics,&options,&updated)==0 && updated==(scenario!=3));
  _Alignas(8) double weight,m[2],v[2];assert(liblrhsmm_rs_gaussian_copy_field_f64(source,0,component,&weight,1)==0);assert(liblrhsmm_rs_gaussian_copy_field_f64(source,1,2*component,m,2)==0);assert(liblrhsmm_rs_gaussian_copy_field_f64(source,2,2*component,v,2)==0);
  _Alignas(8) double actual[5]={weight,m[0],v[0],m[1],v[1]};
  for(unsigned index=0;index<5;index++){_Alignas(8) double target=(double)expected[index];assert(fabs((double)actual[index]-target)<=2e-14*fmax(1,fabs(target)));compared++;}
  options.duration_mode=99;updated=99;assert(liblrhsmm_rs_gaussian_update_f64(source,statistics,&options,&updated)==3 && updated==99);
  assert(liblrhsmm_rs_gaussian_statistics_release_f64(&statistics)==0);assert(liblrhsmm_rs_gaussian_release_f64(&source)==0);
 }
 fclose(oracle);assert(compared==35);
}
int main(void) {check_f32();check_f64();puts("C Gaussian statistics: all24 exports and70 original C update values passed");return 0;}
