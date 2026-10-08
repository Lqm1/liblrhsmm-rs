#include "liblrhsmm_rs.h"
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <float.h>
#include <stdio.h>
#include <string.h>

static float feature_f32(size_t index,size_t stream,size_t variant) {
    volatile float value=(float)1+(float)(index+stream+variant)/(float)8;
    return value+(float)FLT_EPSILON;
}
static LiblrhsmmRsObservationF32 *make_observation_f32(size_t variant) {
    size_t frames=variant?2:3, streams=variant?1:2;
    LiblrhsmmRsObservationStreamF32 *owners[2]={0};
    const LiblrhsmmRsObservationStreamF32 *pointers[2];
    for(size_t stream=0;stream<streams;stream++) {
        size_t dimensions=variant?1:(stream?1:2); float values[6];
        for(size_t index=0;index<frames*dimensions;index++) values[index]=feature_f32(index,stream,variant);
        assert(liblrhsmm_rs_observation_stream_create_f32(dimensions,values,frames*dimensions,&owners[stream])==0);pointers[stream]=owners[stream];
    }
    LiblrhsmmRsObservationF32 *owner=NULL;assert(liblrhsmm_rs_observation_create_f32(frames,pointers,streams,&owner)==0);
    for(size_t stream=0;stream<streams;stream++) assert(liblrhsmm_rs_observation_stream_release_f32(&owners[stream])==0);
    return owner;
}
static void compare_observation_f32(const LiblrhsmmRsObservationF32 *owner,size_t variant) {
    size_t frames=variant?2:3,streams=variant?1:2;LiblrhsmmRsObservationLayout shape;
    assert(liblrhsmm_rs_observation_layout_f32(owner,&shape)==0 && shape.frames==frames && shape.streams==streams);
    for(size_t stream=0;stream<streams;stream++) {
        size_t dimensions=variant?1:(stream?1:2);LiblrhsmmRsObservationStreamLayout layout;
        assert(liblrhsmm_rs_observation_get_stream_layout_f32(owner,stream,&layout)==0 && layout.frames==frames && layout.dimensions==dimensions && layout.values==frames*dimensions);
        float actual[6];assert(liblrhsmm_rs_observation_copy_values_f32(owner,stream,0,actual,frames*dimensions)==0);
        for(size_t index=0;index<frames*dimensions;index++) {float expected=feature_f32(index,stream,variant);assert(!memcmp(&actual[index],&expected,sizeof(expected)));}
    }
}

static void exercise_observation_f32(void) {
    LiblrhsmmRsObservationF32 *first=make_observation_f32(0),*second=make_observation_f32(1),*child=(LiblrhsmmRsObservationF32 *)(uintptr_t)0x1000;
    LiblrhsmmRsObservationSetBuilderF32 *builder=NULL,*builder_copy=NULL;
    LiblrhsmmRsObservationSetF32 *built=NULL,*created=NULL,*copied=NULL,*sentinel=(LiblrhsmmRsObservationSetF32 *)(uintptr_t)0x1000;
    size_t length=99;uint32_t presence=99;
    assert(liblrhsmm_rs_observation_set_builder_new_f32(3,&builder)==0);
    assert(liblrhsmm_rs_observation_set_builder_length_f32(builder,&length)==0 && length==3);
    for(size_t index=0;index<3;index++) {
        assert(liblrhsmm_rs_observation_set_builder_has_sample_f32(builder,index,&presence)==0 && presence==0);
        assert(liblrhsmm_rs_observation_set_builder_get_sample_f32(builder,index,&child)==3 && (uintptr_t)child==0x1000);
    }
    assert(liblrhsmm_rs_observation_set_builder_build_f32(builder,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_observation_set_builder_set_sample_f32(builder,0,first)==0);
    assert(liblrhsmm_rs_observation_set_builder_clone_f32(builder,&builder_copy)==0);
    assert(liblrhsmm_rs_observation_set_builder_has_sample_f32(builder_copy,0,&presence)==0 && presence==1);
    assert(liblrhsmm_rs_observation_set_builder_has_sample_f32(builder_copy,1,&presence)==0 && presence==0);
    assert(liblrhsmm_rs_observation_set_builder_build_f32(builder_copy,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_observation_set_builder_release_f32(&builder_copy)==0);
    assert(liblrhsmm_rs_observation_set_builder_set_sample_f32(builder,1,second)==0 && liblrhsmm_rs_observation_set_builder_set_sample_f32(builder,2,first)==0);
    assert(liblrhsmm_rs_observation_set_builder_clone_f32(builder,&builder_copy)==0);
    assert(liblrhsmm_rs_observation_set_builder_clear_sample_f32(builder,1)==0);
    assert(liblrhsmm_rs_observation_set_builder_has_sample_f32(builder,1,&presence)==0 && presence==0);
    assert(liblrhsmm_rs_observation_set_builder_get_sample_f32(builder_copy,1,&child)==0);compare_observation_f32(child,1);assert(liblrhsmm_rs_observation_release_f32(&child)==0);
    assert(liblrhsmm_rs_observation_set_builder_resize_f32(builder,4)==0);
    assert(liblrhsmm_rs_observation_set_builder_has_sample_f32(builder,3,&presence)==0 && presence==0);
    assert(liblrhsmm_rs_observation_set_builder_resize_f32(builder,1)==0 && liblrhsmm_rs_observation_set_builder_build_f32(builder,&built)==0);
    assert(liblrhsmm_rs_observation_set_length_f32(built,&length)==0 && length==1);
    assert(liblrhsmm_rs_observation_set_get_f32(built,0,&child)==0);compare_observation_f32(child,0);assert(liblrhsmm_rs_observation_release_f32(&child)==0);
    assert(liblrhsmm_rs_observation_set_release_f32(&built)==0 && liblrhsmm_rs_observation_set_builder_build_f32(builder_copy,&built)==0);
    assert(liblrhsmm_rs_observation_set_set_f32(built,1,first)==0 && liblrhsmm_rs_observation_set_append_f32(built,second)==0);
    assert(liblrhsmm_rs_observation_set_length_f32(built,&length)==0 && length==4);
    const size_t changed[4]={0,0,0,1};
    for(size_t index=0;index<4;index++) {assert(liblrhsmm_rs_observation_set_get_f32(built,index,&child)==0);compare_observation_f32(child,changed[index]);assert(liblrhsmm_rs_observation_release_f32(&child)==0);}
    const LiblrhsmmRsObservationF32 *inputs[3]={first,second,first},*bad[2]={first,NULL};
    assert(liblrhsmm_rs_observation_set_replace_f32(built,inputs,3)==0 && liblrhsmm_rs_observation_set_clone_f32(built,&copied)==0 && liblrhsmm_rs_observation_set_create_f32(inputs,3,&created)==0);
    sentinel=created;assert(liblrhsmm_rs_observation_set_create_f32(bad,2,&sentinel)==1 && sentinel==created);
    assert(liblrhsmm_rs_observation_set_replace_f32(built,bad,2)==1);
    assert(liblrhsmm_rs_observation_set_create_f32(NULL,SIZE_MAX,&sentinel)==2 && sentinel==created);
    assert(liblrhsmm_rs_observation_set_create_f32(NULL,1,&sentinel)==1);
    assert(liblrhsmm_rs_observation_set_set_f32(built,99,first)==2 && liblrhsmm_rs_observation_set_set_f32(built,0,NULL)==1 && liblrhsmm_rs_observation_set_append_f32(built,NULL)==1);
    child=(LiblrhsmmRsObservationF32 *)(uintptr_t)0x1000;assert(liblrhsmm_rs_observation_set_get_f32(built,99,&child)==2 && (uintptr_t)child==0x1000);
    assert(liblrhsmm_rs_observation_set_get_f32(built,0,NULL)==1 && liblrhsmm_rs_observation_set_clone_f32(built,NULL)==1 && liblrhsmm_rs_observation_set_length_f32(built,NULL)==1);
    assert(liblrhsmm_rs_observation_set_builder_resize_f32(builder_copy,(size_t)INT32_MAX+1)==2);
    assert(liblrhsmm_rs_observation_set_builder_length_f32(builder_copy,&length)==0 && length==3);
    assert(liblrhsmm_rs_observation_set_builder_has_sample_f32(builder_copy,99,&presence)==2 && liblrhsmm_rs_observation_set_builder_has_sample_f32(builder_copy,0,NULL)==1);
    assert(liblrhsmm_rs_observation_set_builder_set_sample_f32(builder_copy,0,NULL)==1 && liblrhsmm_rs_observation_set_builder_clear_sample_f32(builder_copy,99)==2);
    assert(liblrhsmm_rs_observation_set_builder_build_f32(builder_copy,NULL)==1 && liblrhsmm_rs_observation_set_builder_clone_f32(builder_copy,NULL)==1 && liblrhsmm_rs_observation_set_builder_length_f32(builder_copy,NULL)==1);
    LiblrhsmmRsObservationSetBuilderF32 *builder_sentinel=builder;assert(liblrhsmm_rs_observation_set_builder_new_f32((size_t)INT32_MAX+1,&builder_sentinel)==3 && builder_sentinel==builder);
    assert(liblrhsmm_rs_observation_set_builder_new_f32(SIZE_MAX,NULL)==1);
    assert(liblrhsmm_rs_observation_set_get_f32(copied,1,&child)==0);
    assert(liblrhsmm_rs_observation_set_builder_release_f32(&builder)==0 && liblrhsmm_rs_observation_set_builder_release_f32(&builder_copy)==0);
    assert(liblrhsmm_rs_observation_release_f32(&first)==0 && liblrhsmm_rs_observation_release_f32(&second)==0);
    assert(liblrhsmm_rs_observation_set_release_f32(&copied)==0);compare_observation_f32(child,1);assert(liblrhsmm_rs_observation_release_f32(&child)==0);
    LiblrhsmmRsObservationSetF32 *sets[2]={built,created};const size_t variants[3]={0,1,0};
    for(size_t set=0;set<2;set++) for(size_t index=0;index<3;index++) {assert(liblrhsmm_rs_observation_set_get_f32(sets[set],index,&child)==0);compare_observation_f32(child,variants[index]);assert(liblrhsmm_rs_observation_release_f32(&child)==0);}
    assert(liblrhsmm_rs_observation_set_replace_f32(built,NULL,0)==0 && liblrhsmm_rs_observation_set_length_f32(built,&length)==0 && length==0);
    assert(liblrhsmm_rs_observation_set_release_f32(&built)==0 && liblrhsmm_rs_observation_set_release_f32(&created)==0 && liblrhsmm_rs_observation_set_release_f32(&created)==0 && liblrhsmm_rs_observation_set_release_f32(NULL)==1);
    assert(liblrhsmm_rs_observation_set_builder_release_f32(&builder_copy)==0 && liblrhsmm_rs_observation_set_builder_release_f32(NULL)==1);
    assert(liblrhsmm_rs_observation_set_builder_new_f32(0,&builder)==0 && liblrhsmm_rs_observation_set_builder_build_f32(builder,&built)==0);
    assert(liblrhsmm_rs_observation_set_length_f32(built,&length)==0 && length==0);
    assert(liblrhsmm_rs_observation_set_release_f32(&built)==0 && liblrhsmm_rs_observation_set_builder_release_f32(&builder)==0);
}

static double feature_f64(size_t index,size_t stream,size_t variant) {
    volatile double value=(double)1+(double)(index+stream+variant)/(double)8;
    return value+(double)DBL_EPSILON;
}
static LiblrhsmmRsObservationF64 *make_observation_f64(size_t variant) {
    size_t frames=variant?2:3, streams=variant?1:2;
    LiblrhsmmRsObservationStreamF64 *owners[2]={0};
    const LiblrhsmmRsObservationStreamF64 *pointers[2];
    for(size_t stream=0;stream<streams;stream++) {
        size_t dimensions=variant?1:(stream?1:2); double values[6];
        for(size_t index=0;index<frames*dimensions;index++) values[index]=feature_f64(index,stream,variant);
        assert(liblrhsmm_rs_observation_stream_create_f64(dimensions,values,frames*dimensions,&owners[stream])==0);pointers[stream]=owners[stream];
    }
    LiblrhsmmRsObservationF64 *owner=NULL;assert(liblrhsmm_rs_observation_create_f64(frames,pointers,streams,&owner)==0);
    for(size_t stream=0;stream<streams;stream++) assert(liblrhsmm_rs_observation_stream_release_f64(&owners[stream])==0);
    return owner;
}
static void compare_observation_f64(const LiblrhsmmRsObservationF64 *owner,size_t variant) {
    size_t frames=variant?2:3,streams=variant?1:2;LiblrhsmmRsObservationLayout shape;
    assert(liblrhsmm_rs_observation_layout_f64(owner,&shape)==0 && shape.frames==frames && shape.streams==streams);
    for(size_t stream=0;stream<streams;stream++) {
        size_t dimensions=variant?1:(stream?1:2);LiblrhsmmRsObservationStreamLayout layout;
        assert(liblrhsmm_rs_observation_get_stream_layout_f64(owner,stream,&layout)==0 && layout.frames==frames && layout.dimensions==dimensions && layout.values==frames*dimensions);
        double actual[6];assert(liblrhsmm_rs_observation_copy_values_f64(owner,stream,0,actual,frames*dimensions)==0);
        for(size_t index=0;index<frames*dimensions;index++) {double expected=feature_f64(index,stream,variant);assert(!memcmp(&actual[index],&expected,sizeof(expected)));}
    }
}

static void exercise_observation_f64(void) {
    LiblrhsmmRsObservationF64 *first=make_observation_f64(0),*second=make_observation_f64(1),*child=(LiblrhsmmRsObservationF64 *)(uintptr_t)0x1000;
    LiblrhsmmRsObservationSetBuilderF64 *builder=NULL,*builder_copy=NULL;
    LiblrhsmmRsObservationSetF64 *built=NULL,*created=NULL,*copied=NULL,*sentinel=(LiblrhsmmRsObservationSetF64 *)(uintptr_t)0x1000;
    size_t length=99;uint32_t presence=99;
    assert(liblrhsmm_rs_observation_set_builder_new_f64(3,&builder)==0);
    assert(liblrhsmm_rs_observation_set_builder_length_f64(builder,&length)==0 && length==3);
    for(size_t index=0;index<3;index++) {
        assert(liblrhsmm_rs_observation_set_builder_has_sample_f64(builder,index,&presence)==0 && presence==0);
        assert(liblrhsmm_rs_observation_set_builder_get_sample_f64(builder,index,&child)==3 && (uintptr_t)child==0x1000);
    }
    assert(liblrhsmm_rs_observation_set_builder_build_f64(builder,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_observation_set_builder_set_sample_f64(builder,0,first)==0);
    assert(liblrhsmm_rs_observation_set_builder_clone_f64(builder,&builder_copy)==0);
    assert(liblrhsmm_rs_observation_set_builder_has_sample_f64(builder_copy,0,&presence)==0 && presence==1);
    assert(liblrhsmm_rs_observation_set_builder_has_sample_f64(builder_copy,1,&presence)==0 && presence==0);
    assert(liblrhsmm_rs_observation_set_builder_build_f64(builder_copy,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_observation_set_builder_release_f64(&builder_copy)==0);
    assert(liblrhsmm_rs_observation_set_builder_set_sample_f64(builder,1,second)==0 && liblrhsmm_rs_observation_set_builder_set_sample_f64(builder,2,first)==0);
    assert(liblrhsmm_rs_observation_set_builder_clone_f64(builder,&builder_copy)==0);
    assert(liblrhsmm_rs_observation_set_builder_clear_sample_f64(builder,1)==0);
    assert(liblrhsmm_rs_observation_set_builder_has_sample_f64(builder,1,&presence)==0 && presence==0);
    assert(liblrhsmm_rs_observation_set_builder_get_sample_f64(builder_copy,1,&child)==0);compare_observation_f64(child,1);assert(liblrhsmm_rs_observation_release_f64(&child)==0);
    assert(liblrhsmm_rs_observation_set_builder_resize_f64(builder,4)==0);
    assert(liblrhsmm_rs_observation_set_builder_has_sample_f64(builder,3,&presence)==0 && presence==0);
    assert(liblrhsmm_rs_observation_set_builder_resize_f64(builder,1)==0 && liblrhsmm_rs_observation_set_builder_build_f64(builder,&built)==0);
    assert(liblrhsmm_rs_observation_set_length_f64(built,&length)==0 && length==1);
    assert(liblrhsmm_rs_observation_set_get_f64(built,0,&child)==0);compare_observation_f64(child,0);assert(liblrhsmm_rs_observation_release_f64(&child)==0);
    assert(liblrhsmm_rs_observation_set_release_f64(&built)==0 && liblrhsmm_rs_observation_set_builder_build_f64(builder_copy,&built)==0);
    assert(liblrhsmm_rs_observation_set_set_f64(built,1,first)==0 && liblrhsmm_rs_observation_set_append_f64(built,second)==0);
    assert(liblrhsmm_rs_observation_set_length_f64(built,&length)==0 && length==4);
    const size_t changed[4]={0,0,0,1};
    for(size_t index=0;index<4;index++) {assert(liblrhsmm_rs_observation_set_get_f64(built,index,&child)==0);compare_observation_f64(child,changed[index]);assert(liblrhsmm_rs_observation_release_f64(&child)==0);}
    const LiblrhsmmRsObservationF64 *inputs[3]={first,second,first},*bad[2]={first,NULL};
    assert(liblrhsmm_rs_observation_set_replace_f64(built,inputs,3)==0 && liblrhsmm_rs_observation_set_clone_f64(built,&copied)==0 && liblrhsmm_rs_observation_set_create_f64(inputs,3,&created)==0);
    sentinel=created;assert(liblrhsmm_rs_observation_set_create_f64(bad,2,&sentinel)==1 && sentinel==created);
    assert(liblrhsmm_rs_observation_set_replace_f64(built,bad,2)==1);
    assert(liblrhsmm_rs_observation_set_create_f64(NULL,SIZE_MAX,&sentinel)==2 && sentinel==created);
    assert(liblrhsmm_rs_observation_set_create_f64(NULL,1,&sentinel)==1);
    assert(liblrhsmm_rs_observation_set_set_f64(built,99,first)==2 && liblrhsmm_rs_observation_set_set_f64(built,0,NULL)==1 && liblrhsmm_rs_observation_set_append_f64(built,NULL)==1);
    child=(LiblrhsmmRsObservationF64 *)(uintptr_t)0x1000;assert(liblrhsmm_rs_observation_set_get_f64(built,99,&child)==2 && (uintptr_t)child==0x1000);
    assert(liblrhsmm_rs_observation_set_get_f64(built,0,NULL)==1 && liblrhsmm_rs_observation_set_clone_f64(built,NULL)==1 && liblrhsmm_rs_observation_set_length_f64(built,NULL)==1);
    assert(liblrhsmm_rs_observation_set_builder_resize_f64(builder_copy,(size_t)INT32_MAX+1)==2);
    assert(liblrhsmm_rs_observation_set_builder_length_f64(builder_copy,&length)==0 && length==3);
    assert(liblrhsmm_rs_observation_set_builder_has_sample_f64(builder_copy,99,&presence)==2 && liblrhsmm_rs_observation_set_builder_has_sample_f64(builder_copy,0,NULL)==1);
    assert(liblrhsmm_rs_observation_set_builder_set_sample_f64(builder_copy,0,NULL)==1 && liblrhsmm_rs_observation_set_builder_clear_sample_f64(builder_copy,99)==2);
    assert(liblrhsmm_rs_observation_set_builder_build_f64(builder_copy,NULL)==1 && liblrhsmm_rs_observation_set_builder_clone_f64(builder_copy,NULL)==1 && liblrhsmm_rs_observation_set_builder_length_f64(builder_copy,NULL)==1);
    LiblrhsmmRsObservationSetBuilderF64 *builder_sentinel=builder;assert(liblrhsmm_rs_observation_set_builder_new_f64((size_t)INT32_MAX+1,&builder_sentinel)==3 && builder_sentinel==builder);
    assert(liblrhsmm_rs_observation_set_builder_new_f64(SIZE_MAX,NULL)==1);
    assert(liblrhsmm_rs_observation_set_get_f64(copied,1,&child)==0);
    assert(liblrhsmm_rs_observation_set_builder_release_f64(&builder)==0 && liblrhsmm_rs_observation_set_builder_release_f64(&builder_copy)==0);
    assert(liblrhsmm_rs_observation_release_f64(&first)==0 && liblrhsmm_rs_observation_release_f64(&second)==0);
    assert(liblrhsmm_rs_observation_set_release_f64(&copied)==0);compare_observation_f64(child,1);assert(liblrhsmm_rs_observation_release_f64(&child)==0);
    LiblrhsmmRsObservationSetF64 *sets[2]={built,created};const size_t variants[3]={0,1,0};
    for(size_t set=0;set<2;set++) for(size_t index=0;index<3;index++) {assert(liblrhsmm_rs_observation_set_get_f64(sets[set],index,&child)==0);compare_observation_f64(child,variants[index]);assert(liblrhsmm_rs_observation_release_f64(&child)==0);}
    assert(liblrhsmm_rs_observation_set_replace_f64(built,NULL,0)==0 && liblrhsmm_rs_observation_set_length_f64(built,&length)==0 && length==0);
    assert(liblrhsmm_rs_observation_set_release_f64(&built)==0 && liblrhsmm_rs_observation_set_release_f64(&created)==0 && liblrhsmm_rs_observation_set_release_f64(&created)==0 && liblrhsmm_rs_observation_set_release_f64(NULL)==1);
    assert(liblrhsmm_rs_observation_set_builder_release_f64(&builder_copy)==0 && liblrhsmm_rs_observation_set_builder_release_f64(NULL)==1);
    assert(liblrhsmm_rs_observation_set_builder_new_f64(0,&builder)==0 && liblrhsmm_rs_observation_set_builder_build_f64(builder,&built)==0);
    assert(liblrhsmm_rs_observation_set_length_f64(built,&length)==0 && length==0);
    assert(liblrhsmm_rs_observation_set_release_f64(&built)==0 && liblrhsmm_rs_observation_set_builder_release_f64(&builder)==0);
}

static LiblrhsmmRsSegmentationF32 *make_segmentation_f32(size_t variant) {
    size_t streams=variant?1:2,segments=variant?2:3;
    int32_t boundaries[3]={1,2,3},duration[3]={10,11,12},labels[2][3]={{20,21,22},{30,31,32}};
    const int32_t *rows[2]={labels[0],labels[1]};
    volatile float base=(float)0.5;float probability=base+(float)FLT_EPSILON;
    LiblrhsmmRsJumpF32 first[2]={{2,(float)0.25},{1,probability}},ordinary={1,(float)1};
    LiblrhsmmRsJumpSliceF32 jumps[3]={{first,2},{&ordinary,1},{&ordinary,1}};
    LiblrhsmmRsSegmentationF32 *owner=NULL;
    assert(liblrhsmm_rs_segmentation_create_f32(streams,segments,boundaries,duration,rows,jumps,&owner)==0);
    return owner;
}
static void compare_segmentation_f32(const LiblrhsmmRsSegmentationF32 *owner,size_t variant) {
    size_t streams=variant?1:2,segments=variant?2:3;LiblrhsmmRsSegmentationLayout shape;
    assert(liblrhsmm_rs_segmentation_layout_f32(owner,&shape)==0 && shape.streams==streams && shape.segments==segments);
    int32_t actual[3];assert(liblrhsmm_rs_segmentation_copy_boundaries_f32(owner,0,actual,segments)==0);
    for(size_t i=0;i<segments;i++) assert(actual[i]==(int32_t)i+1);
    assert(liblrhsmm_rs_segmentation_copy_duration_states_f32(owner,0,actual,segments)==0);
    for(size_t i=0;i<segments;i++) assert(actual[i]==(int32_t)i+10);
    for(size_t stream=0;stream<streams;stream++) {
        assert(liblrhsmm_rs_segmentation_copy_output_states_f32(owner,stream,0,actual,segments)==0);
        for(size_t i=0;i<segments;i++) assert(actual[i]==(int32_t)(20+stream*10+i));
    }
    for(size_t state=0;state<segments;state++) {
        size_t count=99;assert(liblrhsmm_rs_segmentation_outgoing_count_f32(owner,state,&count)==0 && count==(state?1:2));
        LiblrhsmmRsJumpF32 jumps[2];assert(liblrhsmm_rs_segmentation_copy_outgoing_f32(owner,state,0,jumps,count)==0);
        volatile float base=(float)0.5;
        for(size_t index=0;index<count;index++) {
            int32_t delta=state?1:(index?1:2);
            float probability=state?(float)1:(index?base+(float)FLT_EPSILON:(float)0.25);
            assert(jumps[index].delta==delta && !memcmp(&jumps[index].probability,&probability,sizeof(probability)));
        }
    }
}

static void exercise_segmentation_f32(void) {
    LiblrhsmmRsSegmentationF32 *first=make_segmentation_f32(0),*second=make_segmentation_f32(1),*child=(LiblrhsmmRsSegmentationF32 *)(uintptr_t)0x1000;
    LiblrhsmmRsSegmentationSetBuilderF32 *builder=NULL,*builder_copy=NULL;
    LiblrhsmmRsSegmentationSetF32 *built=NULL,*created=NULL,*copied=NULL,*sentinel=(LiblrhsmmRsSegmentationSetF32 *)(uintptr_t)0x1000;
    size_t length=99;uint32_t presence=99;
    assert(liblrhsmm_rs_segmentation_set_builder_new_f32(3,&builder)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_length_f32(builder,&length)==0 && length==3);
    for(size_t index=0;index<3;index++) {
        assert(liblrhsmm_rs_segmentation_set_builder_has_sample_f32(builder,index,&presence)==0 && presence==0);
        assert(liblrhsmm_rs_segmentation_set_builder_get_sample_f32(builder,index,&child)==3 && (uintptr_t)child==0x1000);
    }
    assert(liblrhsmm_rs_segmentation_set_builder_build_f32(builder,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_segmentation_set_builder_set_sample_f32(builder,0,first)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_clone_f32(builder,&builder_copy)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_has_sample_f32(builder_copy,0,&presence)==0 && presence==1);
    assert(liblrhsmm_rs_segmentation_set_builder_has_sample_f32(builder_copy,1,&presence)==0 && presence==0);
    assert(liblrhsmm_rs_segmentation_set_builder_build_f32(builder_copy,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_segmentation_set_builder_release_f32(&builder_copy)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_set_sample_f32(builder,1,second)==0 && liblrhsmm_rs_segmentation_set_builder_set_sample_f32(builder,2,first)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_clone_f32(builder,&builder_copy)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_clear_sample_f32(builder,1)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_has_sample_f32(builder,1,&presence)==0 && presence==0);
    assert(liblrhsmm_rs_segmentation_set_builder_get_sample_f32(builder_copy,1,&child)==0);compare_segmentation_f32(child,1);assert(liblrhsmm_rs_segmentation_release_f32(&child)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_resize_f32(builder,4)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_has_sample_f32(builder,3,&presence)==0 && presence==0);
    assert(liblrhsmm_rs_segmentation_set_builder_resize_f32(builder,1)==0 && liblrhsmm_rs_segmentation_set_builder_build_f32(builder,&built)==0);
    assert(liblrhsmm_rs_segmentation_set_length_f32(built,&length)==0 && length==1);
    assert(liblrhsmm_rs_segmentation_set_get_f32(built,0,&child)==0);compare_segmentation_f32(child,0);assert(liblrhsmm_rs_segmentation_release_f32(&child)==0);
    assert(liblrhsmm_rs_segmentation_set_release_f32(&built)==0 && liblrhsmm_rs_segmentation_set_builder_build_f32(builder_copy,&built)==0);
    assert(liblrhsmm_rs_segmentation_set_set_f32(built,1,first)==0 && liblrhsmm_rs_segmentation_set_append_f32(built,second)==0);
    assert(liblrhsmm_rs_segmentation_set_length_f32(built,&length)==0 && length==4);
    const size_t changed[4]={0,0,0,1};
    for(size_t index=0;index<4;index++) {assert(liblrhsmm_rs_segmentation_set_get_f32(built,index,&child)==0);compare_segmentation_f32(child,changed[index]);assert(liblrhsmm_rs_segmentation_release_f32(&child)==0);}
    const LiblrhsmmRsSegmentationF32 *inputs[3]={first,second,first},*bad[2]={first,NULL};
    assert(liblrhsmm_rs_segmentation_set_replace_f32(built,inputs,3)==0 && liblrhsmm_rs_segmentation_set_clone_f32(built,&copied)==0 && liblrhsmm_rs_segmentation_set_create_f32(inputs,3,&created)==0);
    sentinel=created;assert(liblrhsmm_rs_segmentation_set_create_f32(bad,2,&sentinel)==1 && sentinel==created);
    assert(liblrhsmm_rs_segmentation_set_replace_f32(built,bad,2)==1);
    assert(liblrhsmm_rs_segmentation_set_create_f32(NULL,SIZE_MAX,&sentinel)==2 && sentinel==created);
    assert(liblrhsmm_rs_segmentation_set_create_f32(NULL,1,&sentinel)==1);
    assert(liblrhsmm_rs_segmentation_set_set_f32(built,99,first)==2 && liblrhsmm_rs_segmentation_set_set_f32(built,0,NULL)==1 && liblrhsmm_rs_segmentation_set_append_f32(built,NULL)==1);
    child=(LiblrhsmmRsSegmentationF32 *)(uintptr_t)0x1000;assert(liblrhsmm_rs_segmentation_set_get_f32(built,99,&child)==2 && (uintptr_t)child==0x1000);
    assert(liblrhsmm_rs_segmentation_set_get_f32(built,0,NULL)==1 && liblrhsmm_rs_segmentation_set_clone_f32(built,NULL)==1 && liblrhsmm_rs_segmentation_set_length_f32(built,NULL)==1);
    assert(liblrhsmm_rs_segmentation_set_builder_resize_f32(builder_copy,(size_t)INT32_MAX+1)==2);
    assert(liblrhsmm_rs_segmentation_set_builder_length_f32(builder_copy,&length)==0 && length==3);
    assert(liblrhsmm_rs_segmentation_set_builder_has_sample_f32(builder_copy,99,&presence)==2 && liblrhsmm_rs_segmentation_set_builder_has_sample_f32(builder_copy,0,NULL)==1);
    assert(liblrhsmm_rs_segmentation_set_builder_set_sample_f32(builder_copy,0,NULL)==1 && liblrhsmm_rs_segmentation_set_builder_clear_sample_f32(builder_copy,99)==2);
    assert(liblrhsmm_rs_segmentation_set_builder_build_f32(builder_copy,NULL)==1 && liblrhsmm_rs_segmentation_set_builder_clone_f32(builder_copy,NULL)==1 && liblrhsmm_rs_segmentation_set_builder_length_f32(builder_copy,NULL)==1);
    LiblrhsmmRsSegmentationSetBuilderF32 *builder_sentinel=builder;assert(liblrhsmm_rs_segmentation_set_builder_new_f32((size_t)INT32_MAX+1,&builder_sentinel)==3 && builder_sentinel==builder);
    assert(liblrhsmm_rs_segmentation_set_builder_new_f32(SIZE_MAX,NULL)==1);
    assert(liblrhsmm_rs_segmentation_set_get_f32(copied,1,&child)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_release_f32(&builder)==0 && liblrhsmm_rs_segmentation_set_builder_release_f32(&builder_copy)==0);
    assert(liblrhsmm_rs_segmentation_release_f32(&first)==0 && liblrhsmm_rs_segmentation_release_f32(&second)==0);
    assert(liblrhsmm_rs_segmentation_set_release_f32(&copied)==0);compare_segmentation_f32(child,1);assert(liblrhsmm_rs_segmentation_release_f32(&child)==0);
    LiblrhsmmRsSegmentationSetF32 *sets[2]={built,created};const size_t variants[3]={0,1,0};
    for(size_t set=0;set<2;set++) for(size_t index=0;index<3;index++) {assert(liblrhsmm_rs_segmentation_set_get_f32(sets[set],index,&child)==0);compare_segmentation_f32(child,variants[index]);assert(liblrhsmm_rs_segmentation_release_f32(&child)==0);}
    assert(liblrhsmm_rs_segmentation_set_replace_f32(built,NULL,0)==0 && liblrhsmm_rs_segmentation_set_length_f32(built,&length)==0 && length==0);
    assert(liblrhsmm_rs_segmentation_set_release_f32(&built)==0 && liblrhsmm_rs_segmentation_set_release_f32(&created)==0 && liblrhsmm_rs_segmentation_set_release_f32(&created)==0 && liblrhsmm_rs_segmentation_set_release_f32(NULL)==1);
    assert(liblrhsmm_rs_segmentation_set_builder_release_f32(&builder_copy)==0 && liblrhsmm_rs_segmentation_set_builder_release_f32(NULL)==1);
    assert(liblrhsmm_rs_segmentation_set_builder_new_f32(0,&builder)==0 && liblrhsmm_rs_segmentation_set_builder_build_f32(builder,&built)==0);
    assert(liblrhsmm_rs_segmentation_set_length_f32(built,&length)==0 && length==0);
    assert(liblrhsmm_rs_segmentation_set_release_f32(&built)==0 && liblrhsmm_rs_segmentation_set_builder_release_f32(&builder)==0);
}

static LiblrhsmmRsSegmentationF64 *make_segmentation_f64(size_t variant) {
    size_t streams=variant?1:2,segments=variant?2:3;
    int32_t boundaries[3]={1,2,3},duration[3]={10,11,12},labels[2][3]={{20,21,22},{30,31,32}};
    const int32_t *rows[2]={labels[0],labels[1]};
    volatile double base=(double)0.5;double probability=base+(double)DBL_EPSILON;
    LiblrhsmmRsJumpF64 first[2]={{2,(double)0.25},{1,probability}},ordinary={1,(double)1};
    LiblrhsmmRsJumpSliceF64 jumps[3]={{first,2},{&ordinary,1},{&ordinary,1}};
    LiblrhsmmRsSegmentationF64 *owner=NULL;
    assert(liblrhsmm_rs_segmentation_create_f64(streams,segments,boundaries,duration,rows,jumps,&owner)==0);
    return owner;
}
static void compare_segmentation_f64(const LiblrhsmmRsSegmentationF64 *owner,size_t variant) {
    size_t streams=variant?1:2,segments=variant?2:3;LiblrhsmmRsSegmentationLayout shape;
    assert(liblrhsmm_rs_segmentation_layout_f64(owner,&shape)==0 && shape.streams==streams && shape.segments==segments);
    int32_t actual[3];assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(owner,0,actual,segments)==0);
    for(size_t i=0;i<segments;i++) assert(actual[i]==(int32_t)i+1);
    assert(liblrhsmm_rs_segmentation_copy_duration_states_f64(owner,0,actual,segments)==0);
    for(size_t i=0;i<segments;i++) assert(actual[i]==(int32_t)i+10);
    for(size_t stream=0;stream<streams;stream++) {
        assert(liblrhsmm_rs_segmentation_copy_output_states_f64(owner,stream,0,actual,segments)==0);
        for(size_t i=0;i<segments;i++) assert(actual[i]==(int32_t)(20+stream*10+i));
    }
    for(size_t state=0;state<segments;state++) {
        size_t count=99;assert(liblrhsmm_rs_segmentation_outgoing_count_f64(owner,state,&count)==0 && count==(state?1:2));
        LiblrhsmmRsJumpF64 jumps[2];assert(liblrhsmm_rs_segmentation_copy_outgoing_f64(owner,state,0,jumps,count)==0);
        volatile double base=(double)0.5;
        for(size_t index=0;index<count;index++) {
            int32_t delta=state?1:(index?1:2);
            double probability=state?(double)1:(index?base+(double)DBL_EPSILON:(double)0.25);
            assert(jumps[index].delta==delta && !memcmp(&jumps[index].probability,&probability,sizeof(probability)));
        }
    }
}

static void exercise_segmentation_f64(void) {
    LiblrhsmmRsSegmentationF64 *first=make_segmentation_f64(0),*second=make_segmentation_f64(1),*child=(LiblrhsmmRsSegmentationF64 *)(uintptr_t)0x1000;
    LiblrhsmmRsSegmentationSetBuilderF64 *builder=NULL,*builder_copy=NULL;
    LiblrhsmmRsSegmentationSetF64 *built=NULL,*created=NULL,*copied=NULL,*sentinel=(LiblrhsmmRsSegmentationSetF64 *)(uintptr_t)0x1000;
    size_t length=99;uint32_t presence=99;
    assert(liblrhsmm_rs_segmentation_set_builder_new_f64(3,&builder)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_length_f64(builder,&length)==0 && length==3);
    for(size_t index=0;index<3;index++) {
        assert(liblrhsmm_rs_segmentation_set_builder_has_sample_f64(builder,index,&presence)==0 && presence==0);
        assert(liblrhsmm_rs_segmentation_set_builder_get_sample_f64(builder,index,&child)==3 && (uintptr_t)child==0x1000);
    }
    assert(liblrhsmm_rs_segmentation_set_builder_build_f64(builder,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_segmentation_set_builder_set_sample_f64(builder,0,first)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_clone_f64(builder,&builder_copy)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_has_sample_f64(builder_copy,0,&presence)==0 && presence==1);
    assert(liblrhsmm_rs_segmentation_set_builder_has_sample_f64(builder_copy,1,&presence)==0 && presence==0);
    assert(liblrhsmm_rs_segmentation_set_builder_build_f64(builder_copy,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_segmentation_set_builder_release_f64(&builder_copy)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_set_sample_f64(builder,1,second)==0 && liblrhsmm_rs_segmentation_set_builder_set_sample_f64(builder,2,first)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_clone_f64(builder,&builder_copy)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_clear_sample_f64(builder,1)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_has_sample_f64(builder,1,&presence)==0 && presence==0);
    assert(liblrhsmm_rs_segmentation_set_builder_get_sample_f64(builder_copy,1,&child)==0);compare_segmentation_f64(child,1);assert(liblrhsmm_rs_segmentation_release_f64(&child)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_resize_f64(builder,4)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_has_sample_f64(builder,3,&presence)==0 && presence==0);
    assert(liblrhsmm_rs_segmentation_set_builder_resize_f64(builder,1)==0 && liblrhsmm_rs_segmentation_set_builder_build_f64(builder,&built)==0);
    assert(liblrhsmm_rs_segmentation_set_length_f64(built,&length)==0 && length==1);
    assert(liblrhsmm_rs_segmentation_set_get_f64(built,0,&child)==0);compare_segmentation_f64(child,0);assert(liblrhsmm_rs_segmentation_release_f64(&child)==0);
    assert(liblrhsmm_rs_segmentation_set_release_f64(&built)==0 && liblrhsmm_rs_segmentation_set_builder_build_f64(builder_copy,&built)==0);
    assert(liblrhsmm_rs_segmentation_set_set_f64(built,1,first)==0 && liblrhsmm_rs_segmentation_set_append_f64(built,second)==0);
    assert(liblrhsmm_rs_segmentation_set_length_f64(built,&length)==0 && length==4);
    const size_t changed[4]={0,0,0,1};
    for(size_t index=0;index<4;index++) {assert(liblrhsmm_rs_segmentation_set_get_f64(built,index,&child)==0);compare_segmentation_f64(child,changed[index]);assert(liblrhsmm_rs_segmentation_release_f64(&child)==0);}
    const LiblrhsmmRsSegmentationF64 *inputs[3]={first,second,first},*bad[2]={first,NULL};
    assert(liblrhsmm_rs_segmentation_set_replace_f64(built,inputs,3)==0 && liblrhsmm_rs_segmentation_set_clone_f64(built,&copied)==0 && liblrhsmm_rs_segmentation_set_create_f64(inputs,3,&created)==0);
    sentinel=created;assert(liblrhsmm_rs_segmentation_set_create_f64(bad,2,&sentinel)==1 && sentinel==created);
    assert(liblrhsmm_rs_segmentation_set_replace_f64(built,bad,2)==1);
    assert(liblrhsmm_rs_segmentation_set_create_f64(NULL,SIZE_MAX,&sentinel)==2 && sentinel==created);
    assert(liblrhsmm_rs_segmentation_set_create_f64(NULL,1,&sentinel)==1);
    assert(liblrhsmm_rs_segmentation_set_set_f64(built,99,first)==2 && liblrhsmm_rs_segmentation_set_set_f64(built,0,NULL)==1 && liblrhsmm_rs_segmentation_set_append_f64(built,NULL)==1);
    child=(LiblrhsmmRsSegmentationF64 *)(uintptr_t)0x1000;assert(liblrhsmm_rs_segmentation_set_get_f64(built,99,&child)==2 && (uintptr_t)child==0x1000);
    assert(liblrhsmm_rs_segmentation_set_get_f64(built,0,NULL)==1 && liblrhsmm_rs_segmentation_set_clone_f64(built,NULL)==1 && liblrhsmm_rs_segmentation_set_length_f64(built,NULL)==1);
    assert(liblrhsmm_rs_segmentation_set_builder_resize_f64(builder_copy,(size_t)INT32_MAX+1)==2);
    assert(liblrhsmm_rs_segmentation_set_builder_length_f64(builder_copy,&length)==0 && length==3);
    assert(liblrhsmm_rs_segmentation_set_builder_has_sample_f64(builder_copy,99,&presence)==2 && liblrhsmm_rs_segmentation_set_builder_has_sample_f64(builder_copy,0,NULL)==1);
    assert(liblrhsmm_rs_segmentation_set_builder_set_sample_f64(builder_copy,0,NULL)==1 && liblrhsmm_rs_segmentation_set_builder_clear_sample_f64(builder_copy,99)==2);
    assert(liblrhsmm_rs_segmentation_set_builder_build_f64(builder_copy,NULL)==1 && liblrhsmm_rs_segmentation_set_builder_clone_f64(builder_copy,NULL)==1 && liblrhsmm_rs_segmentation_set_builder_length_f64(builder_copy,NULL)==1);
    LiblrhsmmRsSegmentationSetBuilderF64 *builder_sentinel=builder;assert(liblrhsmm_rs_segmentation_set_builder_new_f64((size_t)INT32_MAX+1,&builder_sentinel)==3 && builder_sentinel==builder);
    assert(liblrhsmm_rs_segmentation_set_builder_new_f64(SIZE_MAX,NULL)==1);
    assert(liblrhsmm_rs_segmentation_set_get_f64(copied,1,&child)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_release_f64(&builder)==0 && liblrhsmm_rs_segmentation_set_builder_release_f64(&builder_copy)==0);
    assert(liblrhsmm_rs_segmentation_release_f64(&first)==0 && liblrhsmm_rs_segmentation_release_f64(&second)==0);
    assert(liblrhsmm_rs_segmentation_set_release_f64(&copied)==0);compare_segmentation_f64(child,1);assert(liblrhsmm_rs_segmentation_release_f64(&child)==0);
    LiblrhsmmRsSegmentationSetF64 *sets[2]={built,created};const size_t variants[3]={0,1,0};
    for(size_t set=0;set<2;set++) for(size_t index=0;index<3;index++) {assert(liblrhsmm_rs_segmentation_set_get_f64(sets[set],index,&child)==0);compare_segmentation_f64(child,variants[index]);assert(liblrhsmm_rs_segmentation_release_f64(&child)==0);}
    assert(liblrhsmm_rs_segmentation_set_replace_f64(built,NULL,0)==0 && liblrhsmm_rs_segmentation_set_length_f64(built,&length)==0 && length==0);
    assert(liblrhsmm_rs_segmentation_set_release_f64(&built)==0 && liblrhsmm_rs_segmentation_set_release_f64(&created)==0 && liblrhsmm_rs_segmentation_set_release_f64(&created)==0 && liblrhsmm_rs_segmentation_set_release_f64(NULL)==1);
    assert(liblrhsmm_rs_segmentation_set_builder_release_f64(&builder_copy)==0 && liblrhsmm_rs_segmentation_set_builder_release_f64(NULL)==1);
    assert(liblrhsmm_rs_segmentation_set_builder_new_f64(0,&builder)==0 && liblrhsmm_rs_segmentation_set_builder_build_f64(builder,&built)==0);
    assert(liblrhsmm_rs_segmentation_set_length_f64(built,&length)==0 && length==0);
    assert(liblrhsmm_rs_segmentation_set_release_f64(&built)==0 && liblrhsmm_rs_segmentation_set_builder_release_f64(&builder)==0);
}

static void empty_observation_f32(void) {
    LiblrhsmmRsObservationF32 *empty=NULL,*child=NULL;
    LiblrhsmmRsObservationSetBuilderF32 *builder=NULL,*copy=NULL;
    LiblrhsmmRsObservationSetF32 *set=NULL,*sentinel=(LiblrhsmmRsObservationSetF32 *)(uintptr_t)0x1000;
    size_t length=99;uint32_t present=99;LiblrhsmmRsObservationLayout shape;
    size_t dimensions[1]={2}; assert(liblrhsmm_rs_observation_new_f32(0,dimensions,1,&empty)==0);
    assert(liblrhsmm_rs_observation_set_builder_new_f32(2,&builder)==0);
    assert(liblrhsmm_rs_observation_set_builder_set_sample_f32(builder,0,empty)==0);
    assert(liblrhsmm_rs_observation_set_builder_clone_f32(builder,&copy)==0);
    assert(liblrhsmm_rs_observation_set_builder_has_sample_f32(copy,0,&present)==0 && present==1);
    assert(liblrhsmm_rs_observation_set_builder_has_sample_f32(copy,1,&present)==0 && present==0);
    assert(liblrhsmm_rs_observation_set_builder_build_f32(copy,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_observation_set_builder_get_sample_f32(copy,0,&child)==0);
    assert(liblrhsmm_rs_observation_layout_f32(child,&shape)==0 && shape.frames==0 && shape.streams==1);
    assert(liblrhsmm_rs_observation_release_f32(&child)==0);
    assert(liblrhsmm_rs_observation_set_builder_resize_f32(copy,1)==0 && liblrhsmm_rs_observation_set_builder_build_f32(copy,&set)==0);
    assert(liblrhsmm_rs_observation_set_length_f32(set,&length)==0 && length==1);
    assert(liblrhsmm_rs_observation_set_get_f32(set,0,&child)==0);
    assert(liblrhsmm_rs_observation_release_f32(&empty)==0);
    assert(liblrhsmm_rs_observation_set_builder_release_f32(&builder)==0 && liblrhsmm_rs_observation_set_builder_release_f32(&copy)==0);
    assert(liblrhsmm_rs_observation_set_release_f32(&set)==0);
    assert(liblrhsmm_rs_observation_layout_f32(child,&shape)==0 && shape.frames==0 && shape.streams==1);
    assert(liblrhsmm_rs_observation_release_f32(&child)==0);
}

static void empty_observation_f64(void) {
    LiblrhsmmRsObservationF64 *empty=NULL,*child=NULL;
    LiblrhsmmRsObservationSetBuilderF64 *builder=NULL,*copy=NULL;
    LiblrhsmmRsObservationSetF64 *set=NULL,*sentinel=(LiblrhsmmRsObservationSetF64 *)(uintptr_t)0x1000;
    size_t length=99;uint32_t present=99;LiblrhsmmRsObservationLayout shape;
    size_t dimensions[1]={2}; assert(liblrhsmm_rs_observation_new_f64(0,dimensions,1,&empty)==0);
    assert(liblrhsmm_rs_observation_set_builder_new_f64(2,&builder)==0);
    assert(liblrhsmm_rs_observation_set_builder_set_sample_f64(builder,0,empty)==0);
    assert(liblrhsmm_rs_observation_set_builder_clone_f64(builder,&copy)==0);
    assert(liblrhsmm_rs_observation_set_builder_has_sample_f64(copy,0,&present)==0 && present==1);
    assert(liblrhsmm_rs_observation_set_builder_has_sample_f64(copy,1,&present)==0 && present==0);
    assert(liblrhsmm_rs_observation_set_builder_build_f64(copy,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_observation_set_builder_get_sample_f64(copy,0,&child)==0);
    assert(liblrhsmm_rs_observation_layout_f64(child,&shape)==0 && shape.frames==0 && shape.streams==1);
    assert(liblrhsmm_rs_observation_release_f64(&child)==0);
    assert(liblrhsmm_rs_observation_set_builder_resize_f64(copy,1)==0 && liblrhsmm_rs_observation_set_builder_build_f64(copy,&set)==0);
    assert(liblrhsmm_rs_observation_set_length_f64(set,&length)==0 && length==1);
    assert(liblrhsmm_rs_observation_set_get_f64(set,0,&child)==0);
    assert(liblrhsmm_rs_observation_release_f64(&empty)==0);
    assert(liblrhsmm_rs_observation_set_builder_release_f64(&builder)==0 && liblrhsmm_rs_observation_set_builder_release_f64(&copy)==0);
    assert(liblrhsmm_rs_observation_set_release_f64(&set)==0);
    assert(liblrhsmm_rs_observation_layout_f64(child,&shape)==0 && shape.frames==0 && shape.streams==1);
    assert(liblrhsmm_rs_observation_release_f64(&child)==0);
}

static void empty_segmentation_f32(void) {
    LiblrhsmmRsSegmentationF32 *empty=NULL,*child=NULL;
    LiblrhsmmRsSegmentationSetBuilderF32 *builder=NULL,*copy=NULL;
    LiblrhsmmRsSegmentationSetF32 *set=NULL,*sentinel=(LiblrhsmmRsSegmentationSetF32 *)(uintptr_t)0x1000;
    size_t length=99;uint32_t present=99;LiblrhsmmRsSegmentationLayout shape;
    assert(liblrhsmm_rs_segmentation_new_f32(1,0,&empty)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_new_f32(2,&builder)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_set_sample_f32(builder,0,empty)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_clone_f32(builder,&copy)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_has_sample_f32(copy,0,&present)==0 && present==1);
    assert(liblrhsmm_rs_segmentation_set_builder_has_sample_f32(copy,1,&present)==0 && present==0);
    assert(liblrhsmm_rs_segmentation_set_builder_build_f32(copy,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_segmentation_set_builder_get_sample_f32(copy,0,&child)==0);
    assert(liblrhsmm_rs_segmentation_layout_f32(child,&shape)==0 && shape.streams==1 && shape.segments==0);
    assert(liblrhsmm_rs_segmentation_release_f32(&child)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_resize_f32(copy,1)==0 && liblrhsmm_rs_segmentation_set_builder_build_f32(copy,&set)==0);
    assert(liblrhsmm_rs_segmentation_set_length_f32(set,&length)==0 && length==1);
    assert(liblrhsmm_rs_segmentation_set_get_f32(set,0,&child)==0);
    assert(liblrhsmm_rs_segmentation_release_f32(&empty)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_release_f32(&builder)==0 && liblrhsmm_rs_segmentation_set_builder_release_f32(&copy)==0);
    assert(liblrhsmm_rs_segmentation_set_release_f32(&set)==0);
    assert(liblrhsmm_rs_segmentation_layout_f32(child,&shape)==0 && shape.streams==1 && shape.segments==0);
    assert(liblrhsmm_rs_segmentation_release_f32(&child)==0);
}

static void empty_segmentation_f64(void) {
    LiblrhsmmRsSegmentationF64 *empty=NULL,*child=NULL;
    LiblrhsmmRsSegmentationSetBuilderF64 *builder=NULL,*copy=NULL;
    LiblrhsmmRsSegmentationSetF64 *set=NULL,*sentinel=(LiblrhsmmRsSegmentationSetF64 *)(uintptr_t)0x1000;
    size_t length=99;uint32_t present=99;LiblrhsmmRsSegmentationLayout shape;
    assert(liblrhsmm_rs_segmentation_new_f64(1,0,&empty)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_new_f64(2,&builder)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_set_sample_f64(builder,0,empty)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_clone_f64(builder,&copy)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_has_sample_f64(copy,0,&present)==0 && present==1);
    assert(liblrhsmm_rs_segmentation_set_builder_has_sample_f64(copy,1,&present)==0 && present==0);
    assert(liblrhsmm_rs_segmentation_set_builder_build_f64(copy,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_segmentation_set_builder_get_sample_f64(copy,0,&child)==0);
    assert(liblrhsmm_rs_segmentation_layout_f64(child,&shape)==0 && shape.streams==1 && shape.segments==0);
    assert(liblrhsmm_rs_segmentation_release_f64(&child)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_resize_f64(copy,1)==0 && liblrhsmm_rs_segmentation_set_builder_build_f64(copy,&set)==0);
    assert(liblrhsmm_rs_segmentation_set_length_f64(set,&length)==0 && length==1);
    assert(liblrhsmm_rs_segmentation_set_get_f64(set,0,&child)==0);
    assert(liblrhsmm_rs_segmentation_release_f64(&empty)==0);
    assert(liblrhsmm_rs_segmentation_set_builder_release_f64(&builder)==0 && liblrhsmm_rs_segmentation_set_builder_release_f64(&copy)==0);
    assert(liblrhsmm_rs_segmentation_set_release_f64(&set)==0);
    assert(liblrhsmm_rs_segmentation_layout_f64(child,&shape)==0 && shape.streams==1 && shape.segments==0);
    assert(liblrhsmm_rs_segmentation_release_f64(&child)==0);
}
int main(void) {
empty_observation_f32();
empty_observation_f64();
empty_segmentation_f32();
empty_segmentation_f64();

exercise_observation_f32();
exercise_observation_f64();
exercise_segmentation_f32();
exercise_segmentation_f64();
puts("Sample sets C: all72 exports, complete fields, missing slots and independent lifetimes passed");return 0;}
