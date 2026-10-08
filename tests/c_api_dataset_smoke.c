#include "liblrhsmm_rs.h"
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <float.h>
#include <stdio.h>
#include <string.h>


static float feature_f32(size_t variant,size_t stream,size_t dimension,size_t index) {
    if(!variant) { (void)dimension;return (float)(index / dimension) * 10 + (float)stream * 100 + (float)(index % dimension); }
    volatile float base=(float)2+(float)index/(float)8;
    return base+FLT_EPSILON;
}
static void sample_f32(size_t variant,LiblrhsmmRsObservationF32 **observation,LiblrhsmmRsSegmentationF32 **segmentation) {
    size_t frames=variant==0?8:variant==1?3:0,streams=variant==0?2:1;
    LiblrhsmmRsObservationStreamF32 *owners[2]={0};
    const LiblrhsmmRsObservationStreamF32 *pointers[2];
    for(size_t stream=0;stream<streams;stream++) {
        size_t dimension=variant==0 && stream==0?2:1;float values[16];
        for(size_t i=0;i<frames*dimension;i++) values[i]=feature_f32(variant,stream,dimension,i);
        assert(liblrhsmm_rs_observation_stream_create_f32(dimension,values,frames*dimension,&owners[stream])==0);
        pointers[stream]=owners[stream];
    }
    assert(liblrhsmm_rs_observation_create_f32(frames,pointers,streams,observation)==0);
    for(size_t stream=0;stream<streams;stream++) assert(liblrhsmm_rs_observation_stream_release_f32(&owners[stream])==0);
    size_t segments=variant==0?5:variant==1?3:0;
    int32_t first[5]={1,3,3,6,8},second[3]={0,1,3},durations[5]={10,11,12,13,14};
    int32_t labels[2][5]={{20,21,22,23,24},{40,39,38,37,36}};
    const int32_t *rows[2]={labels[0],labels[1]};
    LiblrhsmmRsJumpF32 ordinary={1,1},extra[2]={{2,(float).25+FLT_EPSILON},{1,1}};
    LiblrhsmmRsJumpSliceF32 jumps[5];
    for(size_t state=0;state<segments;state++) { jumps[state].jumps=&ordinary;jumps[state].count=1; }
    if(variant==1) { jumps[0].jumps=extra;jumps[0].count=2; }
    assert(liblrhsmm_rs_segmentation_create_f32(streams,segments,variant?second:first,durations,rows,jumps,segmentation)==0);
}
static void compare_f32(const LiblrhsmmRsObservationF32 *observation,const LiblrhsmmRsSegmentationF32 *segmentation,size_t variant,size_t first,size_t end) {
    const int32_t bounds0[5]={1,3,3,6,8},bounds1[3]={0,1,3};
    const int32_t *bounds=variant?bounds1:bounds0;
    size_t begin=first?(size_t)bounds[first-1]:0,frames=(size_t)bounds[end-1]-begin,streams=variant?1:2,segments=end-first;
    LiblrhsmmRsObservationLayout shape;
    assert(liblrhsmm_rs_observation_layout_f32(observation,&shape)==0 && shape.frames==frames && shape.streams==streams);
    for(size_t stream=0;stream<streams;stream++) {
        size_t dimension=!variant && !stream?2:1;LiblrhsmmRsObservationStreamLayout layout;
        assert(liblrhsmm_rs_observation_get_stream_layout_f32(observation,stream,&layout)==0 && layout.frames==frames && layout.dimensions==dimension && layout.values==frames*dimension);
        float actual[16];assert(liblrhsmm_rs_observation_copy_values_f32(observation,stream,0,actual,frames*dimension)==0);
        for(size_t i=0;i<frames*dimension;i++) { float expected=feature_f32(variant,stream,dimension,begin*dimension+i);assert(!memcmp(&actual[i],&expected,sizeof(expected))); }
    }
    LiblrhsmmRsSegmentationLayout layout;
    assert(liblrhsmm_rs_segmentation_layout_f32(segmentation,&layout)==0 && layout.streams==streams && layout.segments==segments);
    int32_t actual[5];assert(liblrhsmm_rs_segmentation_copy_boundaries_f32(segmentation,0,actual,segments)==0);
    for(size_t i=0;i<segments;i++) assert(actual[i]==bounds[first+i]-(int32_t)begin);
    assert(liblrhsmm_rs_segmentation_copy_duration_states_f32(segmentation,0,actual,segments)==0);
    for(size_t i=0;i<segments;i++) assert(actual[i]==(int32_t)(10+first+i));
    for(size_t stream=0;stream<streams;stream++) {
        assert(liblrhsmm_rs_segmentation_copy_output_states_f32(segmentation,stream,0,actual,segments)==0);
        for(size_t i=0;i<segments;i++) assert(actual[i]==(stream?(int32_t)(40-first-i):(int32_t)(20+first+i)));
    }
    for(size_t state=0;state<segments;state++) {
        size_t count=99;assert(liblrhsmm_rs_segmentation_outgoing_count_f32(segmentation,state,&count)==0 && count==1);
        LiblrhsmmRsJumpF32 jump;float one=1;
        assert(liblrhsmm_rs_segmentation_copy_outgoing_f32(segmentation,state,0,&jump,1)==0 && jump.delta==1 && !memcmp(&jump.probability,&one,sizeof(one)));
    }
}

static void exercise_f32(void) {
    LiblrhsmmRsObservationF32 *children_obs[3]={0};LiblrhsmmRsSegmentationF32 *children_seg[3]={0};
    for(size_t variant=0;variant<3;variant++) sample_f32(variant,&children_obs[variant],&children_seg[variant]);
    const LiblrhsmmRsObservationF32 *obs[4]={children_obs[0],children_obs[1],children_obs[0],children_obs[2]};
    const LiblrhsmmRsSegmentationF32 *seg[4]={children_seg[0],children_seg[1],children_seg[0],children_seg[2]};
    LiblrhsmmRsObservationSetF32 *observations=NULL;LiblrhsmmRsSegmentationSetF32 *segmentations=NULL;
    assert(liblrhsmm_rs_observation_set_create_f32(obs,4,&observations)==0);
    assert(liblrhsmm_rs_segmentation_set_create_f32(seg,4,&segmentations)==0);
    const size_t units[6]={1,2,3,5,8,SIZE_MAX};
    for(size_t u=0;u<6;u++) {
        size_t unit=units[u],length=99;
        LiblrhsmmRsDatasetF32 *dataset=NULL,*copy=NULL;
        assert(liblrhsmm_rs_regroup_data_f32(observations,segmentations,unit,&dataset)==0);
        assert(liblrhsmm_rs_dataset_length_f32(dataset,&length)==0 && length==2*(5/unit+(5%unit!=0))+3/unit+(3%unit!=0));
        assert(liblrhsmm_rs_dataset_clone_f32(dataset,&copy)==0);
        assert(liblrhsmm_rs_dataset_release_f32(&dataset)==0);
        LiblrhsmmRsObservationSetF32 *result_observations=NULL;LiblrhsmmRsSegmentationSetF32 *result_segmentations=NULL;
        assert(liblrhsmm_rs_dataset_get_observations_f32(copy,&result_observations)==0);
        assert(liblrhsmm_rs_dataset_get_segmentations_f32(copy,&result_segmentations)==0);
        assert(liblrhsmm_rs_dataset_release_f32(&copy)==0);
        size_t index=0;
        for(size_t sample=0;sample<3;sample++) {
            size_t variant=sample==1?1:0,segments=variant?3:5;
            for(size_t first=0;first<segments;) {
                size_t end=unit>segments-first?segments:first+unit;
                LiblrhsmmRsObservationF32 *observation=NULL;LiblrhsmmRsSegmentationF32 *segmentation=NULL;
                assert(liblrhsmm_rs_observation_set_get_f32(result_observations,index,&observation)==0);
                assert(liblrhsmm_rs_segmentation_set_get_f32(result_segmentations,index,&segmentation)==0);
                compare_f32(observation,segmentation,variant,first,end);
                assert(liblrhsmm_rs_observation_release_f32(&observation)==0 && liblrhsmm_rs_segmentation_release_f32(&segmentation)==0);
                first=end;index++;
            }
        }
        assert(index==length);
        assert(liblrhsmm_rs_observation_set_release_f32(&result_observations)==0 && liblrhsmm_rs_segmentation_set_release_f32(&result_segmentations)==0);
    }

    LiblrhsmmRsDatasetF32 *sentinel=(LiblrhsmmRsDatasetF32 *)(uintptr_t)0x1000,*dataset=NULL;
    assert(liblrhsmm_rs_regroup_data_f32(observations,segmentations,0,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_regroup_data_f32(NULL,segmentations,2,&sentinel)==1);
    assert(liblrhsmm_rs_regroup_data_f32(observations,NULL,2,&sentinel)==1);
    assert(liblrhsmm_rs_regroup_data_f32(observations,segmentations,2,NULL)==1);
    assert(liblrhsmm_rs_regroup_data_f32(observations,segmentations,2,&dataset)==0);
    assert(liblrhsmm_rs_dataset_length_f32(dataset,NULL)==1 && liblrhsmm_rs_dataset_clone_f32(dataset,NULL)==1);
    assert(liblrhsmm_rs_dataset_get_observations_f32(dataset,NULL)==1 && liblrhsmm_rs_dataset_get_segmentations_f32(dataset,NULL)==1);
    assert(liblrhsmm_rs_dataset_clone_f32(NULL,&sentinel)==1 && (uintptr_t)sentinel==0x1000);
    int32_t wrong[3]={0,1,2};assert(liblrhsmm_rs_segmentation_set_boundaries_f32(children_seg[1],0,wrong,3)==0);
    assert(liblrhsmm_rs_segmentation_set_set_f32(segmentations,1,children_seg[1])==0);
    assert(liblrhsmm_rs_regroup_data_f32(observations,segmentations,2,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_segmentation_set_set_f32(segmentations,1,children_seg[0])==0);
    assert(liblrhsmm_rs_regroup_data_f32(observations,segmentations,2,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_segmentation_set_replace_f32(segmentations,NULL,0)==0);
    assert(liblrhsmm_rs_regroup_data_f32(observations,segmentations,2,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_observation_set_release_f32(&observations)==0 && liblrhsmm_rs_segmentation_set_release_f32(&segmentations)==0);
    for(size_t variant=0;variant<3;variant++) {assert(liblrhsmm_rs_observation_release_f32(&children_obs[variant])==0 && liblrhsmm_rs_segmentation_release_f32(&children_seg[variant])==0);}
    LiblrhsmmRsObservationSetF32 *retained_obs=NULL;LiblrhsmmRsSegmentationSetF32 *retained_seg=NULL;
    assert(liblrhsmm_rs_dataset_get_observations_f32(dataset,&retained_obs)==0 && liblrhsmm_rs_dataset_get_segmentations_f32(dataset,&retained_seg)==0);
    assert(liblrhsmm_rs_dataset_release_f32(&dataset)==0 && dataset==NULL && liblrhsmm_rs_dataset_release_f32(&dataset)==0);
    assert(liblrhsmm_rs_dataset_release_f32(NULL)==1);
    LiblrhsmmRsObservationF32 *child_obs=NULL;LiblrhsmmRsSegmentationF32 *child_seg=NULL;
    assert(liblrhsmm_rs_observation_set_get_f32(retained_obs,0,&child_obs)==0 && liblrhsmm_rs_segmentation_set_get_f32(retained_seg,0,&child_seg)==0);
    assert(liblrhsmm_rs_observation_set_release_f32(&retained_obs)==0 && liblrhsmm_rs_segmentation_set_release_f32(&retained_seg)==0);
    compare_f32(child_obs,child_seg,0,0,2);
    assert(liblrhsmm_rs_observation_release_f32(&child_obs)==0 && liblrhsmm_rs_segmentation_release_f32(&child_seg)==0);
    assert(liblrhsmm_rs_observation_set_create_f32(NULL,0,&observations)==0 && liblrhsmm_rs_segmentation_set_create_f32(NULL,0,&segmentations)==0);
    assert(liblrhsmm_rs_regroup_data_f32(observations,segmentations,1,&dataset)==0);
    size_t length=99;assert(liblrhsmm_rs_dataset_length_f32(dataset,&length)==0 && length==0);
    assert(liblrhsmm_rs_dataset_release_f32(&dataset)==0);
    assert(liblrhsmm_rs_observation_set_release_f32(&observations)==0 && liblrhsmm_rs_segmentation_set_release_f32(&segmentations)==0);
}

static double feature_f64(size_t variant,size_t stream,size_t dimension,size_t index) {
    if(!variant) { (void)dimension;return 1.0 + (double)(index + stream + 1) * DBL_EPSILON; }
    volatile double base=(double)2+(double)index/(double)8;
    return base+DBL_EPSILON;
}
static void sample_f64(size_t variant,LiblrhsmmRsObservationF64 **observation,LiblrhsmmRsSegmentationF64 **segmentation) {
    size_t frames=variant==0?8:variant==1?3:0,streams=variant==0?2:1;
    LiblrhsmmRsObservationStreamF64 *owners[2]={0};
    const LiblrhsmmRsObservationStreamF64 *pointers[2];
    for(size_t stream=0;stream<streams;stream++) {
        size_t dimension=variant==0 && stream==0?2:1;double values[16];
        for(size_t i=0;i<frames*dimension;i++) values[i]=feature_f64(variant,stream,dimension,i);
        assert(liblrhsmm_rs_observation_stream_create_f64(dimension,values,frames*dimension,&owners[stream])==0);
        pointers[stream]=owners[stream];
    }
    assert(liblrhsmm_rs_observation_create_f64(frames,pointers,streams,observation)==0);
    for(size_t stream=0;stream<streams;stream++) assert(liblrhsmm_rs_observation_stream_release_f64(&owners[stream])==0);
    size_t segments=variant==0?5:variant==1?3:0;
    int32_t first[5]={1,3,3,6,8},second[3]={0,1,3},durations[5]={10,11,12,13,14};
    int32_t labels[2][5]={{20,21,22,23,24},{40,39,38,37,36}};
    const int32_t *rows[2]={labels[0],labels[1]};
    LiblrhsmmRsJumpF64 ordinary={1,1},extra[2]={{2,(double).25+DBL_EPSILON},{1,1}};
    LiblrhsmmRsJumpSliceF64 jumps[5];
    for(size_t state=0;state<segments;state++) { jumps[state].jumps=&ordinary;jumps[state].count=1; }
    if(variant==1) { jumps[0].jumps=extra;jumps[0].count=2; }
    assert(liblrhsmm_rs_segmentation_create_f64(streams,segments,variant?second:first,durations,rows,jumps,segmentation)==0);
}
static void compare_f64(const LiblrhsmmRsObservationF64 *observation,const LiblrhsmmRsSegmentationF64 *segmentation,size_t variant,size_t first,size_t end) {
    const int32_t bounds0[5]={1,3,3,6,8},bounds1[3]={0,1,3};
    const int32_t *bounds=variant?bounds1:bounds0;
    size_t begin=first?(size_t)bounds[first-1]:0,frames=(size_t)bounds[end-1]-begin,streams=variant?1:2,segments=end-first;
    LiblrhsmmRsObservationLayout shape;
    assert(liblrhsmm_rs_observation_layout_f64(observation,&shape)==0 && shape.frames==frames && shape.streams==streams);
    for(size_t stream=0;stream<streams;stream++) {
        size_t dimension=!variant && !stream?2:1;LiblrhsmmRsObservationStreamLayout layout;
        assert(liblrhsmm_rs_observation_get_stream_layout_f64(observation,stream,&layout)==0 && layout.frames==frames && layout.dimensions==dimension && layout.values==frames*dimension);
        double actual[16];assert(liblrhsmm_rs_observation_copy_values_f64(observation,stream,0,actual,frames*dimension)==0);
        for(size_t i=0;i<frames*dimension;i++) { double expected=feature_f64(variant,stream,dimension,begin*dimension+i);assert(!memcmp(&actual[i],&expected,sizeof(expected))); }
    }
    LiblrhsmmRsSegmentationLayout layout;
    assert(liblrhsmm_rs_segmentation_layout_f64(segmentation,&layout)==0 && layout.streams==streams && layout.segments==segments);
    int32_t actual[5];assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(segmentation,0,actual,segments)==0);
    for(size_t i=0;i<segments;i++) assert(actual[i]==bounds[first+i]-(int32_t)begin);
    assert(liblrhsmm_rs_segmentation_copy_duration_states_f64(segmentation,0,actual,segments)==0);
    for(size_t i=0;i<segments;i++) assert(actual[i]==(int32_t)(10+first+i));
    for(size_t stream=0;stream<streams;stream++) {
        assert(liblrhsmm_rs_segmentation_copy_output_states_f64(segmentation,stream,0,actual,segments)==0);
        for(size_t i=0;i<segments;i++) assert(actual[i]==(stream?(int32_t)(40-first-i):(int32_t)(20+first+i)));
    }
    for(size_t state=0;state<segments;state++) {
        size_t count=99;assert(liblrhsmm_rs_segmentation_outgoing_count_f64(segmentation,state,&count)==0 && count==1);
        LiblrhsmmRsJumpF64 jump;double one=1;
        assert(liblrhsmm_rs_segmentation_copy_outgoing_f64(segmentation,state,0,&jump,1)==0 && jump.delta==1 && !memcmp(&jump.probability,&one,sizeof(one)));
    }
}

static size_t corrected_c_records(size_t unit,const LiblrhsmmRsObservationSetF64 *observations,const LiblrhsmmRsSegmentationSetF64 *segmentations) {
    FILE *file=fopen("tests/fixtures/corrected-c-double-datasets.txt","r");assert(file);
    char line[512],kind[16];size_t checked=0,total=0;
    while(fgets(line,sizeof(line),file)) {
        size_t record_unit,index;assert(sscanf(line,"%15s %zu %zu",kind,&record_unit,&index)==3);total++;
        if(record_unit!=unit) continue;
        if(!strcmp(kind,"count")) {assert(index==5/unit+(5%unit!=0));checked++;continue;}
        LiblrhsmmRsObservationF64 *observation=NULL;LiblrhsmmRsSegmentationF64 *segmentation=NULL;
        assert(liblrhsmm_rs_observation_set_get_f64(observations,index,&observation)==0);
        assert(liblrhsmm_rs_segmentation_set_get_f64(segmentations,index,&segmentation)==0);
        if(!strcmp(kind,"value")) {
            size_t stream,element;double expected,actual;
            assert(sscanf(line,"%15s %zu %zu %zu %zu %lf",kind,&record_unit,&index,&stream,&element,&expected)==6);
            assert(liblrhsmm_rs_observation_copy_values_f64(observation,stream,element,&actual,1)==0 && !memcmp(&actual,&expected,sizeof(actual)));
        } else {
            assert(!strcmp(kind,"state"));size_t state;int expected[4];int32_t actual[4];
            assert(sscanf(line,"%15s %zu %zu %zu %d %d %d %d",kind,&record_unit,&index,&state,&expected[0],&expected[1],&expected[2],&expected[3])==8);
            assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(segmentation,state,&actual[0],1)==0);
            assert(liblrhsmm_rs_segmentation_copy_duration_states_f64(segmentation,state,&actual[1],1)==0);
            assert(liblrhsmm_rs_segmentation_copy_output_states_f64(segmentation,0,state,&actual[2],1)==0);
            assert(liblrhsmm_rs_segmentation_copy_output_states_f64(segmentation,1,state,&actual[3],1)==0);
            for(size_t i=0;i<4;i++) assert(actual[i]==expected[i]);
        }
        checked++;
        assert(liblrhsmm_rs_observation_release_f64(&observation)==0 && liblrhsmm_rs_segmentation_release_f64(&segmentation)==0);
    }
    assert(total==150);assert(fclose(file)==0);return checked;
}

static void exercise_f64(void) {
    LiblrhsmmRsObservationF64 *children_obs[3]={0};LiblrhsmmRsSegmentationF64 *children_seg[3]={0};
    for(size_t variant=0;variant<3;variant++) sample_f64(variant,&children_obs[variant],&children_seg[variant]);
    const LiblrhsmmRsObservationF64 *obs[4]={children_obs[0],children_obs[1],children_obs[0],children_obs[2]};
    const LiblrhsmmRsSegmentationF64 *seg[4]={children_seg[0],children_seg[1],children_seg[0],children_seg[2]};
    LiblrhsmmRsObservationSetF64 *observations=NULL;LiblrhsmmRsSegmentationSetF64 *segmentations=NULL;
    assert(liblrhsmm_rs_observation_set_create_f64(obs,4,&observations)==0);
    assert(liblrhsmm_rs_segmentation_set_create_f64(seg,4,&segmentations)==0);
    const size_t units[6]={1,2,3,5,8,SIZE_MAX};size_t checked=0;
    for(size_t u=0;u<6;u++) {
        size_t unit=units[u],length=99;
        LiblrhsmmRsDatasetF64 *dataset=NULL,*copy=NULL;
        assert(liblrhsmm_rs_regroup_data_f64(observations,segmentations,unit,&dataset)==0);
        assert(liblrhsmm_rs_dataset_length_f64(dataset,&length)==0 && length==2*(5/unit+(5%unit!=0))+3/unit+(3%unit!=0));
        assert(liblrhsmm_rs_dataset_clone_f64(dataset,&copy)==0);
        assert(liblrhsmm_rs_dataset_release_f64(&dataset)==0);
        LiblrhsmmRsObservationSetF64 *result_observations=NULL;LiblrhsmmRsSegmentationSetF64 *result_segmentations=NULL;
        assert(liblrhsmm_rs_dataset_get_observations_f64(copy,&result_observations)==0);
        assert(liblrhsmm_rs_dataset_get_segmentations_f64(copy,&result_segmentations)==0);
        assert(liblrhsmm_rs_dataset_release_f64(&copy)==0);
        size_t index=0;
        for(size_t sample=0;sample<3;sample++) {
            size_t variant=sample==1?1:0,segments=variant?3:5;
            for(size_t first=0;first<segments;) {
                size_t end=unit>segments-first?segments:first+unit;
                LiblrhsmmRsObservationF64 *observation=NULL;LiblrhsmmRsSegmentationF64 *segmentation=NULL;
                assert(liblrhsmm_rs_observation_set_get_f64(result_observations,index,&observation)==0);
                assert(liblrhsmm_rs_segmentation_set_get_f64(result_segmentations,index,&segmentation)==0);
                compare_f64(observation,segmentation,variant,first,end);
                assert(liblrhsmm_rs_observation_release_f64(&observation)==0 && liblrhsmm_rs_segmentation_release_f64(&segmentation)==0);
                first=end;index++;
            }
        }
        assert(index==length);checked+=corrected_c_records(unit,result_observations,result_segmentations);
        assert(liblrhsmm_rs_observation_set_release_f64(&result_observations)==0 && liblrhsmm_rs_segmentation_set_release_f64(&result_segmentations)==0);
    }
    assert(checked==150);
    LiblrhsmmRsDatasetF64 *sentinel=(LiblrhsmmRsDatasetF64 *)(uintptr_t)0x1000,*dataset=NULL;
    assert(liblrhsmm_rs_regroup_data_f64(observations,segmentations,0,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_regroup_data_f64(NULL,segmentations,2,&sentinel)==1);
    assert(liblrhsmm_rs_regroup_data_f64(observations,NULL,2,&sentinel)==1);
    assert(liblrhsmm_rs_regroup_data_f64(observations,segmentations,2,NULL)==1);
    assert(liblrhsmm_rs_regroup_data_f64(observations,segmentations,2,&dataset)==0);
    assert(liblrhsmm_rs_dataset_length_f64(dataset,NULL)==1 && liblrhsmm_rs_dataset_clone_f64(dataset,NULL)==1);
    assert(liblrhsmm_rs_dataset_get_observations_f64(dataset,NULL)==1 && liblrhsmm_rs_dataset_get_segmentations_f64(dataset,NULL)==1);
    assert(liblrhsmm_rs_dataset_clone_f64(NULL,&sentinel)==1 && (uintptr_t)sentinel==0x1000);
    int32_t wrong[3]={0,1,2};assert(liblrhsmm_rs_segmentation_set_boundaries_f64(children_seg[1],0,wrong,3)==0);
    assert(liblrhsmm_rs_segmentation_set_set_f64(segmentations,1,children_seg[1])==0);
    assert(liblrhsmm_rs_regroup_data_f64(observations,segmentations,2,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_segmentation_set_set_f64(segmentations,1,children_seg[0])==0);
    assert(liblrhsmm_rs_regroup_data_f64(observations,segmentations,2,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_segmentation_set_replace_f64(segmentations,NULL,0)==0);
    assert(liblrhsmm_rs_regroup_data_f64(observations,segmentations,2,&sentinel)==3 && (uintptr_t)sentinel==0x1000);
    assert(liblrhsmm_rs_observation_set_release_f64(&observations)==0 && liblrhsmm_rs_segmentation_set_release_f64(&segmentations)==0);
    for(size_t variant=0;variant<3;variant++) {assert(liblrhsmm_rs_observation_release_f64(&children_obs[variant])==0 && liblrhsmm_rs_segmentation_release_f64(&children_seg[variant])==0);}
    LiblrhsmmRsObservationSetF64 *retained_obs=NULL;LiblrhsmmRsSegmentationSetF64 *retained_seg=NULL;
    assert(liblrhsmm_rs_dataset_get_observations_f64(dataset,&retained_obs)==0 && liblrhsmm_rs_dataset_get_segmentations_f64(dataset,&retained_seg)==0);
    assert(liblrhsmm_rs_dataset_release_f64(&dataset)==0 && dataset==NULL && liblrhsmm_rs_dataset_release_f64(&dataset)==0);
    assert(liblrhsmm_rs_dataset_release_f64(NULL)==1);
    LiblrhsmmRsObservationF64 *child_obs=NULL;LiblrhsmmRsSegmentationF64 *child_seg=NULL;
    assert(liblrhsmm_rs_observation_set_get_f64(retained_obs,0,&child_obs)==0 && liblrhsmm_rs_segmentation_set_get_f64(retained_seg,0,&child_seg)==0);
    assert(liblrhsmm_rs_observation_set_release_f64(&retained_obs)==0 && liblrhsmm_rs_segmentation_set_release_f64(&retained_seg)==0);
    compare_f64(child_obs,child_seg,0,0,2);
    assert(liblrhsmm_rs_observation_release_f64(&child_obs)==0 && liblrhsmm_rs_segmentation_release_f64(&child_seg)==0);
    assert(liblrhsmm_rs_observation_set_create_f64(NULL,0,&observations)==0 && liblrhsmm_rs_segmentation_set_create_f64(NULL,0,&segmentations)==0);
    assert(liblrhsmm_rs_regroup_data_f64(observations,segmentations,1,&dataset)==0);
    size_t length=99;assert(liblrhsmm_rs_dataset_length_f64(dataset,&length)==0 && length==0);
    assert(liblrhsmm_rs_dataset_release_f64(&dataset)==0);
    assert(liblrhsmm_rs_observation_set_release_f64(&observations)==0 && liblrhsmm_rs_segmentation_set_release_f64(&segmentations)==0);
}

int main(void) {
    exercise_f32();exercise_f64();
    puts("Dataset C: all12 exports, complete fields,150 corrected-C double records and independent lifetimes passed");
    return 0;
}
