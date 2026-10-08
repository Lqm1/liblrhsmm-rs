"""Actual ctypes caller for every stream-statistics export and original update corpora."""
import ctypes as c
from pathlib import Path
import sys

lib=c.CDLL(sys.argv[1])
N=c.c_size_t
U=c.c_uint32
P=c.c_void_p
for suffix,scalar,fixture in [('f32',c.c_float,'c-updates.txt'),('f64',c.c_double,'c-double-update.txt')]:
    class Parameters(c.Structure):
        _fields_=[('mixtures',N),('dimensions',N)]+[(name,c.POINTER(scalar)) for name in ['weights','means','variances','variance_floors']]
    class Values(c.Structure):
        _fields_=[('mixtures',N),('dimensions',N),('state_occupancy',scalar)]+[(name,c.POINTER(scalar)) for name in ['weighted_sums','weighted_squared_sums','component_occupancies']]
    class Options(c.Structure):
        _fields_=[('duration_mode',U),('duration_variance_floor',scalar),('emission_variance_floor',scalar)]
    signatures={
        'stream_statistics_create':[c.POINTER(P),N,c.POINTER(P)],
        'stream_statistics_replace':[P,c.POINTER(P),N],
        'stream_statistics_from_stream':[P,c.POINTER(P)],
        'stream_statistics_count':[P,c.POINTER(N)],
        'stream_statistics_get_emission':[P,N,c.POINTER(P)],
        'stream_statistics_set_emission':[P,N,P],
        'stream_statistics_clone':[P,c.POINTER(P)],
        'stream_statistics_clear':[P],
        'stream_statistics_release':[c.POINTER(P)],
        'stream_update':[P,P,c.POINTER(Options)],
        'stream_new':[N,N,N,c.POINTER(P)],
        'stream_create':[scalar,c.POINTER(P),N,c.POINTER(P)],
        'stream_get_emission':[P,N,c.POINTER(P)],
        'stream_release':[c.POINTER(P)],
        'gaussian_create':[c.POINTER(Parameters),c.POINTER(P)],
        'gaussian_copy_field':[P,U,N,c.POINTER(scalar),N],
        'gaussian_release':[c.POINTER(P)],
        'gaussian_statistics_new':[N,N,c.POINTER(P)],
        'gaussian_statistics_create':[c.POINTER(Values),c.POINTER(P)],
        'gaussian_statistics_set_field':[P,U,N,c.POINTER(scalar),N],
        'gaussian_statistics_copy_field':[P,U,N,c.POINTER(scalar),N],
        'gaussian_statistics_clear':[P],
        'gaussian_statistics_release':[c.POINTER(P)],
        'update_options_default':[c.POINTER(Options)]
    }
    functions={}
    called=set()
    for name,args in signatures.items():
        fn=getattr(lib,f'liblrhsmm_rs_{name}_{suffix}')
        fn.argtypes=args
        fn.restype=U
        functions[name]=fn
    def status(name,expected,*args):
        actual=functions[name](*args)
        assert actual==expected,(name,actual,expected)
        called.add(name)
    def call(name,*args):
        status(name,0,*args)
    stream,owner,created,copy,matched,a,b,child=[P() for _ in range(8)]
    call('stream_new',2,1,1,c.byref(stream))
    call('stream_statistics_from_stream',stream,c.byref(matched))
    call('gaussian_statistics_new',1,1,c.byref(a))
    call('gaussian_statistics_new',2,3,c.byref(b))
    changed=scalar(-9)
    call('gaussian_statistics_set_field',a,0,0,c.byref(changed),1)
    children=(P*3)(a.value,b.value,a.value)
    call('stream_statistics_create',children,3,c.byref(owner))
    call('stream_statistics_create',children,3,c.byref(created))
    call('stream_statistics_clone',owner,c.byref(copy))
    count=N(99)
    call('stream_statistics_count',owner,c.byref(count))
    assert count.value==3
    bad=(P*2)(a.value,None)
    status('stream_statistics_replace',1,owner,bad,2)
    call('stream_statistics_get_emission',owner,0,c.byref(child))
    result=scalar(99)
    call('gaussian_statistics_copy_field',child,0,0,c.byref(result),1)
    assert result.value==-9
    call('gaussian_statistics_clear',child)
    call('gaussian_statistics_release',c.byref(child))
    call('stream_statistics_set_emission',owner,0,b)
    status('stream_statistics_set_emission',2,owner,3,a)
    status('stream_statistics_get_emission',2,owner,3,c.byref(child))
    assert child.value is None
    call('stream_statistics_replace',owner,children,3)
    call('stream_statistics_clear',owner)
    call('stream_statistics_get_emission',owner,0,c.byref(child))
    call('gaussian_statistics_copy_field',child,0,0,c.byref(result),1)
    assert result.value==0
    call('gaussian_statistics_release',c.byref(child))
    call('gaussian_statistics_release',c.byref(a))
    call('gaussian_statistics_release',c.byref(b))
    call('stream_release',c.byref(stream))
    call('stream_statistics_count',matched,c.byref(count))
    assert count.value==2
    call('stream_statistics_get_emission',copy,0,c.byref(child))
    call('gaussian_statistics_copy_field',child,0,0,c.byref(result),1)
    assert result.value==-9
    call('gaussian_statistics_release',c.byref(child))
    for handle in (owner,created,copy,matched):
        call('stream_statistics_release',c.byref(handle))
    call('stream_statistics_release',c.byref(owner))
    call('stream_statistics_create',None,0,c.byref(owner))
    call('stream_new',0,0,0,c.byref(stream))
    options=Options()
    call('update_options_default',c.byref(options))
    options.emission_variance_floor=-1
    status('stream_update',3,stream,owner,c.byref(options))
    options.emission_variance_floor=0
    call('stream_update',stream,owner,c.byref(options))
    call('stream_release',c.byref(stream))
    call('stream_statistics_release',c.byref(owner))
    status('stream_statistics_release',1,None)
    compared=0
    for line in (Path(__file__).parent/'fixtures'/fixture).read_text().splitlines():
        tokens=line.split()
        if tokens[0]!='mixture':
            continue
        scenario,component=map(int,tokens[1:3])
        count=1 if scenario==0 else 2
        arrays=[(scalar*len(values))(*values) for values in [[0.5]*count,[-7]*(count*2),[8]*(count*2),[0,0.75]*count]]
        gaussian=P()
        statistics=P()
        call('gaussian_create',c.byref(Parameters(count,2,*arrays)),c.byref(gaussian))
        occupancy=[2+k for k in range(count)]
        if scenario==2:
            occupancy[1]=1e-8
        arrays=[(scalar*len(values))(*values) for values in [[-3.5,6]*count,[9.25,12]*count,occupancy]]
        call('gaussian_statistics_create',c.byref(Values(count,2,1e-8 if scenario==3 else 4,*arrays)),c.byref(statistics))
        inputs=(P*1)(gaussian.value)
        accumulators=(P*1)(statistics.value)
        call('stream_create',1,inputs,1,c.byref(stream))
        call('stream_statistics_create',accumulators,1,c.byref(owner))
        call('update_options_default',c.byref(options))
        call('stream_update',stream,owner,c.byref(options))
        call('gaussian_release',c.byref(gaussian))
        call('stream_get_emission',stream,0,c.byref(gaussian))
        weight=scalar()
        means=(scalar*2)()
        variances=(scalar*2)()
        call('gaussian_copy_field',gaussian,0,component,c.byref(weight),1)
        call('gaussian_copy_field',gaussian,1,2*component,means,2)
        call('gaussian_copy_field',gaussian,2,2*component,variances,2)
        for value,text in zip([weight.value,means[0],variances[0],means[1],variances[1]],tokens[3:]):
            target=scalar(float(text)).value
            assert abs(value-target)<=2e-14*max(1.0,abs(target))
            compared+=1
        options.duration_mode=99
        status('stream_update',3,stream,owner,c.byref(options))
        call('gaussian_release',c.byref(gaussian))
        call('gaussian_statistics_release',c.byref(statistics))
        call('stream_statistics_release',c.byref(owner))
        call('stream_release',c.byref(stream))
    assert compared==35
    assert called==set(signatures)
print('stream statistics: all20 exports and70 original C update values passed')
