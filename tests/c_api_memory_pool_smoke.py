"""Actual ctypes native pool accounting, byte regions and original C growth oracle."""
import ctypes as c
from pathlib import Path
import sys

lib=c.CDLL(sys.argv[1])
P,N,U,B=c.c_void_p,c.c_size_t,c.c_uint32,c.c_uint8

class Layout(c.Structure):
    _fields_=[('page_size',N),('pages',N),('total_capacity',N),('allocated_bytes',N)]

class Page(c.Structure):
    _fields_=[('used_bytes',N),('capacity',N)]

functions,called={},set()
def register(name,args):
    fn=getattr(lib,'liblrhsmm_rs_'+name)
    fn.argtypes,fn.restype=args,U
    functions[name]=fn

def status(name,expected,*args):
    actual=functions[name](*args)
    assert actual==expected,(name,actual,expected)
    called.add(name)

def call(name,*args):
    status(name,0,*args)

register('memory_pool_new',[N,c.POINTER(P)])
register('memory_pool_layout',[P,c.POINTER(Layout)])
register('memory_pool_page',[P,N,c.POINTER(Page)])
register('memory_pool_allocate',[P,N,c.POINTER(P)])
register('memory_pool_allocate_zeroed',[P,N,N,c.POINTER(P)])
for name in ['copy','set']:
    register('memory_pool_'+name+'_bytes',[P,P,N,c.POINTER(B),N])
register('pool_allocation_length',[P,c.POINTER(N)])
register('pool_allocation_clone',[P,c.POINTER(P)])
for name in ['memory_pool','pool_allocation']:
    register(name+'_release',[c.POINTER(P)])

def layout(pool):
    result=Layout()
    call('memory_pool_layout',pool,c.byref(result))
    return tuple(getattr(result,name) for name,_ in Layout._fields_)

records=iter((Path(__file__).parent/'fixtures/c-memory-pool.txt').read_text().splitlines())
requests=[(False,1,0),(False,1,3),(True,1,5),(False,1,1),(True,3,4),(True,0,17),(False,1,2),(True,2,3)]
for scenario,page_size in enumerate([8,0]):
    pool=P()
    call('memory_pool_new',page_size,c.byref(pool))
    assert layout(pool)==(page_size,1,page_size,0)
    used,capacity,regions,allocated=[0],[page_size],[],0
    for index,(zeroed,number,size) in enumerate(requests):
        owner=P();length=number*size
        if zeroed: call('memory_pool_allocate_zeroed',pool,number,size,c.byref(owner))
        else: call('memory_pool_allocate',pool,length,c.byref(owner))
        shape=layout(pool);allocated+=length
        assert shape[0]==page_size and shape[3]==allocated
        if shape[1]>len(used):
            assert shape[1]==len(used)+1
            used.append(0);capacity.append(max(page_size,length))
        offset=used[-1];used[-1]+=length
        for index_page,(expected_used,expected_capacity) in enumerate(zip(used,capacity)):
            page=Page()
            call('memory_pool_page',pool,index_page,c.byref(page))
            assert (page.used_bytes,page.capacity)==(expected_used,expected_capacity)
        assert next(records)==f'A {scenario} {index} {shape[1]} {shape[2]} {used[-1]} {offset} {length} 1'
        result=N(99);call('pool_allocation_length',owner,c.byref(result));assert result.value==length
        data=(B*length)(*[99]*length)
        call('memory_pool_copy_bytes',pool,owner,0,data,length);assert list(data)==[0]*length
        call('memory_pool_set_bytes',pool,owner,0,(B*length)(*[index+1]*length),length)
        regions.append((owner,length))
    for index,(owner,length) in enumerate(regions):
        data=(B*length)()
        call('memory_pool_copy_bytes',pool,owner,0,data,length)
        assert next(records)==f'V {scenario} {index} {sum(data)}'
        call('pool_allocation_release',c.byref(owner));assert owner.value is None
    call('memory_pool_release',c.byref(pool));assert pool.value is None
assert list(records)==[]

pool,other,owner,copy=P(),P(),P(),P()
call('memory_pool_new',4,c.byref(pool));call('memory_pool_new',4,c.byref(other))
call('memory_pool_allocate',pool,3,c.byref(owner));call('pool_allocation_clone',owner,c.byref(copy))
data=(B*3)(1,3,5);result=(B*3)()
call('memory_pool_set_bytes',pool,copy,0,data,3)
call('memory_pool_copy_bytes',pool,owner,0,result,3);assert list(result)==[1,3,5]
before=layout(pool)
status('memory_pool_allocate',1,pool,100,None)
status('memory_pool_allocate_zeroed',1,pool,100,100,None)
sentinel=P(owner.value)
maximum=N(-1).value
status('memory_pool_allocate',3,pool,maximum,c.byref(sentinel));assert sentinel.value==owner.value
status('memory_pool_allocate_zeroed',3,pool,maximum,2,c.byref(sentinel));assert sentinel.value==owner.value
status('memory_pool_new',3,maximum,c.byref(pool));assert layout(pool)==before
page=Page(71,71)
status('memory_pool_page',2,pool,99,c.byref(page));assert (page.used_bytes,page.capacity)==(71,71)
status('memory_pool_copy_bytes',3,other,owner,0,result,3)
status('memory_pool_set_bytes',3,other,owner,0,data,3)
for offset in [2,maximum]:
    result=(B*3)(71,71,71)
    status('memory_pool_copy_bytes',2,pool,owner,offset,result,3);assert list(result)==[71]*3
    status('memory_pool_set_bytes',2,pool,owner,offset,(B*3)(9,9,9),3)
call('memory_pool_copy_bytes',pool,owner,3,None,0)
call('memory_pool_set_bytes',pool,owner,3,None,0)
status('memory_pool_copy_bytes',2,pool,owner,4,None,0)
status('memory_pool_copy_bytes',1,pool,owner,0,None,1)
status('memory_pool_set_bytes',1,pool,owner,0,None,1)
status('memory_pool_copy_bytes',2,pool,owner,0,None,maximum)
status('pool_allocation_length',1,owner,None)
status('pool_allocation_clone',1,owner,None)
status('memory_pool_layout',1,pool,None)
call('memory_pool_copy_bytes',pool,copy,0,result,3);assert list(result)==[1,3,5]
call('pool_allocation_release',c.byref(owner));call('memory_pool_release',c.byref(pool))
length=N(99);call('pool_allocation_length',copy,c.byref(length));assert length.value==3
status('memory_pool_copy_bytes',3,other,copy,0,result,3)
retained=P();call('pool_allocation_clone',copy,c.byref(retained))
call('pool_allocation_release',c.byref(copy));call('pool_allocation_release',c.byref(retained))
call('pool_allocation_release',c.byref(retained));status('pool_allocation_release',1,None)
call('memory_pool_release',c.byref(pool));status('memory_pool_release',1,None)
call('memory_pool_release',c.byref(other))
assert called==set(functions) and len(called)==11
print('Memory pool ctypes: all11 exports, original C accounting, shared regions and failure transactions passed')
