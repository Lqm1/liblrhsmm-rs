/* Actual C caller for byte ownership, borrowed I/O and original observation codecs. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <stdio.h>
#include <string.h>
#include "liblrhsmm_rs.h"
struct Input { unsigned char bytes[300000]; uintptr_t length,position,calls; uint32_t status; int interrupt,oversized; };
struct Output { unsigned char bytes[300000]; uintptr_t length,calls,flushes; uint32_t status; int interrupt,oversized,zero; };
static uint32_t read_callback(void *context,uint8_t *output,uintptr_t capacity,uintptr_t *count) {
    struct Input *input=context; input->calls++;
    if(input->interrupt) {input->interrupt=0;return LIBLRHSMM_RS_IO_INTERRUPTED;}
    if(input->status) return input->status;
    if(input->oversized) {*count=capacity+1;return 0;}
    *count=capacity && input->position<input->length?1:0;
    if(*count) *output=input->bytes[input->position++];
    return 0;
}
static uint32_t write_callback(void *context,const uint8_t *input,uintptr_t capacity,uintptr_t *count) {
    struct Output *output=context; output->calls++;
    if(output->interrupt) {output->interrupt=0;return LIBLRHSMM_RS_IO_INTERRUPTED;}
    if(output->status) return output->status;
    if(output->oversized) {*count=capacity+1;return 0;}
    *count=capacity && !output->zero?1:0;
    if(*count) {assert(output->length<300000);output->bytes[output->length++]=*input;}
    return 0;
}
static uint32_t flush_callback(void *context) {struct Output *output=context;output->flushes++;return output->status;}
static void check_bytes(void) {
    uint8_t input[]={1,2,3,4}, result[4]={0}; LiblrhsmmRsBytes *owner=NULL,*copy=NULL;
    assert(liblrhsmm_rs_bytes_create(input,4,&owner)==0); input[0]=99;
    uintptr_t length=0;assert(liblrhsmm_rs_bytes_length(owner,&length)==0 && length==4);
    assert(liblrhsmm_rs_bytes_copy(owner,0,result,4)==0 && result[0]==1);
    assert(liblrhsmm_rs_bytes_clone(owner,&copy)==0);
    assert(liblrhsmm_rs_bytes_release(&owner)==0 && owner==NULL);
    assert(liblrhsmm_rs_bytes_release(&owner)==0);
    assert(liblrhsmm_rs_bytes_copy(copy,4,NULL,0)==0);
    assert(liblrhsmm_rs_bytes_copy(copy,5,NULL,0)==2);
    result[0]=71;assert(liblrhsmm_rs_bytes_copy(copy,3,result,2)==2 && result[0]==71);
    assert(liblrhsmm_rs_bytes_copy(copy,UINTPTR_MAX,result,1)==2 && result[0]==71);
    assert(liblrhsmm_rs_bytes_copy(copy,0,NULL,1)==1);
    assert(liblrhsmm_rs_bytes_length(copy,NULL)==1 && liblrhsmm_rs_bytes_clone(copy,NULL)==1);
    assert(liblrhsmm_rs_bytes_release(&copy)==0);
    assert(liblrhsmm_rs_bytes_create(NULL,UINTPTR_MAX,&owner)==2 && owner==NULL);
    assert(liblrhsmm_rs_bytes_create(NULL,0,&owner)==0);
    assert(liblrhsmm_rs_bytes_length(owner,&length)==0 && length==0);
    assert(liblrhsmm_rs_bytes_copy(owner,0,NULL,0)==0);
    assert(liblrhsmm_rs_bytes_release(&owner)==0 && liblrhsmm_rs_bytes_release(NULL)==1);
    static struct Output output; memset(&output,0,sizeof(output));LiblrhsmmRsWriteStream stream={&output,write_callback,flush_callback};
    assert(liblrhsmm_rs_write_stream_flush(&stream)==0 && output.flushes==1);
    output.status=LIBLRHSMM_RS_IO_INTERRUPTED;assert(liblrhsmm_rs_write_stream_flush(&stream)==3);
    output.status=LIBLRHSMM_RS_IO_ERROR;assert(liblrhsmm_rs_write_stream_flush(&stream)==3);
    stream.flush=NULL;assert(liblrhsmm_rs_write_stream_flush(&stream)==3);
    assert(liblrhsmm_rs_write_stream_flush(NULL)==1);
}
#if defined(LIBLRHSMM_RS_SERIALIZATION)

static void check_f32(const char *path,uintptr_t budget) {
    static uint8_t fixture[150000], result[150000];
    FILE *file=fopen(path,"rb");assert(file);
    uintptr_t count=fread(fixture,1,sizeof(fixture),file);assert(feof(file));assert(fclose(file)==0 && count>0);
    LiblrhsmmRsDecodeLimits limits;assert(liblrhsmm_rs_decode_limits_default(&limits)==0 && limits.max_array_entries==16*1024*1024);
    LiblrhsmmRsObservationF32 *owner=NULL,*decoded=NULL;LiblrhsmmRsBytes *bytes=NULL;
    assert(liblrhsmm_rs_observation_read_bytes_f32(fixture,count,&limits,&owner)==0);
    assert(liblrhsmm_rs_observation_write_bytes_f32(owner,&bytes)==0);
    uintptr_t length=0;assert(liblrhsmm_rs_bytes_length(bytes,&length)==0 && length==count);
    assert(liblrhsmm_rs_bytes_copy(bytes,0,result,count)==0 && !memcmp(result,fixture,count));
    static struct Input input; memset(&input,0,sizeof(input));memcpy(input.bytes,fixture,count);input.length=count;input.interrupt=1;
    LiblrhsmmRsReadStream source={&input,read_callback};
    assert(liblrhsmm_rs_observation_read_stream_f32(&source,&limits,&decoded)==0 && input.position==count && input.calls>count);
    static struct Output output; memset(&output,0,sizeof(output));output.interrupt=1;LiblrhsmmRsWriteStream sink={&output,write_callback,flush_callback};
    assert(liblrhsmm_rs_observation_write_stream_f32(decoded,&sink)==0 && output.length==count && !memcmp(output.bytes,fixture,count));
    assert(output.flushes==0);assert(liblrhsmm_rs_write_stream_flush(&sink)==0 && output.flushes==1);
    assert(liblrhsmm_rs_observation_release_f32(&decoded)==0);
    memset(&input,0,sizeof(input));memcpy(input.bytes,fixture,count);memcpy(input.bytes+count,fixture,count);input.bytes[count*2]=42;input.length=count*2+1;input.interrupt=1;
    for(uintptr_t i=1;i<=2;i++) {
        assert(liblrhsmm_rs_observation_read_stream_prefix_f32(&source,&limits,&decoded)==0 && input.position==count*i);
        assert(liblrhsmm_rs_observation_release_f32(&decoded)==0);
    }
    assert(input.bytes[input.position]==42);
    for(uintptr_t end=0;end<count;end++) if(count<=512 || end<64 || end>=count-8 || end%1024==0) assert(liblrhsmm_rs_observation_read_bytes_f32(fixture,end,&limits,&decoded)==3 && decoded==NULL);
    fixture[count]=0;assert(liblrhsmm_rs_observation_read_bytes_f32(fixture,count+1,&limits,&decoded)==3 && decoded==NULL);
    LiblrhsmmRsDecodeLimits tight={budget-1};assert(liblrhsmm_rs_observation_read_bytes_f32(fixture,count,&tight,&decoded)==3);
    tight.max_array_entries=budget;assert(liblrhsmm_rs_observation_read_bytes_f32(fixture,count,&tight,&decoded)==0);assert(liblrhsmm_rs_observation_release_f32(&decoded)==0);
    input.position=0;input.status=LIBLRHSMM_RS_IO_ERROR;
    assert(liblrhsmm_rs_observation_read_stream_f32(&source,&limits,&decoded)==3 && decoded==NULL);
    input.status=0;input.oversized=1;assert(liblrhsmm_rs_observation_read_stream_f32(&source,&limits,&decoded)==3);
    source.read=NULL;assert(liblrhsmm_rs_observation_read_stream_f32(&source,&limits,&decoded)==3);
    source.read=read_callback;input.calls=0;
    assert(liblrhsmm_rs_observation_read_stream_f32(&source,&limits,NULL)==1 && input.calls==0);
    output.status=LIBLRHSMM_RS_IO_ERROR;assert(liblrhsmm_rs_observation_write_stream_f32(owner,&sink)==3);
    output.status=0;output.oversized=1;assert(liblrhsmm_rs_observation_write_stream_f32(owner,&sink)==3);
    output.oversized=0;output.zero=1;assert(liblrhsmm_rs_observation_write_stream_f32(owner,&sink)==3);
    sink.write=NULL;assert(liblrhsmm_rs_observation_write_stream_f32(owner,&sink)==3);
    assert(liblrhsmm_rs_observation_read_bytes_f32(NULL,UINTPTR_MAX,&limits,&decoded)==2);
    assert(liblrhsmm_rs_observation_read_bytes_f32(NULL,0,&limits,&decoded)==3);
    assert(liblrhsmm_rs_observation_read_bytes_f32(fixture,count,&limits,NULL)==1);
    assert(liblrhsmm_rs_observation_read_bytes_f32(fixture,count,NULL,&decoded)==1);
    assert(liblrhsmm_rs_observation_write_bytes_f32(owner,NULL)==1);
    assert(liblrhsmm_rs_observation_write_stream_f32(owner,NULL)==1);
    sink.write=write_callback;memset(&output,0,sizeof(output));
    assert(liblrhsmm_rs_observation_release_f32(&owner)==0 && liblrhsmm_rs_bytes_release(&bytes)==0);
}

static void check_f64(const char *path,uintptr_t budget) {
    static uint8_t fixture[150000], result[150000];
    FILE *file=fopen(path,"rb");assert(file);
    uintptr_t count=fread(fixture,1,sizeof(fixture),file);assert(feof(file));assert(fclose(file)==0 && count>0);
    LiblrhsmmRsDecodeLimits limits;assert(liblrhsmm_rs_decode_limits_default(&limits)==0 && limits.max_array_entries==16*1024*1024);
    LiblrhsmmRsObservationF64 *owner=NULL,*decoded=NULL;LiblrhsmmRsBytes *bytes=NULL;
    assert(liblrhsmm_rs_observation_read_bytes_f64(fixture,count,&limits,&owner)==0);
    assert(liblrhsmm_rs_observation_write_bytes_f64(owner,&bytes)==0);
    uintptr_t length=0;assert(liblrhsmm_rs_bytes_length(bytes,&length)==0 && length==count);
    assert(liblrhsmm_rs_bytes_copy(bytes,0,result,count)==0 && !memcmp(result,fixture,count));
    static struct Input input; memset(&input,0,sizeof(input));memcpy(input.bytes,fixture,count);input.length=count;input.interrupt=1;
    LiblrhsmmRsReadStream source={&input,read_callback};
    assert(liblrhsmm_rs_observation_read_stream_f64(&source,&limits,&decoded)==0 && input.position==count && input.calls>count);
    static struct Output output; memset(&output,0,sizeof(output));output.interrupt=1;LiblrhsmmRsWriteStream sink={&output,write_callback,flush_callback};
    assert(liblrhsmm_rs_observation_write_stream_f64(decoded,&sink)==0 && output.length==count && !memcmp(output.bytes,fixture,count));
    assert(output.flushes==0);assert(liblrhsmm_rs_write_stream_flush(&sink)==0 && output.flushes==1);
    assert(liblrhsmm_rs_observation_release_f64(&decoded)==0);
    memset(&input,0,sizeof(input));memcpy(input.bytes,fixture,count);memcpy(input.bytes+count,fixture,count);input.bytes[count*2]=42;input.length=count*2+1;input.interrupt=1;
    for(uintptr_t i=1;i<=2;i++) {
        assert(liblrhsmm_rs_observation_read_stream_prefix_f64(&source,&limits,&decoded)==0 && input.position==count*i);
        assert(liblrhsmm_rs_observation_release_f64(&decoded)==0);
    }
    assert(input.bytes[input.position]==42);
    for(uintptr_t end=0;end<count;end++) if(count<=512 || end<64 || end>=count-8 || end%1024==0) assert(liblrhsmm_rs_observation_read_bytes_f64(fixture,end,&limits,&decoded)==3 && decoded==NULL);
    fixture[count]=0;assert(liblrhsmm_rs_observation_read_bytes_f64(fixture,count+1,&limits,&decoded)==3 && decoded==NULL);
    LiblrhsmmRsDecodeLimits tight={budget-1};assert(liblrhsmm_rs_observation_read_bytes_f64(fixture,count,&tight,&decoded)==3);
    tight.max_array_entries=budget;assert(liblrhsmm_rs_observation_read_bytes_f64(fixture,count,&tight,&decoded)==0);assert(liblrhsmm_rs_observation_release_f64(&decoded)==0);
    input.position=0;input.status=LIBLRHSMM_RS_IO_ERROR;
    assert(liblrhsmm_rs_observation_read_stream_f64(&source,&limits,&decoded)==3 && decoded==NULL);
    input.status=0;input.oversized=1;assert(liblrhsmm_rs_observation_read_stream_f64(&source,&limits,&decoded)==3);
    source.read=NULL;assert(liblrhsmm_rs_observation_read_stream_f64(&source,&limits,&decoded)==3);
    source.read=read_callback;input.calls=0;
    assert(liblrhsmm_rs_observation_read_stream_f64(&source,&limits,NULL)==1 && input.calls==0);
    output.status=LIBLRHSMM_RS_IO_ERROR;assert(liblrhsmm_rs_observation_write_stream_f64(owner,&sink)==3);
    output.status=0;output.oversized=1;assert(liblrhsmm_rs_observation_write_stream_f64(owner,&sink)==3);
    output.oversized=0;output.zero=1;assert(liblrhsmm_rs_observation_write_stream_f64(owner,&sink)==3);
    sink.write=NULL;assert(liblrhsmm_rs_observation_write_stream_f64(owner,&sink)==3);
    assert(liblrhsmm_rs_observation_read_bytes_f64(NULL,UINTPTR_MAX,&limits,&decoded)==2);
    assert(liblrhsmm_rs_observation_read_bytes_f64(NULL,0,&limits,&decoded)==3);
    assert(liblrhsmm_rs_observation_read_bytes_f64(fixture,count,&limits,NULL)==1);
    assert(liblrhsmm_rs_observation_read_bytes_f64(fixture,count,NULL,&decoded)==1);
    assert(liblrhsmm_rs_observation_write_bytes_f64(owner,NULL)==1);
    assert(liblrhsmm_rs_observation_write_stream_f64(owner,NULL)==1);
    sink.write=write_callback;memset(&output,0,sizeof(output));
    assert(liblrhsmm_rs_observation_release_f64(&owner)==0 && liblrhsmm_rs_bytes_release(&bytes)==0);
}

#endif
int main(void) {check_bytes();
#if defined(LIBLRHSMM_RS_SERIALIZATION)
check_f32("tests/fixtures/c-observation.bin",16);
check_f64("tests/fixtures/c-observation.bin",16);
puts("C Observation codec: all10 exports, exact original wire and sequential object boundaries passed");
#else
puts("C byte/I/O core: all6 new exports passed without serialization");
#endif
return 0;}
