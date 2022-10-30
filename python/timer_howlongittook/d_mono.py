#!/usr/bin/python3

#for https://stackoverflow.com/a/73727392/19999437



import time as t
WORST="() highest time"
AVG="() average time"
BEST="() lowest time"

stats: dict[str,float]=dict({})

def TimeTakenDecorator(func):
    #src: https://stackoverflow.com/a/70954147/19999437
    def wraper(*args,**kwargs):
        global stats
        tmp_avg=stats.get(func.__name__+AVG)
        #too_fast:bool
        if (None == tmp_avg) or (tmp_avg > 0.5 ): #500000000):
            print(f'Calling "{func.__name__}()"') #,end="")
            #too_fast=False
        #else:
            #too_fast=True
        #start = t.monotonic_ns()
        start = t.monotonic()
        func(*args,**kwargs)
        end = t.monotonic()
        #end = t.monotonic_ns()
        diff=end - start
        if None == stats.get(func.__name__+WORST):
            stats[func.__name__+WORST]=diff
        if None == tmp_avg:
            stats[func.__name__+AVG]=diff
            tmp_avg=diff
        if None == stats.get(func.__name__+BEST):
            stats[func.__name__+BEST]=diff
        if diff > stats[func.__name__+WORST]:
            stats[func.__name__+WORST]=diff
        if diff < stats[func.__name__+BEST]:
            stats[func.__name__+BEST]=diff
        stats[func.__name__+AVG]=(tmp_avg + diff) /2
        if diff > 0.5: #500000000:
            print(f'Elapsed time for function call "{func.__name__}": {diff:.20f}')
            #print(" "+str(diff))
        #if not too_fast:
        #    print()
    return wraper

something=5_234_567
REPEATS=1_000_000

#init_set() highest time==========================5.374_412_298_202_514_648_438_
#init_set() average time==========================5.374_412_298_202_514_648_438_
#init_set() lowest time===========================5.374_412_298_202_514_648_438_
@TimeTakenDecorator
def init_set():
    #print("Initializing the set:")
    global g1_set
    g1_set = set()
    for i in range(10_000_000):
        g1_set.add(i)

#re_set() highest time============================1.283_961_534_500_122_070_312_
#re_set() average time============================1.233_813_919_126_987_457_275_
#re_set() lowest time=============================1.027_379_512_786_865_234_375_
@TimeTakenDecorator
def re_set():
    #print("Resetting the set:")
    global g2_set
    global g1_set
    g2_set=g1_set.copy()

#double_lookup_many() highest time================9.843_203_306_198_120_117_188_
#double_lookup_many() average time================9.723_606_467_247_009_277_344_
#double_lookup_many() lowest time=================9.604_009_628_295_898_437_500_
@TimeTakenDecorator
def double_lookup_many():
    #print("Using double lookup:")
    for i in range(REPEATS):
        double_lookup_never_add()

#double_lookup_always_add_many() highest time====10.278_126_478_195_190_429_688_
#double_lookup_always_add_many() average time====10.234_028_577_804_565_429_688_
#double_lookup_always_add_many() lowest time=====10.189_930_677_413_940_429_688_
@TimeTakenDecorator
def double_lookup_always_add_many():
    #print("Using double lookup:")
    for i in range(REPEATS):
        double_lookup_always_add()

#using_len_many() highest time===================10.652_944_326_400_756_835_938_
#using_len_many() average time===================10.642_948_746_681_213_378_906_
#using_len_many() lowest time====================10.632_953_166_961_669_921_875_
@TimeTakenDecorator
def using_len_many():
    #print("Using len():")
    for i in range(REPEATS):
        use_len_and_always_add()

#double_lookup_never_add() highest time===========0.000_107_288_360_595_703_125_
#double_lookup_never_add() average time===========0.000_001_333_327_164_114_879_
#double_lookup_never_add() lowest time============0.000_000_953_674_316_406_250_
@TimeTakenDecorator
def double_lookup_never_add():
    global g2_set
    if something not in g2_set:
        g2_set.add(something)
        #pass
        #print(f"The element({something}) was added therefore it was not already in the set.")
    #else:
        #g2_set.add(something)
        #pass
        #print(f"The element({something}) was not added because it was already in the set.")

#double_lookup_always_add() highest time==========0.000_087_261_199_951_171_875_
#double_lookup_always_add() average time==========0.000_001_681_037_948_399_603_
#double_lookup_always_add() lowest time===========0.000_001_192_092_895_507_812_
@TimeTakenDecorator
def double_lookup_always_add():
    global g2_set
    if something not in g2_set:
        g2_set.add(something)
    else:
        g2_set.add(something)

#use_len_and_always_add() highest time============0.000_089_168_548_583_984_375_
#use_len_and_always_add() average time============0.000_002_022_699_288_719_765_
#use_len_and_always_add() lowest time=============0.000_001_668_930_053_710_938_
@TimeTakenDecorator
def use_len_and_always_add():
    global g2_set
    pre_len = len(g2_set)
    g2_set.add(something)
    #pass
    if pre_len != len(g2_set):
        pass
        #print(f"The element({something}) was added therefore it was not already in the set.")
    #else:
    #    pass
        #print(f"The element({something}) was not added because it was already in the set.")

#double_lookup_never_add2() highest time==========0.000_120_401_382_446_289_062_
#double_lookup_never_add2() average time==========0.000_001_423_196_238_256_303_
#double_lookup_never_add2() lowest time===========0.000_001_192_092_895_507_812_
@TimeTakenDecorator
def double_lookup_never_add2():
    global g2_set
    if something not in g2_set:
        g2_set.add(something)

#double_lookup_always_add_many2() highest time===10.584_211_587_905_883_789_062_
#double_lookup_always_add_many2() average time===10.565_821_886_062_622_070_312_
#double_lookup_always_add_many2() lowest time====10.547_432_184_219_360_351_562_
@TimeTakenDecorator
def double_lookup_always_add_many2():
    global g2_set
    for i in range(REPEATS):
        g2_set.clear()
        double_lookup_never_add2()


def main():
    init_set()
    re_set()
    using_len_many()
    re_set()
    double_lookup_many()
    re_set()
    double_lookup_always_add_many()
    re_set()
    double_lookup_always_add_many2()
    print("Once more in reverse order:")
    re_set()
    double_lookup_always_add_many2()
    re_set()
    double_lookup_always_add_many()
    re_set()
    double_lookup_many()
    re_set()
    using_len_many()
    import json
    #from json import encoder
    #encoder.FLOAT_REPR = lambda o: f'{o:.20f}' #.format(o) #format(o, '.20f')
    #src: https://stackoverflow.com/a/69056325/19999437
    class RoundingFloat(float):
        __repr__ = staticmethod(lambda x: format(x, '.30f'))

    json.encoder.c_make_encoder = None
    #if hasattr(json.encoder, 'FLOAT_REPR'):
    #    # Python 2
    #    json.encoder.FLOAT_REPR = RoundingFloat.__repr__
    #else:
        # Python 3
    json.encoder.float = RoundingFloat
    print(json.dumps(stats, sort_keys=False, indent=2)) #, parse_float=lambda x: f"{x:.20f}"))
    import re
    for k in stats:
        time_form=re.sub(r'([0-9_]+\.)?([0-9]{3})', '\\1\\2_', f"{stats[k]:_.21f}")
        #print(f"{k:-<45} {stats[k]:.20f}")
        print(f"{k:=<45}={time_form:=>33}")


main();
