"""Actual ctypes complete sample sets, missing slots and independent ownership."""
import ctypes as c
import struct
import sys

lib=c.CDLL(sys.argv[1])
P,N,U,I=c.c_void_p,c.c_size_t,c.c_uint32,c.c_int32
called=set()

class ObservationLayout(c.Structure):
    _fields_=[('frames',N),('streams',N)]

class StreamLayout(c.Structure):
    _fields_=[('frames',N),('dimensions',N),('values',N)]

class SegmentationLayout(c.Structure):
    _fields_=[('streams',N),('segments',N)]

for family in ['observation','segmentation']:
    for suffix,scalar,code,epsilon in [('f32',c.c_float,'f',2.0**-23),('f64',c.c_double,'d',2.0**-52)]:
        functions={}
        prefix=f'liblrhsmm_rs_{family}_set_'

        def register(method,args):
            name=prefix+method+'_'+suffix
            fn=getattr(lib,name);fn.argtypes,fn.restype=args,U
            functions[method]=(name,fn)

        def status(method,expected,*args):
            name,fn=functions[method]
            result=fn(*args)
            assert result==expected,(name,result,expected)
            called.add(name)

        def call(method,*args):
            status(method,0,*args)

        def aux(method,args):
            fn=getattr(lib,f'liblrhsmm_rs_{method}_{suffix}')
            fn.argtypes,fn.restype=args,U
            return fn

        def bits(value):
            return struct.pack('<'+code,value)

        for method in ['create']:
            register(method,[c.POINTER(P),N,c.POINTER(P)])
        for method in ['length','builder_length']:
            register(method,[P,c.POINTER(N)])
        for method in ['get','builder_get_sample']:
            register(method,[P,N,c.POINTER(P)])
        for method in ['set','builder_set_sample']:
            register(method,[P,N,P])
        register('append',[P,P])
        register('replace',[P,c.POINTER(P),N])
        for method in ['clone','builder_clone','builder_build']:
            register(method,[P,c.POINTER(P)])
        for method in ['release','builder_release']:
            register(method,[c.POINTER(P)])
        register('builder_new',[N,c.POINTER(P)])
        register('builder_has_sample',[P,N,c.POINTER(U)])
        for method in ['builder_clear_sample','builder_resize']:
            register(method,[P,N])
        assert len(functions)==18
        child_release=aux(f'{family}_release',[c.POINTER(P)])

        if family=='observation':
            stream_create=aux('observation_stream_create',[N,c.POINTER(scalar),N,c.POINTER(P)])
            stream_release=aux('observation_stream_release',[c.POINTER(P)])
            child_create=aux('observation_create',[N,c.POINTER(P),N,c.POINTER(P)])
            child_layout=aux('observation_layout',[P,c.POINTER(ObservationLayout)])
            stream_layout=aux('observation_get_stream_layout',[P,N,c.POINTER(StreamLayout)])
            copy_values=aux('observation_copy_values',[P,N,N,c.POINTER(scalar),N])

            def make(variant):
                frames,dimensions=(3,[2,1]) if variant==0 else (2,[1])
                owners,expected=[],[]
                for stream,dimension in enumerate(dimensions):
                    values=[scalar(scalar(1+(index+stream+variant)/8).value+epsilon).value for index in range(frames*dimension)]
                    data=(scalar*len(values))(*values)
                    owner=P();assert stream_create(dimension,data,len(values),c.byref(owner))==0
                    owners.append(owner);expected.append((dimension,[bits(v) for v in values]))
                child=P()
                assert child_create(frames,(P*len(owners))(*[p.value for p in owners]),len(owners),c.byref(child))==0
                for owner in owners: assert stream_release(c.byref(owner))==0
                return child,(frames,expected)

            def compare(child,expected):
                frames,streams=expected
                layout=ObservationLayout()
                assert child_layout(child,c.byref(layout))==0 and (layout.frames,layout.streams)==(frames,len(streams))
                for stream,(dimension,values) in enumerate(streams):
                    shape=StreamLayout()
                    assert stream_layout(child,stream,c.byref(shape))==0 and (shape.frames,shape.dimensions,shape.values)==(frames,dimension,len(values))
                    actual=(scalar*len(values))()
                    assert copy_values(child,stream,0,actual,len(values))==0 and [bits(v) for v in actual]==values
        else:
            class Jump(c.Structure):
                _fields_=[('delta',I),('probability',scalar)]

            class JumpSlice(c.Structure):
                _fields_=[('jumps',c.POINTER(Jump)),('count',N)]

            child_create=aux('segmentation_create',[N,N,c.POINTER(I),c.POINTER(I),c.POINTER(c.POINTER(I)),c.POINTER(JumpSlice),c.POINTER(P)])
            child_layout=aux('segmentation_layout',[P,c.POINTER(SegmentationLayout)])
            copy_boundaries=aux('segmentation_copy_boundaries',[P,N,c.POINTER(I),N])
            copy_duration=aux('segmentation_copy_duration_states',[P,N,c.POINTER(I),N])
            copy_labels=aux('segmentation_copy_output_states',[P,N,N,c.POINTER(I),N])
            jump_count=aux('segmentation_outgoing_count',[P,N,c.POINTER(N)])
            copy_jumps=aux('segmentation_copy_outgoing',[P,N,N,c.POINTER(Jump),N])

            def make(variant):
                streams,segments=(2,3) if variant==0 else (1,2)
                boundaries=list(range(1,segments+1));durations=list(range(10,10+segments))
                labels=[[20+stream*10+index for index in range(segments)] for stream in range(streams)]
                outputs=[(I*segments)(*row) for row in labels]
                rows=(c.POINTER(I)*streams)(*outputs)
                groups=[[(2,.25),(1,scalar(.5+epsilon).value)]]+[[(1,1.0)] for _ in range(segments-1)]
                jumps=[(Jump*len(row))(*[Jump(delta,value) for delta,value in row]) for row in groups]
                slices=(JumpSlice*segments)(*[JumpSlice(row,len(row)) for row in jumps])
                child=P()
                assert child_create(streams,segments,(I*segments)(*boundaries),(I*segments)(*durations),rows,slices,c.byref(child))==0
                expected=(boundaries,durations,labels,[[(delta,bits(value)) for delta,value in row] for row in groups])
                return child,expected

            def compare(child,expected):
                boundaries,durations,labels,jumps=expected
                shape=SegmentationLayout();segments=len(boundaries)
                assert child_layout(child,c.byref(shape))==0 and (shape.streams,shape.segments)==(len(labels),segments)
                values=(I*segments)()
                assert copy_boundaries(child,0,values,segments)==0 and list(values)==boundaries
                assert copy_duration(child,0,values,segments)==0 and list(values)==durations
                for stream,row in enumerate(labels):
                    assert copy_labels(child,stream,0,values,segments)==0 and list(values)==row
                for state,row in enumerate(jumps):
                    count=N()
                    assert jump_count(child,state,c.byref(count))==0 and count.value==len(row)
                    actual=(Jump*count.value)()
                    assert copy_jumps(child,state,0,actual,len(actual))==0
                    assert [(jump.delta,bits(jump.probability)) for jump in actual]==row

        first,first_value=make(0);second,second_value=make(1)
        builder,builder_copy,built,created,copied=P(),P(),P(),P(),P()
        child,sentinel=P(0x1000),P(0x1000)
        length,presence=N(99),U(99)
        call('builder_new',3,c.byref(builder))
        call('builder_length',builder,c.byref(length));assert length.value==3
        for index in range(3):
            call('builder_has_sample',builder,index,c.byref(presence));assert presence.value==0
            status('builder_get_sample',3,builder,index,c.byref(child));assert child.value==0x1000
        status('builder_build',3,builder,c.byref(sentinel));assert sentinel.value==0x1000
        call('builder_set_sample',builder,0,first);call('builder_clone',builder,c.byref(builder_copy))
        for index,expected in [(0,1),(1,0)]:
            call('builder_has_sample',builder_copy,index,c.byref(presence));assert presence.value==expected
        status('builder_build',3,builder_copy,c.byref(sentinel));assert sentinel.value==0x1000
        call('builder_release',c.byref(builder_copy))
        call('builder_set_sample',builder,1,second);call('builder_set_sample',builder,2,first)
        call('builder_clone',builder,c.byref(builder_copy));call('builder_clear_sample',builder,1)
        call('builder_has_sample',builder,1,c.byref(presence));assert presence.value==0
        call('builder_get_sample',builder_copy,1,c.byref(child));compare(child,second_value);assert child_release(c.byref(child))==0
        call('builder_resize',builder,4);call('builder_has_sample',builder,3,c.byref(presence));assert presence.value==0
        call('builder_resize',builder,1);call('builder_build',builder,c.byref(built))
        call('length',built,c.byref(length));assert length.value==1
        call('get',built,0,c.byref(child));compare(child,first_value);assert child_release(c.byref(child))==0
        call('release',c.byref(built));call('builder_build',builder_copy,c.byref(built))
        call('set',built,1,first);call('append',built,second)
        call('length',built,c.byref(length));assert length.value==4
        for index,expected in enumerate([first_value,first_value,first_value,second_value]):
            call('get',built,index,c.byref(child));compare(child,expected);assert child_release(c.byref(child))==0
        inputs,bad=(P*3)(first.value,second.value,first.value),(P*2)(first.value,None)
        call('replace',built,inputs,3);call('clone',built,c.byref(copied));call('create',inputs,3,c.byref(created))
        sentinel=P(created.value)
        status('create',1,bad,2,c.byref(sentinel));assert sentinel.value==created.value
        status('replace',1,built,bad,2)
        status('create',2,None,N(-1).value,c.byref(sentinel));assert sentinel.value==created.value
        status('create',1,None,1,c.byref(sentinel))
        status('set',2,built,99,first);status('set',1,built,0,None);status('append',1,built,None)
        child=P(0x1000);status('get',2,built,99,c.byref(child));assert child.value==0x1000
        status('get',1,built,0,None);status('clone',1,built,None);status('length',1,built,None)
        status('builder_resize',2,builder_copy,2**31)
        call('builder_length',builder_copy,c.byref(length));assert length.value==3
        status('builder_has_sample',2,builder_copy,99,c.byref(presence));status('builder_has_sample',1,builder_copy,0,None)
        status('builder_set_sample',1,builder_copy,0,None);status('builder_clear_sample',2,builder_copy,99)
        status('builder_build',1,builder_copy,None);status('builder_clone',1,builder_copy,None);status('builder_length',1,builder_copy,None)
        builder_sentinel=P(builder.value)
        status('builder_new',3,2**31,c.byref(builder_sentinel));assert builder_sentinel.value==builder.value
        status('builder_new',1,N(-1).value,None)
        call('get',copied,1,c.byref(child))
        call('builder_release',c.byref(builder));call('builder_release',c.byref(builder_copy))
        assert child_release(c.byref(first))==0 and child_release(c.byref(second))==0
        call('release',c.byref(copied));compare(child,second_value);assert child_release(c.byref(child))==0
        for owner in [built,created]:
            for index,expected in enumerate([first_value,second_value,first_value]):
                call('get',owner,index,c.byref(child));compare(child,expected);assert child_release(c.byref(child))==0
        call('replace',built,None,0);call('length',built,c.byref(length));assert length.value==0
        call('release',c.byref(built));call('release',c.byref(created));call('release',c.byref(created));status('release',1,None)
        call('builder_release',c.byref(builder_copy));status('builder_release',1,None)
        call('builder_new',0,c.byref(builder));call('builder_build',builder,c.byref(built))
        call('length',built,c.byref(length));assert length.value==0
        call('release',c.byref(built));call('builder_release',c.byref(builder))


        # A present empty sample is different from an unfilled builder slot.
        empty = P()
        if family == 'observation':
            child_new = aux('observation_new', [N, c.POINTER(N), N, c.POINTER(P)])
            assert child_new(0, (N * 1)(2), 1, c.byref(empty)) == 0
            empty_value = (0, [(2, [])])
        else:
            child_new = aux('segmentation_new', [N, N, c.POINTER(P)])
            assert child_new(1, 0, c.byref(empty)) == 0
            empty_value = ([], [], [[]], [])
        call('builder_new', 2, c.byref(builder))
        call('builder_set_sample', builder, 0, empty)
        call('builder_clone', builder, c.byref(builder_copy))
        call('builder_has_sample', builder_copy, 0, c.byref(presence))
        assert presence.value == 1
        call('builder_has_sample', builder_copy, 1, c.byref(presence))
        assert presence.value == 0
        sentinel = P(0x1000)
        status('builder_build', 3, builder_copy, c.byref(sentinel))
        assert sentinel.value == 0x1000
        call('builder_get_sample', builder_copy, 0, c.byref(child))
        compare(child, empty_value)
        assert child_release(c.byref(child)) == 0
        call('builder_resize', builder_copy, 1)
        call('builder_build', builder_copy, c.byref(built))
        call('length', built, c.byref(length))
        assert length.value == 1
        call('get', built, 0, c.byref(child))
        assert child_release(c.byref(empty)) == 0
        call('builder_release', c.byref(builder))
        call('builder_release', c.byref(builder_copy))
        call('release', c.byref(built))
        compare(child, empty_value)
        assert child_release(c.byref(child)) == 0

expected={f'liblrhsmm_rs_{family}_set_{method}_{precision}' for family in ['observation','segmentation'] for precision in ['f32','f64'] for method in functions}
assert called==expected and len(called)==72
print('Sample sets ctypes: all72 exports, complete fields, missing slots and independent lifetimes passed')
