"""Actual ctypes byte/I/O and original segmentation codec verification."""
import ctypes as c
from pathlib import Path
import sys
import struct
lib=c.CDLL(sys.argv[1]); N=c.c_size_t; U=c.c_uint32; P=c.c_void_p; B=c.c_uint8
READ=c.CFUNCTYPE(U,P,c.POINTER(B),N,c.POINTER(N))
WRITE=c.CFUNCTYPE(U,P,c.POINTER(B),N,c.POINTER(N))
FLUSH=c.CFUNCTYPE(U,P)
class ReadStream(c.Structure): _fields_=[("context",P),("read",READ)]
class WriteStream(c.Structure): _fields_=[("context",P),("write",WRITE),("flush",FLUSH)]
class Limits(c.Structure): _fields_=[("max_array_entries",N)]
called=set()

def function(name,args):
    fn=getattr(lib,"liblrhsmm_rs_"+name);fn.argtypes,fn.restype=args,U
    def invoke(*arguments):
        called.add(name)
        return fn(*arguments)
    return invoke
create=function("bytes_create",[c.POINTER(B),N,c.POINTER(P)])
clone=function("bytes_clone",[P,c.POINTER(P)])
length=function("bytes_length",[P,c.POINTER(N)])
copy=function("bytes_copy",[P,N,c.POINTER(B),N])
release=function("bytes_release",[c.POINTER(P)])
flush_stream=function("write_stream_flush",[c.POINTER(WriteStream)])
def owner_bytes(owner):
    size=N();assert length(owner,c.byref(size))==0
    data=(B*size.value)();assert copy(owner,0,data,size.value)==0;return bytes(data)
original=(B*4)(1,2,3,4);owner=P();copied=P()
assert create(original,4,c.byref(owner))==0;original[0]=99
assert owner_bytes(owner)==bytes([1,2,3,4]) and clone(owner,c.byref(copied))==0
assert release(c.byref(owner))==0 and release(c.byref(owner))==0
assert owner_bytes(copied)==bytes([1,2,3,4])
assert copy(copied,4,None,0)==0 and copy(copied,5,None,0)==2
result=(B*2)(71,71);assert copy(copied,3,result,2)==2 and list(result)==[71,71]
assert copy(copied,N(-1).value,result,1)==2 and copy(copied,0,None,1)==1
assert length(copied,None)==1 and clone(copied,None)==1
assert release(c.byref(copied))==0 and create(None,N(-1).value,c.byref(owner))==2
assert create(None,0,c.byref(owner))==0 and owner_bytes(owner)==b""
assert release(c.byref(owner))==0 and release(None)==1
state={"flushes":0,"status":0}
@FLUSH
def flushing(context):
    state["flushes"]+=1;return state["status"]
sink=WriteStream(None,WRITE(),flushing)
assert flush_stream(c.byref(sink))==0 and state["flushes"]==1
state["status"]=1;assert flush_stream(c.byref(sink))==3
state["status"]=2;assert flush_stream(c.byref(sink))==3
sink.flush=FLUSH();assert flush_stream(c.byref(sink))==3 and flush_stream(None)==1
if len(sys.argv)>2:
    assert sys.argv[2]=='--no-serialization'
    optional=[f'{family}_set_{method}_{precision}' for family in ['observation','segmentation'] for precision in ['f32','f64'] for method in ['read_bytes','read_stream','read_stream_prefix','write_bytes','write_stream']]
    for name in optional: assert not hasattr(lib,'liblrhsmm_rs_'+name),name
    print('Set codecs ctypes: all20 optional exports absent without serialization')
    sys.exit(0)
limits=Limits();assert function('decode_limits_default',[c.POINTER(Limits)])(c.byref(limits))==0
assert limits.max_array_entries==16*1024*1024
for family in ['observation','segmentation']:
    for t in ['f32','f64']:
        fixtures = [('c-observation-set.bin',34,0)] if family=='observation' else [('c-segmentation-set.bin',66,0),('c-copy-segmentation-set.bin',62,1)]
        fixtures.append((None,0,2))
        scalar,code=(c.c_float,'f') if t=='f32' else (c.c_double,'d')
        class ObservationLayout(c.Structure):
            _fields_=[('frames',N),('streams',N)]
        class StreamLayout(c.Structure):
            _fields_=[('frames',N),('dimensions',N),('values',N)]
        class SegmentationLayout(c.Structure):
            _fields_=[('streams',N),('segments',N)]
        class Jump(c.Structure):
            _fields_=[('delta',c.c_int32),('probability',scalar)]
        set_length=function(f'{family}_set_length_{t}',[P,c.POINTER(N)])
        get=function(f'{family}_set_get_{t}',[P,N,c.POINTER(P)])
        child_release=function(f'{family}_release_{t}',[c.POINTER(P)])
        if family=='observation':
            shape=function(f'observation_layout_{t}',[P,c.POINTER(ObservationLayout)])
            stream_shape=function(f'observation_get_stream_layout_{t}',[P,N,c.POINTER(StreamLayout)])
            copy_values=function(f'observation_copy_values_{t}',[P,N,N,c.POINTER(scalar),N])
        else:
            shape=function(f'segmentation_layout_{t}',[P,c.POINTER(SegmentationLayout)])
            copy_boundaries=function(f'segmentation_copy_boundaries_{t}',[P,N,c.POINTER(c.c_int32),N])
            copy_duration=function(f'segmentation_copy_duration_states_{t}',[P,N,c.POINTER(c.c_int32),N])
            copy_labels=function(f'segmentation_copy_output_states_{t}',[P,N,N,c.POINTER(c.c_int32),N])
            jump_count=function(f'segmentation_outgoing_count_{t}',[P,N,c.POINTER(N)])
            copy_jumps=function(f'segmentation_copy_outgoing_{t}',[P,N,N,c.POINTER(Jump),N])
        def bits(value): return struct.pack('<'+code,value)
        for path,budget,variant in fixtures:
            fixture=(Path(__file__).parent/'fixtures'/path).read_bytes() if path else bytes([0x90])
            raw=(B*len(fixture)).from_buffer_copy(fixture)
            read_bytes=function(f'{family}_set_read_bytes_{t}',[c.POINTER(B),N,c.POINTER(Limits),c.POINTER(P)])
            read_stream=function(f'{family}_set_read_stream_{t}',[c.POINTER(ReadStream),c.POINTER(Limits),c.POINTER(P)])
            read_prefix=function(f'{family}_set_read_stream_prefix_{t}',[c.POINTER(ReadStream),c.POINTER(Limits),c.POINTER(P)])
            write_bytes=function(f'{family}_set_write_bytes_{t}',[P,c.POINTER(P)])
            write_stream=function(f'{family}_set_write_stream_{t}',[P,c.POINTER(WriteStream)])
            release_stream=function(f'{family}_set_release_{t}',[c.POINTER(P)])
            def compare(owner):
                length=N(99)
                assert set_length(owner,c.byref(length))==0 and length.value==(0 if variant==2 else 2)
                for index in range(length.value):
                    child=P();assert get(owner,index,c.byref(child))==0
                    if family=='observation':
                        layout=ObservationLayout()
                        assert shape(child,c.byref(layout))==0 and (layout.frames,layout.streams)==(3,2)
                        for stream,dimension in enumerate([2,1]):
                            layout=StreamLayout()
                            assert stream_shape(child,stream,c.byref(layout))==0 and (layout.frames,layout.dimensions,layout.values)==(3,dimension,3*dimension)
                            actual=(scalar*(3*dimension))()
                            assert copy_values(child,stream,0,actual,len(actual))==0
                            expected=[(frame-1)*.25+d+stream*10 for frame in range(3) for d in range(dimension)]
                            assert [bits(v) for v in actual]==[bits(v) for v in expected]
                    else:
                        layout=SegmentationLayout();values=(c.c_int32*3)()
                        assert shape(child,c.byref(layout))==0 and (layout.streams,layout.segments)==(2,3)
                        assert copy_boundaries(child,0,values,3)==0 and list(values)==[2,4,6]
                        assert copy_duration(child,0,values,3)==0 and list(values)==([3,4,5] if variant else [0,1,2])
                        labels=[[10,11,12],[20,19,18]] if variant else [[0,1,2],[2,1,0]]
                        for stream,row in enumerate(labels):
                            assert copy_labels(child,stream,0,values,3)==0 and list(values)==row
                        groups=[[(2,.25),(1,.75)],[(0,.1),(1,.9)],[(1,1.)]] if variant else [[(2,.25),(1,.75)],[(1,1.)],[(0,.2),(-1,.1),(1,.7)]]
                        for state,row in enumerate(groups):
                            count=N();assert jump_count(child,state,c.byref(count))==0 and count.value==len(row)
                            jumps=(Jump*count.value)();assert copy_jumps(child,state,0,jumps,count.value)==0
                            assert [(j.delta,bits(j.probability)) for j in jumps]==[(delta,bits(c.c_float(probability).value)) for delta,probability in row]
                    assert child_release(c.byref(child))==0
            stream=P();encoded=P();decoded=P()
            assert read_bytes(raw,len(fixture),c.byref(limits),c.byref(stream))==0;compare(stream)
            assert write_bytes(stream,c.byref(encoded))==0 and owner_bytes(encoded)==fixture
            incoming={"bytes":fixture,"position":0,"calls":0,"interrupt":True,"status":0,"oversized":False}
            @READ
            def reading(context,output,capacity,count):
                incoming["calls"]+=1
                if incoming["interrupt"]: incoming["interrupt"]=False;return 1
                if incoming["status"]: return incoming["status"]
                if incoming["oversized"]: count[0]=capacity+1;return 0
                if incoming.get("reject_eof") and incoming["position"]==len(incoming["bytes"]): return 2
                size=min(capacity,1,len(incoming["bytes"])-incoming["position"])
                if size: output[0]=incoming["bytes"][incoming["position"]]
                incoming["position"]+=size;count[0]=size;return 0
            source=ReadStream(None,reading)
            assert read_stream(c.byref(source),c.byref(limits),c.byref(decoded))==0
            assert incoming["position"]==len(fixture) and incoming["calls"]>len(fixture);compare(decoded)
            outgoing={"bytes":bytearray(),"flushes":0,"interrupt":True,"status":0,"oversized":False,"zero":False}
            @WRITE
            def writing(context,data,capacity,count):
                outgoing["calls"]=outgoing.get("calls",0)+1
                if outgoing["interrupt"]: outgoing["interrupt"]=False;return 1
                if outgoing["status"]: return outgoing["status"]
                if outgoing["oversized"]: count[0]=capacity+1;return 0
                if outgoing.get("fail_after") and len(outgoing["bytes"])>=outgoing["fail_after"]: return 2
                size=0 if outgoing["zero"] else min(capacity,1)
                if size: outgoing["bytes"].append(data[0])
                count[0]=size;return 0
            @FLUSH
            def output_flush(context): outgoing["flushes"]+=1;return outgoing["status"]
            sink=WriteStream(None,writing,output_flush)
            assert write_stream(decoded,c.byref(sink))==0 and outgoing["bytes"]==fixture and outgoing["flushes"]==0
            assert flush_stream(c.byref(sink))==0 and outgoing["flushes"]==1
            compare(decoded);assert release_stream(c.byref(decoded))==0
            incoming.update(bytes=fixture+fixture+bytes([42]),position=0,interrupt=True)
            for position in [len(fixture),len(fixture)*2]:
                assert read_prefix(c.byref(source),c.byref(limits),c.byref(decoded))==0 and incoming["position"]==position
                assert release_stream(c.byref(decoded))==0
            assert incoming["bytes"][incoming["position"]]==42
            incoming.update(bytes=fixture,position=0,interrupt=False,reject_eof=True)
            assert read_prefix(c.byref(source),c.byref(limits),c.byref(decoded))==0 and incoming['position']==len(fixture)
            compare(decoded);assert release_stream(c.byref(decoded))==0
            incoming['position']=0
            assert read_stream(c.byref(source),c.byref(limits),c.byref(decoded))==3 and decoded.value is None
            incoming['reject_eof']=False
            for end in range(len(fixture)):
                assert read_bytes(raw,end,c.byref(limits),c.byref(decoded))==3 and decoded.value is None
            trailing=(B*(len(fixture)+1)).from_buffer_copy(fixture+bytes([0]))
            assert read_bytes(trailing,len(trailing),c.byref(limits),c.byref(decoded))==3
            if budget:
                tight=Limits(budget-1);assert read_bytes(raw,len(fixture),c.byref(tight),c.byref(decoded))==3
            tight=Limits(budget);assert read_bytes(raw,len(fixture),c.byref(tight),c.byref(decoded))==0
            assert release_stream(c.byref(decoded))==0
            incoming.update(interrupt=False,status=2,position=0)
            assert read_stream(c.byref(source),c.byref(limits),c.byref(decoded))==3
            incoming.update(status=0,oversized=True)
            assert read_stream(c.byref(source),c.byref(limits),c.byref(decoded))==3
            calls=incoming["calls"];assert read_stream(c.byref(source),c.byref(limits),None)==1 and incoming["calls"]==calls
            source.read=READ();assert read_stream(c.byref(source),c.byref(limits),c.byref(decoded))==3
            outgoing.update(status=2);assert write_stream(stream,c.byref(sink))==3
            outgoing.update(status=0,oversized=True);assert write_stream(stream,c.byref(sink))==3
            outgoing.update(oversized=False,zero=True);assert write_stream(stream,c.byref(sink))==3
            sink.write=WRITE();assert write_stream(stream,c.byref(sink))==3
            assert write_bytes(stream,None)==1 and write_stream(stream,None)==1
            assert read_bytes(None,N(-1).value,c.byref(limits),c.byref(decoded))==2
            assert read_bytes(None,0,c.byref(limits),c.byref(decoded))==3
            assert read_bytes(raw,len(fixture),c.byref(limits),None)==1
            sink.write=writing;outgoing.update(status=0,oversized=False,zero=False,interrupt=False,calls=0)
            if len(fixture)>7:
                outgoing.update(bytes=bytearray(),fail_after=7)
                assert write_stream(stream,c.byref(sink))==3 and len(outgoing['bytes'])==7
            sentinel=P(stream.value)
            assert read_bytes(raw,len(fixture)-1,c.byref(limits),c.byref(sentinel))==3 and sentinel.value==stream.value
            compare(stream)
            assert release_stream(c.byref(stream))==0 and owner_bytes(encoded)==fixture
            assert release(c.byref(encoded))==0
expected={f'{family}_set_{method}_{precision}' for family in ['observation','segmentation'] for precision in ['f32','f64'] for method in ['read_bytes','read_stream','read_stream_prefix','write_bytes','write_stream']}
assert expected<=called and len(expected)==20
print('Set codecs ctypes: all20 exports, complete original C fields/wire, limits and sequential object boundaries passed')
