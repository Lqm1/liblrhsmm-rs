#ifdef NDEBUG
#undef NDEBUG
#endif
#include "liblrhsmm_rs.h"
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

static void check_f32(void) {
 LiblrhsmmRsGaussianF32 *owner=NULL,*copy=NULL,*created=NULL,*combined=NULL,*converted=NULL;
 assert(liblrhsmm_rs_gaussian_new_f32(1,2,&owner)==0);
 LiblrhsmmRsGaussianLayout layout={0,0};
 assert(liblrhsmm_rs_gaussian_layout_f32(owner,&layout)==0 && layout.mixtures==1 && layout.dimensions==2);
 float weights[]={1},means[]={1.25,-2.5},variances[]={2.5,0.75},floors[]={0.1,0.2};
 LiblrhsmmRsGaussianParametersF32 descriptor={1,2,weights,means,variances,floors};
 assert(liblrhsmm_rs_gaussian_replace_f32(owner,&descriptor)==0);
 assert(liblrhsmm_rs_gaussian_create_f32(&descriptor,&created)==0);
 assert(liblrhsmm_rs_gaussian_clone_f32(owner,&copy)==0);
 assert(liblrhsmm_rs_gaussian_increase_mixtures_f32(owner,5,(float)0.2)==0);
 float output[4][10]={0};
 for(unsigned selector=0;selector<4;selector++) {
  size_t count=selector==0?5:10;
  assert(liblrhsmm_rs_gaussian_copy_field_f32(owner,selector,0,output[selector],count)==0);
  assert(liblrhsmm_rs_gaussian_copy_field_f32(owner,selector,count,NULL,0)==0);
 }
 FILE *oracle=fopen("tests/fixtures/c-precision-float.txt","r"); assert(oracle);
 char line[2048];unsigned compared=0;
 while(fgets(line,sizeof line,oracle)) {
  if(strncmp(line,"mixture ",8)!=0) continue;
  size_t index,secondary;double expected[7];
  assert(sscanf(line,"mixture %zu %zu %lf %lf %lf %lf %lf %lf %lf",&index,&secondary,&expected[0],&expected[1],&expected[2],&expected[3],&expected[4],&expected[5],&expected[6])==9);
  float actual[7]={output[0][index],output[1][2*index],output[2][2*index],output[3][2*index],output[1][2*index+1],output[2][2*index+1],output[3][2*index+1]};
  for(unsigned i=0;i<7;i++) { double target=(float)expected[i];assert(fabs((double)actual[i]-target)<=2e-14*fmax(1,fabs(target)));compared++; }
 }
 fclose(oracle);assert(compared==35);
 float changed=-9,scalar=0;
 assert(liblrhsmm_rs_gaussian_set_field_f32(copy,1,0,&changed,1)==0);
 assert(liblrhsmm_rs_gaussian_copy_field_f32(created,1,0,&scalar,1)==0 && scalar==1.25);
 assert(liblrhsmm_rs_gaussian_set_field_f32(copy,1,99,&changed,1)==2);
 assert(liblrhsmm_rs_gaussian_copy_field_f32(copy,1,0,&scalar,1)==0 && scalar==-9);
 const LiblrhsmmRsGaussianF32 *handles[]={created,created};
 assert(liblrhsmm_rs_gaussian_combine_f32(handles,2,&combined)==0);
 assert(liblrhsmm_rs_gaussian_layout_f32(combined,&layout)==0 && layout.dimensions==4);
 LiblrhsmmRsDurationF32 *duration=NULL;
 LiblrhsmmRsDurationParametersF32 duration_parameters={2,3,-1,-1,-1,0.4};
 assert(liblrhsmm_rs_duration_create_f32(&duration_parameters,&duration)==0);
 const LiblrhsmmRsDurationF32 *durations[]={duration};
 assert(liblrhsmm_rs_gaussian_from_durations_f32(durations,1,&converted)==0);
 assert(liblrhsmm_rs_gaussian_copy_field_f32(converted,3,0,&scalar,1)==0 && scalar==0);
 assert(liblrhsmm_rs_duration_release_f32(&duration)==0);
 assert(liblrhsmm_rs_gaussian_release_f32(&owner)==0 && owner==NULL);
 assert(liblrhsmm_rs_gaussian_release_f32(&copy)==0 && copy==NULL);
 assert(liblrhsmm_rs_gaussian_release_f32(&created)==0 && created==NULL);
 assert(liblrhsmm_rs_gaussian_release_f32(&combined)==0 && combined==NULL);
 assert(liblrhsmm_rs_gaussian_release_f32(&converted)==0 && converted==NULL);
 assert(liblrhsmm_rs_gaussian_release_f32(&owner)==0);
}

static void check_f64(void) {
 LiblrhsmmRsGaussianF64 *owner=NULL,*copy=NULL,*created=NULL,*combined=NULL,*converted=NULL;
 assert(liblrhsmm_rs_gaussian_new_f64(1,2,&owner)==0);
 LiblrhsmmRsGaussianLayout layout={0,0};
 assert(liblrhsmm_rs_gaussian_layout_f64(owner,&layout)==0 && layout.mixtures==1 && layout.dimensions==2);
 double weights[]={1},means[]={1.25,-2.5},variances[]={2.5,0.75},floors[]={0.1,0.2};
 LiblrhsmmRsGaussianParametersF64 descriptor={1,2,weights,means,variances,floors};
 assert(liblrhsmm_rs_gaussian_replace_f64(owner,&descriptor)==0);
 assert(liblrhsmm_rs_gaussian_create_f64(&descriptor,&created)==0);
 assert(liblrhsmm_rs_gaussian_clone_f64(owner,&copy)==0);
 assert(liblrhsmm_rs_gaussian_increase_mixtures_f64(owner,5,(double)0.2)==0);
 double output[4][10]={0};
 for(unsigned selector=0;selector<4;selector++) {
  size_t count=selector==0?5:10;
  assert(liblrhsmm_rs_gaussian_copy_field_f64(owner,selector,0,output[selector],count)==0);
  assert(liblrhsmm_rs_gaussian_copy_field_f64(owner,selector,count,NULL,0)==0);
 }
 FILE *oracle=fopen("tests/fixtures/c-precision-double.txt","r"); assert(oracle);
 char line[2048];unsigned compared=0;
 while(fgets(line,sizeof line,oracle)) {
  if(strncmp(line,"mixture ",8)!=0) continue;
  size_t index,secondary;double expected[7];
  assert(sscanf(line,"mixture %zu %zu %lf %lf %lf %lf %lf %lf %lf",&index,&secondary,&expected[0],&expected[1],&expected[2],&expected[3],&expected[4],&expected[5],&expected[6])==9);
  double actual[7]={output[0][index],output[1][2*index],output[2][2*index],output[3][2*index],output[1][2*index+1],output[2][2*index+1],output[3][2*index+1]};
  for(unsigned i=0;i<7;i++) { double target=(double)expected[i];assert(fabs((double)actual[i]-target)<=2e-14*fmax(1,fabs(target)));compared++; }
 }
 fclose(oracle);assert(compared==35);
 double changed=-9,scalar=0;
 assert(liblrhsmm_rs_gaussian_set_field_f64(copy,1,0,&changed,1)==0);
 assert(liblrhsmm_rs_gaussian_copy_field_f64(created,1,0,&scalar,1)==0 && scalar==1.25);
 assert(liblrhsmm_rs_gaussian_set_field_f64(copy,1,99,&changed,1)==2);
 assert(liblrhsmm_rs_gaussian_copy_field_f64(copy,1,0,&scalar,1)==0 && scalar==-9);
 const LiblrhsmmRsGaussianF64 *handles[]={created,created};
 assert(liblrhsmm_rs_gaussian_combine_f64(handles,2,&combined)==0);
 assert(liblrhsmm_rs_gaussian_layout_f64(combined,&layout)==0 && layout.dimensions==4);
 LiblrhsmmRsDurationF64 *duration=NULL;
 LiblrhsmmRsDurationParametersF64 duration_parameters={2,3,-1,-1,-1,0.4};
 assert(liblrhsmm_rs_duration_create_f64(&duration_parameters,&duration)==0);
 const LiblrhsmmRsDurationF64 *durations[]={duration};
 assert(liblrhsmm_rs_gaussian_from_durations_f64(durations,1,&converted)==0);
 assert(liblrhsmm_rs_gaussian_copy_field_f64(converted,3,0,&scalar,1)==0 && scalar==0);
 assert(liblrhsmm_rs_duration_release_f64(&duration)==0);
 assert(liblrhsmm_rs_gaussian_release_f64(&owner)==0 && owner==NULL);
 assert(liblrhsmm_rs_gaussian_release_f64(&copy)==0 && copy==NULL);
 assert(liblrhsmm_rs_gaussian_release_f64(&created)==0 && created==NULL);
 assert(liblrhsmm_rs_gaussian_release_f64(&combined)==0 && combined==NULL);
 assert(liblrhsmm_rs_gaussian_release_f64(&converted)==0 && converted==NULL);
 assert(liblrhsmm_rs_gaussian_release_f64(&owner)==0);
}
int main(void) { check_f32(); check_f64(); puts("C Gaussian: all22 exports and70 original C parameter values passed"); return 0; }
