/* Actual C caller for byte ownership, borrowed I/O and complete original set codecs. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <stdio.h>
#include <string.h>
#include "liblrhsmm_rs.h"
struct Input { unsigned char bytes[300000]; uintptr_t length,position,calls; uint32_t status; int interrupt,oversized,reject_eof; };
struct Output { unsigned char bytes[300000]; uintptr_t length,calls,flushes; uint32_t status; int interrupt,oversized,zero; uintptr_t fail_after; };
static uint32_t read_callback(void *context,uint8_t *output,uintptr_t capacity,uintptr_t *count) {
    struct Input *input=context; input->calls++;
    if(input->interrupt) {input->interrupt=0;return LIBLRHSMM_RS_IO_INTERRUPTED;}
    if(input->status) return input->status;
    if(input->oversized) {*count=capacity+1;return 0;}
    if(input->reject_eof && input->position==input->length) return LIBLRHSMM_RS_IO_ERROR;
    *count=capacity && input->position<input->length?1:0;
    if(*count) *output=input->bytes[input->position++];
    return 0;
}
static uint32_t write_callback(void *context,const uint8_t *input,uintptr_t capacity,uintptr_t *count) {
    struct Output *output=context; output->calls++;
    if(output->interrupt) {output->interrupt=0;return LIBLRHSMM_RS_IO_INTERRUPTED;}
    if(output->status) return output->status;
    if(output->oversized) {*count=capacity+1;return 0;}
    if(output->fail_after && output->length>=output->fail_after) return LIBLRHSMM_RS_IO_ERROR;
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
    static struct Output output; memset(&output,0,sizeof(output));_Alignas(8) LiblrhsmmRsWriteStream stream={&output,write_callback,flush_callback};
    assert(liblrhsmm_rs_write_stream_flush(&stream)==0 && output.flushes==1);
    output.status=LIBLRHSMM_RS_IO_INTERRUPTED;assert(liblrhsmm_rs_write_stream_flush(&stream)==3);
    output.status=LIBLRHSMM_RS_IO_ERROR;assert(liblrhsmm_rs_write_stream_flush(&stream)==3);
    stream.flush=NULL;assert(liblrhsmm_rs_write_stream_flush(&stream)==3);
    assert(liblrhsmm_rs_write_stream_flush(NULL)==1);
}
#if defined(LIBLRHSMM_RS_SERIALIZATION)
static void compare_child_observation_f32(const LiblrhsmmRsObservationF32 *owner,int variant) {
    (void)variant;_Alignas(8) LiblrhsmmRsObservationLayout shape;
    assert(liblrhsmm_rs_observation_layout_f32(owner,&shape)==0 && shape.frames==3 && shape.streams==2);
    for(size_t stream=0;stream<2;stream++) {
        size_t dimensions=stream?1:2;_Alignas(8) LiblrhsmmRsObservationStreamLayout layout;
        assert(liblrhsmm_rs_observation_get_stream_layout_f32(owner,stream,&layout)==0 && layout.frames==3 && layout.dimensions==dimensions && layout.values==3*dimensions);
        float actual[6];assert(liblrhsmm_rs_observation_copy_values_f32(owner,stream,0,actual,3*dimensions)==0);
        for(size_t frame=0;frame<3;frame++) for(size_t d=0;d<dimensions;d++) {
            float expected=(float)((int)frame-1)*(float).25+(float)d+(float)stream*10;
            assert(!memcmp(&actual[frame*dimensions+d],&expected,sizeof(expected)));
        }
    }
}
static void compare_set_observation_f32(const LiblrhsmmRsObservationSetF32 *owner,int variant) {
    size_t length=99;assert(liblrhsmm_rs_observation_set_length_f32(owner,&length)==0 && length==(variant==2?0:2));
    for(size_t index=0;index<length;index++) {
        LiblrhsmmRsObservationF32 *child=NULL;
        assert(liblrhsmm_rs_observation_set_get_f32(owner,index,&child)==0);
        compare_child_observation_f32(child,variant);
        assert(liblrhsmm_rs_observation_release_f32(&child)==0);
    }
}
static void check_observation_f32(const char *path,uintptr_t budget,int variant) {
    static uint8_t fixture[150000], result[150000];
    uintptr_t count=1;
    if(path) {FILE *file=fopen(path,"rb");assert(file);count=fread(fixture,1,sizeof(fixture),file);assert(feof(file));assert(fclose(file)==0 && count>0);}
    else fixture[0]=0x90;
    _Alignas(8) LiblrhsmmRsDecodeLimits limits;assert(liblrhsmm_rs_decode_limits_default(&limits)==0 && limits.max_array_entries==16*1024*1024);
    LiblrhsmmRsObservationSetF32 *owner=NULL,*decoded=NULL;LiblrhsmmRsBytes *bytes=NULL;
    assert(liblrhsmm_rs_observation_set_read_bytes_f32(fixture,count,&limits,&owner)==0); compare_set_observation_f32(owner,variant);
    assert(liblrhsmm_rs_observation_set_write_bytes_f32(owner,&bytes)==0);
    uintptr_t length=0;assert(liblrhsmm_rs_bytes_length(bytes,&length)==0 && length==count);
    assert(liblrhsmm_rs_bytes_copy(bytes,0,result,count)==0 && !memcmp(result,fixture,count));
    static struct Input input; memset(&input,0,sizeof(input));memcpy(input.bytes,fixture,count);input.length=count;input.interrupt=1;
    _Alignas(8) LiblrhsmmRsReadStream source={&input,read_callback};
    assert(liblrhsmm_rs_observation_set_read_stream_f32(&source,&limits,&decoded)==0 && input.position==count && input.calls>count); compare_set_observation_f32(decoded,variant);
    static struct Output output; memset(&output,0,sizeof(output));output.interrupt=1;_Alignas(8) LiblrhsmmRsWriteStream sink={&output,write_callback,flush_callback};
    assert(liblrhsmm_rs_observation_set_write_stream_f32(decoded,&sink)==0 && output.length==count && !memcmp(output.bytes,fixture,count));
    assert(output.flushes==0);assert(liblrhsmm_rs_write_stream_flush(&sink)==0 && output.flushes==1);
    assert(liblrhsmm_rs_observation_set_release_f32(&decoded)==0);
    memset(&input,0,sizeof(input));memcpy(input.bytes,fixture,count);memcpy(input.bytes+count,fixture,count);input.bytes[count*2]=42;input.length=count*2+1;input.interrupt=1;
    for(uintptr_t i=1;i<=2;i++) {
        assert(liblrhsmm_rs_observation_set_read_stream_prefix_f32(&source,&limits,&decoded)==0 && input.position==count*i);
        assert(liblrhsmm_rs_observation_set_release_f32(&decoded)==0);
    }
    assert(input.bytes[input.position]==42);
    memset(&input,0,sizeof(input));memcpy(input.bytes,fixture,count);input.length=count;input.reject_eof=1;
    assert(liblrhsmm_rs_observation_set_read_stream_prefix_f32(&source,&limits,&decoded)==0 && input.position==count);
    compare_set_observation_f32(decoded,variant);assert(liblrhsmm_rs_observation_set_release_f32(&decoded)==0);
    input.position=0;assert(liblrhsmm_rs_observation_set_read_stream_f32(&source,&limits,&decoded)==3 && decoded==NULL);

    for(uintptr_t end=0;end<count;end++) assert(liblrhsmm_rs_observation_set_read_bytes_f32(fixture,end,&limits,&decoded)==3 && decoded==NULL);
    fixture[count]=0;assert(liblrhsmm_rs_observation_set_read_bytes_f32(fixture,count+1,&limits,&decoded)==3 && decoded==NULL);
    if(budget) {
    _Alignas(8) LiblrhsmmRsDecodeLimits tight={budget-1};assert(liblrhsmm_rs_observation_set_read_bytes_f32(fixture,count,&tight,&decoded)==3);
    tight.max_array_entries=budget;assert(liblrhsmm_rs_observation_set_read_bytes_f32(fixture,count,&tight,&decoded)==0);assert(liblrhsmm_rs_observation_set_release_f32(&decoded)==0);
    }
    input.position=0;input.status=LIBLRHSMM_RS_IO_ERROR;
    assert(liblrhsmm_rs_observation_set_read_stream_f32(&source,&limits,&decoded)==3 && decoded==NULL);
    input.status=0;input.oversized=1;assert(liblrhsmm_rs_observation_set_read_stream_f32(&source,&limits,&decoded)==3);
    source.read=NULL;assert(liblrhsmm_rs_observation_set_read_stream_f32(&source,&limits,&decoded)==3);
    source.read=read_callback;input.calls=0;
    assert(liblrhsmm_rs_observation_set_read_stream_f32(&source,&limits,NULL)==1 && input.calls==0);
    output.status=LIBLRHSMM_RS_IO_ERROR;assert(liblrhsmm_rs_observation_set_write_stream_f32(owner,&sink)==3);
    output.status=0;output.oversized=1;assert(liblrhsmm_rs_observation_set_write_stream_f32(owner,&sink)==3);
    output.oversized=0;output.zero=1;assert(liblrhsmm_rs_observation_set_write_stream_f32(owner,&sink)==3);
    sink.write=NULL;assert(liblrhsmm_rs_observation_set_write_stream_f32(owner,&sink)==3);
    assert(liblrhsmm_rs_observation_set_read_bytes_f32(NULL,UINTPTR_MAX,&limits,&decoded)==2);
    assert(liblrhsmm_rs_observation_set_read_bytes_f32(NULL,0,&limits,&decoded)==3);
    assert(liblrhsmm_rs_observation_set_read_bytes_f32(fixture,count,&limits,NULL)==1);
    assert(liblrhsmm_rs_observation_set_read_bytes_f32(fixture,count,NULL,&decoded)==1);
    assert(liblrhsmm_rs_observation_set_write_bytes_f32(owner,NULL)==1);
    assert(liblrhsmm_rs_observation_set_write_stream_f32(owner,NULL)==1);
    sink.write=write_callback;memset(&output,0,sizeof(output));
    if(count>7) {
    output.fail_after=7;
    assert(liblrhsmm_rs_observation_set_write_stream_f32(owner,&sink)==3 && output.length==7);
    }
    LiblrhsmmRsObservationSetF32 *sentinel=owner;
    assert(liblrhsmm_rs_observation_set_read_bytes_f32(fixture,count-1,&limits,&sentinel)==3 && sentinel==owner);compare_set_observation_f32(owner,variant);
    assert(liblrhsmm_rs_observation_set_release_f32(&owner)==0);
    assert(liblrhsmm_rs_bytes_copy(bytes,0,result,count)==0 && !memcmp(result,fixture,count));
    assert(liblrhsmm_rs_bytes_release(&bytes)==0);
}


static void compare_child_observation_f64(const LiblrhsmmRsObservationF64 *owner,int variant) {
    (void)variant;_Alignas(8) LiblrhsmmRsObservationLayout shape;
    assert(liblrhsmm_rs_observation_layout_f64(owner,&shape)==0 && shape.frames==3 && shape.streams==2);
    for(size_t stream=0;stream<2;stream++) {
        size_t dimensions=stream?1:2;_Alignas(8) LiblrhsmmRsObservationStreamLayout layout;
        assert(liblrhsmm_rs_observation_get_stream_layout_f64(owner,stream,&layout)==0 && layout.frames==3 && layout.dimensions==dimensions && layout.values==3*dimensions);
        _Alignas(8) double actual[6];assert(liblrhsmm_rs_observation_copy_values_f64(owner,stream,0,actual,3*dimensions)==0);
        for(size_t frame=0;frame<3;frame++) for(size_t d=0;d<dimensions;d++) {
            _Alignas(8) double expected=(double)((int)frame-1)*(double).25+(double)d+(double)stream*10;
            assert(!memcmp(&actual[frame*dimensions+d],&expected,sizeof(expected)));
        }
    }
}
static void compare_set_observation_f64(const LiblrhsmmRsObservationSetF64 *owner,int variant) {
    size_t length=99;assert(liblrhsmm_rs_observation_set_length_f64(owner,&length)==0 && length==(variant==2?0:2));
    for(size_t index=0;index<length;index++) {
        LiblrhsmmRsObservationF64 *child=NULL;
        assert(liblrhsmm_rs_observation_set_get_f64(owner,index,&child)==0);
        compare_child_observation_f64(child,variant);
        assert(liblrhsmm_rs_observation_release_f64(&child)==0);
    }
}
static void check_observation_f64(const char *path,uintptr_t budget,int variant) {
    static uint8_t fixture[150000], result[150000];
    uintptr_t count=1;
    if(path) {FILE *file=fopen(path,"rb");assert(file);count=fread(fixture,1,sizeof(fixture),file);assert(feof(file));assert(fclose(file)==0 && count>0);}
    else fixture[0]=0x90;
    _Alignas(8) LiblrhsmmRsDecodeLimits limits;assert(liblrhsmm_rs_decode_limits_default(&limits)==0 && limits.max_array_entries==16*1024*1024);
    LiblrhsmmRsObservationSetF64 *owner=NULL,*decoded=NULL;LiblrhsmmRsBytes *bytes=NULL;
    assert(liblrhsmm_rs_observation_set_read_bytes_f64(fixture,count,&limits,&owner)==0); compare_set_observation_f64(owner,variant);
    assert(liblrhsmm_rs_observation_set_write_bytes_f64(owner,&bytes)==0);
    uintptr_t length=0;assert(liblrhsmm_rs_bytes_length(bytes,&length)==0 && length==count);
    assert(liblrhsmm_rs_bytes_copy(bytes,0,result,count)==0 && !memcmp(result,fixture,count));
    static struct Input input; memset(&input,0,sizeof(input));memcpy(input.bytes,fixture,count);input.length=count;input.interrupt=1;
    _Alignas(8) LiblrhsmmRsReadStream source={&input,read_callback};
    assert(liblrhsmm_rs_observation_set_read_stream_f64(&source,&limits,&decoded)==0 && input.position==count && input.calls>count); compare_set_observation_f64(decoded,variant);
    static struct Output output; memset(&output,0,sizeof(output));output.interrupt=1;_Alignas(8) LiblrhsmmRsWriteStream sink={&output,write_callback,flush_callback};
    assert(liblrhsmm_rs_observation_set_write_stream_f64(decoded,&sink)==0 && output.length==count && !memcmp(output.bytes,fixture,count));
    assert(output.flushes==0);assert(liblrhsmm_rs_write_stream_flush(&sink)==0 && output.flushes==1);
    assert(liblrhsmm_rs_observation_set_release_f64(&decoded)==0);
    memset(&input,0,sizeof(input));memcpy(input.bytes,fixture,count);memcpy(input.bytes+count,fixture,count);input.bytes[count*2]=42;input.length=count*2+1;input.interrupt=1;
    for(uintptr_t i=1;i<=2;i++) {
        assert(liblrhsmm_rs_observation_set_read_stream_prefix_f64(&source,&limits,&decoded)==0 && input.position==count*i);
        assert(liblrhsmm_rs_observation_set_release_f64(&decoded)==0);
    }
    assert(input.bytes[input.position]==42);
    memset(&input,0,sizeof(input));memcpy(input.bytes,fixture,count);input.length=count;input.reject_eof=1;
    assert(liblrhsmm_rs_observation_set_read_stream_prefix_f64(&source,&limits,&decoded)==0 && input.position==count);
    compare_set_observation_f64(decoded,variant);assert(liblrhsmm_rs_observation_set_release_f64(&decoded)==0);
    input.position=0;assert(liblrhsmm_rs_observation_set_read_stream_f64(&source,&limits,&decoded)==3 && decoded==NULL);

    for(uintptr_t end=0;end<count;end++) assert(liblrhsmm_rs_observation_set_read_bytes_f64(fixture,end,&limits,&decoded)==3 && decoded==NULL);
    fixture[count]=0;assert(liblrhsmm_rs_observation_set_read_bytes_f64(fixture,count+1,&limits,&decoded)==3 && decoded==NULL);
    if(budget) {
    _Alignas(8) LiblrhsmmRsDecodeLimits tight={budget-1};assert(liblrhsmm_rs_observation_set_read_bytes_f64(fixture,count,&tight,&decoded)==3);
    tight.max_array_entries=budget;assert(liblrhsmm_rs_observation_set_read_bytes_f64(fixture,count,&tight,&decoded)==0);assert(liblrhsmm_rs_observation_set_release_f64(&decoded)==0);
    }
    input.position=0;input.status=LIBLRHSMM_RS_IO_ERROR;
    assert(liblrhsmm_rs_observation_set_read_stream_f64(&source,&limits,&decoded)==3 && decoded==NULL);
    input.status=0;input.oversized=1;assert(liblrhsmm_rs_observation_set_read_stream_f64(&source,&limits,&decoded)==3);
    source.read=NULL;assert(liblrhsmm_rs_observation_set_read_stream_f64(&source,&limits,&decoded)==3);
    source.read=read_callback;input.calls=0;
    assert(liblrhsmm_rs_observation_set_read_stream_f64(&source,&limits,NULL)==1 && input.calls==0);
    output.status=LIBLRHSMM_RS_IO_ERROR;assert(liblrhsmm_rs_observation_set_write_stream_f64(owner,&sink)==3);
    output.status=0;output.oversized=1;assert(liblrhsmm_rs_observation_set_write_stream_f64(owner,&sink)==3);
    output.oversized=0;output.zero=1;assert(liblrhsmm_rs_observation_set_write_stream_f64(owner,&sink)==3);
    sink.write=NULL;assert(liblrhsmm_rs_observation_set_write_stream_f64(owner,&sink)==3);
    assert(liblrhsmm_rs_observation_set_read_bytes_f64(NULL,UINTPTR_MAX,&limits,&decoded)==2);
    assert(liblrhsmm_rs_observation_set_read_bytes_f64(NULL,0,&limits,&decoded)==3);
    assert(liblrhsmm_rs_observation_set_read_bytes_f64(fixture,count,&limits,NULL)==1);
    assert(liblrhsmm_rs_observation_set_read_bytes_f64(fixture,count,NULL,&decoded)==1);
    assert(liblrhsmm_rs_observation_set_write_bytes_f64(owner,NULL)==1);
    assert(liblrhsmm_rs_observation_set_write_stream_f64(owner,NULL)==1);
    sink.write=write_callback;memset(&output,0,sizeof(output));
    if(count>7) {
    output.fail_after=7;
    assert(liblrhsmm_rs_observation_set_write_stream_f64(owner,&sink)==3 && output.length==7);
    }
    LiblrhsmmRsObservationSetF64 *sentinel=owner;
    assert(liblrhsmm_rs_observation_set_read_bytes_f64(fixture,count-1,&limits,&sentinel)==3 && sentinel==owner);compare_set_observation_f64(owner,variant);
    assert(liblrhsmm_rs_observation_set_release_f64(&owner)==0);
    assert(liblrhsmm_rs_bytes_copy(bytes,0,result,count)==0 && !memcmp(result,fixture,count));
    assert(liblrhsmm_rs_bytes_release(&bytes)==0);
}


static void compare_child_segmentation_f32(const LiblrhsmmRsSegmentationF32 *owner,int variant) {
    _Alignas(8) LiblrhsmmRsSegmentationLayout layout;
    assert(liblrhsmm_rs_segmentation_layout_f32(owner,&layout)==0 && layout.streams==2 && layout.segments==3);
    const int32_t boundaries[3]={2,4,6}, duration[3]={variant?3:0,variant?4:1,variant?5:2}, labels[2][3]={{variant?10:0,variant?11:1,variant?12:2},{variant?20:2,variant?19:1,variant?18:0}};
    int32_t values[3];
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f32(owner,0,values,3)==0 && !memcmp(values,boundaries,sizeof(values)));
    assert(liblrhsmm_rs_segmentation_copy_duration_states_f32(owner,0,values,3)==0 && !memcmp(values,duration,sizeof(values)));
    for(uintptr_t stream=0;stream<2;stream++) assert(liblrhsmm_rs_segmentation_copy_output_states_f32(owner,stream,0,values,3)==0 && !memcmp(values,labels[stream],sizeof(values)));
    const uintptr_t counts[3]={2,variant?2:1,variant?1:3};
    const int32_t deltas[3][3]={{2,1,0},{variant?0:1,1,0},{variant?1:0,-1,1}};
    volatile float probabilities[3][3]={{0.25f,0.75f,0},{1,0,0},{0.2f,0.1f,0.7f}};
    for(uintptr_t state=0;state<3;state++) {
        uintptr_t count=99;
        assert(liblrhsmm_rs_segmentation_outgoing_count_f32(owner,state,&count)==0 && count==counts[state]);
        _Alignas(8) LiblrhsmmRsJumpF32 jumps[3];
        assert(liblrhsmm_rs_segmentation_copy_outgoing_f32(owner,state,0,jumps,count)==0);
        for(uintptr_t index=0;index<count;index++) {
            float expected=probabilities[state][index];
            if(variant) {if(state==1) expected=index?(float)0.9:(float)0.1; if(state==2) expected=1;}
            assert(jumps[index].delta==deltas[state][index] && !memcmp(&jumps[index].probability,&expected,sizeof(expected)));
        }
    }
}

static void compare_set_segmentation_f32(const LiblrhsmmRsSegmentationSetF32 *owner,int variant) {
    size_t length=99;assert(liblrhsmm_rs_segmentation_set_length_f32(owner,&length)==0 && length==(variant==2?0:2));
    for(size_t index=0;index<length;index++) {
        LiblrhsmmRsSegmentationF32 *child=NULL;
        assert(liblrhsmm_rs_segmentation_set_get_f32(owner,index,&child)==0);
        compare_child_segmentation_f32(child,variant);
        assert(liblrhsmm_rs_segmentation_release_f32(&child)==0);
    }
}
static void check_segmentation_f32(const char *path,uintptr_t budget,int variant) {
    static uint8_t fixture[150000], result[150000];
    uintptr_t count=1;
    if(path) {FILE *file=fopen(path,"rb");assert(file);count=fread(fixture,1,sizeof(fixture),file);assert(feof(file));assert(fclose(file)==0 && count>0);}
    else fixture[0]=0x90;
    _Alignas(8) LiblrhsmmRsDecodeLimits limits;assert(liblrhsmm_rs_decode_limits_default(&limits)==0 && limits.max_array_entries==16*1024*1024);
    LiblrhsmmRsSegmentationSetF32 *owner=NULL,*decoded=NULL;LiblrhsmmRsBytes *bytes=NULL;
    assert(liblrhsmm_rs_segmentation_set_read_bytes_f32(fixture,count,&limits,&owner)==0); compare_set_segmentation_f32(owner,variant);
    assert(liblrhsmm_rs_segmentation_set_write_bytes_f32(owner,&bytes)==0);
    uintptr_t length=0;assert(liblrhsmm_rs_bytes_length(bytes,&length)==0 && length==count);
    assert(liblrhsmm_rs_bytes_copy(bytes,0,result,count)==0 && !memcmp(result,fixture,count));
    static struct Input input; memset(&input,0,sizeof(input));memcpy(input.bytes,fixture,count);input.length=count;input.interrupt=1;
    _Alignas(8) LiblrhsmmRsReadStream source={&input,read_callback};
    assert(liblrhsmm_rs_segmentation_set_read_stream_f32(&source,&limits,&decoded)==0 && input.position==count && input.calls>count); compare_set_segmentation_f32(decoded,variant);
    static struct Output output; memset(&output,0,sizeof(output));output.interrupt=1;_Alignas(8) LiblrhsmmRsWriteStream sink={&output,write_callback,flush_callback};
    assert(liblrhsmm_rs_segmentation_set_write_stream_f32(decoded,&sink)==0 && output.length==count && !memcmp(output.bytes,fixture,count));
    assert(output.flushes==0);assert(liblrhsmm_rs_write_stream_flush(&sink)==0 && output.flushes==1);
    assert(liblrhsmm_rs_segmentation_set_release_f32(&decoded)==0);
    memset(&input,0,sizeof(input));memcpy(input.bytes,fixture,count);memcpy(input.bytes+count,fixture,count);input.bytes[count*2]=42;input.length=count*2+1;input.interrupt=1;
    for(uintptr_t i=1;i<=2;i++) {
        assert(liblrhsmm_rs_segmentation_set_read_stream_prefix_f32(&source,&limits,&decoded)==0 && input.position==count*i);
        assert(liblrhsmm_rs_segmentation_set_release_f32(&decoded)==0);
    }
    assert(input.bytes[input.position]==42);
    memset(&input,0,sizeof(input));memcpy(input.bytes,fixture,count);input.length=count;input.reject_eof=1;
    assert(liblrhsmm_rs_segmentation_set_read_stream_prefix_f32(&source,&limits,&decoded)==0 && input.position==count);
    compare_set_segmentation_f32(decoded,variant);assert(liblrhsmm_rs_segmentation_set_release_f32(&decoded)==0);
    input.position=0;assert(liblrhsmm_rs_segmentation_set_read_stream_f32(&source,&limits,&decoded)==3 && decoded==NULL);

    for(uintptr_t end=0;end<count;end++) assert(liblrhsmm_rs_segmentation_set_read_bytes_f32(fixture,end,&limits,&decoded)==3 && decoded==NULL);
    fixture[count]=0;assert(liblrhsmm_rs_segmentation_set_read_bytes_f32(fixture,count+1,&limits,&decoded)==3 && decoded==NULL);
    if(budget) {
    _Alignas(8) LiblrhsmmRsDecodeLimits tight={budget-1};assert(liblrhsmm_rs_segmentation_set_read_bytes_f32(fixture,count,&tight,&decoded)==3);
    tight.max_array_entries=budget;assert(liblrhsmm_rs_segmentation_set_read_bytes_f32(fixture,count,&tight,&decoded)==0);assert(liblrhsmm_rs_segmentation_set_release_f32(&decoded)==0);
    }
    input.position=0;input.status=LIBLRHSMM_RS_IO_ERROR;
    assert(liblrhsmm_rs_segmentation_set_read_stream_f32(&source,&limits,&decoded)==3 && decoded==NULL);
    input.status=0;input.oversized=1;assert(liblrhsmm_rs_segmentation_set_read_stream_f32(&source,&limits,&decoded)==3);
    source.read=NULL;assert(liblrhsmm_rs_segmentation_set_read_stream_f32(&source,&limits,&decoded)==3);
    source.read=read_callback;input.calls=0;
    assert(liblrhsmm_rs_segmentation_set_read_stream_f32(&source,&limits,NULL)==1 && input.calls==0);
    output.status=LIBLRHSMM_RS_IO_ERROR;assert(liblrhsmm_rs_segmentation_set_write_stream_f32(owner,&sink)==3);
    output.status=0;output.oversized=1;assert(liblrhsmm_rs_segmentation_set_write_stream_f32(owner,&sink)==3);
    output.oversized=0;output.zero=1;assert(liblrhsmm_rs_segmentation_set_write_stream_f32(owner,&sink)==3);
    sink.write=NULL;assert(liblrhsmm_rs_segmentation_set_write_stream_f32(owner,&sink)==3);
    assert(liblrhsmm_rs_segmentation_set_read_bytes_f32(NULL,UINTPTR_MAX,&limits,&decoded)==2);
    assert(liblrhsmm_rs_segmentation_set_read_bytes_f32(NULL,0,&limits,&decoded)==3);
    assert(liblrhsmm_rs_segmentation_set_read_bytes_f32(fixture,count,&limits,NULL)==1);
    assert(liblrhsmm_rs_segmentation_set_read_bytes_f32(fixture,count,NULL,&decoded)==1);
    assert(liblrhsmm_rs_segmentation_set_write_bytes_f32(owner,NULL)==1);
    assert(liblrhsmm_rs_segmentation_set_write_stream_f32(owner,NULL)==1);
    sink.write=write_callback;memset(&output,0,sizeof(output));
    if(count>7) {
    output.fail_after=7;
    assert(liblrhsmm_rs_segmentation_set_write_stream_f32(owner,&sink)==3 && output.length==7);
    }
    LiblrhsmmRsSegmentationSetF32 *sentinel=owner;
    assert(liblrhsmm_rs_segmentation_set_read_bytes_f32(fixture,count-1,&limits,&sentinel)==3 && sentinel==owner);compare_set_segmentation_f32(owner,variant);
    assert(liblrhsmm_rs_segmentation_set_release_f32(&owner)==0);
    assert(liblrhsmm_rs_bytes_copy(bytes,0,result,count)==0 && !memcmp(result,fixture,count));
    assert(liblrhsmm_rs_bytes_release(&bytes)==0);
}


static void compare_child_segmentation_f64(const LiblrhsmmRsSegmentationF64 *owner,int variant) {
    _Alignas(8) LiblrhsmmRsSegmentationLayout layout;
    assert(liblrhsmm_rs_segmentation_layout_f64(owner,&layout)==0 && layout.streams==2 && layout.segments==3);
    const int32_t boundaries[3]={2,4,6}, duration[3]={variant?3:0,variant?4:1,variant?5:2}, labels[2][3]={{variant?10:0,variant?11:1,variant?12:2},{variant?20:2,variant?19:1,variant?18:0}};
    int32_t values[3];
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(owner,0,values,3)==0 && !memcmp(values,boundaries,sizeof(values)));
    assert(liblrhsmm_rs_segmentation_copy_duration_states_f64(owner,0,values,3)==0 && !memcmp(values,duration,sizeof(values)));
    for(uintptr_t stream=0;stream<2;stream++) assert(liblrhsmm_rs_segmentation_copy_output_states_f64(owner,stream,0,values,3)==0 && !memcmp(values,labels[stream],sizeof(values)));
    const uintptr_t counts[3]={2,variant?2:1,variant?1:3};
    const int32_t deltas[3][3]={{2,1,0},{variant?0:1,1,0},{variant?1:0,-1,1}};
    volatile float probabilities[3][3]={{0.25f,0.75f,0},{1,0,0},{0.2f,0.1f,0.7f}};
    for(uintptr_t state=0;state<3;state++) {
        uintptr_t count=99;
        assert(liblrhsmm_rs_segmentation_outgoing_count_f64(owner,state,&count)==0 && count==counts[state]);
        _Alignas(8) LiblrhsmmRsJumpF64 jumps[3];
        assert(liblrhsmm_rs_segmentation_copy_outgoing_f64(owner,state,0,jumps,count)==0);
        for(uintptr_t index=0;index<count;index++) {
            _Alignas(8) double expected=probabilities[state][index];
            if(variant) {if(state==1) expected=index?(float)0.9:(float)0.1; if(state==2) expected=1;}
            assert(jumps[index].delta==deltas[state][index] && !memcmp(&jumps[index].probability,&expected,sizeof(expected)));
        }
    }
}

static void compare_set_segmentation_f64(const LiblrhsmmRsSegmentationSetF64 *owner,int variant) {
    size_t length=99;assert(liblrhsmm_rs_segmentation_set_length_f64(owner,&length)==0 && length==(variant==2?0:2));
    for(size_t index=0;index<length;index++) {
        LiblrhsmmRsSegmentationF64 *child=NULL;
        assert(liblrhsmm_rs_segmentation_set_get_f64(owner,index,&child)==0);
        compare_child_segmentation_f64(child,variant);
        assert(liblrhsmm_rs_segmentation_release_f64(&child)==0);
    }
}
static void check_segmentation_f64(const char *path,uintptr_t budget,int variant) {
    static uint8_t fixture[150000], result[150000];
    uintptr_t count=1;
    if(path) {FILE *file=fopen(path,"rb");assert(file);count=fread(fixture,1,sizeof(fixture),file);assert(feof(file));assert(fclose(file)==0 && count>0);}
    else fixture[0]=0x90;
    _Alignas(8) LiblrhsmmRsDecodeLimits limits;assert(liblrhsmm_rs_decode_limits_default(&limits)==0 && limits.max_array_entries==16*1024*1024);
    LiblrhsmmRsSegmentationSetF64 *owner=NULL,*decoded=NULL;LiblrhsmmRsBytes *bytes=NULL;
    assert(liblrhsmm_rs_segmentation_set_read_bytes_f64(fixture,count,&limits,&owner)==0); compare_set_segmentation_f64(owner,variant);
    assert(liblrhsmm_rs_segmentation_set_write_bytes_f64(owner,&bytes)==0);
    uintptr_t length=0;assert(liblrhsmm_rs_bytes_length(bytes,&length)==0 && length==count);
    assert(liblrhsmm_rs_bytes_copy(bytes,0,result,count)==0 && !memcmp(result,fixture,count));
    static struct Input input; memset(&input,0,sizeof(input));memcpy(input.bytes,fixture,count);input.length=count;input.interrupt=1;
    _Alignas(8) LiblrhsmmRsReadStream source={&input,read_callback};
    assert(liblrhsmm_rs_segmentation_set_read_stream_f64(&source,&limits,&decoded)==0 && input.position==count && input.calls>count); compare_set_segmentation_f64(decoded,variant);
    static struct Output output; memset(&output,0,sizeof(output));output.interrupt=1;_Alignas(8) LiblrhsmmRsWriteStream sink={&output,write_callback,flush_callback};
    assert(liblrhsmm_rs_segmentation_set_write_stream_f64(decoded,&sink)==0 && output.length==count && !memcmp(output.bytes,fixture,count));
    assert(output.flushes==0);assert(liblrhsmm_rs_write_stream_flush(&sink)==0 && output.flushes==1);
    assert(liblrhsmm_rs_segmentation_set_release_f64(&decoded)==0);
    memset(&input,0,sizeof(input));memcpy(input.bytes,fixture,count);memcpy(input.bytes+count,fixture,count);input.bytes[count*2]=42;input.length=count*2+1;input.interrupt=1;
    for(uintptr_t i=1;i<=2;i++) {
        assert(liblrhsmm_rs_segmentation_set_read_stream_prefix_f64(&source,&limits,&decoded)==0 && input.position==count*i);
        assert(liblrhsmm_rs_segmentation_set_release_f64(&decoded)==0);
    }
    assert(input.bytes[input.position]==42);
    memset(&input,0,sizeof(input));memcpy(input.bytes,fixture,count);input.length=count;input.reject_eof=1;
    assert(liblrhsmm_rs_segmentation_set_read_stream_prefix_f64(&source,&limits,&decoded)==0 && input.position==count);
    compare_set_segmentation_f64(decoded,variant);assert(liblrhsmm_rs_segmentation_set_release_f64(&decoded)==0);
    input.position=0;assert(liblrhsmm_rs_segmentation_set_read_stream_f64(&source,&limits,&decoded)==3 && decoded==NULL);

    for(uintptr_t end=0;end<count;end++) assert(liblrhsmm_rs_segmentation_set_read_bytes_f64(fixture,end,&limits,&decoded)==3 && decoded==NULL);
    fixture[count]=0;assert(liblrhsmm_rs_segmentation_set_read_bytes_f64(fixture,count+1,&limits,&decoded)==3 && decoded==NULL);
    if(budget) {
    _Alignas(8) LiblrhsmmRsDecodeLimits tight={budget-1};assert(liblrhsmm_rs_segmentation_set_read_bytes_f64(fixture,count,&tight,&decoded)==3);
    tight.max_array_entries=budget;assert(liblrhsmm_rs_segmentation_set_read_bytes_f64(fixture,count,&tight,&decoded)==0);assert(liblrhsmm_rs_segmentation_set_release_f64(&decoded)==0);
    }
    input.position=0;input.status=LIBLRHSMM_RS_IO_ERROR;
    assert(liblrhsmm_rs_segmentation_set_read_stream_f64(&source,&limits,&decoded)==3 && decoded==NULL);
    input.status=0;input.oversized=1;assert(liblrhsmm_rs_segmentation_set_read_stream_f64(&source,&limits,&decoded)==3);
    source.read=NULL;assert(liblrhsmm_rs_segmentation_set_read_stream_f64(&source,&limits,&decoded)==3);
    source.read=read_callback;input.calls=0;
    assert(liblrhsmm_rs_segmentation_set_read_stream_f64(&source,&limits,NULL)==1 && input.calls==0);
    output.status=LIBLRHSMM_RS_IO_ERROR;assert(liblrhsmm_rs_segmentation_set_write_stream_f64(owner,&sink)==3);
    output.status=0;output.oversized=1;assert(liblrhsmm_rs_segmentation_set_write_stream_f64(owner,&sink)==3);
    output.oversized=0;output.zero=1;assert(liblrhsmm_rs_segmentation_set_write_stream_f64(owner,&sink)==3);
    sink.write=NULL;assert(liblrhsmm_rs_segmentation_set_write_stream_f64(owner,&sink)==3);
    assert(liblrhsmm_rs_segmentation_set_read_bytes_f64(NULL,UINTPTR_MAX,&limits,&decoded)==2);
    assert(liblrhsmm_rs_segmentation_set_read_bytes_f64(NULL,0,&limits,&decoded)==3);
    assert(liblrhsmm_rs_segmentation_set_read_bytes_f64(fixture,count,&limits,NULL)==1);
    assert(liblrhsmm_rs_segmentation_set_read_bytes_f64(fixture,count,NULL,&decoded)==1);
    assert(liblrhsmm_rs_segmentation_set_write_bytes_f64(owner,NULL)==1);
    assert(liblrhsmm_rs_segmentation_set_write_stream_f64(owner,NULL)==1);
    sink.write=write_callback;memset(&output,0,sizeof(output));
    if(count>7) {
    output.fail_after=7;
    assert(liblrhsmm_rs_segmentation_set_write_stream_f64(owner,&sink)==3 && output.length==7);
    }
    LiblrhsmmRsSegmentationSetF64 *sentinel=owner;
    assert(liblrhsmm_rs_segmentation_set_read_bytes_f64(fixture,count-1,&limits,&sentinel)==3 && sentinel==owner);compare_set_segmentation_f64(owner,variant);
    assert(liblrhsmm_rs_segmentation_set_release_f64(&owner)==0);
    assert(liblrhsmm_rs_bytes_copy(bytes,0,result,count)==0 && !memcmp(result,fixture,count));
    assert(liblrhsmm_rs_bytes_release(&bytes)==0);
}



#endif
int main(void) {check_bytes();
#if defined(LIBLRHSMM_RS_SERIALIZATION)
check_observation_f32("tests/fixtures/c-observation-set.bin",34,0);
check_segmentation_f32("tests/fixtures/c-segmentation-set.bin",66,0);
check_segmentation_f32("tests/fixtures/c-copy-segmentation-set.bin",62,1);
check_observation_f32(NULL,0,2);check_segmentation_f32(NULL,0,2);
check_observation_f64("tests/fixtures/c-observation-set.bin",34,0);
check_segmentation_f64("tests/fixtures/c-segmentation-set.bin",66,0);
check_segmentation_f64("tests/fixtures/c-copy-segmentation-set.bin",62,1);
check_observation_f64(NULL,0,2);check_segmentation_f64(NULL,0,2);
puts("Set codecs C: all20 exports, complete original C fields/wire, limits and sequential object boundaries passed");
#else
puts("Set codecs C: byte/I/O core passed without serialization");
#endif
return 0;
}
