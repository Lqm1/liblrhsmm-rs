"""ctypes comparisons against both complete original numerical corpora."""
import ctypes as c
from pathlib import Path
import struct
import sys

lib=c.CDLL(sys.argv[1])
U=c.c_uint32
N=c.c_size_t


class Constants(c.Structure):
    _fields_=[("log_add_count",N),("exponential_count",N),("log_add_step",c.c_double),
              ("exponential_step",c.c_double),("occupancy_epsilon",c.c_double),
              ("log_zero_f32",c.c_float),("log_zero_f64",c.c_double),("pi",c.c_double)]


def function(name,args):
    fn=getattr(lib,"liblrhsmm_rs_"+name)
    fn.argtypes,fn.restype=args,U
    return fn


assert function("abi_version",[])()==1
constants=Constants()
get_constants=function("numerical_constants",[c.POINTER(Constants)])
assert get_constants(c.byref(constants))==0
assert (constants.log_add_count,constants.exponential_count)==(7000,7000)
assert (constants.log_add_step,constants.exponential_step,constants.occupancy_epsilon)==(0.001,0.002,1e-7)
assert constants.log_zero_f32==c.c_float(-1e15).value and constants.log_zero_f64==-1e15
assert constants.pi==3.14159265358979323846 and get_constants(None)==1
for suffix,scalar,fmt,label in [("f32",c.c_float,"f","float"),("f64",c.c_double,"d","double")]:
    width=c.sizeof(scalar)
    copy=function("table_copy_"+suffix,[U,N,c.POINTER(scalar),N])
    get=function("table_get_"+suffix,[U,N,c.POINTER(scalar)])
    replace=function("table_replace_"+suffix,[U,N,scalar,c.POINTER(scalar)])
    add=function("log_add_"+suffix,[scalar,scalar,c.POINTER(scalar)])
    negative=function("negative_exponential_"+suffix,[scalar,c.POINTER(scalar)])
    exp=function("exponential_"+suffix,[scalar,c.POINTER(scalar)])
    data=(Path(__file__).parent/"fixtures"/f"c-numerics-{label}.bin").read_bytes()
    assert data[:4]==b"NUM1" and struct.unpack_from("<II",data,4)==(width,7000)
    offset=12
    for table in range(2):
        output=(scalar*7000)()
        assert copy(table,0,output,7000)==0
        expected=data[offset:offset+7000*width]
        for i in range(7000):
            assert struct.pack("<"+fmt,output[i])==expected[i*width:(i+1)*width]
        offset+=7000*width
    assert struct.unpack_from("<I",data,offset)[0]==42008
    offset+=4
    for record in range(42008):
        kind=struct.unpack_from("<I",data,offset)[0]
        offset+=4
        a,b=struct.unpack_from("<"+fmt*2,data,offset)
        expected=data[offset+2*width:offset+3*width]
        offset+=3*width
        output=scalar()
        status=add(a,b,c.byref(output)) if kind==0 else negative(a,c.byref(output)) if kind==1 else exp(a,c.byref(output))
        assert kind<3 and status==0
        assert struct.pack("<"+fmt,output.value)==expected,(suffix,record)
    assert offset==len(data)
    for table in range(2):
        original,previous,current=scalar(),scalar(),scalar()
        assert get(table,0,c.byref(original))==0
        changed=0.125 if table==0 else 0.375
        assert replace(table,0,changed,c.byref(previous))==0 and previous.value==original.value
        try:
            assert (add(0,0,c.byref(current)) if table==0 else negative(-0.001,c.byref(current)))==0
            assert current.value==changed
            assert replace(table,0,1,None)==1
            assert get(table,0,c.byref(current))==0 and current.value==changed
            output=(scalar*2)(71,71)
            assert copy(table,6999,output,2)==2 and list(output)==[71,71]
            assert copy(table,7000,None,0)==0 and copy(table,7001,None,0)==2
            assert get(99,0,c.byref(current))==3 and get(table,7000,c.byref(current))==2
        finally:
            assert replace(table,0,original,c.byref(previous))==0 and previous.value==changed
    assert exp(0,None)==1
