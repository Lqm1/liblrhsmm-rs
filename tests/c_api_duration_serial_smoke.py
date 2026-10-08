"""Actual ctypes byte/I/O and legacy duration codec verification."""
import ctypes as c
from pathlib import Path
import sys
lib=c.CDLL(sys.argv[1]); N=c.c_size_t; U=c.c_uint32; P=c.c_void_p; B=c.c_uint8
READ=c.CFUNCTYPE(U,P,c.POINTER(B),N,c.POINTER(N))
WRITE=c.CFUNCTYPE(U,P,c.POINTER(B),N,c.POINTER(N))
FLUSH=c.CFUNCTYPE(U,P)
class ReadStream(c.Structure): _fields_=[("context",P),("read",READ)]
class WriteStream(c.Structure): _fields_=[("context",P),("write",WRITE),("flush",FLUSH)]
class Limits(c.Structure): _fields_=[("max_array_entries",N)]
def function(name,args):
    fn=getattr(lib,"liblrhsmm_rs_"+name);fn.argtypes,fn.restype=args,U;return fn
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
    assert sys.argv[2]=="--no-serialization"
    optional=["decode_limits_default"]+[f"duration_{name}_{t}" for t in ["f32","f64"] for name in
              ["read_bytes","read_stream","read_stream_prefix","write_bytes","write_stream"]]
    for name in optional: assert not hasattr(lib,"liblrhsmm_rs_"+name),name
    print("Python byte/I/O core: all6 exports passed; all11 optional codec exports absent")
    sys.exit(0)
limits=Limits();assert function("decode_limits_default",[c.POINTER(Limits)])(c.byref(limits))==0
assert limits.max_array_entries==16*1024*1024
fixture=(Path(__file__).parent/"fixtures/c-standalone-duration.bin").read_bytes()
raw=(B*len(fixture)).from_buffer_copy(fixture)
for t in ["f32","f64"]:
    read_bytes=function(f"duration_read_bytes_{t}",[c.POINTER(B),N,c.POINTER(Limits),c.POINTER(P)])
    read_stream=function(f"duration_read_stream_{t}",[c.POINTER(ReadStream),c.POINTER(Limits),c.POINTER(P)])
    read_prefix=function(f"duration_read_stream_prefix_{t}",[c.POINTER(ReadStream),c.POINTER(Limits),c.POINTER(P)])
    write_bytes=function(f"duration_write_bytes_{t}",[P,c.POINTER(P)])
    write_stream=function(f"duration_write_stream_{t}",[P,c.POINTER(WriteStream)])
    release_duration=function(f"duration_release_{t}",[c.POINTER(P)])
    duration=P();encoded=P();decoded=P()
    assert read_bytes(raw,len(fixture),c.byref(limits),c.byref(duration))==0
    assert write_bytes(duration,c.byref(encoded))==0 and owner_bytes(encoded)==fixture
    incoming={"bytes":fixture,"position":0,"calls":0,"interrupt":True,"status":0,"oversized":False}
    @READ
    def reading(context,output,capacity,count):
        incoming["calls"]+=1
        if incoming["interrupt"]: incoming["interrupt"]=False;return 1
        if incoming["status"]: return incoming["status"]
        if incoming["oversized"]: count[0]=capacity+1;return 0
        size=min(capacity,1,len(incoming["bytes"])-incoming["position"])
        if size: output[0]=incoming["bytes"][incoming["position"]]
        incoming["position"]+=size;count[0]=size;return 0
    source=ReadStream(None,reading)
    assert read_stream(c.byref(source),c.byref(limits),c.byref(decoded))==0
    assert incoming["position"]==len(fixture) and incoming["calls"]>len(fixture)
    outgoing={"bytes":bytearray(),"flushes":0,"interrupt":True,"status":0,"oversized":False,"zero":False}
    @WRITE
    def writing(context,data,capacity,count):
        if outgoing["interrupt"]: outgoing["interrupt"]=False;return 1
        if outgoing["status"]: return outgoing["status"]
        if outgoing["oversized"]: count[0]=capacity+1;return 0
        size=0 if outgoing["zero"] else min(capacity,1)
        if size: outgoing["bytes"].append(data[0])
        count[0]=size;return 0
    @FLUSH
    def output_flush(context): outgoing["flushes"]+=1;return outgoing["status"]
    sink=WriteStream(None,writing,output_flush)
    assert write_stream(decoded,c.byref(sink))==0 and outgoing["bytes"]==fixture and outgoing["flushes"]==0
    assert flush_stream(c.byref(sink))==0 and outgoing["flushes"]==1
    assert release_duration(c.byref(decoded))==0
    incoming.update(bytes=fixture+fixture+bytes([42]),position=0,interrupt=True)
    for position in [len(fixture),len(fixture)*2]:
        assert read_prefix(c.byref(source),c.byref(limits),c.byref(decoded))==0 and incoming["position"]==position
        assert release_duration(c.byref(decoded))==0
    assert incoming["bytes"][incoming["position"]]==42
    for end in range(len(fixture)):
        assert read_bytes(raw,end,c.byref(limits),c.byref(decoded))==3 and decoded.value is None
    trailing=(B*(len(fixture)+1)).from_buffer_copy(fixture+bytes([0]))
    assert read_bytes(trailing,len(trailing),c.byref(limits),c.byref(decoded))==3
    tight=Limits(5);assert read_bytes(raw,len(fixture),c.byref(tight),c.byref(decoded))==3
    tight.max_array_entries=6;assert read_bytes(raw,len(fixture),c.byref(tight),c.byref(decoded))==0
    assert release_duration(c.byref(decoded))==0
    incoming.update(interrupt=False,status=2,position=0)
    assert read_stream(c.byref(source),c.byref(limits),c.byref(decoded))==3
    incoming.update(status=0,oversized=True)
    assert read_stream(c.byref(source),c.byref(limits),c.byref(decoded))==3
    calls=incoming["calls"];assert read_stream(c.byref(source),c.byref(limits),None)==1 and incoming["calls"]==calls
    source.read=READ();assert read_stream(c.byref(source),c.byref(limits),c.byref(decoded))==3
    outgoing.update(status=2);assert write_stream(duration,c.byref(sink))==3
    outgoing.update(status=0,oversized=True);assert write_stream(duration,c.byref(sink))==3
    outgoing.update(oversized=False,zero=True);assert write_stream(duration,c.byref(sink))==3
    sink.write=WRITE();assert write_stream(duration,c.byref(sink))==3
    assert write_bytes(duration,None)==1 and write_stream(duration,None)==1
    assert read_bytes(None,N(-1).value,c.byref(limits),c.byref(decoded))==2
    assert read_bytes(None,0,c.byref(limits),c.byref(decoded))==3
    assert read_bytes(raw,len(fixture),c.byref(limits),None)==1
    assert release_duration(c.byref(duration))==0 and release(c.byref(encoded))==0
print("Python duration codec: all17 new exports, original wire and sequential object boundaries passed")
