/* Actual C caller for complete stream statistics and transactional maximization. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include "liblrhsmm_rs.h"
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

static void exercise_f32(void) {
 LiblrhsmmRsStreamF32 *stream=NULL;
 LiblrhsmmRsStreamStatisticsF32 *owner=NULL,*created=NULL,*copy=NULL,*matched=NULL;
 LiblrhsmmRsGaussianStatisticsF32 *a=NULL,*b=NULL,*child=NULL;
 assert(liblrhsmm_rs_stream_new_f32(2,1,1,&stream)==0);
 assert(liblrhsmm_rs_stream_statistics_from_stream_f32(stream,&matched)==0);
 assert(liblrhsmm_rs_gaussian_statistics_new_f32(1,1,&a)==0);
 assert(liblrhsmm_rs_gaussian_statistics_new_f32(2,3,&b)==0);
 float sum=-9,output=99;
 assert(liblrhsmm_rs_gaussian_statistics_set_field_f32(a,0,0,&sum,1)==0);
 const LiblrhsmmRsGaussianStatisticsF32 *children[]={a,b,a};
 assert(liblrhsmm_rs_stream_statistics_create_f32(children,3,&owner)==0);
 assert(liblrhsmm_rs_stream_statistics_create_f32(children,3,&created)==0);
 assert(liblrhsmm_rs_stream_statistics_clone_f32(owner,&copy)==0);
 uintptr_t count=99;
 assert(liblrhsmm_rs_stream_statistics_count_f32(owner,&count)==0 && count==3);
 const LiblrhsmmRsGaussianStatisticsF32 *bad[]={a,NULL};
 assert(liblrhsmm_rs_stream_statistics_replace_f32(owner,bad,2)==1);
 assert(liblrhsmm_rs_stream_statistics_get_emission_f32(owner,0,&child)==0);
 assert(liblrhsmm_rs_gaussian_statistics_copy_field_f32(child,0,0,&output,1)==0 && output==-9);
 assert(liblrhsmm_rs_gaussian_statistics_clear_f32(child)==0);
 assert(liblrhsmm_rs_gaussian_statistics_release_f32(&child)==0);
 assert(liblrhsmm_rs_stream_statistics_set_emission_f32(owner,0,b)==0);
 assert(liblrhsmm_rs_stream_statistics_set_emission_f32(owner,3,a)==2);
 assert(liblrhsmm_rs_stream_statistics_get_emission_f32(owner,3,&child)==2 && child==NULL);
 assert(liblrhsmm_rs_stream_statistics_replace_f32(owner,children,3)==0);
 assert(liblrhsmm_rs_stream_statistics_clear_f32(owner)==0);
 assert(liblrhsmm_rs_stream_statistics_get_emission_f32(owner,0,&child)==0);
 assert(liblrhsmm_rs_gaussian_statistics_copy_field_f32(child,0,0,&output,1)==0 && output==0);
 assert(liblrhsmm_rs_gaussian_statistics_release_f32(&child)==0);
 assert(liblrhsmm_rs_gaussian_statistics_release_f32(&a)==0);
 assert(liblrhsmm_rs_gaussian_statistics_release_f32(&b)==0);
 assert(liblrhsmm_rs_stream_release_f32(&stream)==0);
 assert(liblrhsmm_rs_stream_statistics_count_f32(matched,&count)==0 && count==2);
 assert(liblrhsmm_rs_stream_statistics_get_emission_f32(copy,0,&child)==0);
 assert(liblrhsmm_rs_gaussian_statistics_copy_field_f32(child,0,0,&output,1)==0 && output==-9);
 assert(liblrhsmm_rs_gaussian_statistics_release_f32(&child)==0);
 assert(liblrhsmm_rs_stream_statistics_release_f32(&owner)==0);
 assert(liblrhsmm_rs_stream_statistics_release_f32(&created)==0);
 assert(liblrhsmm_rs_stream_statistics_release_f32(&copy)==0);
 assert(liblrhsmm_rs_stream_statistics_release_f32(&matched)==0);
 assert(liblrhsmm_rs_stream_statistics_release_f32(&owner)==0);
 assert(liblrhsmm_rs_stream_statistics_create_f32(NULL,0,&owner)==0);
 assert(liblrhsmm_rs_stream_new_f32(0,0,0,&stream)==0);
 LiblrhsmmRsUpdateOptionsF32 empty_options;
 assert(liblrhsmm_rs_update_options_default_f32(&empty_options)==0);
 empty_options.emission_variance_floor=-1;
 assert(liblrhsmm_rs_stream_update_f32(stream,owner,&empty_options)==3);
 empty_options.emission_variance_floor=0;
 assert(liblrhsmm_rs_stream_update_f32(stream,owner,&empty_options)==0);
 assert(liblrhsmm_rs_stream_release_f32(&stream)==0);
 assert(liblrhsmm_rs_stream_statistics_release_f32(&owner)==0);
 assert(liblrhsmm_rs_stream_statistics_release_f32(NULL)==1);
 LiblrhsmmRsGaussianF32 *source=NULL;LiblrhsmmRsGaussianStatisticsF32 *statistics=NULL;
 FILE *oracle=fopen("tests/fixtures/c-updates.txt","r");assert(oracle);char line[2048];unsigned compared=0;
 while(fgets(line,sizeof line,oracle)) {
  if(strncmp(line,"mixture ",8)!=0)continue;
  size_t scenario,component;double expected[5];
  assert(sscanf(line,"mixture %zu %zu %lf %lf %lf %lf %lf",&scenario,&component,&expected[0],&expected[1],&expected[2],&expected[3],&expected[4])==7);
  size_t count=scenario==0?1:2;float weights[2]={0.5,0.5},means[4]={-7,-7,-7,-7},vars[4]={8,8,8,8},floors[4]={0,0.75,0,0.75};
  float sums[4]={-3.5,6,-3.5,6},squares[4]={9.25,12,9.25,12},occupancies[2]={2,3};
  if(scenario==2)occupancies[1]=(float)1e-8;
  LiblrhsmmRsGaussianParametersF32 model={count,2,weights,means,vars,floors};
  LiblrhsmmRsGaussianStatisticsValuesF32 values={count,2,scenario==3?(float)1e-8:4,sums,squares,occupancies};
  assert(liblrhsmm_rs_gaussian_create_f32(&model,&source)==0);assert(liblrhsmm_rs_gaussian_statistics_create_f32(&values,&statistics)==0);
  LiblrhsmmRsUpdateOptionsF32 options;assert(liblrhsmm_rs_update_options_default_f32(&options)==0);
  const LiblrhsmmRsGaussianF32 *models[]={source};const LiblrhsmmRsGaussianStatisticsF32 *accumulators[]={statistics};
  assert(liblrhsmm_rs_stream_create_f32(1,models,1,&stream)==0);
  assert(liblrhsmm_rs_stream_statistics_create_f32(accumulators,1,&owner)==0);
  assert(liblrhsmm_rs_stream_update_f32(stream,owner,&options)==0);
  assert(liblrhsmm_rs_gaussian_release_f32(&source)==0);
  assert(liblrhsmm_rs_stream_get_emission_f32(stream,0,&source)==0);
  float weight,m[2],v[2];assert(liblrhsmm_rs_gaussian_copy_field_f32(source,0,component,&weight,1)==0);assert(liblrhsmm_rs_gaussian_copy_field_f32(source,1,2*component,m,2)==0);assert(liblrhsmm_rs_gaussian_copy_field_f32(source,2,2*component,v,2)==0);
  float actual[5]={weight,m[0],v[0],m[1],v[1]};
  for(unsigned index=0;index<5;index++){double target=(float)expected[index];assert(fabs((double)actual[index]-target)<=2e-14*fmax(1,fabs(target)));compared++;}
  options.duration_mode=99;assert(liblrhsmm_rs_stream_update_f32(stream,owner,&options)==3);
  assert(liblrhsmm_rs_gaussian_statistics_release_f32(&statistics)==0);assert(liblrhsmm_rs_gaussian_release_f32(&source)==0);
  assert(liblrhsmm_rs_stream_statistics_release_f32(&owner)==0);assert(liblrhsmm_rs_stream_release_f32(&stream)==0);
 }
 fclose(oracle);assert(compared==35);
}

static void exercise_f64(void) {
 LiblrhsmmRsStreamF64 *stream=NULL;
 LiblrhsmmRsStreamStatisticsF64 *owner=NULL,*created=NULL,*copy=NULL,*matched=NULL;
 LiblrhsmmRsGaussianStatisticsF64 *a=NULL,*b=NULL,*child=NULL;
 assert(liblrhsmm_rs_stream_new_f64(2,1,1,&stream)==0);
 assert(liblrhsmm_rs_stream_statistics_from_stream_f64(stream,&matched)==0);
 assert(liblrhsmm_rs_gaussian_statistics_new_f64(1,1,&a)==0);
 assert(liblrhsmm_rs_gaussian_statistics_new_f64(2,3,&b)==0);
 double sum=-9,output=99;
 assert(liblrhsmm_rs_gaussian_statistics_set_field_f64(a,0,0,&sum,1)==0);
 const LiblrhsmmRsGaussianStatisticsF64 *children[]={a,b,a};
 assert(liblrhsmm_rs_stream_statistics_create_f64(children,3,&owner)==0);
 assert(liblrhsmm_rs_stream_statistics_create_f64(children,3,&created)==0);
 assert(liblrhsmm_rs_stream_statistics_clone_f64(owner,&copy)==0);
 uintptr_t count=99;
 assert(liblrhsmm_rs_stream_statistics_count_f64(owner,&count)==0 && count==3);
 const LiblrhsmmRsGaussianStatisticsF64 *bad[]={a,NULL};
 assert(liblrhsmm_rs_stream_statistics_replace_f64(owner,bad,2)==1);
 assert(liblrhsmm_rs_stream_statistics_get_emission_f64(owner,0,&child)==0);
 assert(liblrhsmm_rs_gaussian_statistics_copy_field_f64(child,0,0,&output,1)==0 && output==-9);
 assert(liblrhsmm_rs_gaussian_statistics_clear_f64(child)==0);
 assert(liblrhsmm_rs_gaussian_statistics_release_f64(&child)==0);
 assert(liblrhsmm_rs_stream_statistics_set_emission_f64(owner,0,b)==0);
 assert(liblrhsmm_rs_stream_statistics_set_emission_f64(owner,3,a)==2);
 assert(liblrhsmm_rs_stream_statistics_get_emission_f64(owner,3,&child)==2 && child==NULL);
 assert(liblrhsmm_rs_stream_statistics_replace_f64(owner,children,3)==0);
 assert(liblrhsmm_rs_stream_statistics_clear_f64(owner)==0);
 assert(liblrhsmm_rs_stream_statistics_get_emission_f64(owner,0,&child)==0);
 assert(liblrhsmm_rs_gaussian_statistics_copy_field_f64(child,0,0,&output,1)==0 && output==0);
 assert(liblrhsmm_rs_gaussian_statistics_release_f64(&child)==0);
 assert(liblrhsmm_rs_gaussian_statistics_release_f64(&a)==0);
 assert(liblrhsmm_rs_gaussian_statistics_release_f64(&b)==0);
 assert(liblrhsmm_rs_stream_release_f64(&stream)==0);
 assert(liblrhsmm_rs_stream_statistics_count_f64(matched,&count)==0 && count==2);
 assert(liblrhsmm_rs_stream_statistics_get_emission_f64(copy,0,&child)==0);
 assert(liblrhsmm_rs_gaussian_statistics_copy_field_f64(child,0,0,&output,1)==0 && output==-9);
 assert(liblrhsmm_rs_gaussian_statistics_release_f64(&child)==0);
 assert(liblrhsmm_rs_stream_statistics_release_f64(&owner)==0);
 assert(liblrhsmm_rs_stream_statistics_release_f64(&created)==0);
 assert(liblrhsmm_rs_stream_statistics_release_f64(&copy)==0);
 assert(liblrhsmm_rs_stream_statistics_release_f64(&matched)==0);
 assert(liblrhsmm_rs_stream_statistics_release_f64(&owner)==0);
 assert(liblrhsmm_rs_stream_statistics_create_f64(NULL,0,&owner)==0);
 assert(liblrhsmm_rs_stream_new_f64(0,0,0,&stream)==0);
 LiblrhsmmRsUpdateOptionsF64 empty_options;
 assert(liblrhsmm_rs_update_options_default_f64(&empty_options)==0);
 empty_options.emission_variance_floor=-1;
 assert(liblrhsmm_rs_stream_update_f64(stream,owner,&empty_options)==3);
 empty_options.emission_variance_floor=0;
 assert(liblrhsmm_rs_stream_update_f64(stream,owner,&empty_options)==0);
 assert(liblrhsmm_rs_stream_release_f64(&stream)==0);
 assert(liblrhsmm_rs_stream_statistics_release_f64(&owner)==0);
 assert(liblrhsmm_rs_stream_statistics_release_f64(NULL)==1);
 LiblrhsmmRsGaussianF64 *source=NULL;LiblrhsmmRsGaussianStatisticsF64 *statistics=NULL;
 FILE *oracle=fopen("tests/fixtures/c-double-update.txt","r");assert(oracle);char line[2048];unsigned compared=0;
 while(fgets(line,sizeof line,oracle)) {
  if(strncmp(line,"mixture ",8)!=0)continue;
  size_t scenario,component;double expected[5];
  assert(sscanf(line,"mixture %zu %zu %lf %lf %lf %lf %lf",&scenario,&component,&expected[0],&expected[1],&expected[2],&expected[3],&expected[4])==7);
  size_t count=scenario==0?1:2;double weights[2]={0.5,0.5},means[4]={-7,-7,-7,-7},vars[4]={8,8,8,8},floors[4]={0,0.75,0,0.75};
  double sums[4]={-3.5,6,-3.5,6},squares[4]={9.25,12,9.25,12},occupancies[2]={2,3};
  if(scenario==2)occupancies[1]=(double)1e-8;
  LiblrhsmmRsGaussianParametersF64 model={count,2,weights,means,vars,floors};
  LiblrhsmmRsGaussianStatisticsValuesF64 values={count,2,scenario==3?(double)1e-8:4,sums,squares,occupancies};
  assert(liblrhsmm_rs_gaussian_create_f64(&model,&source)==0);assert(liblrhsmm_rs_gaussian_statistics_create_f64(&values,&statistics)==0);
  LiblrhsmmRsUpdateOptionsF64 options;assert(liblrhsmm_rs_update_options_default_f64(&options)==0);
  const LiblrhsmmRsGaussianF64 *models[]={source};const LiblrhsmmRsGaussianStatisticsF64 *accumulators[]={statistics};
  assert(liblrhsmm_rs_stream_create_f64(1,models,1,&stream)==0);
  assert(liblrhsmm_rs_stream_statistics_create_f64(accumulators,1,&owner)==0);
  assert(liblrhsmm_rs_stream_update_f64(stream,owner,&options)==0);
  assert(liblrhsmm_rs_gaussian_release_f64(&source)==0);
  assert(liblrhsmm_rs_stream_get_emission_f64(stream,0,&source)==0);
  double weight,m[2],v[2];assert(liblrhsmm_rs_gaussian_copy_field_f64(source,0,component,&weight,1)==0);assert(liblrhsmm_rs_gaussian_copy_field_f64(source,1,2*component,m,2)==0);assert(liblrhsmm_rs_gaussian_copy_field_f64(source,2,2*component,v,2)==0);
  double actual[5]={weight,m[0],v[0],m[1],v[1]};
  for(unsigned index=0;index<5;index++){double target=(double)expected[index];assert(fabs((double)actual[index]-target)<=2e-14*fmax(1,fabs(target)));compared++;}
  options.duration_mode=99;assert(liblrhsmm_rs_stream_update_f64(stream,owner,&options)==3);
  assert(liblrhsmm_rs_gaussian_statistics_release_f64(&statistics)==0);assert(liblrhsmm_rs_gaussian_release_f64(&source)==0);
  assert(liblrhsmm_rs_stream_statistics_release_f64(&owner)==0);assert(liblrhsmm_rs_stream_release_f64(&stream)==0);
 }
 fclose(oracle);assert(compared==35);
}
int main(void) {exercise_f32();exercise_f64();puts("stream statistics: all20 exports and70 original C update values passed");return 0;}
