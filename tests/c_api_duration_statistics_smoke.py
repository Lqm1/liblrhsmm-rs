"""Actual ctypes duration statistics/updates in both precisions."""
import ctypes as c
from pathlib import Path
import sys
library = c.CDLL(sys.argv[1])
for suffix, scalar, fixture in [("f32",c.c_float,"c-updates.txt"),("f64",c.c_double,"c-double-update.txt")]:
    class Values(c.Structure):
        _fields_ = [("value_sum",scalar),("squared_sum",scalar),("occupancy",scalar)]
    class Parameters(c.Structure):
        _fields_ = [("mean",scalar),("variance",scalar),("minimum",c.c_int32),("maximum",c.c_int32),
                    ("fixed_mean",c.c_int32),("variance_floor",scalar)]
    class Options(c.Structure):
        _fields_ = [("duration_mode",c.c_uint32),("duration_variance_floor",scalar),("emission_variance_floor",scalar)]
    def function(name,args):
        result=getattr(library,f"liblrhsmm_rs_{name}_{suffix}")
        result.argtypes,result.restype=args,c.c_uint32
        return result
    default=function("duration_statistics_default",[c.POINTER(Values)])
    create=function("duration_statistics_create",[c.POINTER(Values),c.POINTER(c.c_void_p)])
    from_duration=function("duration_statistics_from_duration",[c.c_void_p,c.POINTER(c.c_void_p)])
    get=function("duration_statistics_get",[c.c_void_p,c.POINTER(Values)])
    set_values=function("duration_statistics_set",[c.c_void_p,c.POINTER(Values)])
    clone=function("duration_statistics_clone",[c.c_void_p,c.POINTER(c.c_void_p)])
    clear=function("duration_statistics_clear",[c.c_void_p])
    release=function("duration_statistics_release",[c.POINTER(c.c_void_p)])
    options_default=function("update_options_default",[c.POINTER(Options)])
    update=function("duration_update",[c.c_void_p,c.c_void_p,c.POINTER(Options),c.POINTER(c.c_uint32)])
    parameter_default=function("duration_default",[c.POINTER(Parameters)])
    parameter_create=function("duration_create",[c.POINTER(Parameters),c.POINTER(c.c_void_p)])
    parameter_set=function("duration_set",[c.c_void_p,c.POINTER(Parameters)])
    parameter_get=function("duration_get",[c.c_void_p,c.POINTER(Parameters)])
    parameter_release=function("duration_release",[c.POINTER(c.c_void_p)])
    values, fetched, parameters, options = Values(99,99,99), Values(), Parameters(), Options()
    assert default(c.byref(values))==0 and (values.value_sum,values.squared_sum,values.occupancy)==(0,0,0)
    assert parameter_default(c.byref(parameters))==0
    duration, statistics, copied, direct = [c.c_void_p() for _ in range(4)]
    assert parameter_create(c.byref(parameters),c.byref(duration))==0
    assert from_duration(duration,c.byref(statistics))==0
    assert parameter_release(c.byref(duration))==0
    values=Values(6,20,2)
    assert set_values(statistics,c.byref(values))==0 and clone(statistics,c.byref(copied))==0
    assert clear(statistics)==0 and get(statistics,c.byref(fetched))==0
    assert (fetched.value_sum,fetched.squared_sum,fetched.occupancy)==(0,0,0)
    assert get(copied,c.byref(fetched))==0 and (fetched.value_sum,fetched.squared_sum,fetched.occupancy)==(6,20,2)
    assert create(c.byref(values),c.byref(direct))==0
    assert release(c.byref(direct))==0 and direct.value is None
    assert release(c.byref(direct))==0
    assert create(None,c.byref(direct))==1 and create(c.byref(values),None)==1
    assert from_duration(None,c.byref(direct))==1
    for fn in [get,set_values,clone]: assert fn(copied,None)==1
    assert clear(None)==1 and default(None)==1 and release(None)==1
    assert release(c.byref(copied))==0
    assert options_default(c.byref(options))==0
    assert (options.duration_mode,options.duration_variance_floor,options.emission_variance_floor)==(0,scalar(.05).value,scalar(.01).value)
    parameters.mean,parameters.variance=7,3
    assert parameter_create(c.byref(parameters),c.byref(duration))==0
    compared=0
    updated=c.c_uint32(99)
    for line in (Path(__file__).parent/"fixtures"/fixture).read_text().splitlines():
        fields=line.split()
        if fields[0]!="duration": continue
        scenario=int(fields[1])
        parameters.mean,parameters.variance,parameters.variance_floor=7,3,2.5 if scenario==2 else 0
        values=Values(1.5 if scenario==3 else 9,29,1e-8 if scenario==4 else 3)
        options.duration_mode=1 if scenario==3 else 0
        assert parameter_set(duration,c.byref(parameters))==0 and set_values(statistics,c.byref(values))==0
        updated.value=99
        assert update(duration,statistics,c.byref(options),c.byref(updated))==0 and updated.value==(0 if scenario==4 else 1)
        assert parameter_get(duration,c.byref(parameters))==0
        for actual,expected in zip([parameters.mean,parameters.variance],map(float,fields[2:])):
            if suffix=="f32": assert actual==scalar(expected).value
            else: assert abs(actual-expected)/max(abs(expected),1)<=2e-14
            compared+=1
    assert compared==10
    options.duration_mode=0
    parameters.fixed_mean,parameters.variance_floor=4,0
    values=Values(6,20,2)
    assert parameter_set(duration,c.byref(parameters))==0 and set_values(statistics,c.byref(values))==0
    assert update(duration,statistics,c.byref(options),c.byref(updated))==0 and updated.value==1
    assert parameter_get(duration,c.byref(parameters))==0 and (parameters.mean,parameters.variance)==(4,2)
    options.duration_mode=99; updated.value=99
    assert update(duration,statistics,c.byref(options),c.byref(updated))==3 and updated.value==99
    options.duration_mode=0
    assert update(duration,statistics,c.byref(options),None)==1
    options.emission_variance_floor=float("nan")
    assert update(duration,statistics,c.byref(options),c.byref(updated))==3 and updated.value==99
    options.emission_variance_floor=.01
    values.value_sum=float("nan")
    assert set_values(statistics,c.byref(values))==0
    assert update(duration,statistics,c.byref(options),c.byref(updated))==3 and updated.value==99
    assert parameter_get(duration,c.byref(parameters))==0 and (parameters.mean,parameters.variance)==(4,2)
    assert update(duration,None,c.byref(options),c.byref(updated))==1
    assert update(duration,statistics,None,c.byref(updated))==1
    assert update(None,statistics,c.byref(options),c.byref(updated))==1 and options_default(None)==1
    assert clear(statistics)==0 and release(c.byref(statistics))==0
    assert parameter_release(c.byref(duration))==0
print("Python duration statistics: all20 exports and20 original C values passed")
