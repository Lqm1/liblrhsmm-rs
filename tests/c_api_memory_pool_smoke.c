#include "liblrhsmm_rs.h"
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <stdio.h>
#include <string.h>

static LiblrhsmmRsPoolLayout layout(const LiblrhsmmRsMemoryPool *pool) {
    LiblrhsmmRsPoolLayout result;
    assert(liblrhsmm_rs_memory_pool_layout(pool,&result)==0);
    return result;
}

static void original_c_growth(void) {
    FILE *reference=fopen("tests/fixtures/c-memory-pool.txt","r");assert(reference);
    const size_t numbers[8]={1,1,1,1,3,0,1,2}, sizes[8]={0,3,5,1,4,17,2,3};
    const int zeroed[8]={0,0,1,0,1,1,0,1};
    for(unsigned scenario=0;scenario<2;scenario++) {
        size_t page_size=scenario==0?8:0, allocated=0;
        LiblrhsmmRsMemoryPool *pool=NULL; LiblrhsmmRsPoolAllocation *regions[8]={0};
        assert(liblrhsmm_rs_memory_pool_new(page_size,&pool)==0);
        LiblrhsmmRsPoolLayout initial=layout(pool);
        assert(initial.page_size==page_size && initial.pages==1 && initial.total_capacity==page_size && initial.allocated_bytes==0);
        size_t used[16]={0},capacity[16]={0};capacity[0]=page_size;
        for(unsigned index=0;index<8;index++) {
            size_t length=numbers[index]*sizes[index], before=layout(pool).pages;
            uint32_t status=zeroed[index]?liblrhsmm_rs_memory_pool_allocate_zeroed(pool,numbers[index],sizes[index],&regions[index]):liblrhsmm_rs_memory_pool_allocate(pool,length,&regions[index]);
            assert(status==0);
            LiblrhsmmRsPoolLayout shape=layout(pool);allocated+=length;
            assert(shape.page_size==page_size && shape.allocated_bytes==allocated);
            if(shape.pages>before) {assert(shape.pages==before+1);capacity[shape.pages-1]=page_size>length?page_size:length;}
            size_t offset=used[shape.pages-1];used[shape.pages-1]+=length;
            for(size_t page=0;page<shape.pages;page++) {
                LiblrhsmmRsPoolPage actual;assert(liblrhsmm_rs_memory_pool_page(pool,page,&actual)==0);
                assert(actual.used_bytes==used[page] && actual.capacity==capacity[page]);
            }
            unsigned actual_scenario,actual_index;size_t pages,total,top,expected_offset,expected_length,initialized;
            assert(fscanf(reference," A %u %u %zu %zu %zu %zu %zu %zu",&actual_scenario,&actual_index,&pages,&total,&top,&expected_offset,&expected_length,&initialized)==8);
            assert(actual_scenario==scenario && actual_index==index && pages==shape.pages && total==shape.total_capacity && top==used[shape.pages-1] && expected_offset==offset && expected_length==length && initialized==1);
            size_t actual_length=99;assert(liblrhsmm_rs_pool_allocation_length(regions[index],&actual_length)==0 && actual_length==length);
            uint8_t values[32];memset(values,99,sizeof(values));
            assert(liblrhsmm_rs_memory_pool_copy_bytes(pool,regions[index],0,values,length)==0);
            for(size_t byte=0;byte<length;byte++) assert(values[byte]==0);
            memset(values,(int)index+1,length);assert(liblrhsmm_rs_memory_pool_set_bytes(pool,regions[index],0,values,length)==0);
        }
        for(unsigned index=0;index<8;index++) {
            size_t length=numbers[index]*sizes[index],sum=0,expected;
            uint8_t values[32];assert(liblrhsmm_rs_memory_pool_copy_bytes(pool,regions[index],0,values,length)==0);
            for(size_t byte=0;byte<length;byte++) sum+=values[byte];
            unsigned actual_scenario,actual_index;
            assert(fscanf(reference," V %u %u %zu",&actual_scenario,&actual_index,&expected)==3);
            assert(actual_scenario==scenario && actual_index==index && sum==expected);
            assert(liblrhsmm_rs_pool_allocation_release(&regions[index])==0 && regions[index]==NULL);
        }
        assert(liblrhsmm_rs_memory_pool_release(&pool)==0 && pool==NULL);
    }
    char tail;assert(fscanf(reference," %c",&tail)==EOF);assert(fclose(reference)==0);
}

static void sharing_and_errors(void) {
    LiblrhsmmRsMemoryPool *pool=NULL,*other=NULL;
    LiblrhsmmRsPoolAllocation *owner=NULL,*copy=NULL,*retained=NULL;
    assert(liblrhsmm_rs_memory_pool_new(4,&pool)==0 && liblrhsmm_rs_memory_pool_new(4,&other)==0);
    assert(liblrhsmm_rs_memory_pool_allocate(pool,3,&owner)==0);
    assert(liblrhsmm_rs_pool_allocation_clone(owner,&copy)==0);
    uint8_t input[3]={1,3,5},values[3]={99,99,99};
    assert(liblrhsmm_rs_memory_pool_set_bytes(pool,copy,0,input,3)==0);
    assert(liblrhsmm_rs_memory_pool_copy_bytes(pool,owner,0,values,3)==0 && !memcmp(values,input,3));
    LiblrhsmmRsPoolLayout before=layout(pool);
    assert(liblrhsmm_rs_memory_pool_allocate(pool,100,NULL)==1);
    assert(liblrhsmm_rs_memory_pool_allocate_zeroed(pool,100,100,NULL)==1);
    LiblrhsmmRsPoolAllocation *sentinel=owner;
    assert(liblrhsmm_rs_memory_pool_allocate(pool,SIZE_MAX,&sentinel)==3 && sentinel==owner);
    assert(liblrhsmm_rs_memory_pool_allocate_zeroed(pool,SIZE_MAX,2,&sentinel)==3 && sentinel==owner);
    assert(liblrhsmm_rs_memory_pool_new(SIZE_MAX,&pool)==3);
    LiblrhsmmRsPoolLayout after=layout(pool);assert(!memcmp(&before,&after,sizeof(before)));
    LiblrhsmmRsPoolPage page={71,71};assert(liblrhsmm_rs_memory_pool_page(pool,99,&page)==2 && page.used_bytes==71 && page.capacity==71);
    assert(liblrhsmm_rs_memory_pool_copy_bytes(other,owner,0,values,3)==3);
    assert(liblrhsmm_rs_memory_pool_set_bytes(other,owner,0,input,3)==3);
    memset(values,71,3);assert(liblrhsmm_rs_memory_pool_copy_bytes(pool,owner,2,values,3)==2 && values[0]==71 && values[1]==71 && values[2]==71);
    assert(liblrhsmm_rs_memory_pool_set_bytes(pool,owner,2,input,3)==2);
    assert(liblrhsmm_rs_memory_pool_copy_bytes(pool,owner,SIZE_MAX,values,3)==2);
    assert(liblrhsmm_rs_memory_pool_set_bytes(pool,owner,SIZE_MAX,input,3)==2);
    assert(liblrhsmm_rs_memory_pool_copy_bytes(pool,owner,3,NULL,0)==0 && liblrhsmm_rs_memory_pool_set_bytes(pool,owner,3,NULL,0)==0);
    assert(liblrhsmm_rs_memory_pool_copy_bytes(pool,owner,4,NULL,0)==2);
    assert(liblrhsmm_rs_memory_pool_copy_bytes(pool,owner,0,NULL,1)==1 && liblrhsmm_rs_memory_pool_set_bytes(pool,owner,0,NULL,1)==1);
    assert(liblrhsmm_rs_memory_pool_copy_bytes(pool,owner,0,NULL,SIZE_MAX)==2);
    assert(liblrhsmm_rs_pool_allocation_length(owner,NULL)==1 && liblrhsmm_rs_pool_allocation_clone(owner,NULL)==1);
    assert(liblrhsmm_rs_memory_pool_layout(pool,NULL)==1);
    assert(liblrhsmm_rs_memory_pool_copy_bytes(pool,copy,0,values,3)==0 && !memcmp(values,input,3));
    assert(liblrhsmm_rs_pool_allocation_release(&owner)==0 && owner==NULL);
    assert(liblrhsmm_rs_memory_pool_release(&pool)==0 && pool==NULL);
    size_t length=99;assert(liblrhsmm_rs_pool_allocation_length(copy,&length)==0 && length==3);
    assert(liblrhsmm_rs_memory_pool_copy_bytes(other,copy,0,values,3)==3);
    assert(liblrhsmm_rs_pool_allocation_clone(copy,&retained)==0);
    assert(liblrhsmm_rs_pool_allocation_release(&copy)==0 && liblrhsmm_rs_pool_allocation_release(&retained)==0);
    assert(liblrhsmm_rs_pool_allocation_release(&retained)==0 && liblrhsmm_rs_pool_allocation_release(NULL)==1);
    assert(liblrhsmm_rs_memory_pool_release(&pool)==0 && liblrhsmm_rs_memory_pool_release(NULL)==1);
    assert(liblrhsmm_rs_memory_pool_release(&other)==0);
}

int main(void) {
    original_c_growth();sharing_and_errors();
    puts("Memory pool C: all11 exports, original C accounting, shared regions and failure transactions passed");
    return 0;
}
