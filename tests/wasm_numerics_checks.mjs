function check(value,label){if(!value)throw new Error(label);}
function throws(action,label){let failed=false;try{action();}catch{failed=true;}check(failed,label);}
function same(a,b,label){check(a.length===b.length&&a.every((v,i)=>Object.is(v,b[i])),label);}
export async function verifyNumericalBindings(api,fixture){
    let cases=0,coefficientComparisons=0;
    const constants=api.numerical_constants();same(constants,new Float64Array([7000,0.001,7000,0.002,1e-7,Math.fround(-1e15),-1e15]),'all native constants');constants[0]=0;check(api.numerical_constants()[0]===7000,'copied constants');
    for(const width of [4,8]){
        const T=width===4?Float32Array:Float64Array,suffix=width===4?'f32':'f64',Owner=api['NumericalTableF'+width*8],raw=new Uint8Array(await fixture('c-numerics-'+(width===4?'float':'double')+'.bin')),view=new DataView(raw.buffer,raw.byteOffset,raw.byteLength);let offset=4;
        const integer=()=>{const value=view.getUint32(offset,true);offset+=4;return value;},scalar=()=>{const value=width===4?view.getFloat32(offset,true):view.getFloat64(offset,true);offset+=width;return value;};
        check(String.fromCharCode(...raw.slice(0,4))==='NUM1'&&integer()===width&&integer()===7000,'original numerical corpus header');
        for(const code of [0,1]){
            const expected=new T(Array.from({length:7000},scalar)),table=new Owner(code),values=table.values(),iterator=table.iter(),globalIterator=api['numerical_'+(code===0?'log_add':'exponential')+'_table_'+suffix]();
            check(table.length===7000&&!table.is_empty()&&iterator.length===7000,'complete table metadata');same(values,expected,'all original coefficients');coefficientComparisons+=7000;
            check(Object.is(iterator.next(),expected[0])&&iterator.length===6999,'forward iterator');check(Object.is(iterator.next_back(),expected[6999])&&iterator.length===6998,'backward iterator');
            for(let i=1;i<6999;i++)check(Object.is(iterator.next(),expected[i]),'complete exact iterator');check(iterator.length===0&&iterator.next()===undefined&&iterator.next_back()===undefined&&iterator.next_storage().length===0,'exhausted iterator');same(globalIterator.next_storage(),expected.slice(0,1),'global table iterator');
            check(table.get(7000)===undefined&&table.replace(7000,1)===undefined&&table.get_storage(7000).length===0,'invalid index returns none');
            const clone=table.cloned(),live=table.iter(),other=new Owner(1-code),otherBefore=other.get(0),old=table.get_storage(0),oldLast=table.get_storage(6999),snapshot=table.values();
            try{
                check(Object.is(table.replace(0,0.125),old[0]),'atomic replacement previous value');check(clone.get(0)===0.125&&live.next()===0.125,'shared global handle and live iterator');check(other.get(0)===otherBefore,'table identities independent');table.replace(6999,0.25);check(live.next_back()===0.25,'live backward iterator');
                if(code===0)check(api['numerical_log_add_'+suffix](0,0)===0.125,'table edit affects log addition');else check(api['numerical_negative_exponential_'+suffix](-0.001)===0.125,'table edit affects exponential');
                const bits=width===4?new Uint32Array([0xffc00123]):new BigUint64Array([0xfff8000000000123n]),payload=new T(bits.buffer);table.replace_storage(0,payload);const actual=table.get_storage(0),actualBits=width===4?new Uint32Array(actual.buffer):new BigUint64Array(actual.buffer);same(actualBits,bits,'all coefficient IEEE bits retained');check(Number.isNaN(table.get(0)),'numeric optional NaN remains present');
                const typedIterator=table.iter(),first=typedIterator.next_storage();same(width===4?new Uint32Array(first.buffer):new BigUint64Array(first.buffer),bits,'iterator raw coefficient payload');same(typedIterator.next_back_storage(),new T([0.25]),'typed reverse iterator');typedIterator.free();
                throws(()=>table.replace_storage(0,new T()),'replacement shape');same(width===4?new Uint32Array(table.get_storage(0).buffer):new BigUint64Array(table.get_storage(0).buffer),bits,'replacement shape failure atomic');
            }finally{table.replace_storage(0,old);table.replace_storage(6999,oldLast);}
            same(table.values(),expected,'original table restored');same(snapshot,expected,'snapshot independent of edits');live.free();other.free();clone.free();iterator.free();globalIterator.free();table.free();same(values,expected,'snapshot survives handle release');
        }
        const count=integer();check(count===42008,'all source helper records');
        for(let c=0;c<count;c++){cases++;const kind=integer(),a=scalar(),b=scalar(),expected=scalar(),actual=kind===0?api['numerical_log_add_'+suffix](a,b):kind===1?api['numerical_negative_exponential_'+suffix](a):api['numerical_exponential_'+suffix](a);check(Object.is(actual,expected),'exact original C numerical record '+c);}
        check(offset===raw.length,'complete numerical corpus consumption');
        const log=api['numerical_log_add_'+suffix],negative=api['numerical_negative_exponential_'+suffix],exp=api['numerical_exponential_'+suffix];
        for(const infinity of [Infinity,-Infinity])check(log(infinity,infinity)===infinity,'equal infinite log inputs');check(log(width===4?3.4028234663852886e38:Number.MAX_VALUE,width===4?-3.4028234663852886e38:-Number.MAX_VALUE)===(width===4?3.4028234663852886e38:Number.MAX_VALUE),'overflow difference cutoff');
        check(Number.isNaN(log(NaN,0))&&Number.isNaN(negative(NaN))&&Number.isNaN(exp(NaN)),'nonfinite domains');check(negative(-Infinity)===0&&negative(Infinity)===1&&exp(Infinity)===Infinity&&negative(-0)===1,'source saturation endpoints');
        for(const code of [2,0xffffffff])throws(()=>new Owner(code),'invalid table selector');
        const survivor=new Owner(0),iterator=survivor.iter();survivor.free();check(iterator.next()!==undefined,'iterator independent handle lifetime');iterator.free();
    }
    check(api.numerical_log_add_f64(0,0)!==api.numerical_log_add_f32(0,0),'double table precision retained');const precise=1+Number.EPSILON;check(api.numerical_exponential_f64(precise)===api.numerical_exponential_f32(Math.fround(precise)),'float-only positive approximation');
    return{cases,coefficientComparisons};
}
